use super::*;
use crate::{core::Error, implement_cartridge_getters};

#[derive(Debug)]
pub struct MBC1 {
    /*
    Small games of not more than 32 KiB ROM do not require a MBC chip for ROM banking.
    The ROM is directly mapped to memory at $0000-7FFF.
    Optionally up to 8 KiB of RAM could be connected at $A000-BFFF
    using a discrete logic decoder in place of a full MBC chip.
    */
    rom: Vec<u8>,
    ram: Vec<u8>,
    header: CartridgeHeader,
}

impl Cartridge for MBC1 {
    fn read_byte(&self, address: u16) -> Result<u8, Error> {
        unimplemented!()
    }

    fn write_byte(&mut self, address: u16, value: u8) -> Result<(), Error> {
        unimplemented!()
    }

    fn from_bytes(bytes: Vec<u8>) -> Result<Self, Error>
    where
        Self: Cartridge + Sized,
    {
        unimplemented!()
    }
    implement_cartridge_getters!();
}
