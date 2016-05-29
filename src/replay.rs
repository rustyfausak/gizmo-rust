use super::reader;
use std::io;
use std::fs;
use std::path::Path;

use {
    Error, Result
};

pub struct Replay<R> {
    reader: R
}

impl<R: io::Read> Replay<R> {
    pub fn from_reader(reader: R) -> Replay<R> {
        Replay {
            reader: reader
        }
    }
}

impl Replay<fs::File> {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Replay<fs::File>> {
        Ok(Replay::from_reader(try!(fs::File::open(path))))
    }
}

/*
impl Replay {
    pub fn new(path: &str) -> Result<Replay, io::Error> {
        let mut f = try!(File::open(path));
        let mut buffer = Vec::new();

        // read the whole file
        let n = try!(f.read_to_end(&mut buffer));
        println!("{} bytes placed into buffer", n);

        println!("{}", reader::hello());

        let x: u8 = buffer[0];
        println!("{:b}", x);

        Ok(Replay {
            path: path
        })
    }
}
*/
