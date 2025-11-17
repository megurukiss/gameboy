use crate::cartridge::load_cartridge_from_file;
use crate::core::{Error, CPU, CYCLES_PER_FRAME, DEFAULT_FPS};
use std::result::Result;
use std::thread::sleep;
use std::time::{Duration, Instant};

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
        // compute time per frame
        const FRAME_DURATION: Duration = Duration::from_micros((1_000_000.0 / DEFAULT_FPS) as u64);

        // call cpu update
        loop {
            let frame_start_time = Instant::now();
            let mut cycles_this_frame = 0;
            while cycles_this_frame < CYCLES_PER_FRAME {
                let cycles_executed = self.cpu.tick();
                cycles_this_frame += cycles_executed

                // self.ppu.step(cycles_executed);
                // self.timer.step(cycles_executed);
            }

            // update screen, draw screen

            let time_taken = frame_start_time.elapsed();
            if let Some(sleep_duration) = FRAME_DURATION.checked_sub(time_taken) {
                sleep(sleep_duration);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::debug;

    #[test]
    #[test_log::test]
    fn test_06() {
        let path = "../cpu_instrs/individual/06-ld r,r.gb";
        let mut app = GameBoyApp::new(path).unwrap();
        app.boot();
        app.run();
    }

    #[test]
    #[test_log::test]
    fn test_05() {
        let path = "../cpu_instrs/individual/05-op rp.gb";
        let mut app = GameBoyApp::new(path).unwrap();
        app.boot();
        app.run();
    }

    #[test]
    #[test_log::test]
    fn test_04() {
        let path = "../cpu_instrs/individual/04-op r,imm.gb";
        let mut app = GameBoyApp::new(path).unwrap();
        app.boot();
        app.run();
    }

    #[test]
    #[test_log::test]
    fn test_03() {
        let path = "../cpu_instrs/individual/03-op sp,hl.gb";
        let mut app = GameBoyApp::new(path).unwrap();
        app.boot();
        app.run();
    }

    #[test]
    #[test_log::test]
    fn test_02() {
        let path = "../cpu_instrs/individual/02-interrupts.gb";
        let mut app = GameBoyApp::new(path).unwrap();
        app.boot();
        app.run();
    }

    #[test]
    #[test_log::test]
    fn test_01() {
        let path = "../cpu_instrs/individual/01-special.gb";
        let mut app = GameBoyApp::new(path).unwrap();
        app.boot();
        app.run();
    }

    #[test]
    #[test_log::test]
    fn test_07() {
        let path = "../cpu_instrs/individual/07-jr,jp,call,ret,rst.gb";
        let mut app = GameBoyApp::new(path).unwrap();
        app.boot();
        app.run();
    }

    #[test]
    #[test_log::test]
    fn test_08() {
        let path = "../cpu_instrs/individual/08-misc instrs.gb";
        let mut app = GameBoyApp::new(path).unwrap();
        app.boot();
        app.run();
    }

    #[test]
    #[test_log::test]
    fn test_09() {
        let path = "../cpu_instrs/individual/09-op r,r.gb";
        let mut app = GameBoyApp::new(path).unwrap();
        app.boot();
        app.run();
    }

    #[test]
    #[test_log::test]
    fn test_10() {
        let path = "../cpu_instrs/individual/10-bit ops.gb";
        let mut app = GameBoyApp::new(path).unwrap();
        app.boot();
        app.run();
    }

    #[test]
    #[test_log::test]
    fn test_11() {
        let path = "../cpu_instrs/individual/11-op a,(hl).gb";
        let mut app = GameBoyApp::new(path).unwrap();
        app.boot();
        app.run();
    }

    #[test]
    #[test_log::test]
    fn test_all_gb() {
        let path = "../cpu_instrs/cpu_instrs.gb";
        let mut app = GameBoyApp::new(path).unwrap();
        app.boot();
        app.run();
    }

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
