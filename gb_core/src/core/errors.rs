use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    OPCodeParseError,
    CartridgeCheckSumError,
    CartridgeFileHeaderError,
    CartridgeAddressError,
    CartridgeTypeUnsupported,
    IO(io::Error),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IO(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::OPCodeParseError => write!(f, "Failed to parse OPCode"),
            Error::CartridgeCheckSumError => write!(f, "CheckSum Error in Cartridge"),
            Error::IO(error) => write!(f, "IO Error Occurred: {}", error),
            Error::CartridgeFileHeaderError => write!(f, "The Cartridge File Header is invalid"),
            Error::CartridgeAddressError => write!(f, "The Cartridge Address is invalid"),
            Error::CartridgeTypeUnsupported => write!(f, "The Cartridge Type is not supported"),
            // _ => write!(f, "Unknown Error"),
        }
    }
}
