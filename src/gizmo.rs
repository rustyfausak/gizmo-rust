/*
use std::io;
use std::fs::File;

pub fn parse(path: &str) -> io::Result<String> {
    let mut f = try!(File::open(path));

    Ok(String::new())
}
*/

pub fn hello(s: &str) -> String {
    s.to_string()
}
