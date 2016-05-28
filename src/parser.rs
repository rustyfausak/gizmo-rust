use super::reader;
use std::io;
use std::io::prelude::*;
use std::fs::File;

pub fn parse(path: &str) -> io::Result<String> {
    let mut f = try!(File::open(path));
    let mut buffer = Vec::new();

    // read the whole file
    let n = try!(f.read_to_end(&mut buffer));
    println!("{} bytes placed into buffer", n);

    println!("{}", reader::hello());

    let x: u8 = buffer[0];
    println!("{:b}", x);

    Ok("Success".to_string())
}
