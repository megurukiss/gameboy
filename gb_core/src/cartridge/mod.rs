pub mod interface;
mod mbc1;
pub mod rom_only;

use crate::core::Error;
pub use interface::{Cartridge, CartridgeHeader};

use log::debug;
pub use mbc1::MBC1Cartridge;
pub use rom_only::RomOnlyCartridge;
use std::fs::File;
use std::io::Read;
use std::result::Result;

// when reading to an undefined region, return open bus value
pub const OPENBUS: u8 = 0xFF;

pub fn read_file(path: &str) -> Result<Vec<u8>, Error> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

// TODO: support more cartridge types
pub fn load_cartridge_from_file(path: &str) -> Result<Box<dyn Cartridge>, Error> {
    let bytes = read_file(path)?;
    // keep the owndership
    let header = CartridgeHeader::from_bytes(&bytes)?;
    debug!("{:?}", header);
    // match cartridge type
    let cartridge: Box<dyn Cartridge> = match header.cartridge_type {
        0x00 => Box::new(RomOnlyCartridge::from_bytes(bytes)?),
        0x01 => Box::new(MBC1Cartridge::from_bytes(bytes)?),
        _ => return Err(Error::CartridgeTypeUnsupported),
    };
    Ok(cartridge)
}

#[macro_export]
macro_rules! implement_cartridge_getters {
    () => {
        fn get_rom(&self) -> &Vec<Vec<u8>> {
            &self.rom
        }

        fn get_ram(&mut self) -> &mut Vec<Vec<u8>> {
            &mut self.ram
        }

        fn get_header(&self) -> &CartridgeHeader {
            &self.header
        }
    };
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use log::debug;
    use std::env;
    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom};

    #[test]
    #[test_log::test]
    fn test_load_file() {
        debug!("Current dir: {}", env::current_dir().unwrap().display());

        // Replace "your_game.gb" with the actual path to your file.
        let file_path = "../cpu_instrs/cpu_instrs.gb";

        // Open the file in read-only mode.
        let mut file = File::open(file_path).unwrap();

        file.seek(SeekFrom::Start(100)).unwrap();

        // Create a vector to store the bytes of the file.
        let mut buffer = Vec::new();

        // Read the entire file into the buffer.
        file.take(120).read_to_end(&mut buffer).unwrap();

        // Iterate over the buffer in chunks of 5 bytes.
        for chunk in buffer.chunks(5) {
            for byte in chunk {
                // debug each byte in hexadecimal format.
                debug!("{:02X} ", byte);
            }
            println!(); // Move to the next line after printing the chunk.
        }
    }

    #[test]
    #[test_log::test]
    fn test_load_header() {
        let file_path = "../cpu_instrs/cpu_instrs.gb";

        // Open the file in read-only mode.
        let mut file = File::open(file_path).unwrap();

        file.seek(SeekFrom::Start(0)).unwrap();
        let mut buffer = Vec::new();

        // Read the entire file into the buffer.
        file.read_to_end(&mut buffer).unwrap();
        debug!("{}", buffer.len());
        let header = CartridgeHeader::from_bytes(&buffer).unwrap();
        debug!("{:?}", header);
        assert!(buffer[0x0148] == 0x01);
    }

    #[test]
    #[test_log::test]
    fn test_load_rom_from_file() {
        let file_path = "../cpu_instrs/cpu_instrs.gb";
        let mut cartridge = load_cartridge_from_file(file_path).unwrap();
        debug!("{:?}", cartridge.get_header());
        debug!("{:?}", cartridge.get_rom()[0].len());
        debug!("{:?}", cartridge.get_ram().len());
        assert_eq!(
            cartridge.get_rom()[0].len() * cartridge.get_rom().len(),
            cartridge.get_header().rom_size as usize
        );
        assert_eq!(
            cartridge.get_ram().len(),
            cartridge.get_header().ram_size as usize
        );
    }
}
