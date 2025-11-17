use log::info;

use super::memory::*;
use super::time::Timer;
use crate::opcodes::OPCode;
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
    pub pc: u16,
    pub sp: u16,
    pub ime: bool,
    pub is_halted: bool,
    // Program Counter
    pub memory_bus: MemoryBus,
    pub timer: Timer,
}

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

    pub fn reset(&mut self) {
        self.a = 0;
        self.f = 0;
        self.b = 0;
        self.c = 0;
        self.d = 0;
        self.e = 0;
        self.h = 0;
        self.l = 0;
        self.sp = 0;
        self.pc = 0;
        self.ime = false;
        self.is_halted = false;
        self.timer.reset();
        self.memory_bus.reset();
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

    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }

    pub fn set_sp(&mut self, value: u16) {
        self.sp = value;
    }

    // set flags
    pub fn set_z(&mut self, value: bool) {
        match value {
            true => self.f |= 0b1000_0000,
            false => self.f &= 0b0111_1111,
        }
    }
    pub fn z(&self) -> bool {
        self.f & 0b1000_0000 != 0
    }

    // N flag
    pub fn set_n(&mut self, value: bool) {
        match value {
            true => self.f |= 0b0100_0000,
            false => self.f &= 0b1011_1111,
        }
    }
    pub fn n(&self) -> bool {
        self.f & 0b0100_0000 != 0
    }

    // H flag
    pub fn set_h(&mut self, value: bool) {
        match value {
            true => self.f |= 0b0010_0000,
            false => self.f &= 0b1101_1111,
        }
    }
    pub fn h(&self) -> bool {
        self.f & 0b0010_0000 != 0
    }

    // C flag
    pub fn set_c(&mut self, value: bool) {
        match value {
            true => self.f |= 0b0001_0000,
            false => self.f &= 0b1110_1111,
        }
    }
    pub fn c(&self) -> bool {
        self.f & 0b0001_0000 != 0
    }

    pub fn set_ime(&mut self, value: bool) {
        self.ime = value;
    }
    pub fn ime(&mut self) -> bool {
        self.ime
    }

    // get IF
    pub fn r#if(&self) -> u8 {
        self.memory_bus.read_byte(IF)
    }

    // get IE
    pub fn ie(&self) -> u8 {
        self.memory_bus.read_byte(IE)
    }

    /*
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
    */

    // fetch-decode-execute cycle, return cycles taken
    // be careful about CB prefix, if CB prefix encountered, fetch the next bit manipulation opcode.
    pub fn tick(&mut self) -> u32 {
        let cycles = {
            if !self.is_halted {
                // fetch and execute instruction
                // fetch byte from pc
                let (opcode, is_cb, is_stop) = {
                    let first_byte = self.memory_bus.read_byte(self.pc);
                    self.pc += 1;
                    // if the first byte is 0xcb, then its a bit opcode
                    if first_byte == 0xcb {
                        let second_byte = self.memory_bus.read_byte(self.pc);
                        self.pc += 1;
                        (second_byte, true, false)
                    } else if first_byte == 0x10 {
                        // execute stop instruction
                        let second_byte = self.memory_bus.read_byte(self.pc);
                        if second_byte == 0x00 {
                            // update pc only when stop instruction encountered
                            self.pc += 1;
                            (0x10, false, true)
                        } else {
                            (first_byte, false, false)
                        }
                    } else {
                        (first_byte, false, false)
                    }
                };

                info!(
                    "fetched opcode {:02x}, is_cb: {:?}, is_stop: {:?}, pc: {:04x}",
                    opcode, is_cb, is_stop, self.pc
                );
                if is_stop {
                    OPCode::exec_stop(self)
                } else {
                    OPCode::exec(self, opcode, is_cb)
                }
            } else {
                // cpu halted
                0
            }
        };

        // TODO: implement interrupt handler
        // check and handle interrupts
        // unset the is_halted when interrupt
        // when is_halted is set and ime = false, interrupt handler is not called.

        // return t cycles
        (cycles * 4) as u32
    }
}
