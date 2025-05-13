use std::result;

use crate::cpu::CPU;
use crate::opcodes::opcode::OPCode;

enum ALU_OP {
    ADD,
    SUB,
    INC,
    DEC,
    AND,
    OR,
    XOR,
}

fn ALU_helper(cpu: &mut CPU, op: ALU_OP, num1: u8, num2: u8, carry_in: u8) -> u8 {
    let result = match op {
        ALU_OP::ADD => {
            let intermediate = (num1 as u16)
                .wrapping_add(num2 as u16)
                .wrapping_add(carry_in as u16);
            let result = intermediate as u8;

            let half_carry = (num1 & 0x0F)
                .wrapping_add(num2 & 0x0F)
                .wrapping_add(carry_in)
                > 0x0F;
            let carry = intermediate > 0xFF;

            cpu.set_Z(result == 0);
            cpu.set_N(false);
            cpu.set_H(half_carry);
            cpu.set_C(carry);
            result
        }
        ALU_OP::SUB => {
            let intermediate = (num1 as i16)
                .wrapping_sub(num2 as i16)
                .wrapping_sub(carry_in as i16);
            let result = intermediate as u8;

            let half_carry = (num1 & 0x0F) < (num2 & 0x0F) + carry_in;
            let carry = intermediate < 0;

            cpu.set_Z(result == 0);
            cpu.set_N(true);
            cpu.set_H(half_carry);
            cpu.set_C(carry);
            result
        }
        // INC don't set carry flag
        ALU_OP::INC => {
            let intermediate = (num1 as u16).wrapping_add(1);
            let result = intermediate as u8;

            let half_carry = (num1 & 0x0F).wrapping_add(1) > 0x0F;

            cpu.set_Z(result == 0);
            cpu.set_N(false);
            cpu.set_H(half_carry);
            result
        }
        ALU_OP::DEC => {
            let intermediate = (num1 as i16).wrapping_sub(1);
            let result = intermediate as u8;
            let half_carry = (num1 & 0x0F) == 0;
            cpu.set_Z(result == 0);
            cpu.set_N(true);
            cpu.set_H(half_carry);
            result
        }
        ALU_OP::AND => {
            let result = num1 & num2;
            cpu.set_Z(result == 0);
            cpu.set_N(false);
            cpu.set_H(true);
            cpu.set_C(false);
            result
        }
        ALU_OP::OR => {
            let result = num1 | num2;
            cpu.set_Z(result == 0);
            cpu.set_N(false);
            cpu.set_H(false);
            cpu.set_C(false);
            result
        }
        ALU_OP::XOR => {
            let result = num1 ^ num2;
            cpu.set_Z(result == 0);
            cpu.set_N(false);
            cpu.set_H(false);
            cpu.set_C(false);
            result
        }
        _ => panic!("Not implemented"),
    };
    result
}

impl OPCode {
    // add opcodes

    //ADD r: Add (register) 0b10000xxx
    pub(super) fn op_10000xxx(cpu: &mut CPU, bits: &[u8]) {
        let value = OPCode::concat_bits(&bits[5..]);
        // let half_carry = (cpu.A & 0x0F).wrapping_add(value & 0x0F) > 0x0F;
        // let carry = (cpu.A as u16).wrapping_add(value as u16) > 0xFF;
        // cpu.A = cpu.A.wrapping_add(value);
        // cpu.set_Z(cpu.A == 0);
        // cpu.set_N(false);
        // cpu.set_H(half_carry);
        // cpu.set_C(carry);
        cpu.A = ALU_helper(cpu, ALU_OP::ADD, cpu.A, value, 0);
    }

    // ADD (HL) 0b10000110
    pub(super) fn op_10000110(cpu: &mut CPU) {
        let address = cpu.HL();
        let value = cpu.memory_bus.read_byte(address);
        cpu.A = ALU_helper(cpu, ALU_OP::ADD, cpu.A, value, 0);
    }

    // ADD n: 0b11000110
    pub(super) fn op_11000110(cpu: &mut CPU) {
        let value = cpu.memory_bus.read_byte(cpu.PC);
        cpu.PC = cpu.PC.wrapping_add(1);
        cpu.A = ALU_helper(cpu, ALU_OP::ADD, cpu.A, value, 0);
    }

    // ADC r 0b10001xxx
    pub(super) fn op_10001xxx(cpu: &mut CPU, bits: &[u8]) {
        let index = OPCode::concat_bits(&bits[5..]);
        let value = *OPCode::get_register_by_index(index, cpu).unwrap();
        let carry = if cpu.C() { 1 } else { 0 };
        cpu.A = ALU_helper(cpu, ALU_OP::ADD, cpu.A, value, carry);
    }

    // ADC (HL) 0b10001110
    pub(super) fn op_10001110(cpu: &mut CPU) {
        let address = cpu.HL();
        let value = cpu.memory_bus.read_byte(address);
        let carry = if cpu.C() { 1 } else { 0 };
        cpu.A = ALU_helper(cpu, ALU_OP::ADD, cpu.A, value, carry);
    }

    // ADC n 0b11001110
    pub(super) fn op_11001110(cpu: &mut CPU) {
        let value = cpu.memory_bus.read_byte(cpu.PC);
        cpu.PC = cpu.PC.wrapping_add(1);
        let carry = if cpu.C() { 1 } else { 0 };
        cpu.A = ALU_helper(cpu, ALU_OP::ADD, cpu.A, value, carry);
    }
}

impl OPCode {
    // sub opcodes

    // SUB r 0b10010xxx
    pub(super) fn op_10010xxx(cpu: &mut CPU, bits: &[u8]) {
        let index = OPCode::concat_bits(&bits[5..]);
        let value = *OPCode::get_register_by_index(index, cpu).unwrap();
        cpu.A = ALU_helper(cpu, ALU_OP::SUB, cpu.A, value, 0);
    }

    // SUB (HL) 0b10010110
    pub(super) fn op_10010110(cpu: &mut CPU) {
        let address = cpu.HL();
        let value = cpu.memory_bus.read_byte(address);
        cpu.A = ALU_helper(cpu, ALU_OP::SUB, cpu.A, value, 0);
    }

    // SUB n 0b11010110
    pub(super) fn op_11010110(cpu: &mut CPU) {
        let value = cpu.memory_bus.read_byte(cpu.PC);
        cpu.PC = cpu.PC.wrapping_add(1);
        cpu.A = ALU_helper(cpu, ALU_OP::SUB, cpu.A, value, 0);
    }

    // SBC r 0b10011xxx
    pub(super) fn op_10011xxx(cpu: &mut CPU, bits: &[u8]) {
        let index = OPCode::concat_bits(&bits[5..]);
        let value = *OPCode::get_register_by_index(index, cpu).unwrap();
        let carry = if cpu.C() { 1 } else { 0 };
        cpu.A = ALU_helper(cpu, ALU_OP::SUB, cpu.A, value, carry);
    }

    // SBC (HL) 0b10011110
    pub(super) fn op_10011110(cpu: &mut CPU) {
        let address = cpu.HL();
        let value = cpu.memory_bus.read_byte(address);
        let carry = if cpu.C() { 1 } else { 0 };
        cpu.A = ALU_helper(cpu, ALU_OP::SUB, cpu.A, value, carry);
    }

    // SBC n 0b11011110
    pub(super) fn op_11011110(cpu: &mut CPU) {
        let value = cpu.memory_bus.read_byte(cpu.PC);
        cpu.PC = cpu.PC.wrapping_add(1);
        let carry = if cpu.C() { 1 } else { 0 };
        cpu.A = ALU_helper(cpu, ALU_OP::SUB, cpu.A, value, carry);
    }
}

impl OPCode {
    // logical opcodes

    // AND r 0b10100xxx
    pub(super) fn op_10100xxx(cpu: &mut CPU, bits: &[u8]) {
        let index = OPCode::concat_bits(&bits[5..]);
        let value = *OPCode::get_register_by_index(index, cpu).unwrap();
        cpu.A = ALU_helper(cpu, ALU_OP::AND, cpu.A, value, 0);
    }

    // AND (HL) 0b10100110
    pub(super) fn op_10100110(cpu: &mut CPU) {
        let address = cpu.HL();
        let value = cpu.memory_bus.read_byte(address);
        cpu.A = ALU_helper(cpu, ALU_OP::AND, cpu.A, value, 0);
    }

    // AND n 0b11100110
    pub(super) fn op_11100110(cpu: &mut CPU) {
        let value = cpu.memory_bus.read_byte(cpu.PC);
        cpu.PC = cpu.PC.wrapping_add(1);
        cpu.A = ALU_helper(cpu, ALU_OP::AND, cpu.A, value, 0);
    }

    // OR r  0b10110xxx
    pub(super) fn op_10110xxx(cpu: &mut CPU, bits: &[u8]) {
        let index = OPCode::concat_bits(&bits[5..]);
        let value = *OPCode::get_register_by_index(index, cpu).unwrap();
        cpu.A = ALU_helper(cpu, ALU_OP::OR, cpu.A, value, 0);
    }

    // OR (HL) 0b10110110
    pub(super) fn op_10110110(cpu: &mut CPU) {
        let address = cpu.HL();
        let value = cpu.memory_bus.read_byte(address);
        cpu.A = ALU_helper(cpu, ALU_OP::OR, cpu.A, value, 0);
    }

    // OR n 0b11110110
    pub(super) fn op_11110110(cpu: &mut CPU) {
        let value = cpu.memory_bus.read_byte(cpu.PC);
        cpu.PC = cpu.PC.wrapping_add(1);
        cpu.A = ALU_helper(cpu, ALU_OP::OR, cpu.A, value, 0);
    }

    // XOR r 0b10101xxx
    pub(super) fn op_10101xxx(cpu: &mut CPU, bits: &[u8]) {
        let index = OPCode::concat_bits(&bits[5..]);
        let value = *OPCode::get_register_by_index(index, cpu).unwrap();
        cpu.A = ALU_helper(cpu, ALU_OP::XOR, cpu.A, value, 0);
    }

    // XOR (HL) 0b10101110
    pub(super) fn op_10101110(cpu: &mut CPU) {
        let address = cpu.HL();
        let value = cpu.memory_bus.read_byte(address);
        cpu.A = ALU_helper(cpu, ALU_OP::XOR, cpu.A, value, 0);
    }

    // XOR n 0b11101110
    pub(super) fn op_11101110(cpu: &mut CPU) {
        let value = cpu.memory_bus.read_byte(cpu.PC);
        cpu.PC = cpu.PC.wrapping_add(1);
        cpu.A = ALU_helper(cpu, ALU_OP::XOR, cpu.A, value, 0);
    }
}

impl OPCode {
    // other ALU opcodes

    // CP r 0b10111xxx
    pub(super) fn op_10111xxx(cpu: &mut CPU, bits: &[u8]) {
        let index = OPCode::concat_bits(&bits[5..]);
        let value = *OPCode::get_register_by_index(index, cpu).unwrap();
        ALU_helper(cpu, ALU_OP::SUB, cpu.A, value, 0);
    }

    // CP (HL) 0b10011110
    pub(super) fn op_10111110(cpu: &mut CPU) {
        let address = cpu.HL();
        let value = cpu.memory_bus.read_byte(address);
        ALU_helper(cpu, ALU_OP::SUB, cpu.A, value, 0);
    }

    // CP n 0b11111110
    pub(super) fn op_11111110(cpu: &mut CPU) {
        let value = cpu.memory_bus.read_byte(cpu.PC);
        cpu.PC = cpu.PC.wrapping_add(1);
        ALU_helper(cpu, ALU_OP::SUB, cpu.A, value, 0);
    }

    // INC r 0b00xxx100
    pub(super) fn op_00xxx100(cpu: &mut CPU, bits: &[u8]) {
        let index = OPCode::concat_bits(&bits[3..]);
        let value = {
            let register = OPCode::get_register_by_index(index, cpu).unwrap();
            *register
        };
        let result = ALU_helper(cpu, ALU_OP::INC, value, 1, 0);
        *OPCode::get_register_by_index(index, cpu).unwrap() = result;
    }

    // INC (HL) 0b00110100
    pub(super) fn op_00110100(cpu: &mut CPU) {
        let address = cpu.HL();
        let value = cpu.memory_bus.read_byte(address);
        let result = ALU_helper(cpu, ALU_OP::INC, value, 1, 0);
        cpu.memory_bus.write_byte(address, result);
    }

    // DEC r 0b00xxx101
    pub(super) fn op_00xxx101(cpu: &mut CPU, bits: &[u8]) {
        let index = OPCode::concat_bits(&bits[3..]);
        let value = {
            let register = OPCode::get_register_by_index(index, cpu).unwrap();
            *register
        };
        let result = ALU_helper(cpu, ALU_OP::DEC, value, 1, 0);
        *OPCode::get_register_by_index(index, cpu).unwrap() = result;
    }

    // DEC (HL) 0b00110101
    pub(super) fn op_00110101(cpu: &mut CPU) {
        let address = cpu.HL();
        let value = cpu.memory_bus.read_byte(address);
        let result = ALU_helper(cpu, ALU_OP::DEC, value, 1, 0);
        cpu.memory_bus.write_byte(address, result);
    }

    // CCF 0b00111111
    pub(super) fn op_00111111(cpu: &mut CPU) {
        cpu.set_C(!cpu.C());
        cpu.set_N(false);
        cpu.set_H(false);
    }

    // SCF 0b00110111
    pub(super) fn op_00110111(cpu: &mut CPU) {
        cpu.set_C(true);
        cpu.set_N(false);
        cpu.set_H(false);
    }

    // DAA 0b00100111
    pub(super) fn op_00100111(cpu: &mut CPU) {
        if cpu.N() {
            let mut adjustment = 0;
            if cpu.H() {
                adjustment += 0x06;
            }
            if cpu.C() {
                adjustment += 0x60;
            }
            let c_flag = cpu.C();
            cpu.A = cpu.A.wrapping_sub(adjustment);
            cpu.set_Z(cpu.A == 0);
            cpu.set_H(false);
            cpu.set_C(c_flag);
        } else {
            let mut adjustment = 0;
            if cpu.H() || (cpu.A & 0x0F) > 9 {
                adjustment += 0x06;
            }
            if cpu.C() || cpu.A > 0x99 {
                adjustment += 0x60;
            }
            let c_flag = cpu.C();
            let a = cpu.A;
            cpu.A = cpu.A.wrapping_add(adjustment);
            cpu.set_Z(cpu.A == 0);
            cpu.set_H(false);
            cpu.set_C(c_flag || a > 0x99);
        }
    }

    // CPL 0b00101111
    pub(super) fn op_00101111(cpu: &mut CPU) {
        cpu.A = !cpu.A;
        cpu.set_N(true);
        cpu.set_H(true);
    }
}

impl OPCode {
    // 16-bit ALU opcodes

    // INC rr 0b00xx0011
    pub(super) fn op_00xx0011(cpu: &mut CPU, bits: &[u8]) {
        let index = OPCode::concat_bits(&bits[2..4]);
        let value = OPCode::get_16b_register_by_index(index, cpu);
        let result = value.wrapping_add(1);
        OPCode::set_16b_register_by_index(index, cpu, result);
    }

    // DEC rr 0b00xx1011
    pub(super) fn op_00xx1011(cpu: &mut CPU, bits: &[u8]) {
        let index = OPCode::concat_bits(&bits[2..4]);
        let value = OPCode::get_16b_register_by_index(index, cpu);
        let result = value.wrapping_sub(1);
        OPCode::set_16b_register_by_index(index, cpu, result);
    }

    // ADD HL, rr 0b00xx1001
    pub(super) fn op_00xx1001(cpu: &mut CPU, bits: &[u8]) {
        let index = OPCode::concat_bits(&bits[2..4]);
        let rr = OPCode::get_16b_register_by_index(index, cpu);
        let hl = cpu.HL();

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
        cpu.set_HL(sum);
        cpu.set_N(false);
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
        cpu.set_C((u32::from(hl) + u32::from(rr)) > 0xFFFF);
        cpu.set_H(((hl & 0x0FFF) + (rr & 0x0FFF)) > 0x0FFF);
    }

    // ADD SP, e  11101000
    pub(super) fn op_11101000(cpu: &mut CPU) {
        let sp = cpu.SP;
        let ee = cpu.memory_bus.read_byte(cpu.PC);
        let ee_i8 = ee as i8;
        cpu.PC = cpu.PC.wrapping_add(1);

        let sp_low = (sp & 0xFF) as u8;
        let h_flag = ((sp_low & 0x0F) + (ee & 0x0F)) > 0x0F;
        let c_flag = ((sp_low as u16) + (ee as u16)) > 0xFF;
        let result = (sp as i32) + (ee_i8 as i32);
        cpu.SP = result as u16;
        cpu.set_Z(false);
        cpu.set_N(false);
        cpu.set_H(h_flag);
        cpu.set_C(c_flag);
    }
}

impl OPCode {
    // rotation instructions

    // RLCA 00000111
    pub(super) fn op_00000111(cpu: &mut CPU) {
        let value = cpu.A;
        let bit7 = value >> 7;
        cpu.A = (value << 1) | bit7;
        cpu.set_C(bit7 == 1);
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_Z(false);
    }

    // RRCA 00001111
    pub(super) fn op_00001111(cpu: &mut CPU) {
        let value = cpu.A;
        let bit0 = value & 1;
        cpu.A = (value >> 1) | (bit0 << 7);
        cpu.set_C(bit0 == 1);
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_Z(false);
    }

    // RLA 00010111
    pub(super) fn op_00010111(cpu: &mut CPU) {
        let value = cpu.A;
        let cflag = {
            if cpu.C() {
                1u8
            } else {
                0u8
            }
        };
        let bit7 = value >> 7;
        cpu.A = (value << 1) | cflag;
        cpu.set_C(bit7 == 1);
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_Z(false);
    }

    // RRA 00011111
    pub(super) fn op_00011111(cpu: &mut CPU) {
        let value = cpu.A;
        let cflag = {
            if cpu.C() {
                1u8
            } else {
                0u8
            }
        };
        let bit0 = value & 1;
        cpu.A = (value >> 1) | (cflag << 7);
        cpu.set_C(bit0 == 1);
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_Z(false);
    }

    // RLC r 00000xxx
    pub(super) fn op_00000xxx(cpu: &mut CPU, bits: &[u8]) {
        let index = OPCode::concat_bits(&bits[5..]);
        let r = OPCode::get_register_by_index(index, cpu).unwrap();
        let value = r.clone();
        let bit7 = value >> 7;
        *r = (value << 1) | bit7;
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_C(bit7 == 1);
        cpu.set_Z(value == 0);
    }

    // RLC (HL) 00000110
    pub(super) fn op_00000110(cpu: &mut CPU) {
        let hl = cpu.HL();
        let value = cpu.memory_bus.read_byte(hl);
        let bit7 = value >> 7;
        let result = (value << 1) | bit7;
        cpu.memory_bus.write_byte(hl, result);
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_C(bit7 == 1);
        cpu.set_Z(value == 0);
    }

    // RRC r 00001xxx
    pub(super) fn op_00001xxx(cpu: &mut CPU, bits: &[u8]) {
        let index = OPCode::concat_bits(&bits[5..]);
        let r = OPCode::get_register_by_index(index, cpu).unwrap();
        let value = r.clone();
        let bit0 = value & 1;
        *r = (value >> 1) | (bit0 << 7);
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_C(bit0 == 1);
        cpu.set_Z(value == 0);
    }

    // RRC (HL) 00001110
    pub(super) fn op_00001110(cpu: &mut CPU) {
        let hl = cpu.HL();
        let value = cpu.memory_bus.read_byte(hl);
        let bit0 = value & 1;
        let result = (value >> 1) | (bit0 << 7);
        cpu.memory_bus.write_byte(hl, result);
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_C(bit0 == 1);
        cpu.set_Z(value == 0);
    }

    // RL r 00010xxx
    pub(super) fn op_00010xxx(cpu: &mut CPU, bits: &[u8]) {
        let r = {
            let index = OPCode::concat_bits(&bits[5..]);
            OPCode::get_register_by_index(index, cpu).unwrap()
        };
        let value = r.clone();
    }
}
