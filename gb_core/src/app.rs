use crate::cartridge::load_cartridge_from_file;
use crate::core::{Error, CPU};
use std::result::Result;

// TODO: cpu updates controlled by frame rates.
pub struct GameBoyApp {
    cpu: CPU,
}

impl GameBoyApp {
    pub fn new(path: &str) -> Result<Self, Error> {
        let mut cpu = CPU::new();
        // load cartridge file
        cpu.memory_bus
            .load_cartridge(load_cartridge_from_file(path)?);
        Ok(Self { cpu })
    }

    pub fn load_new_cartridge(&mut self, path: &str) -> Result<(), Error> {
        // reset cpu and memory
        self.cpu.reset();
        self.cpu
            .memory_bus
            .load_cartridge(load_cartridge_from_file(path)?);
        self.boot();
        Ok(())
    }

    pub fn boot(&mut self) {
        // check cartridge is valid

        // play di-ding sound

        // set boot rom register to disable boot rom

        // set pc
        self.cpu.pc = 0x0100;
    }

    pub fn run(&mut self) {
        // compute frame rate

        // call cpu update

        // update screen, draw screen
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::debug;

    #[test]
    #[test_log::test]
    fn test_init_app() {}

    #[test]
    #[test_log::test]
    fn test_boot_app() {}

    #[test]
    #[test_log::test]
    fn test_run_app() {}

    #[test]
    #[test_log::test]
    fn test_change_cartridge() {}
}
