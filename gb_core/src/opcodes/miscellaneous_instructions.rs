use crate::core::CPU;
use crate::opcodes::opcode::OPCode;

impl OPCode {
    // Miscellaneous instructions

    // HALT 01110110
    pub(super) fn op_01110110(cpu: &mut CPU) -> u8 {
        // check ime, ie, if
        if cpu.ime() {
            // if ime enabled
            cpu.is_halted = true;
        } else {
            // ime disabled
            // check if any interrupts pending
            if (cpu.ie() == 0) && (cpu.r#if() == 0) {
                // no interrupts pending, As soon as an interrupt becomes pending, the CPU resumes execution. This is like the above, except that the handler is not called.
                cpu.is_halted = true;
            } else {
                // do nothing, next byte read twice in real machine due to hardware bug
            }
        }
        0
    }

    // STOP 00010000 00000000
    pub(super) fn op_00010000_00000000(cpu: &mut CPU) -> u8 {
        // TODO

        // STOP can trigger frequency change in GBC

        // in GB, STOP enters deeper sleep state, and waken up by joypad.

        0
    }

    //DI 11110011
    pub(super) fn op_11110011(cpu: &mut CPU) -> u8 {
        cpu.set_ime(false);
        1
    }

    // EI 11111011
    pub(super) fn op_11111011(cpu: &mut CPU) -> u8 {
        cpu.set_ime(true);
        1
    }

    // NOP 00000000
    pub(super) fn op_00000000() -> u8 {
        // do nothing
        1
    }
}
