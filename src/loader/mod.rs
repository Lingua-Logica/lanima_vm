use std::{error::Error, fs};

use bincode::{Decode, Encode, config, decode_from_slice};

use crate::{loader::constants::{BYTECODE_HEAD, BYTECODE_HEAD2, VERSION}, vm::code::op_code::Op};

pub mod constants;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
pub struct Header {
    pub head: [u8; 4],
    pub ver: u32,
}

pub fn init_loader(file_path: &str) -> Result<Loader, Box<dyn Error>> {
    let contents = fs::read(file_path)?;

    let header: (Header, usize) = decode_from_slice(&contents, config::legacy().with_big_endian())?;

    Ok(Loader {
        contents: contents[header.1..].to_vec(),
        header: header.0,
    })
}

pub struct Loader {
    contents: Vec<u8>,
    header: Header,
}

impl Loader {
    pub fn check_header(&self) -> bool {
        (self.header.head == BYTECODE_HEAD || self.header.head == BYTECODE_HEAD2)
            && self.header.ver == VERSION
    }

    pub fn load(&self) -> Result<Vec<Op>, Box<dyn Error>> {
        if !self.check_header() {
            return Err(String::from("invalid header").into())
        }

        let instructions: (Vec<Op>, _) = decode_from_slice(
            &self.contents,
            config::legacy().with_big_endian()
        )?;

        Ok(instructions.0)
    }
}
