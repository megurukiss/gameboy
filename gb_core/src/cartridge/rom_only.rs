use super::*;
use crate::{core::Error, implement_cartridge_getters};

#[derive(Debug)]
pub struct RomOnlyCartridge {
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

impl RomOnlyCartridge {
    pub fn new(path: &str) -> Result<Self, Error> {
        let raw_bytes = read_file(path)?;
        let header = CartridgeHeader::from_bytes(&raw_bytes)?;
        // create rom vec and load bytes
        let rom = raw_bytes;

        // create ram vec and load bytes
        let ram = vec![0u8; header.ram_size as usize];
        Ok(Self { rom, ram, header })
    }
}

impl Cartridge for RomOnlyCartridge {
    fn read_byte(&self, address: u16) -> Result<u8, Error> {
        match address {
            0x0000..=0x7FFF => {
                // 0..rom_size => rom[address]
                // rom_size..0x7FFF => 0
                if (address as u32) < self.header.rom_size {
                    Ok(self.rom[address as usize])
                } else {
                    Ok(0)
                }
            }
            0xA000..=0xBFFF => {
                if ((address - 0xA000) as u32) < self.header.ram_size {
                    Ok(self.ram[address as usize - 0xA000])
                } else {
                    Ok(0)
                }
            }
            _ => Err(Error::CartridgeAddressError),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) -> Result<(), Error> {
        match address {
            // write to rom should be prohibitted
            // 0x0000..=0x7FFF => {
            //     if (address as u32) < self.header.rom_size {
            //         self.rom[address as usize] = value;
            //     }
            // }
            0xA000..=0xBFFF => {
                if ((address - 0xA000) as u32) < self.header.ram_size {
                    self.ram[address as usize - 0xA000] = value;
                }
            }
            _ => return Err(Error::CartridgeAddressError),
        }
        Ok(())
    }

    fn from_bytes(bytes: Vec<u8>) -> Result<Self, Error> {
        let header = CartridgeHeader::from_bytes(&bytes)?;
        let rom = bytes;
        let ram = vec![0u8; header.ram_size as usize];
        Ok(Self { rom, ram, header })
    }

    implement_cartridge_getters!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::debug;

    #[test]
    #[test_log::test]
    fn test_init_cartridge() {
        let path = "../cpu_instrs/cpu_instrs.gb";
        let cartridge = RomOnlyCartridge::new(path).unwrap();
        debug!("{:?}", cartridge.header);
        assert!(cartridge.ram.len() == cartridge.header.ram_size as usize);
        debug!("The rom has {:?}kb", cartridge.rom.len() / 1024);
        assert!(cartridge.rom.len() == cartridge.header.rom_size as usize);
    }

    #[test]
    #[test_log::test]
    fn test_io() {
        let path = "../cpu_instrs/cpu_instrs.gb";
        let mut cartridge = RomOnlyCartridge::new(path).unwrap();

        matches!(
            cartridge.write_byte(0x0000, 0x12),
            Err(Error::CartridgeAddressError)
        );
        // test write requies uncomment rom write as ram size is 0
        // cartridge.write_byte(0x0000, 0x12).unwrap();
        // assert_eq!(cartridge.read_byte(0x0000).unwrap(), 0x12);
        assert_eq!(cartridge.read_byte(0x0000).unwrap(), 60);
    }

    #[test]
    #[test_log::test]
    fn test_init_cartridge_from_bytes() {
        let path = "../cpu_instrs/cpu_instrs.gb";
        let raw_bytes = read_file(path).unwrap();
        let cartridge = RomOnlyCartridge::from_bytes(raw_bytes).unwrap();
        debug!("{:?}", cartridge.header);
        assert_eq!(cartridge.ram.len(), cartridge.header.ram_size as usize);
        debug!("The rom has {:?}kb", cartridge.rom.len() / 1024);
        assert_eq!(cartridge.rom.len(), cartridge.header.rom_size as usize);
        assert_eq!(cartridge.read_byte(0x0000).unwrap(), 60);
    }
}
