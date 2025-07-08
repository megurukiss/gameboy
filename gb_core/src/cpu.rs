use crate::memory::*;
use crate::time::Timer;
use std::time::{Duration, Instant};

pub struct CPU {
    // Registers
    pub a: u8, // Flags
    pub b: u8, 
    pub c: u8,
    pub d: u8,
    pub e: u8,
    // Accumulator
    pub f: u8,
    pub h: u8,
    pub l: u8,
    // Stack Pointer
    pub pc: u16, pub sp: u16, pub ime: bool,
    pub is_halted: bool,
    // Program Counter
    pub memory_bus: MemoryBus,
    pub timer: Timer,
}

struct ALU; // Arithmetic Logic Unit
struct IDU; // Instruction Decode Unit

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            memory_bus: MemoryBus::new(),
            timer: Timer::new(),
            ime: false,
            is_halted: false,
        }
    }

    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    pub fn HL(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_HL(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }

    pub fn set_SP(&mut self, value: u16) {
        self.sp = value;
    }

    // set flags
    pub fn set_Z(&mut self, value: bool) {
        match value {
            true => self.f |= 0b1000_0000,
            false => self.f &= 0b0111_1111,
        }
    }
    pub fn Z(&self) -> bool {
        self.f & 0b1000_0000 != 0
    }

    // N flag
    pub fn set_N(&mut self, value: bool) {
        match value {
            true => self.f |= 0b0100_0000,
            false => self.f &= 0b1011_1111,
        }
    }
    pub fn N(&self) -> bool {
        self.f & 0b0100_0000 != 0
    }

    // H flag
    pub fn set_H(&mut self, value: bool) {
        match value {
            true => self.f |= 0b0010_0000,
            false => self.f &= 0b1101_1111,
        }
    }
    pub fn H(&self) -> bool {
        self.f & 0b0010_0000 != 0
    }

    // C flag
    pub fn set_C(&mut self, value: bool) {
        match value {
            true => self.f |= 0b0001_0000,
            false => self.f &= 0b1110_1111,
        }
    }
    pub fn C(&self) -> bool {
        self.f & 0b0001_0000 != 0
    }

    pub fn set_ime(&mut self, value: bool) {
        self.ime = value;
    }
    pub fn ime(&mut self) -> bool {
        self.ime
    }

    // get IF
    pub fn IF(&self) -> u8 {
        self.memory_bus.read_byte(IF)
    }

    // get IE
    pub fn IE(&self) -> u8 {
        self.memory_bus.read_byte(IE)
    }

    // compute elasped time and call update_cpu
    pub fn update(&mut self) {
        let now = Instant::now();
        let mut elapsed_time = Duration::new(0, 0);
        if let Some(last_called_time) = self.timer.last_called_time {
            elapsed_time = now.duration_since(last_called_time);
        }
        self.timer.last_called_time = Some(now);

        // update cpu
        self.update_cpu(elapsed_time);
    }

    // compute cycles to complete and call tick
    fn update_cpu(&mut self, elapsed_time: Duration) {
        if elapsed_time.is_zero() {
            // first call
            return;
        }
        let elapsed_time = elapsed_time.as_secs_f64();
        let freq = self.timer.get_frequency() as f64;
        let scale = self.timer.get_scale() as f64;
        // calculate cycles
        let cycles = (elapsed_time * freq * scale) as u64; // clock cycles
        let end_cycles = self.timer.cycles_counter + cycles;
        while self.timer.cycles_counter < end_cycles {
            // execute instruction and increment cycles
            let delta_cycles = self.tick();
            self.timer.update_cycles(delta_cycles);
        }
    }

    // fetch-decode-execute cycle, return cycles taken
    // be careful about CB prefix, if CB prefix encountered, fetch the next bit manipulation opcode.
    fn tick(&mut self) -> u64 {
        // TODO: Implement fetch-decode-execute cycle
        if !self.is_halted {
            // fetch and execute instruction
        } else {
            // cpu halted
        }

        // check and handle interrupts
        // unset the is_halted when interrupt
        // when is_halted is set and ime = false, interrupt handler is not called.
        0
    }
}
