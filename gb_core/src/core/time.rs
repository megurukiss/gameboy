use std::time::{Duration, Instant};

pub const DEFAULT_TIMER_FREQUENCY: u64 = 4_194_304;

pub struct Timer {
    frequency: u64,
    scale: f32,
    pub cycles_counter: u64,
    pub last_called_time: Option<Instant>,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            frequency: DEFAULT_TIMER_FREQUENCY,
            cycles_counter: 0,
            scale: 1.0,
            last_called_time: None,
        }
    }

    pub fn reset(&mut self) {
        self.cycles_counter = 0;
        self.last_called_time = None;
        self.scale = 1.0;
    }

    pub fn get_frequency(&self) -> u64 {
        self.frequency
    }

    pub fn get_scale(&self) -> f32 {
        self.scale
    }

    pub fn get_cycles_counter(&self) -> u64 {
        self.cycles_counter
    }

    pub fn set_frequency(&mut self, frequency: u64) {
        self.frequency = frequency;
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn update_cycles(&mut self, cycles: u64) {
        self.cycles_counter += cycles * 4; // machine cycle is 4 clock cycles
    }
}
