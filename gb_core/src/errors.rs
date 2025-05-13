use std::fmt;

#[derive(Debug)]
pub enum Error {
    OPCodeParseError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::OPCodeParseError => write!(f, "Failed to parse OPCode"),
        }
    }
}
