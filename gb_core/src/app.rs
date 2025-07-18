use crate::cartridge::load_cartridge_from_file;
use crate::core::{Error, CPU};
use std::result::Result;

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

    pub fn load_cartridge(&mut self, path: &str) -> Result<(), Error> {
        // reset cpu and memory
        self.cpu.reset();
        self.cpu
            .memory_bus
            .load_cartridge(load_cartridge_from_file(path)?);
        Ok(())
    }

    pub fn run(&mut self) {}
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
