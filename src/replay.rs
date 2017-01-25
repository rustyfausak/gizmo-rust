use byteorder::{ReadBytesExt, LittleEndian};

use super::game::Game;
use std::io;
use std::io::Cursor;
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
        let n = try!(f.read_to_end(&mut bytes));
        Ok(Replay::from_bytes(bytes))
    }

    pub fn parse(self) -> Result<Game> {
        let mut cursor = Cursor::new(self.bytes);
        let mut game = Game::new();
        game.size1 = cursor.read_u32::<LittleEndian>().unwrap();
        game.size2 = cursor.read_u32::<LittleEndian>().unwrap();
        Ok(game)
    }
}
