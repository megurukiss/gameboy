use crate::cartridge::Cartridge;
use crate::io_registers::IOResgisters;

const VRAM_START: u16 = 0x8000;
const WRAM_START: u16 = 0xC000;
const OAM_START: u16 = 0xFE00;
// const IO_REGISTERS_START: u16 = 0xFF00;
const HRAM_START: u16 = 0xFF80;
pub const IF: u16 = 0xFF0F;
pub const IE: u16 = 0xFFFF;

pub struct MemoryBus {
    // memory: Box<[u8; 0x10000]>,
    // 0x0000 - 0x00FF is Boot ROM, write to this area is ignored
    cartridge: Option<Box<dyn Cartridge>>, // 0x0000 - 0x7FFF ROM, A000 - BFFF RAM
    vram: Box<[u8; 0x2000]>,               // 0x8000 - 0x9FFF (Video RAM, stores tile data)
    wram: Box<[u8; 0x2000]>,               // 0xC000 - 0xDFFF
    // Echo RAM: 0xE000 - 0xFDFF
    oam: Box<[u8; 0xA0]>, // 0xFE00 - 0xFE9F (Object Attribute Memory, stores sprite data)
    // Unused: 0xFEA0 - 0xFEFF
    io_registers: IOResgisters, // 0xFF00 - 0xFF7F
    hram: Box<[u8; 0x7F]>,      // 0xFF80 - 0xFFFE
    interrupt_enable: u8,       // 0xFFFF
}

impl MemoryBus {
    pub fn new() -> Self {
        Self {
            // memory: Box::new([0; 0x10000]),
            cartridge: None,
            vram: Box::new([0; 0x2000]),
            wram: Box::new([0; 0x2000]),
            oam: Box::new([0; 0xA0]),
            io_registers: IOResgisters::new(),
            hram: Box::new([0; 0x7F]),
            interrupt_enable: 0,
        }
    }

    pub fn load_cartridge(&mut self, cartridge: Box<dyn Cartridge>) {
        self.cartridge = Some(cartridge);
    }

    pub fn reset(&mut self) {
        self.cartridge = None;
        self.vram = Box::new([0; 0x2000]);
        self.wram = Box::new([0; 0x2000]);
        self.oam = Box::new([0; 0xA0]);
        self.io_registers = IOResgisters::new();
        self.hram = Box::new([0; 0x7F]);
        self.interrupt_enable = 0;
    }

    // TODO: return Result
    pub fn read_byte(&self, address: u16) -> u8 {
        // self.memory[address as usize]
        match address {
            0x0000..=0x7FF | 0xA000..=0xBFFF => {
                self.cartridge.as_ref().unwrap().read_byte(address).unwrap()
            }
            0x8000..=0x9FFF => self.vram[(address - VRAM_START) as usize],
            0xC000..=0xDFFF => self.wram[(address - WRAM_START) as usize],
            0xFE00..=0xFE9F => self.oam[(address - OAM_START) as usize],
            0xFF00..=0xFF7F => self.io_registers.read_byte(address).unwrap(),
            0xFF80..=0xFFFE => self.hram[(address - HRAM_START) as usize],
            0xFFFF => self.interrupt_enable,
            _ => 0,
        }
    }
    // TODO: return Result
    pub fn write_byte(&mut self, address: u16, value: u8) {
        // self.memory[address as usize] = value;
        match address {
            0x0000..=0x7FF | 0xA000..=0xBFFF => self
                .cartridge
                .as_mut()
                .unwrap()
                .write_byte(address, value)
                .unwrap(),
            0x8000..=0x9FFF => self.vram[(address - VRAM_START) as usize] = value,
            0xC000..=0xDFFF => self.wram[(address - WRAM_START) as usize] = value,
            0xFE00..=0xFE9F => self.oam[(address - OAM_START) as usize] = value,
            0xFF00..=0xFF7F => self.io_registers.write_byte(address, value).unwrap(),
            0xFF80..=0xFFFE => self.hram[(address - HRAM_START) as usize] = value,
            0xFFFF => self.interrupt_enable = value,
            _ => (),
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        // Little-endian
        let low = self.read_byte(address) as u16;
        let high = self.read_byte(address.wrapping_add(1)) as u16;
        (high << 8) | low
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        // Little-endian
        // self.memory[address as usize] = (value & 0xFF) as u8;
        // self.memory[(address.wrapping_add(1)) as usize] = (value >> 8) as u8;
        self.write_byte(address, (value & 0xFF) as u8);
        self.write_byte(address.wrapping_add(1), (value >> 8) as u8);
    }
}
