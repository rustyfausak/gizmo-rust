use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::result;

pub use replay::{Replay};

pub mod replay;
pub mod reader;

// A convenience type for representing the result of most replay operations.
pub type Result<T> = result::Result<T, Error>;

// An error produced by an operation on the replay.
#[derive(Debug)]
pub enum Error {
    Io(io::Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "{}", err),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(..) => "Replay IO error",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Io(ref err) => Some(err),
            //_ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error { Error::Io(err) }
}

#[test]
fn it_works() {
}
