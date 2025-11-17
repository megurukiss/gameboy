use log::debug;

use crate::core::CPU;
use crate::opcodes::opcode::OPCode;

enum ALUOP {
    ADD,
    SUB,
    INC,
    DEC,
    AND,
    OR,
    XOR,
}

fn alu_helper(cpu: &mut CPU, op: ALUOP, num1: u8, num2: u8, carry_in: u8) -> u8 {
    let result = match op {
        ALUOP::ADD => {
            let intermediate = (num1 as u16)
                .wrapping_add(num2 as u16)
                .wrapping_add(carry_in as u16);
            let result = intermediate as u8;

            let half_carry = (num1 & 0x0F)
                .wrapping_add(num2 & 0x0F)
                .wrapping_add(carry_in)
                > 0x0F;
            let carry = intermediate > 0xFF;

            cpu.set_z(result == 0);
            cpu.set_n(false);
            cpu.set_h(half_carry);
            cpu.set_c(carry);
            result
        }
        ALUOP::SUB => {
            let intermediate = (num1 as i16)
                .wrapping_sub(num2 as i16)
                .wrapping_sub(carry_in as i16);
            let result = intermediate as u8;

            let half_carry = (num1 & 0x0F) < (num2 & 0x0F) + carry_in;
            let carry = intermediate < 0;

            cpu.set_z(result == 0);
            cpu.set_n(true);
            cpu.set_h(half_carry);
            cpu.set_c(carry);
            result
        }
        // INC don't set carry flag
        ALUOP::INC => {
            let intermediate = (num1 as u16).wrapping_add(1);
            let result = intermediate as u8;

            let half_carry = (num1 & 0x0F).wrapping_add(1) > 0x0F;

            cpu.set_z(result == 0);
            cpu.set_n(false);
            cpu.set_h(half_carry);
            result
        }
        ALUOP::DEC => {
            let intermediate = (num1 as i16).wrapping_sub(1);
            let result = intermediate as u8;
            let half_carry = (num1 & 0x0F) == 0;
            cpu.set_z(result == 0);
            cpu.set_n(true);
            cpu.set_h(half_carry);
            result
        }
        ALUOP::AND => {
            let result = num1 & num2;
            cpu.set_z(result == 0);
            cpu.set_n(false);
            cpu.set_h(true);
            cpu.set_c(false);
            result
        }
        ALUOP::OR => {
            let result = num1 | num2;
            cpu.set_z(result == 0);
            cpu.set_n(false);
            cpu.set_h(false);
            cpu.set_c(false);
            result
        }
        ALUOP::XOR => {
            let result = num1 ^ num2;
            cpu.set_z(result == 0);
            cpu.set_n(false);
            cpu.set_h(false);
            cpu.set_c(false);
            result
        }
    };
    result
}

impl OPCode {
    // add opcodes

    //ADD r: Add (register) 0b10000xxx
    pub(super) fn op_10000xxx(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let value = OPCode::concat_bits(&bits[5..]);
        // let half_carry = (cpu.A & 0x0F).wrapping_add(value & 0x0F) > 0x0F;
        // let carry = (cpu.A as u16).wrapping_add(value as u16) > 0xFF;
        // cpu.A = cpu.A.wrapping_add(value);
        // cpu.set_Z(cpu.A == 0);
        // cpu.set_N(false);
        // cpu.set_H(half_carry);
        // cpu.set_C(carry);
        cpu.a = alu_helper(cpu, ALUOP::ADD, cpu.a, value, 0);
        1
    }

    // ADD (HL) 0b10000110
    pub(super) fn op_10000110(cpu: &mut CPU) -> u8 {
        let address = cpu.hl();
        let value = cpu.memory_bus.read_byte(address);
        cpu.a = alu_helper(cpu, ALUOP::ADD, cpu.a, value, 0);
        2
    }

    // ADD n: 0b11000110
    pub(super) fn op_11000110(cpu: &mut CPU) -> u8 {
        let value = cpu.memory_bus.read_byte(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.a = alu_helper(cpu, ALUOP::ADD, cpu.a, value, 0);
        2
    }

    // ADC r 0b10001xxx
    pub(super) fn op_10001xxx(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let index = OPCode::concat_bits(&bits[5..]);
        let value = *OPCode::get_register_by_index(index, cpu).unwrap();
        let carry = if cpu.c() { 1 } else { 0 };
        cpu.a = alu_helper(cpu, ALUOP::ADD, cpu.a, value, carry);
        1
    }

    // ADC (HL) 0b10001110
    pub(super) fn op_10001110(cpu: &mut CPU) -> u8 {
        let address = cpu.hl();
        let value = cpu.memory_bus.read_byte(address);
        let carry = if cpu.c() { 1 } else { 0 };
        cpu.a = alu_helper(cpu, ALUOP::ADD, cpu.a, value, carry);
        2
    }

    // ADC n 0b11001110
    pub(super) fn op_11001110(cpu: &mut CPU) -> u8 {
        let value = cpu.memory_bus.read_byte(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        let carry = if cpu.c() { 1 } else { 0 };
        cpu.a = alu_helper(cpu, ALUOP::ADD, cpu.a, value, carry);
        2
    }
}

impl OPCode {
    // sub opcodes

    // SUB r 0b10010xxx
    pub(super) fn op_10010xxx(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let index = OPCode::concat_bits(&bits[5..]);
        let value = *OPCode::get_register_by_index(index, cpu).unwrap();
        cpu.a = alu_helper(cpu, ALUOP::SUB, cpu.a, value, 0);
        1
    }

    // SUB (HL) 0b10010110
    pub(super) fn op_10010110(cpu: &mut CPU) -> u8 {
        let address = cpu.hl();
        let value = cpu.memory_bus.read_byte(address);
        cpu.a = alu_helper(cpu, ALUOP::SUB, cpu.a, value, 0);
        2
    }

    // SUB n 0b11010110
    pub(super) fn op_11010110(cpu: &mut CPU) -> u8 {
        let value = cpu.memory_bus.read_byte(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.a = alu_helper(cpu, ALUOP::SUB, cpu.a, value, 0);
        2
    }

    // SBC r 0b10011xxx
    pub(super) fn op_10011xxx(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let index = OPCode::concat_bits(&bits[5..]);
        let value = *OPCode::get_register_by_index(index, cpu).unwrap();
        let carry = if cpu.c() { 1 } else { 0 };
        cpu.a = alu_helper(cpu, ALUOP::SUB, cpu.a, value, carry);
        1
    }

    // SBC (HL) 0b10011110
    pub(super) fn op_10011110(cpu: &mut CPU) -> u8 {
        let address = cpu.hl();
        let value = cpu.memory_bus.read_byte(address);
        let carry = if cpu.c() { 1 } else { 0 };
        cpu.a = alu_helper(cpu, ALUOP::SUB, cpu.a, value, carry);
        2
    }

    // SBC n 0b11011110
    pub(super) fn op_11011110(cpu: &mut CPU) -> u8 {
        let value = cpu.memory_bus.read_byte(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        let carry = if cpu.c() { 1 } else { 0 };
        cpu.a = alu_helper(cpu, ALUOP::SUB, cpu.a, value, carry);
        2
    }
}

impl OPCode {
    // logical opcodes

    // AND r 0b10100xxx
    pub(super) fn op_10100xxx(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let index = OPCode::concat_bits(&bits[5..]);
        let value = *OPCode::get_register_by_index(index, cpu).unwrap();
        cpu.a = alu_helper(cpu, ALUOP::AND, cpu.a, value, 0);
        1
    }

    // AND (HL) 0b10100110
    pub(super) fn op_10100110(cpu: &mut CPU) -> u8 {
        let address = cpu.hl();
        let value = cpu.memory_bus.read_byte(address);
        cpu.a = alu_helper(cpu, ALUOP::AND, cpu.a, value, 0);
        2
    }

    // AND n 0b11100110
    pub(super) fn op_11100110(cpu: &mut CPU) -> u8 {
        let value = cpu.memory_bus.read_byte(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.a = alu_helper(cpu, ALUOP::AND, cpu.a, value, 0);
        2
    }

    // OR r  0b10110xxx
    pub(super) fn op_10110xxx(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let index = OPCode::concat_bits(&bits[5..]);
        let value = *OPCode::get_register_by_index(index, cpu).unwrap();
        cpu.a = alu_helper(cpu, ALUOP::OR, cpu.a, value, 0);
        1
    }

    // OR (HL) 0b10110110
    pub(super) fn op_10110110(cpu: &mut CPU) -> u8 {
        let address = cpu.hl();
        let value = cpu.memory_bus.read_byte(address);
        cpu.a = alu_helper(cpu, ALUOP::OR, cpu.a, value, 0);
        2
    }

    // OR n 0b11110110
    pub(super) fn op_11110110(cpu: &mut CPU) -> u8 {
        let value = cpu.memory_bus.read_byte(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.a = alu_helper(cpu, ALUOP::OR, cpu.a, value, 0);
        2
    }

    // XOR r 0b10101xxx
    pub(super) fn op_10101xxx(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let index = OPCode::concat_bits(&bits[5..]);
        let value = *OPCode::get_register_by_index(index, cpu).unwrap();
        cpu.a = alu_helper(cpu, ALUOP::XOR, cpu.a, value, 0);
        1
    }

    // XOR (HL) 0b10101110
    pub(super) fn op_10101110(cpu: &mut CPU) -> u8 {
        let address = cpu.hl();
        let value = cpu.memory_bus.read_byte(address);
        cpu.a = alu_helper(cpu, ALUOP::XOR, cpu.a, value, 0);
        2
    }

    // XOR n 0b11101110
    pub(super) fn op_11101110(cpu: &mut CPU) -> u8 {
        let value = cpu.memory_bus.read_byte(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.a = alu_helper(cpu, ALUOP::XOR, cpu.a, value, 0);
        2
    }
}

impl OPCode {
    // other ALU opcodes

    // CP r 0b10111xxx
    pub(super) fn op_10111xxx(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let index = OPCode::concat_bits(&bits[5..]);
        let value = *OPCode::get_register_by_index(index, cpu).unwrap();
        alu_helper(cpu, ALUOP::SUB, cpu.a, value, 0);
        1
    }

    // CP (HL) 10111110
    pub(super) fn op_10111110(cpu: &mut CPU) -> u8 {
        let address = cpu.hl();
        let value = cpu.memory_bus.read_byte(address);
        alu_helper(cpu, ALUOP::SUB, cpu.a, value, 0);
        2
    }

    // CP n 0b11111110
    pub(super) fn op_11111110(cpu: &mut CPU) -> u8 {
        let value = cpu.memory_bus.read_byte(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        alu_helper(cpu, ALUOP::SUB, cpu.a, value, 0);
        2
    }

    // INC r 0b00xxx100
    pub(super) fn op_00xxx100(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let index = OPCode::concat_bits(&bits[2..5]);
        // debug!("index {}", index);

        let value = {
            let register = OPCode::get_register_by_index(index, cpu).unwrap();
            *register
        };
        let result = alu_helper(cpu, ALUOP::INC, value, 1, 0);
        *OPCode::get_register_by_index(index, cpu).unwrap() = result;
        1
    }

    // INC (HL) 0b00110100
    pub(super) fn op_00110100(cpu: &mut CPU) -> u8 {
        let address = cpu.hl();
        let value = cpu.memory_bus.read_byte(address);
        let result = alu_helper(cpu, ALUOP::INC, value, 1, 0);
        cpu.memory_bus.write_byte(address, result);
        3
    }

    // DEC r 0b00xxx101
    pub(super) fn op_00xxx101(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let index = OPCode::concat_bits(&bits[2..5]);
        let value = {
            let register = OPCode::get_register_by_index(index, cpu).unwrap();
            *register
        };
        let result = alu_helper(cpu, ALUOP::DEC, value, 1, 0);
        *OPCode::get_register_by_index(index, cpu).unwrap() = result;
        1
    }

    // DEC (HL) 0b00110101
    pub(super) fn op_00110101(cpu: &mut CPU) -> u8 {
        let address = cpu.hl();
        let value = cpu.memory_bus.read_byte(address);
        let result = alu_helper(cpu, ALUOP::DEC, value, 1, 0);
        cpu.memory_bus.write_byte(address, result);
        3
    }

    // CCF 0b00111111
    pub(super) fn op_00111111(cpu: &mut CPU) -> u8 {
        cpu.set_c(!cpu.c());
        cpu.set_n(false);
        cpu.set_h(false);
        1
    }

    // SCF 0b00110111
    pub(super) fn op_00110111(cpu: &mut CPU) -> u8 {
        cpu.set_c(true);
        cpu.set_n(false);
        cpu.set_h(false);
        1
    }

    // DAA 0b00100111
    pub(super) fn op_00100111(cpu: &mut CPU) -> u8 {
        if cpu.n() {
            let mut adjustment = 0;
            if cpu.h() {
                adjustment += 0x06;
            }
            if cpu.c() {
                adjustment += 0x60;
            }
            let c_flag = cpu.c();
            cpu.a = cpu.a.wrapping_sub(adjustment);
            cpu.set_z(cpu.a == 0);
            cpu.set_h(false);
            cpu.set_c(c_flag);
        } else {
            let mut adjustment = 0;
            if cpu.h() || (cpu.a & 0x0F) > 9 {
                adjustment += 0x06;
            }
            if cpu.c() || cpu.a > 0x99 {
                adjustment += 0x60;
            }
            let c_flag = cpu.c();
            let a = cpu.a;
            cpu.a = cpu.a.wrapping_add(adjustment);
            cpu.set_z(cpu.a == 0);
            cpu.set_h(false);
            cpu.set_c(c_flag || a > 0x99);
        }
        1
    }

    // CPL 0b00101111
    pub(super) fn op_00101111(cpu: &mut CPU) -> u8 {
        cpu.a = !cpu.a;
        cpu.set_n(true);
        cpu.set_h(true);
        1
    }
}

impl OPCode {
    // 16-bit ALU opcodes

    // INC rr 0b00xx0011
    pub(super) fn op_00xx0011(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let index = OPCode::concat_bits(&bits[2..4]);
        let value = OPCode::get_16b_register_by_index(index, cpu);
        let result = value.wrapping_add(1);
        OPCode::set_16b_register_by_index(index, cpu, result);
        2
    }

    // DEC rr 0b00xx1011
    pub(super) fn op_00xx1011(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let index = OPCode::concat_bits(&bits[2..4]);
        let value = OPCode::get_16b_register_by_index(index, cpu);
        let result = value.wrapping_sub(1);
        OPCode::set_16b_register_by_index(index, cpu, result);
        2
    }

    // ADD HL, rr 0b00xx1001
    pub(super) fn op_00xx1001(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let index = OPCode::concat_bits(&bits[2..4]);
        let rr = OPCode::get_16b_register_by_index(index, cpu);
        let hl = cpu.hl();

        // let bits_for_num1 = OPCode::parse_bits_u16(hl);
        // let bits_for_num2 = OPCode::parse_bits_u16(rr);
        // let mut carry_bits = [0u8; 17];
        // let mut result = [0u8; 16];
        // for idx in (0..16).rev() {
        //     let temp = bits_for_num1[idx as usize]
        //         + bits_for_num2[idx as usize]
        //         + carry_bits[(idx + 1) as usize];
        //     result[idx as usize] = temp & 1;
        //     carry_bits[idx as usize] = temp >> 1;
        // }
        // // set flags
        // let sum = OPCode::concat_bits_u16(&result);
        let sum = hl.wrapping_add(rr);
        cpu.set_hl(sum);
        cpu.set_n(false);
        // if (carry_bits[11] == 1) {
        //     cpu.set_H(true);
        // } else {
        //     cpu.set_H(false);
        // }

        // if (carry_bits[15] == 1) {
        //     cpu.set_C(true);
        // } else {
        //     cpu.set_C(false);
        // }
        cpu.set_c((u32::from(hl) + u32::from(rr)) > 0xFFFF);
        cpu.set_h(((hl & 0x0FFF) + (rr & 0x0FFF)) > 0x0FFF);
        2
    }

    // ADD SP, e  11101000
    pub(super) fn op_11101000(cpu: &mut CPU) -> u8 {
        let sp = cpu.sp;
        let ee = cpu.memory_bus.read_byte(cpu.pc);
        let ee_i8 = ee as i8;
        cpu.pc = cpu.pc.wrapping_add(1);

        let sp_low = (sp & 0xFF) as u8;
        let h_flag = ((sp_low & 0x0F) + (ee & 0x0F)) > 0x0F;
        let c_flag = ((sp_low as u16) + (ee as u16)) > 0xFF;
        let result = (sp as i32) + (ee_i8 as i32);
        cpu.sp = result as u16;
        cpu.set_z(false);
        cpu.set_n(false);
        cpu.set_h(h_flag);
        cpu.set_c(c_flag);
        4
    }
}
