#[macro_use]
extern crate serde_derive;

use std::env;
use std::path::Path;

mod util;
mod blockchain;
use blockchain::BlockChain;


fn main() {
    let mut args = env::args();
    let chain_file_arg = args.nth(1).unwrap();
    let chain_file = Path::new(&chain_file_arg);

    let mut chain = BlockChain::new();
    chain.read_chain(chain_file);
    println!("{}", chain);
}
