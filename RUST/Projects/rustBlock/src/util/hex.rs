// see rustc-serialize

pub use self::FromHexError::*;

use std::fmt;
use std::error;

pub trait ToHex {
    fn to_hex(&self) -> String;
}

static CHARS: &'static[u8] = b"0123456789abcdef";

impl ToHex for [u8] {
    fn to_hex(&self) -> String {
        let mut v = Vec::with_capacity(self.len() * 2);
        for &byte in self.iter() {
            v.push(CHARS[(byte >> 4) as usize]);
            v.push(CHARS[(byte & 0xf) as usize]);
        }

        unsafe {
            String::from_utf8_unchecked(v)
        }
    }
}

pub trait FromHex {
    fn from_hex(&self) -> Result<Vec<u8>, FromHexError>;
}

#[derive(Copy, Clone)]
pub enum FromHexError {
    InvalidHexCharacter(char, usize),
    InvalidHexLength,
}

impl fmt::Debug for FromHexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InvalidHexCharacter(ch, idx) =>
                write!(f, "Invalid character '{}' at position {}", ch, idx),
            InvalidHexLength => write!(f, "Invalid input length"),
        }
    }
}

impl error::Error for FromHexError {
    fn description(&self) -> &str {
        match *self {
            InvalidHexCharacter(_, _) => "invalid character",
            InvalidHexLength => "invalid length",
        }
    }
}

impl fmt::Display for FromHexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl FromHex for str {
    fn from_hex(&self) -> Result<Vec<u8>, FromHexError> {
        // This may be an overestimate if there is any whitespace
        let mut b = Vec::with_capacity(self.len() / 2);
        let mut modulus = 0;
        let mut buf = 0;

        for (idx, byte) in self.bytes().enumerate() {
            buf <<= 4;

            match byte {
                b'A'...b'F' => buf |= byte - b'A' + 10,
                b'a'...b'f' => buf |= byte - b'a' + 10,
                b'0'...b'9' => buf |= byte - b'0',
                b' '|b'\r'|b'\n'|b'\t' => {
                    buf >>= 4;
                    continue
                }
                _ => {
                    let ch = self[idx..].chars().next().unwrap();
                    return Err(InvalidHexCharacter(ch, idx))
                }
            }

            modulus += 1;
            if modulus == 2 {
                modulus = 0;
                b.push(buf);
            }
        }

        match modulus {
            0 => Ok(b.into_iter().collect()),
            _ => Err(InvalidHexLength),
        }
    }
}
