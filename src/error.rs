use std::result;

#[derive(Debug)]
pub enum Error {}
pub type Result<T> = result::Result<T, Error>;