use crate::cpu::CPU;
use crate::opcodes::opcode::OPCode;

impl OPCode {
    // 8 bit loads

    // LD r8, r8 0b01xxxyyy
    pub(super) fn op_01xxxyyy(cpu: &mut CPU, bits: &[u8]) -> u8 {
        // release the mutable borrow of cpu
        let src_value = {
            let src = OPCode::get_register_by_index(OPCode::concat_bits(&bits[5..8]), cpu).unwrap();
            *src
        };

        let dst = OPCode::get_register_by_index(OPCode::concat_bits(&bits[2..5]), cpu).unwrap();
        *dst = src_value;
        1
    }

    // LD r8, n8 0b00xxx110
    pub(super) fn op_00xxx110(cpu: &mut CPU, bits: &[u8]) -> u8 {
        // load immediate from PC
        let immediate = cpu.memory_bus.read_byte(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        let dst = OPCode::get_register_by_index(OPCode::concat_bits(&bits[2..5]), cpu).unwrap();
        *dst = immediate;
        2
    }

    // LD r8, (HL) 0b01xxx110
    pub(super) fn op_01xxx110(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let address = cpu.HL();
        let value = cpu.memory_bus.read_byte(address);
        let dst = OPCode::get_register_by_index(OPCode::concat_bits(&bits[2..5]), cpu).unwrap();
        *dst = value;
        2
    }

    // LD (HL), r8 0b01110xxx
    pub(super) fn op_01110xxx(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let address = cpu.HL();
        let src_val = {
            let src = OPCode::get_register_by_index(OPCode::concat_bits(&bits[5..8]), cpu).unwrap();
            *src
        };
        cpu.memory_bus.write_byte(address, src_val);
        2
    }

    // LD (HL), n8 0b00110110
    pub(super) fn op_00110110(cpu: &mut CPU) -> u8 {
        let address = cpu.HL();
        let immediate = cpu.memory_bus.read_byte(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        cpu.memory_bus.write_byte(address, immediate);
        3
    }

    // LD A, (BC) 0b00001010
    pub(super) fn op_00001010(cpu: &mut CPU) -> u8 {
        let address = cpu.bc();
        let value = cpu.memory_bus.read_byte(address);
        cpu.a = value;
        2
    }

    // LD A, (DE) 0b00011010
    pub(super) fn op_00011010(cpu: &mut CPU) -> u8 {
        let address = cpu.de();
        let value = cpu.memory_bus.read_byte(address);
        cpu.a = value;
        2
    }

    // LD (BC), A 0b00000010
    pub(super) fn op_00000010(cpu: &mut CPU) -> u8 {
        let address = cpu.bc();
        cpu.memory_bus.write_byte(address, cpu.a);
        2
    }

    // LD (DE), A 0b00010010
    pub(super) fn op_00010010(cpu: &mut CPU) -> u8 {
        let address = cpu.de();
        cpu.memory_bus.write_byte(address, cpu.a);
        2
    }

    // LD A, (nn) 0b11111010
    pub(super) fn op_11111010(cpu: &mut CPU) -> u8 {
        // load address from PC
        let address = cpu.memory_bus.read_word(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(2);
        let value = cpu.memory_bus.read_byte(address);
        cpu.a = value;
        4
    }

    // LD (nn), A 0b11101010
    pub(super) fn op_11101010(cpu: &mut CPU) -> u8 {
        let address = cpu.memory_bus.read_word(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(2);
        cpu.memory_bus.write_byte(address, cpu.a);
        4
    }

    // LDH A, (C) 0b11110010
    pub(super) fn op_11110010(cpu: &mut CPU) -> u8 {
        let address = 0xFF00 + cpu.c as u16;
        let value = cpu.memory_bus.read_byte(address);
        cpu.a = value;
        2
    }

    // LDH (C), A 0b11100010
    pub(super) fn op_11100010(cpu: &mut CPU) -> u8 {
        let address = 0xFF00 + cpu.c as u16;
        cpu.memory_bus.write_byte(address, cpu.a);
        2
    }

    // LDH A, (n) 0b11110000
    pub(super) fn op_11110000(cpu: &mut CPU) -> u8 {
        let address = cpu.memory_bus.read_byte(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        let address = 0xFF00 + address as u16;
        let value = cpu.memory_bus.read_byte(address);
        cpu.a = value;
        3
    }

    // LDH (n), A 0b11100000
    pub(super) fn op_11100000(cpu: &mut CPU) -> u8 {
        let address = cpu.memory_bus.read_byte(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        let address = 0xFF00 + address as u16;
        cpu.memory_bus.write_byte(address, cpu.a);
        3
    }

    // LD A, (HL-) 0b00111010
    pub(super) fn op_00111010(cpu: &mut CPU) -> u8 {
        let address = cpu.HL();
        let value = cpu.memory_bus.read_byte(address);
        // decrement HL
        cpu.set_HL(address.wrapping_sub(1));
        cpu.a = value;
        2
    }

    // LD (HL-), A 0b00110010
    pub(super) fn op_00110010(cpu: &mut CPU) -> u8 {
        let address = cpu.HL();
        let value = cpu.a;
        // decrement HL
        cpu.set_HL(address.wrapping_sub(1));
        cpu.memory_bus.write_byte(address, value);
        2
    }

    // LD A, (HL+) 0b00101010
    pub(super) fn op_00101010(cpu: &mut CPU) -> u8 {
        let address = cpu.HL();
        let value = cpu.memory_bus.read_byte(address);
        // increment HL
        cpu.set_HL(address.wrapping_add(1));
        cpu.a = value;
        2
    }

    // LD (HL+), A 0b00100010
    pub(super) fn op_00100010(cpu: &mut CPU) -> u8 {
        let address = cpu.HL();
        let value = cpu.a;
        // increment HL
        cpu.set_HL(address.wrapping_add(1));
        cpu.memory_bus.write_byte(address, value);
        2
    }
}

impl OPCode {
    // 16 bit loads

    // LD rr, nn    0b00xx0001
    pub(super) fn op_00xx0001(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let index = OPCode::concat_bits(&bits[2..4]);
        let value = cpu.memory_bus.read_word(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(2);
        OPCode::set_16b_register_by_index(index, cpu, value);
        3
    }

    // LD (nn), SP    0b00001000
    pub(super) fn op_00001000(cpu: &mut CPU) -> u8 {
        let address = cpu.memory_bus.read_word(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(2);
        cpu.memory_bus.write_word(address, cpu.sp);
        5
    }

    // LD SP, HL    0b11111001
    pub(super) fn op_11111001(cpu: &mut CPU) -> u8 {
        cpu.sp = cpu.HL();
        2
    }

    // PUSH rr: Push to stack  0b11xx0101
    pub(super) fn op_11xx0101(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let index = OPCode::concat_bits(&bits[2..4]);
        let value = OPCode::get_16b_register_by_index(index, cpu);
        cpu.sp = cpu.sp.wrapping_sub(2);
        cpu.memory_bus.write_word(cpu.sp, value);
        4
    }

    // POP rr 0b11xx0001
    pub(super) fn op_11xx0001(cpu: &mut CPU, bits: &[u8]) -> u8 {
        let index = OPCode::concat_bits(&bits[2..4]);
        let value = cpu.memory_bus.read_word(cpu.sp);
        cpu.sp = cpu.sp.wrapping_add(2);
        OPCode::set_16b_register_by_index(index, cpu, value);
        3
    }

    // LD HL, SP+e: Load HL from adjusted stack pointer, 0b11111000, set flag
    pub(super) fn op_11111000(cpu: &mut CPU) -> u8 {
        // read immediate from PC
        let e = cpu.memory_bus.read_byte(cpu.pc) as i8;
        cpu.pc = cpu.pc.wrapping_add(1);
        // compute SP+e and set flags
        let half_carry = ((cpu.sp & 0x0F) as u8).wrapping_add((e & 0x0F) as u8) > 0x0F;
        let carry = ((cpu.sp & 0xFF) as u16).wrapping_add(e as u16) > 0xFF;
        cpu.set_Z(false);
        cpu.set_N(false);
        cpu.set_H(half_carry);
        cpu.set_C(carry);
        // set HL
        let result = cpu.sp.wrapping_add(e as u16); // i8 as u16 = 2e16 + i8
        cpu.set_HL(result);
        3
    }
}
