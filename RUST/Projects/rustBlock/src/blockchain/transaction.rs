extern crate byteorder;

use std::fmt;
use std::io::Write;

use self::byteorder::{LittleEndian, WriteBytesExt};

use util::hash::{Hash256, HASH256_BYTES};
use util::hex::{FromHex, ToHex};


pub struct OutPoint {
    hash: [u8; HASH256_BYTES],
    index: u32,
}

impl OutPoint {
    fn hash(&self, hash: &mut Hash256) {
        hash.write_all(&self.hash);
        hash.write_u32::<LittleEndian>(self.index).unwrap();
    }
}

pub struct TransactionInput {
    previous_out: OutPoint,
}

impl TransactionInput {
    fn hash(&self, hash: &mut Hash256) {
        self.previous_out.hash(hash);
    }
}

pub struct TransactionOutput {
    amount: u64,
}

impl fmt::Display for TransactionOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "    amount: {}\n", self.amount)
    }
}

impl TransactionOutput {
    fn hash(&self, hash: &mut Hash256) {
        hash.write_u64::<LittleEndian>(self.amount).unwrap();
    }
}

pub struct Transaction {
    version: u32,
    timestamp: u64,
    inputs: Vec<TransactionInput>,
    outputs: Vec<TransactionOutput>,
}

impl Transaction {
    pub fn new() -> Transaction {
        Transaction {
            version: 1,
            timestamp: 0,
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn set_timestamp(&mut self, ts: u64) {
        self.timestamp = ts;
    }

    pub fn add_output(&mut self, amount: u64) {
        let output = TransactionOutput {
            amount: amount,
        };

        self.outputs.push(output);
    }

    pub fn get_hash(&self, mut buf: &mut [u8]) {
        let mut hash = Hash256::new();
        hash.write_u32::<LittleEndian>(self.version).unwrap();
        hash.write_u64::<LittleEndian>(self.timestamp).unwrap();

        for i in &self.inputs {
            i.hash(&mut hash);
        }

        for o in &self.outputs {
            o.hash(&mut hash);
        }

        hash.finalize(&mut buf);
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut hash = [0u8; HASH256_BYTES];
        self.get_hash(&mut hash);

        write!(f, "  tx _hash:    {}\n", hash.to_hex()).unwrap();
        write!(f, "  version:     {}\n", self.version).unwrap();
        write!(f, "  timestamp:   {}\n", self.timestamp).unwrap();
        write!(f, "  inputs:\n").unwrap();
        //for i in &self.inputs {
        //    write!(f, "{}", i).unwrap();
        //}

        write!(f, "  outputs:\n").unwrap();
        for o in &self.outputs {
            write!(f, "{}", o).unwrap();
        }
        write!(f, "\n")
    }
}
