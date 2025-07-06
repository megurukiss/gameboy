use crate::cpu::CPU;
use crate::opcodes::opcode::OPCode;

impl OPCode {
    // control flow instructions

    // JP nn 11000011
    pub(super) fn op_11000011(cpu: &mut CPU) -> u8 {
        // read target address
        let target_address = cpu.memory_bus.read_word(cpu.PC);
        cpu.PC = cpu.PC + 2;
        cpu.PC = target_address;
        4
    }

    // JP HL 11101001
    pub(super) fn op_11101001(cpu: &mut CPU) -> u8 {
        cpu.PC = cpu.HL();
        1
    }

    // JP cc, nn 110xx010
    pub(super) fn op_110xx010(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let condition = OPCode::concat_bits(&bits[3..5]);
        let target_address = cpu.memory_bus.read_word(cpu.PC);
        cpu.PC = cpu.PC + 2;
        match condition {
            0b00 => {
                // NZ
                if !cpu.Z() {
                    cpu.PC = target_address;
                    return 4;
                }
            }
            0b01 => {
                // Z
                if cpu.Z() {
                    cpu.PC = target_address;
                    return 4;
                }
            }
            0b10 => {
                // NC
                if !cpu.C() {
                    cpu.PC = target_address;
                    return 4;
                }
            }
            0b11 => {
                // C
                if cpu.C() {
                    cpu.PC = target_address;
                    return 4;
                }
            }
            _ => {
                // display error
                panic!("Invalid condition code for JP cc, nn instruction")
            }
        }
        3
    }

    // JR e 00011000
    pub(super) fn op_00011000(cpu: &mut CPU) -> u8 {
        let offset: i8 = cpu.memory_bus.read_byte(cpu.PC) as i8;
        cpu.PC = cpu.PC + 1;
        cpu.PC = cpu.PC.wrapping_add(offset as u16);
        3
    }

    // JR cc, e 001xx000
    pub(super) fn op_001xx000(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let condition = OPCode::concat_bits(&bits[3..5]);
        let offset = cpu.memory_bus.read_byte(cpu.PC) as i8;
        cpu.PC = cpu.PC + 1;
        match condition {
            0b00 => {
                // NZ
                if !cpu.Z() {
                    cpu.PC = cpu.PC.wrapping_add(offset as u16);
                    return 3;
                }
            }
            0b01 => {
                // Z
                if cpu.Z() {
                    cpu.PC = cpu.PC.wrapping_add(offset as u16);
                    return 3;
                }
            }
            0b10 => {
                // NC
                if !cpu.C() {
                    cpu.PC = cpu.PC.wrapping_add(offset as u16);
                    return 3;
                }
            }
            0b11 => {
                // C
                if cpu.C() {
                    cpu.PC = cpu.PC.wrapping_add(offset as u16);
                    return 3;
                }
            }
            _ => {
                // display error
                panic!("Invalid condition code for JP cc, nn instruction")
            }
        }
        2
    }

    // CALL nn 11001101
    pub(super) fn op_11001101(cpu: &mut CPU) -> u8 {
        // read address
        let target_address = cpu.memory_bus.read_word(cpu.PC);
        cpu.PC += 2;
        // write return address to stack
        cpu.SP -= 2;
        cpu.memory_bus.write_word(cpu.SP, cpu.PC);
        // set PC to target address
        cpu.PC = target_address;
        6
    }

    // CALL cc, nn 110xx100
    pub(super) fn op_110xx100(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let condition = OPCode::concat_bits(&bits[3..5]);
        let target_address = cpu.memory_bus.read_word(cpu.PC);
        cpu.PC += 1;
        match condition {
            0b00 => {
                // NZ
                if !cpu.Z() {
                    cpu.SP -= 2;
                    cpu.memory_bus.write_word(cpu.SP, cpu.PC);
                    // set PC to target address
                    cpu.PC = target_address;
                    return 6;
                }
            }
            0b01 => {
                // Z
                if cpu.Z() {
                    cpu.SP -= 2;
                    cpu.memory_bus.write_word(cpu.SP, cpu.PC);
                    // set PC to target address
                    cpu.PC = target_address;
                    return 6;
                }
            }
            0b10 => {
                // NC
                if !cpu.C() {
                    cpu.SP -= 2;
                    cpu.memory_bus.write_word(cpu.SP, cpu.PC);
                    // set PC to target address
                    cpu.PC = target_address;
                    return 6;
                }
            }
            0b11 => {
                // C
                if cpu.C() {
                    cpu.SP -= 2;
                    cpu.memory_bus.write_word(cpu.SP, cpu.PC);
                    // set PC to target address
                    cpu.PC = target_address;
                    return 6;
                }
            }
            _ => {
                // display error
                panic!("Invalid condition code for JP cc, nn instruction")
            }
        }
        3
    }

    // RET 11001001
    pub(super) fn op_11001001(cpu: &mut CPU) -> u8 {
        let target = cpu.memory_bus.read_word(cpu.SP);
        cpu.SP += 2;
        cpu.PC = target;
        4
    }

    // RET cc 110xx000
    pub(super) fn op_110xx000(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let condition = OPCode::concat_bits(&bits[3..5]);
        if (condition == 0b00 && !cpu.Z())
            || (condition == 0b01 && cpu.Z())
            || (condition == 0b10 && !cpu.C())
            || (condition == 0b11 && cpu.C())
        {
            let target = cpu.memory_bus.read_word(cpu.SP);
            cpu.SP += 2;
            cpu.PC = target;
            return 5;
        }
        2
    }

    // RETI 11011001
    pub(super) fn op_11011001(cpu: &mut CPU) -> u8 {
        let target = cpu.memory_bus.read_word(cpu.SP);
        cpu.SP += 2;
        cpu.PC = target;
        cpu.set_ime(true);
        4
    }

    // RST n 11xxx111
    pub(super) fn op_11xxx111(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let address = {
            let t = OPCode::concat_bits(&bits[2..5]);
            match t {
                0b000 => 0x0000u16,
                0b001 => 0x0008u16,
                0b010 => 0x0010u16,
                0b011 => 0x0018u16,
                0b100 => 0x0020u16,
                0b101 => 0x0028u16,
                0b110 => 0x0030u16,
                0b111 => 0x0038u16,
                _ => panic!("Invalid RST address"),
            }
        };
        cpu.SP -= 2;
        cpu.memory_bus.write_word(cpu.SP, cpu.PC);
        cpu.PC = address;
        4
    }
}
