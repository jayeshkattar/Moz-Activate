extern crate blake2;

use std::io;
use self::blake2::{Blake2s, Digest};


pub const HASH256_BYTES: usize = 32;

pub struct Hash256 {
    hasher: Blake2s,
}

impl Hash256 {
    pub fn new() -> Hash256 {
        Hash256 {
            hasher: Blake2s::default(),
        }
    }

    pub fn finalize(&mut self, buf: &mut [u8]) {
        let result = self.hasher.result();
        buf.copy_from_slice(&result)
    }

    pub fn reset(&mut self) {
        self.hasher = Blake2s::default();
    }
}

impl io::Write for Hash256 {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.hasher.input(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
