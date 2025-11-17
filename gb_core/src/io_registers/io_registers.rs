use crate::core::Error;
use std::result::Result;

const IO_REGISTERS_START: usize = 0xFF00;
const IO_REGISTERS_END: usize = 0xFF7F;

const BOOT_ROM_ENABLE: usize = 0xFF50;
const LCDC: usize = 0xFF40;

pub struct IOResgisters {
    // The registers used to control io.
    // memory map range from 0xFF00 - 0xFF7F
    registers: Vec<u8>, // 0xFF00 - 0xFF7F
}

impl IOResgisters {
    pub fn new() -> IOResgisters {
        IOResgisters {
            registers: vec![0; 0x7F],
        }
    }

    pub fn boot_rom_enabled(&self) -> bool {
        // 0 when boot rom enabled
        // non-zero when disabled
        self.registers[BOOT_ROM_ENABLE - IO_REGISTERS_START] == 0
    }

    pub fn lcdc(&self) -> [bool; 8] {
        let lcdc = self.registers[LCDC - IO_REGISTERS_START];
        let mut result = [false; 8];
        for i in 0..8 {
            result[i] = (lcdc >> (7 - i)) & 1 == 1;
        }

        result
    }

    pub fn write_byte(&mut self, address: u16, value: u8) -> Result<(), Error> {
        match address as usize {
            IO_REGISTERS_START..=IO_REGISTERS_END => {
                self.registers[address as usize - IO_REGISTERS_START] = value
            }
            _ => return Err(Error::IORegisterAddressError),
        }
        Ok(())
    }

    pub fn read_byte(&self, address: u16) -> Result<u8, Error> {
        match address as usize {
            IO_REGISTERS_START..=IO_REGISTERS_END => {
                Ok(self.registers[address as usize - IO_REGISTERS_START])
            }

            _ => Err(Error::IORegisterAddressError),
        }
    }
}
