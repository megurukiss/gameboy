use super::*;
use crate::{core::Error, implement_cartridge_getters};

#[derive(Debug)]
pub struct MBC1Cartridge {
    /*
    supports up to 512 kb rom, which equals to 32 rom banks
    supports up to 32 kb ram, which equals to 4 ram banks
    some cartridges supports different wire method, which uses up to 2mb.
    bank switch behaviors
    0000 - 1FFF: write 0x0A will enable the RAM, any other values will disable it, RAM is disabled by default.
    2000 - 3FFF: lower 5 bits (0x01-0x1F) select the bank number, if set to 0x00, it will be set as 0x01.
                range decided by rom size, if larger than max rom size, upper bits will be ignored.
    4000 - 5FFF: lower 2 bits used to select bank number. For rom larger than 1mb, 5-6 bits will be used.
    */
    rom: Vec<Vec<u8>>,
    ram: Vec<Vec<u8>>,
    header: CartridgeHeader,
    rom_idx: u8,
    ram_idx: u8,
    ram_enabled: bool,
    bank_mode: bool,
}

impl MBC1Cartridge {
    fn get_ram_len(&self) -> usize {
        self.ram.len()
    }

    fn get_rom_idx(&self) -> u8 {
        self.rom_idx
    }

    fn get_ram_idx(&self) -> u8 {
        self.ram_idx
    }
}

// TODO: support 1mb+ cartridge
impl Cartridge for MBC1Cartridge {
    fn read_byte(&self, address: u16) -> Result<u8, Error> {
        let val = match address {
            0x0000..=0x3FFF => self.rom[0][address as usize],
            0x4000..=0x7FFF => self.rom[self.rom_idx as usize][address as usize - 0x4000],
            0xA000..=0xBFFF => {
                if self.ram_enabled && self.get_ram_len() > 0 {
                    self.ram[self.ram_idx as usize][address as usize - 0xA000]
                } else {
                    OPENBUS
                }
            }
            _ => return Err(Error::CartridgeAddressError),
        };
        Ok(val)
    }

    fn write_byte(&mut self, address: u16, value: u8) -> Result<(), Error> {
        // write byte to ROM will change bank index
        match address {
            0x0000..=0x1FFF => {
                if value == 0x0A {
                    self.ram_enabled = true;
                } else {
                    self.ram_enabled = false;
                }
            }
            0x2000..=0x3FFF => {
                let masked_index = {
                    let rom_bank_numbers = self.get_rom().len();
                    (value & 0x1F) as usize % rom_bank_numbers
                };
                self.rom_idx = if masked_index == 0 {
                    1
                } else {
                    masked_index as u8
                };
                // large bank numbers > 5 will require additional 2 bits as index
            }
            0x4000..=0x5FFF => {
                let masked_index = {
                    let ram_bank_number = self.get_ram().len();
                    if ram_bank_number != 0 {
                        (value & 0x03) as usize % ram_bank_number
                    } else {
                        0
                    }
                };
                self.ram_idx = masked_index as u8;
                // TODO: bit 4,5 will be used when rom > 1mb
            }
            0x6000..=0x7FFF => {
                // switch banking mode
                // 0: default, 0000-3FFF, A000–BFFF are locked to bank 0 and SRAM
                // 1: advanced, 0000-3FFF, A000-BFFF can be switched via 4000-5FFF registers
                self.bank_mode = (value & 0x01) == 0x01;
            }
            _ => return Err(Error::CartridgeAddressError),
        }
        Ok(())
    }

    fn from_bytes(bytes: Vec<u8>) -> Result<Self, Error> {
        let header = CartridgeHeader::from_bytes(&bytes)?;
        // compute bank numbers
        let rom_bank_numbers = {
            let rom_size = header.rom_size / (1 << 10) / 16;
            rom_size as usize
        };
        let ram_bank_numbers = {
            let ram_size = header.ram_size / (1 << 10) / 8;
            ram_size as usize
        };
        let mut rom: Vec<Vec<u8>> = Vec::with_capacity(rom_bank_numbers);
        let ram = vec![vec![0; 8 * (1 << 10)]; ram_bank_numbers];
        for i in 0..rom_bank_numbers {
            rom.push(bytes[0x4000 * i..0x4000 * (i + 1)].to_vec());
        }

        Ok(Self {
            rom,
            ram,
            header,
            rom_idx: 1,
            ram_idx: 0,
            ram_enabled: false,
            bank_mode: false,
        })
    }

    implement_cartridge_getters!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::debug;

    #[test]
    #[test_log::test]
    fn test_mbc1_from_bytes() {
        let path = "../cpu_instrs/cpu_instrs.gb";
        let raw_bytes = read_file(path).unwrap();
        let mut cartridge = MBC1Cartridge::from_bytes(raw_bytes).unwrap();
        debug!("{:?}", cartridge.get_header());
        assert_eq!(cartridge.get_rom().len(), 4);
        assert_eq!(cartridge.get_ram().len(), 0);
    }

    #[test]
    #[test_log::test]
    fn test_mbc1_io_without_bank_switch() {
        let path = "../cpu_instrs/cpu_instrs.gb";
        let raw_bytes = read_file(path).unwrap();
        // debug!("{:?}", raw_bytes[0x9000]);
        let mut cartridge = MBC1Cartridge::from_bytes(raw_bytes).unwrap();
        debug!("{:?}", cartridge.get_header());
        assert_eq!(cartridge.read_byte(0x0148).unwrap(), 0x01);
        assert_eq!(cartridge.read_byte(0x3000).unwrap(), 0x00);

        // switch to rom bank 2, then read 0x9000, equal to 195
        cartridge.write_byte(0x2000, 0x02).unwrap();
        assert_eq!(cartridge.read_byte(0x5000).unwrap(), 195);
        assert_eq!(cartridge.get_rom_idx(), 2);

        assert_eq!(cartridge.read_byte(0xA001).unwrap(), 0xFF);
    }
}
