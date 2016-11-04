use std::result;
use std::net;
use std::io;

#[derive(Debug)]
pub enum Error {
    AddrParseError(net::AddrParseError),
    Io(io::Error),
}

impl From<net::AddrParseError> for Error {
    fn from(other: net::AddrParseError) -> Self {
        Error::AddrParseError(other)
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

pub type Result<T> = result::Result<T, Error>;