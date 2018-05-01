extern crate byteorder;
extern crate serde;

use std::fmt;
use std::io::Write;

use self::byteorder::{LittleEndian, WriteBytesExt};
//use self::serde::ser::{Serialize, Serializer};
use self::serde::de::{Visitor, Deserialize, Deserializer, MapAccess, SeqAccess};

use blockchain::Transaction;
use util::hash::{Hash256, HASH256_BYTES};
use util::hex::{FromHex, ToHex};


//#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct Block {
    version: u32,
    timestamp: u64,
    previous: [u8; HASH256_BYTES],
    merkle_root: [u8; HASH256_BYTES],
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new() -> Block {
        Block {
            version: 1,
            timestamp: 0,
            previous: [0u8; HASH256_BYTES],
            merkle_root: [0u8; HASH256_BYTES],
            transactions: Vec::new(),
        }
    }

    pub fn set_timestamp(&mut self, ts: u64) {
        self.timestamp = ts;
    }

    pub fn set_previous(&mut self, p: &[u8; HASH256_BYTES]) {
        self.previous.copy_from_slice(p);
    }

    pub fn get_previous(&self) -> &[u8] {
        &self.previous
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        self.transactions.push(tx);
    }

    pub fn get_hash(&self, mut buf: &mut [u8]) {
        let mut hash = Hash256::new();

        hash.write_u32::<LittleEndian>(self.version).unwrap();
        hash.write_u64::<LittleEndian>(self.timestamp).unwrap();
        hash.write_all(&self.previous).unwrap();
        hash.write_all(&self.merkle_root).unwrap();

        hash.finalize(&mut buf);
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut hash = [0u8; HASH256_BYTES];
        self.get_hash(&mut hash);

        write!(f, "block _hash: {}\n", hash.to_hex()).unwrap();
        write!(f, "version:     {}\n", self.version).unwrap();
        write!(f, "timestamp:   {}\n", self.timestamp).unwrap();
        write!(f, "previous:    {}\n", self.previous.to_hex()).unwrap();
        write!(f, "merkle_root: {}\n", self.merkle_root.to_hex()).unwrap();
        write!(f, "transactions:\n").unwrap();
        for tx in &self.transactions {
            write!(f, "{}", tx).unwrap();
        }
        write!(f, "\n")
    }
}

impl<'de> Deserialize<'de> for Block {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field { Version, Timestamp, Previous, Merkle_root };

        struct BlockVisitor;

        impl<'de> Visitor<'de> for BlockVisitor {
            type Value = Block;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("block map")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Block, V::Error>
                where V: MapAccess<'de>
            {
                let mut version: Option<u32> = None;
                let mut timestamp: Option<u64> = None;
                let mut previous: Option<String> = None;
                let mut merkle_root: Option<String> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Version => {
                            if version.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version = Some(map.next_value()?);
                        },
                        Field::Timestamp => {
                            if timestamp.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp = Some(map.next_value()?);
                        },
                        Field::Previous => {
                            if previous.is_some() {
                                return Err(serde::de::Error::duplicate_field("previous"));
                            }
                            previous = Some(map.next_value()?);
                        },
                        Field::Merkle_root => {
                            if merkle_root.is_some() {
                                return Err(serde::de::Error::duplicate_field("merkle_root"));
                            }
                            merkle_root = Some(map.next_value()?);
                        },
                    }
                }

                let mut block = Block {
                    version: version.unwrap(),
                    timestamp: timestamp.unwrap(),
                    previous: [0u8; HASH256_BYTES],
                    merkle_root: [0u8; HASH256_BYTES],
                    transactions: Vec::new(),
                };

                let previous_vec = previous.unwrap().from_hex().unwrap();
                block.previous.copy_from_slice(&previous_vec);

                let merkle_root_vec = merkle_root.unwrap().from_hex().unwrap();
                block.merkle_root.copy_from_slice(&merkle_root_vec);

                Ok(block)
            }

        }

        const FIELDS: &'static [&'static str] = &["version", "timestamp", "previous", "merkle_root"];
        deserializer.deserialize_struct("Block", FIELDS, BlockVisitor)
    }
}
