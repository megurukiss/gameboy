use crate::core::Error;
use std::result::Result;

pub trait Cartridge {
    fn read_byte(&self, address: u16) -> Result<u8, Error>;
    fn write_byte(&mut self, address: u16, value: u8) -> Result<(), Error>;

    // default methods
    fn read_word(&self, address: u16) -> Result<u16, Error> {
        let low = self.read_byte(address)? as u16;
        let high = self.read_byte(address + 1)? as u16;
        Ok(low | (high << 8))
    }

    fn write_word(&mut self, address: u16, value: u16) -> Result<(), Error> {
        self.write_byte(address, (value & 0xFF) as u8)?;
        self.write_byte(address + 1, (value >> 8) as u8)?;
        Ok(())
    }

    fn from_bytes(bytes: Vec<u8>) -> Result<Self, Error>
    where
        Self: Cartridge + Sized;

    fn get_rom(&self) -> &Vec<u8>;
    fn get_ram(&mut self) -> &mut Vec<u8>;
    fn get_header(&self) -> &CartridgeHeader;
}

#[derive(Debug)]
pub struct CartridgeHeader {
    // 0100 - 014F, the game code start from 0150
    // 0100 - 0103 entry point, usually nop + jp 0150
    // 0104 - 0133 nintendo logo
    // 0134 - 0143 tile
    // 0144 - 0145 new license code
    // 0146 sgb flag, specifies whether the game supports sgb functions
    // 0147 cartridge type
    // 0148 ROM size
    // 0149 RAM size
    // 014A destination code, japan 0x00, oversea 0x01
    // 014B old license code, if 0x33, the new license code will be used
    // 014C version number, usually 0x00
    // 014D header checksum, range 0134 - 014C
    // 014E - 014F global checksum, checksum of the ROM
    pub cgb_mode: u8, // 0143 in CGB, in older machines, it will be part of the tile
    pub sgb_flag: u8, // if not 0x03, command packets will be ignored
    pub cartridge_type: u8,
    pub rom_size: u32,
    pub ram_size: u32,
    pub destination_code: u8,
}

impl CartridgeHeader {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        // if byte length smaller than header, return error
        if bytes.len() <= 0x150 {
            return Err(Error::CartridgeFileHeaderError);
        }

        // header check sum logic
        let header_checksum = bytes[0x014d];
        // debug!("header checksum: {:04X}", header_checksum);
        let mut checksum = 0i32;
        // debug!("bytes: {:?}", &bytes[0x134..0x14d]);
        for i in 0x134..0x14d {
            checksum = checksum - (bytes[i] as i32) - 1;
        }
        // debug!("checksum: {}", checksum);
        // if the lower 8 bit of checksum doesn't match, throw error
        if ((checksum & 0xFF) as u8) != header_checksum {
            return Err(Error::CartridgeCheckSumError);
        }

        let ram_size = match bytes[0x149] {
            0x00 => 0,
            0x01 => 1 << 11,
            0x02 => 1 << 13,
            0x03 => 1 << 15,
            0x04 => 1 << 17,
            0x05 => 1 << 16,
            _ => 0,
        };

        let rom_size = match bytes[0x148] {
            0x00..0x09 => 1 << (15 + bytes[0x148]),
            _ => 0,
        };

        Ok(Self {
            cgb_mode: bytes[0x143],
            sgb_flag: bytes[0x146],
            cartridge_type: bytes[0x147],
            rom_size,
            ram_size,
            destination_code: bytes[0x14a],
        })
    }
}
