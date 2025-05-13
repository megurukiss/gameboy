use crate::memory::MemoryBus;
use crate::time::Timer;
use std::time::{Duration, Instant};

pub struct CPU {
    // Registers
    pub A: u8, // Accumulator
    pub F: u8, // Flags
    pub B: u8,
    pub C: u8,
    pub D: u8,
    pub E: u8,
    pub H: u8,
    pub L: u8,
    pub SP: u16, // Stack Pointer
    pub PC: u16, // Program Counter
    pub memory_bus: MemoryBus,
    pub timer: Timer,
}

struct ALU; // Arithmetic Logic Unit
struct IDU; // Instruction Decode Unit

impl CPU {
    pub fn new() -> Self {
        CPU {
            A: 0,
            F: 0,
            B: 0,
            C: 0,
            D: 0,
            E: 0,
            H: 0,
            L: 0,
            SP: 0,
            PC: 0,
            memory_bus: MemoryBus::new(),
            timer: Timer::new(),
        }
    }

    pub fn BC(&self) -> u16 {
        (self.B as u16) << 8 | self.C as u16
    }

    pub fn set_BC(&mut self, value: u16) {
        self.B = ((value & 0xFF00) >> 8) as u8;
        self.C = (value & 0x00FF) as u8;
    }

    pub fn DE(&self) -> u16 {
        (self.D as u16) << 8 | self.E as u16
    }

    pub fn set_DE(&mut self, value: u16) {
        self.D = ((value & 0xFF00) >> 8) as u8;
        self.E = (value & 0x00FF) as u8;
    }

    pub fn HL(&self) -> u16 {
        (self.H as u16) << 8 | self.L as u16
    }

    pub fn set_HL(&mut self, value: u16) {
        self.H = ((value & 0xFF00) >> 8) as u8;
        self.L = (value & 0x00FF) as u8;
    }

    pub fn set_SP(&mut self, value: u16) {
        self.SP = value;
    }

    // set flags
    pub fn set_Z(&mut self, value: bool) {
        match value {
            true => self.F |= 0b1000_0000,
            false => self.F &= 0b0111_1111,
        }
    }
    pub fn Z(&self) -> bool {
        self.F & 0b1000_0000 != 0
    }

    // N flag
    pub fn set_N(&mut self, value: bool) {
        match value {
            true => self.F |= 0b0100_0000,
            false => self.F &= 0b1011_1111,
        }
    }
    pub fn N(&self) -> bool {
        self.F & 0b0100_0000 != 0
    }

    // H flag
    pub fn set_H(&mut self, value: bool) {
        match value {
            true => self.F |= 0b0010_0000,
            false => self.F &= 0b1101_1111,
        }
    }
    pub fn H(&self) -> bool {
        self.F & 0b0010_0000 != 0
    }

    // C flag
    pub fn set_C(&mut self, value: bool) {
        match value {
            true => self.F |= 0b0001_0000,
            false => self.F &= 0b1110_1111,
        }
    }
    pub fn C(&self) -> bool {
        self.F & 0b0001_0000 != 0
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
        let cycles = (elapsed_time * freq * scale) as u64;
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
        0
    }
}
