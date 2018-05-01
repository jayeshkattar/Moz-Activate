extern crate serde_yaml;

use std::collections::LinkedList;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::{Read, Write};
use std::path::Path;

use blockchain::Block;
use util::hash::{Hash256, HASH256_BYTES};
use util::hex::ToHex;


pub struct BlockChain {
    //block_map: HashMap<BlockHash, Block>,
    chain: LinkedList<Box<Block>>,
}

impl BlockChain {
    pub fn new() -> BlockChain {
        BlockChain {
            //block_map: HashMap::new(),
            chain: LinkedList::new(),
        }
    }

    fn append(&mut self, block: Box<Block>) -> Result<(), String> {
        match self.chain.back() {
            Some(tail) => {
                let mut previous_hash = [0u8; HASH256_BYTES];
                tail.get_hash(&mut previous_hash);
                if previous_hash != block.get_previous() {
                    return Err(format!("append expected previous '{}'; actual '{}'",
                                       previous_hash.to_hex(), block.get_previous().to_hex()));
                }
            },
            None => {
                let expected = [0u8; HASH256_BYTES];
                if block.get_previous() != expected {
                    return Err(format!("append expected previous '{}'; actual '{}'",
                                       expected.to_hex(), block.get_previous().to_hex()));
                }
            },
        }

        self.chain.push_back(block);
        Ok(())
    }

    /*
    pub fn write_chain(&self, dir: &Path) {
        let block_file = dir.join("blocks.yaml");

        let mut f = match fs::File::create(block_file.as_path()) {
            Ok(f) => f,
            Err(e) => panic!("open file error: {}", e),
        };

        for ref block in self.chain.iter() {
            let serialized = serde_yaml::to_string(block).unwrap();
            write!(f, "{}\n", serialized).unwrap();
        }
    }
    */

    pub fn read_chain(&mut self, file: &Path) {
        let f = File::open(file).unwrap();
        let mut reader = BufReader::new(f);
        let mut contents = String::new();
        reader.read_to_string(&mut contents).unwrap();

        let yaml_blocks: Vec<&str> = contents.split("---").collect();
        for yaml_block in &yaml_blocks[1..] {
            let block: Box<Block> = Box::new(serde_yaml::from_str(&yaml_block).unwrap());
            let result = self.append(block);
            match result {
                Ok(_) => {},
                Err(e) => {
                    writeln!(&mut io::stderr(), "read_chain: {}", e);
                    return;
                }
            }
        }
    }
}

impl fmt::Display for BlockChain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, ref block) in self.chain.iter().enumerate() {
            let mut hash = [0u8; HASH256_BYTES];
            block.get_hash(&mut hash);
            write!(f, "{:08}: {}\n", i, hash.to_hex())?;
        }

        Ok(())
    }
}
