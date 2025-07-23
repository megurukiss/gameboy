use crate::core::Error;
use std::result::Result;

const IO_REGISTERS_START: u16 = 0xFF00;
const IO_REGISTERS_END: u16 = 0xFF7F;

pub struct IOResgisters {
    // The registers used to control io.
    // memory map range from 0xFF00 - 0xFF7F
    boot_rom_enabled: u8, // 0xFF50
    registers: Vec<u8>,   // 0xFF00 - 0xFF7F
}

impl IOResgisters {
    pub fn new() -> IOResgisters {
        IOResgisters {
            boot_rom_enabled: 0,
            registers: vec![0; 0x7F],
        }
    }

    fn write_boot_rom_enabled(&mut self, value: u8) {
        if value == 0 {
            self.boot_rom_enabled = 0;
        } else {
            self.boot_rom_enabled = 1;
        }
    }

    fn read_boot_rom_enabled(&self) -> u8 {
        self.boot_rom_enabled
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF50 => self.write_boot_rom_enabled(value),
            _ => {}
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF50 => self.read_boot_rom_enabled(),
            _ => 0,
        }
    }
}
