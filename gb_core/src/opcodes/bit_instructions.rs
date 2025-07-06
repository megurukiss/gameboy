use crate::cpu::CPU;
use crate::opcodes::opcode::OPCode;

impl OPCode {
    // bit instructions, all bit instructions start with 0xcb

    // RLCA 00000111
    pub(super) fn cb_op_00000111(cpu: &mut CPU) -> u8 {
        let value = cpu.A;
        let bit7 = value >> 7;
        cpu.A = (value << 1) | bit7;
        cpu.set_C(bit7 == 1);
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_Z(false);
        1
    }

    // RRCA 00001111
    pub(super) fn cb_op_00001111(cpu: &mut CPU) -> u8 {
        let value = cpu.A;
        let bit0 = value & 1;
        cpu.A = (value >> 1) | (bit0 << 7);
        cpu.set_C(bit0 == 1);
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_Z(false);
        1
    }

    // RLA 00010111
    pub(super) fn cb_op_00010111(cpu: &mut CPU) -> u8 {
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
        1
    }

    // RRA 00011111
    pub(super) fn cb_op_00011111(cpu: &mut CPU) -> u8 {
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
        1
    }

    // RLC r 00000xxx
    pub(super) fn cb_op_00000xxx(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let index = OPCode::concat_bits(&bits[5..]);
        let r = OPCode::get_register_by_index(index, cpu).unwrap();
        let value = r.clone();
        let bit7 = value >> 7;
        *r = (value << 1) | bit7;
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_C(bit7 == 1);
        cpu.set_Z(value == 0);
        2
    }

    // RLC (HL) 00000110
    pub(super) fn cb_op_00000110(cpu: &mut CPU) -> u8 {
        let hl = cpu.HL();
        let value = cpu.memory_bus.read_byte(hl);
        let bit7 = value >> 7;
        let result = (value << 1) | bit7;
        cpu.memory_bus.write_byte(hl, result);
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_C(bit7 == 1);
        cpu.set_Z(value == 0);
        4
    }

    // RRC r 00001xxx
    pub(super) fn cb_op_00001xxx(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let index = OPCode::concat_bits(&bits[5..]);
        let r = OPCode::get_register_by_index(index, cpu).unwrap();
        let value = r.clone();
        let bit0 = value & 1;
        *r = (value >> 1) | (bit0 << 7);
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_C(bit0 == 1);
        cpu.set_Z(value == 0);
        2
    }

    // RRC (HL) 00001110
    pub(super) fn cb_op_00001110(cpu: &mut CPU) -> u8 {
        let hl = cpu.HL();
        let value = cpu.memory_bus.read_byte(hl);
        let bit0 = value & 1;
        let result = (value >> 1) | (bit0 << 7);
        cpu.memory_bus.write_byte(hl, result);
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_C(bit0 == 1);
        cpu.set_Z(value == 0);
        4
    }

    // RL r 00010xxx
    pub(super) fn cb_op_00010xxx(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let cflag = {
            if cpu.C() {
                1u8
            } else {
                0u8
            }
        };
        let r = {
            let index = OPCode::concat_bits(&bits[5..]);
            OPCode::get_register_by_index(index, cpu).unwrap()
        };
        let value = r.clone();
        let bit7 = value >> 7;
        let res = {
            *r = (value << 1) | cflag;
            *r
        };
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_C(bit7 == 1);
        cpu.set_Z(res == 0);
        2
    }

    // RL (HL) 00010110
    pub(super) fn cb_op_00010110(cpu: &mut CPU) -> u8 {
        let value = {
            let hl = cpu.HL();
            cpu.memory_bus.read_byte(hl)
        };
        let cflag = {
            if cpu.C() {
                1u8
            } else {
                0u8
            }
        };
        let bit7 = value >> 7;
        let res = {
            let hl = cpu.HL();
            let result = (value << 1) | cflag;
            cpu.memory_bus.write_byte(hl, result);
            result
        };
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_C(bit7 == 1);
        cpu.set_Z(res == 0);
        4
    }

    // RR r8 00011xxx
    pub(super) fn cb_op_00011xxx(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let cflag = {
            if cpu.C() {
                1u8
            } else {
                0u8
            }
        };
        let r = {
            let index = OPCode::concat_bits(&bits[5..]);
            OPCode::get_register_by_index(index, cpu).unwrap()
        };
        let value = r.clone();
        let bit0 = value & 1;

        let result = {
            *r = (value >> 1) | (cflag << 7);
            *r
        };
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_C(bit0 == 1);
        cpu.set_Z(result == 0);
        2
    }

    // RR (HL) 00011110
    pub(super) fn cb_op_00011110(cpu: &mut CPU) -> u8 {
        let value = {
            let hl = cpu.HL();
            cpu.memory_bus.read_byte(hl)
        };
        let cflag = {
            if cpu.C() {
                1u8
            } else {
                0u8
            }
        };
        let bit0 = value & 1;
        let res = {
            let hl = cpu.HL();
            let result = (value >> 1) | (cflag << 7);
            cpu.memory_bus.write_byte(hl, result);
            result
        };
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_C(bit0 == 1);
        cpu.set_Z(res == 0);
        4
    }

    // SLA r8 00100xxx
    pub(super) fn cb_op_00100xxx(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let r = {
            let index = OPCode::concat_bits(&bits[5..]);
            OPCode::get_register_by_index(index, cpu).unwrap()
        };
        let bit7 = r.clone() >> 7;
        let res = {
            *r = *r << 1;
            *r
        };
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_C(bit7 == 1);
        cpu.set_Z(res == 0);
        2
    }

    // SLA (HL) 00100110
    pub(super) fn cb_op_00100110(cpu: &mut CPU) -> u8 {
        let value = {
            let hl = cpu.HL();
            cpu.memory_bus.read_byte(hl)
        };
        let bit7 = value >> 7;
        let res = {
            let hl = cpu.HL();
            let result = value << 1;
            cpu.memory_bus.write_byte(hl, result);
            result
        };
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_C(bit7 == 1);
        cpu.set_Z(res == 0);
        4
    }

    // SRA r 00101xxx
    pub(super) fn cb_op_00101xxx(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let r = {
            let index = OPCode::concat_bits(&bits[5..]);
            OPCode::get_register_by_index(index, cpu).unwrap()
        };
        let (bit7, bit0) = {
            let value = r.clone();
            (value >> 7, value & 1)
        };
        let result = {
            *r = (*r >> 1) | (bit7 << 7);
            *r
        };
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_C(bit0 == 1);
        cpu.set_Z(result == 0);
        2
    }

    // SRA (HL) 00101110
    pub(super) fn cb_op_00101110(cpu: &mut CPU) -> u8 {
        let value = {
            let hl = cpu.HL();
            cpu.memory_bus.read_byte(hl)
        };
        let (bit7, bit0) = {
            let value = value.clone();
            (value >> 7, value & 1)
        };
        let result = {
            let hl = cpu.HL();
            let result = (value >> 1) | (bit7 << 7);
            cpu.memory_bus.write_byte(hl, result);
            result
        };
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_C(bit0 == 1);
        cpu.set_Z(result == 0);
        4
    }

    // SWAP r 00110xxx
    pub(super) fn cb_op_00110xxx(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let r = {
            let index = OPCode::concat_bits(&bits[5..]);
            OPCode::get_register_by_index(index, cpu).unwrap()
        };
        let result = {
            *r = ((*r & 0xF0) >> 4) | ((*r & 0x0F) << 4);
            *r
        };
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_C(false);
        cpu.set_Z(result == 0);
        2
    }

    // SWAP (HL) 00110110
    pub(super) fn cb_op_00110110(cpu: &mut CPU) -> u8 {
        let value = {
            let hl = cpu.HL();
            cpu.memory_bus.read_byte(hl)
        };
        let result = {
            let hl = cpu.HL();
            let result = ((value & 0xF0) >> 4) | ((value & 0x0F) << 4);
            cpu.memory_bus.write_byte(hl, result);
            result
        };
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_C(false);
        cpu.set_Z(result == 0);
        4
    }

    // SRL r 00111xxx
    pub(super) fn cb_op_00111xxx(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let r = {
            let index = OPCode::concat_bits(&bits[5..]);
            OPCode::get_register_by_index(index, cpu).unwrap()
        };
        let bit0 = r.clone() & 1;
        let result = {
            *r = *r >> 1;
            *r
        };
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_C(bit0 == 1);
        cpu.set_Z(result == 0);
        2
    }

    // SRL (HL) 00111110
    pub(super) fn cb_op_00111110(cpu: &mut CPU) -> u8 {
        let value = {
            let hl = cpu.HL();
            cpu.memory_bus.read_byte(hl)
        };
        let bit0 = value & 1;
        let result = {
            let hl = cpu.HL();
            let result = value >> 1;
            cpu.memory_bus.write_byte(hl, result);
            result
        };
        cpu.set_N(false);
        cpu.set_H(false);
        cpu.set_C(bit0 == 1);
        cpu.set_Z(result == 0);
        4
    }

    // BIT b, r 01xxxxxx
    pub(super) fn cb_op_01xxxxxx(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let (bindex, rvalue) = {
            let bit_index = OPCode::concat_bits(&bits[2..5]);
            let register_index = OPCode::concat_bits(&bits[5..]);
            let r = OPCode::get_register_by_index(register_index, cpu).unwrap();
            (bit_index, r.clone())
        };
        // if selected bit is 0, set flag Z
        let bit_is_zero = (rvalue & (1 << bindex)) == 0;
        cpu.set_Z(bit_is_zero);
        cpu.set_N(false);
        cpu.set_H(true);
        2
    }

    // BIT b, (HL) 01xxx110
    pub(super) fn cb_op_01xxx110(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let (bindex, value) = {
            let bit_index = OPCode::concat_bits(&bits[2..5]);
            let hl = cpu.HL();
            (bit_index, cpu.memory_bus.read_byte(hl))
        };
        let bit_is_zero = (value & (1 << bindex)) == 0;
        cpu.set_Z(bit_is_zero);
        cpu.set_N(false);
        cpu.set_H(true);
        3
    }

    // RES b, r 10xxxxxx
    pub(super) fn cb_op_10xxxxxx(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let (bindex, r) = {
            let bit_index = OPCode::concat_bits(&bits[2..5]);
            let register_index = OPCode::concat_bits(&bits[5..]);
            let r = OPCode::get_register_by_index(register_index, cpu).unwrap();
            (bit_index, r)
        };
        *r = *r & !(1 << bindex);
        2
    }

    // RES b, (HL) 10xxx110
    pub(super) fn cb_op_10xxx110(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let (bindex, value) = {
            let bit_index = OPCode::concat_bits(&bits[2..5]);
            let hl = cpu.HL();
            (bit_index, cpu.memory_bus.read_byte(hl))
        };
        let result = value & !(1 << bindex);
        cpu.memory_bus.write_byte(cpu.HL(), result);
        4
    }

    //SET b, r 11xxxxxx
    pub(super) fn cb_op_11xxxxxx(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let (bindex, r) = {
            let bit_index = OPCode::concat_bits(&bits[2..5]);
            let register_index = OPCode::concat_bits(&bits[5..]);
            let r = OPCode::get_register_by_index(register_index, cpu).unwrap();
            (bit_index, r)
        };
        *r = *r | (1 << bindex);
        2
    }

    //SET b,(HL) 11xxx110
    pub(super) fn cb_op_11xxx110(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let (bindex, value) = {
            let bit_index = OPCode::concat_bits(&bits[2..5]);
            let hl = cpu.HL();
            (bit_index, cpu.memory_bus.read_byte(hl))
        };
        let result = value | (1 << bindex);
        cpu.memory_bus.write_byte(cpu.HL(), result);
        4
    }
}
