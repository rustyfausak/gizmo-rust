use super::reader;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use {
    Error, Result
};

pub struct Replay {
    bytes: Vec<u8>
}

impl Replay {
    pub fn from_bytes(bytes: Vec<u8>) -> Replay {
        Replay {
            bytes: bytes
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Replay> {
        let mut f = try!(File::open(path));
        let mut bytes = Vec::new();

        // read the whole file
        let n = try!(f.read_to_end(&mut bytes));
        println!("{} bytes placed into buffer", n);

        println!("{}", reader::hello());

        let x: u8 = bytes[0];
        println!("{:b}", x);

        Ok(Replay::from_bytes(bytes))
    }
}
