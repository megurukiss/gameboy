use crate::core::Error;
use crate::core::CPU;

pub struct OPCode;

impl OPCode {
    pub fn get_register_by_index<'a>(index: u8, cpu: &'a mut CPU) -> Result<&'a mut u8, Error> {
        match index {
            0b111 => Ok(&mut cpu.a),
            0b000 => Ok(&mut cpu.b),
            0b001 => Ok(&mut cpu.c),
            0b010 => Ok(&mut cpu.d),
            0b011 => Ok(&mut cpu.e),
            0b100 => Ok(&mut cpu.h),
            0b101 => Ok(&mut cpu.l),
            _ => Err(Error::OPCodeParseError),
        }
    }

    pub fn set_16b_register_by_index(index: u8, cpu: &mut CPU, value: u16) {
        match index {
            0b00 => cpu.set_bc(value),
            0b01 => cpu.set_de(value),
            0b10 => cpu.set_hl(value),
            0b11 => cpu.set_sp(value),
            _ => panic!("Invalid 16-bit register index"),
        }
    }

    pub fn get_16b_register_by_index(index: u8, cpu: &CPU) -> u16 {
        match index {
            0b00 => cpu.bc(),
            0b01 => cpu.de(),
            0b10 => cpu.hl(),
            0b11 => cpu.sp,
            _ => panic!("Invalid 16-bit register index"),
        }
    }

    pub fn fetch_opcode_u8(cpu: &mut CPU) -> Result<u8, Error> {
        // TODO: Implement address boundary check
        let opcode = cpu.memory_bus.read_byte(cpu.pc);
        cpu.pc += 1;
        Ok(opcode)
    }

    pub fn fetch_opcode_u16(cpu: &mut CPU) -> Result<u16, Error> {
        // TODO: Implement address boundary check
        let opcode = cpu.memory_bus.read_word(cpu.pc);
        cpu.pc += 2;
        Ok(opcode)
    }

    pub fn parse_bits_u8(opcode: u8) -> [u8; 8] {
        let mut opcode_bits = [0; 8];
        for i in 0..8 {
            opcode_bits[i] = (opcode >> (7 - i)) & 1;
        }
        opcode_bits
    }

    pub fn parse_bits_u16(opcode: u16) -> [u8; 16] {
        let mut opcode_bits = [0u8; 16];
        for i in 0..16 {
            opcode_bits[i] = ((opcode >> (15 - i)) & 1) as u8;
        }
        opcode_bits
    }

    pub fn concat_bits(bits: &[u8]) -> u8 {
        let mut result = 0;
        let bit_len = bits.len();
        for i in 0..bit_len {
            result |= bits[i] << (bit_len - i - 1);
        }
        result
    }

    pub fn concat_bits_u16(bits: &[u8]) -> u16 {
        let mut result = 0u16;
        let bit_len = bits.len();
        for i in 0..bit_len {
            result |= (bits[i] << (bit_len - i - 1)) as u16;
        }
        result
    }

    // execute the opcode
    pub fn exec(cpu: &mut CPU, opcode: u8, is_cb: bool) -> u8 {
        let opcode_bits = OPCode::parse_bits_u8(opcode);
        if !is_cb {
            match opcode_bits {
                // TODO: move specified opcodes first

                // --- UnReachable Opcodes ---

                // CP (HL) 0b10011110
                [1, 0, 1, 1, 1, 1, 1, 0] => OPCode::op_10111110(cpu),

                // CP n 0b11111110
                [1, 1, 1, 1, 1, 1, 1, 0] => OPCode::op_11111110(cpu),

                // SBC (HL) 0b10011110
                [1, 0, 0, 1, 1, 1, 1, 0] => OPCode::op_10011110(cpu),

                // SBC n 0b11011110
                [1, 1, 0, 1, 1, 1, 1, 0] => OPCode::op_11011110(cpu),

                // HALT 01110110
                [0, 1, 1, 1, 0, 1, 1, 0] => OPCode::op_01110110(cpu),

                // --- 8-bit loads ---

                // LD (HL), n8: 0b00110110 (0x36)
                [0, 0, 1, 1, 0, 1, 1, 0] => OPCode::op_00110110(cpu),

                // LD r8, n8: 0b00xxx110
                [0, 0, _, _, _, 1, 1, 0] => OPCode::op_00xxx110(cpu, &opcode_bits),

                // LD r8, (HL): 0b01xxx110
                [0, 1, _, _, _, 1, 1, 0] => OPCode::op_01xxx110(cpu, &opcode_bits),

                // LD (HL), r8: 0b01110xxx
                [0, 1, 1, 1, 0, _, _, _] => OPCode::op_01110xxx(cpu, &opcode_bits),

                // LD r8, r8: 0b01xxxyyy
                [0, 1, _, _, _, _, _, _] => OPCode::op_01xxxyyy(cpu, &opcode_bits),

                // LD A, (BC): 0b00001010 (0x0A)
                [0, 0, 0, 0, 1, 0, 1, 0] => OPCode::op_00001010(cpu),

                // LD A, (DE): 0b00011010 (0x1A)
                [0, 0, 0, 1, 1, 0, 1, 0] => OPCode::op_00011010(cpu),

                // LD (BC), A: 0b00000010 (0x02)
                [0, 0, 0, 0, 0, 0, 1, 0] => OPCode::op_00000010(cpu),

                // LD (DE), A: 0b00010010 (0x12)
                [0, 0, 0, 1, 0, 0, 1, 0] => OPCode::op_00010010(cpu),

                // LD A, (nn): 0b11111010 (0xFA)
                [1, 1, 1, 1, 1, 0, 1, 0] => OPCode::op_11111010(cpu),

                // LD (nn), A: 0b11101010 (0xEA)
                [1, 1, 1, 0, 1, 0, 1, 0] => OPCode::op_11101010(cpu),

                // LDH A, (C): 0b11110010 (0xF2) (LD A, ($FF00+C))
                [1, 1, 1, 1, 0, 0, 1, 0] => OPCode::op_11110010(cpu),

                // LDH (C), A: 0b11100010 (0xE2) (LD ($FF00+C), A)
                [1, 1, 1, 0, 0, 0, 1, 0] => OPCode::op_11100010(cpu),

                // LDH A, (n): 0b11110000 (0xF0) (LD A, ($FF00+n))
                [1, 1, 1, 1, 0, 0, 0, 0] => OPCode::op_11110000(cpu),

                // LDH (n), A: 0b11100000 (0xE0) (LD ($FF00+n), A)
                [1, 1, 1, 0, 0, 0, 0, 0] => OPCode::op_11100000(cpu),

                // LD A, (HL-): 0b00111010 (0x3A)
                [0, 0, 1, 1, 1, 0, 1, 0] => OPCode::op_00111010(cpu),

                // LD (HL-), A: 0b00110010 (0x32)
                [0, 0, 1, 1, 0, 0, 1, 0] => OPCode::op_00110010(cpu),

                // LD A, (HL+): 0b00101010 (0x2A)
                [0, 0, 1, 0, 1, 0, 1, 0] => OPCode::op_00101010(cpu),

                // LD (HL+), A: 0b00100010 (0x22)
                [0, 0, 1, 0, 0, 0, 1, 0] => OPCode::op_00100010(cpu),

                // --- 16-bit loads ---

                // LD rr, nn: 0b00xx0001
                [0, 0, _, _, 0, 0, 0, 1] => OPCode::op_00xx0001(cpu, &opcode_bits),

                // LD (nn), SP: 0b00001000 (0x08)
                [0, 0, 0, 0, 1, 0, 0, 0] => OPCode::op_00001000(cpu),

                // LD SP, HL: 0b11111001 (0xF9)
                [1, 1, 1, 1, 1, 0, 0, 1] => OPCode::op_11111001(cpu),

                // PUSH rr: 0b11xx0101
                [1, 1, _, _, 0, 1, 0, 1] => OPCode::op_11xx0101(cpu, &opcode_bits),

                // POP rr: 0b11xx0001
                [1, 1, _, _, 0, 0, 0, 1] => OPCode::op_11xx0001(cpu, &opcode_bits),

                // LD HL, SP+e: 0b11111000 (0xF8) (LDHL SP, e)
                [1, 1, 1, 1, 1, 0, 0, 0] => OPCode::op_11111000(cpu),

                // --- add ---

                // ADD (HL) 0b10000110
                [1, 0, 0, 0, 0, 1, 1, 0] => OPCode::op_10000110(cpu),

                // ADD n: 0b11000110
                [1, 1, 0, 0, 0, 1, 1, 0] => OPCode::op_11000110(cpu),

                // ADC (HL) 0b10001110
                [1, 0, 0, 0, 1, 1, 1, 0] => OPCode::op_10001110(cpu),

                // ADC n 0b11001110
                [1, 1, 0, 0, 1, 1, 1, 0] => OPCode::op_11001110(cpu),

                //ADD r: 0b10000xxx
                [1, 0, 0, 0, 0, _, _, _] => OPCode::op_10000xxx(cpu, &opcode_bits),

                // ADC r 0b10001xxx
                [1, 0, 0, 0, 1, _, _, _] => OPCode::op_10001xxx(cpu, &opcode_bits),

                // --- sub ---

                // SUB (HL) 0b10010110
                [1, 0, 0, 1, 0, 1, 1, 0] => OPCode::op_10010110(cpu),

                // SUB n 0b11010110
                [1, 1, 0, 1, 0, 1, 1, 0] => OPCode::op_11010110(cpu),

                // SBC r 0b10011xxx
                [1, 0, 0, 1, 1, _, _, _] => OPCode::op_10011xxx(cpu, &opcode_bits),

                // SUB r 0b10010xxx
                [1, 0, 0, 1, 0, _, _, _] => OPCode::op_10010xxx(cpu, &opcode_bits),

                // --- logical ---

                // AND (HL) 0b10100110
                [1, 0, 1, 0, 0, 1, 1, 0] => OPCode::op_10100110(cpu),

                // AND n 0b11100110
                [1, 1, 1, 0, 0, 1, 1, 0] => OPCode::op_11100110(cpu),

                // OR (HL) 0b10110110
                [1, 0, 1, 1, 0, 1, 1, 0] => OPCode::op_10110110(cpu),

                // OR n 0b11110110
                [1, 1, 1, 1, 0, 1, 1, 0] => OPCode::op_11110110(cpu),

                // XOR (HL) 0b10101110
                [1, 0, 1, 0, 1, 1, 1, 0] => OPCode::op_10101110(cpu),
                // XOR n 0b11101110
                [1, 1, 1, 0, 1, 1, 1, 0] => OPCode::op_11101110(cpu),

                // XOR r 0b10101xxx
                [1, 0, 1, 0, 1, _, _, _] => OPCode::op_10101xxx(cpu, &opcode_bits),

                // OR r  0b10110xxx
                [1, 0, 1, 1, 0, _, _, _] => OPCode::op_10110xxx(cpu, &opcode_bits),

                // AND r 0b10100xxx
                [1, 0, 1, 0, 0, _, _, _] => OPCode::op_10100xxx(cpu, &opcode_bits),

                // --- other ALU ---

                // INC (HL) 0b00110100
                [0, 0, 1, 1, 0, 1, 0, 0] => OPCode::op_00110100(cpu),

                // DEC (HL) 0b00110101
                [0, 0, 1, 1, 0, 1, 0, 1] => OPCode::op_00110101(cpu),

                // CCF 0b00111111
                [0, 0, 1, 1, 1, 1, 1, 1] => OPCode::op_00111111(cpu),

                // SCF 0b00110111
                [0, 0, 1, 1, 0, 1, 1, 1] => OPCode::op_00110111(cpu),

                // DAA 0b00100111
                [0, 0, 1, 0, 0, 1, 1, 1] => OPCode::op_00100111(cpu),

                // CPL 0b00101111
                [0, 0, 1, 0, 1, 1, 1, 1] => OPCode::op_00101111(cpu),

                // CP r 0b10111xxx
                [1, 0, 1, 1, 1, _, _, _] => OPCode::op_10111xxx(cpu, &opcode_bits),

                // INC r 0b00xxx100
                [0, 0, _, _, _, 1, 0, 0] => OPCode::op_00xxx100(cpu, &opcode_bits),

                // DEC r 0b00xxx101
                [0, 0, _, _, _, 1, 0, 1] => OPCode::op_00xxx101(cpu, &opcode_bits),

                // --- 16-bit ALU ---

                // ADD SP, e  11101000
                [1, 1, 1, 0, 1, 0, 0, 0] => OPCode::op_11101000(cpu),

                // INC rr 0b00xx0011
                [0, 0, _, _, 0, 0, 1, 1] => OPCode::op_00xx0011(cpu, &opcode_bits),

                // DEC rr 0b00xx1011
                [0, 0, _, _, 1, 0, 1, 1] => OPCode::op_00xx1011(cpu, &opcode_bits),

                // ADD HL, rr 0b00xx1001
                [0, 0, _, _, 1, 0, 0, 1] => OPCode::op_00xx1001(cpu, &opcode_bits),

                // --- Single Byte Rotations ---
                // RLCA 00000111
                [0, 0, 0, 0, 0, 1, 1, 1] => OPCode::cb_op_00000111(cpu),

                // RRCA 00001111
                [0, 0, 0, 0, 1, 1, 1, 1] => OPCode::cb_op_00001111(cpu),

                // RLA 00010111
                [0, 0, 0, 1, 0, 1, 1, 1] => OPCode::cb_op_00010111(cpu),

                // RRA 00011111
                [0, 0, 0, 1, 1, 1, 1, 1] => OPCode::cb_op_00011111(cpu),

                // --- control flow ---
                // JP nn 11000011
                [1, 1, 0, 0, 0, 0, 1, 1] => OPCode::op_11000011(cpu),

                // JP HL 11101001
                [1, 1, 1, 0, 1, 0, 0, 1] => OPCode::op_11101001(cpu),

                // JP cc, nn 110xx010
                [1, 1, 0, _, _, 0, 1, 0] => OPCode::op_110xx010(cpu, &opcode_bits),

                // JR e 00011000
                [0, 0, 0, 1, 1, 0, 0, 0] => OPCode::op_00011000(cpu),

                // JR cc, e 001xx000
                [0, 0, 1, _, _, 0, 0, 0] => OPCode::op_001xx000(cpu, &opcode_bits),

                // CALL nn 11001101
                [1, 1, 0, 0, 1, 1, 0, 1] => OPCode::op_11001101(cpu),

                // CALL cc, nn 110xx100
                [1, 1, 0, _, _, 1, 0, 0] => OPCode::op_110xx100(cpu, &opcode_bits),

                // RET 11001001
                [1, 1, 0, 0, 1, 0, 0, 1] => OPCode::op_11001001(cpu),

                // RET cc 110xx000
                [1, 1, 0, _, _, 0, 0, 0] => OPCode::op_110xx000(cpu, &opcode_bits),

                // RETI 11011001
                [1, 1, 0, 1, 1, 0, 0, 1] => OPCode::op_11011001(cpu),

                // RST n 11xxx111
                [1, 1, _, _, _, 1, 1, 1] => OPCode::op_11xxx111(cpu, &opcode_bits),

                // --- Miscellaneous instructions ---

                // STOP 00010000 00000000
                // STOP needs a special handler

                // DI 11110011
                [1, 1, 1, 1, 0, 0, 1, 1] => OPCode::op_11110011(cpu),

                // EI 11111011
                [1, 1, 1, 1, 1, 0, 1, 1] => OPCode::op_11111011(cpu),

                // NOP 00000000
                [0, 0, 0, 0, 0, 0, 0, 0] => OPCode::op_00000000(),

                _ => panic!("Invalid opcode"),
            }
        } else {
            match opcode_bits {
                // TODO: add bit opcodes

                // --- Bit Operations ---

                // RLC (HL) 00000110
                [0, 0, 0, 0, 0, 1, 1, 0] => OPCode::cb_op_00000110(cpu),

                //RRC (HL) 00001110
                [0, 0, 0, 0, 1, 1, 1, 0] => OPCode::cb_op_00001110(cpu),

                // RL (HL) 00010110
                [0, 0, 0, 1, 0, 1, 1, 0] => OPCode::cb_op_00010110(cpu),

                // RR (HL) 00011110
                [0, 0, 0, 1, 1, 1, 1, 0] => OPCode::cb_op_00011110(cpu),

                // SLA (HL) 00100110
                [0, 0, 1, 0, 0, 1, 1, 0] => OPCode::cb_op_00100110(cpu),

                // SRA (HL) 00101110
                [0, 0, 1, 0, 1, 1, 1, 0] => OPCode::cb_op_00101110(cpu),

                // SWAP (HL) 00110110
                [0, 0, 1, 1, 0, 1, 1, 0] => OPCode::cb_op_00110110(cpu),

                // SRL (HL) 00111110
                [0, 0, 1, 1, 1, 1, 1, 0] => OPCode::cb_op_00111110(cpu),

                // SET b,(HL) 11xxx110
                [1, 1, _, _, _, 1, 1, 0] => OPCode::cb_op_11xxx110(cpu, &opcode_bits),

                // RES b, (HL) 10xxx110
                [1, 0, _, _, _, 1, 1, 0] => OPCode::cb_op_10xxx110(cpu, &opcode_bits),

                // BIT b, (HL) 01xxx110
                [0, 1, _, _, _, 1, 1, 0] => OPCode::cb_op_01xxx110(cpu, &opcode_bits),

                // RLC r 00000xxx
                [0, 0, 0, 0, 0, _, _, _] => OPCode::cb_op_00000xxx(cpu, &opcode_bits),

                // RRC r 00001xxx
                [0, 0, 0, 0, 1, _, _, _] => OPCode::cb_op_00001xxx(cpu, &opcode_bits),

                // RL r 00010xxx
                [0, 0, 0, 1, 0, _, _, _] => OPCode::cb_op_00010xxx(cpu, &opcode_bits),

                // RR r8 00011xxx
                [0, 0, 0, 1, 1, _, _, _] => OPCode::cb_op_00011xxx(cpu, &opcode_bits),

                // SLA r8 00100xxx
                [0, 0, 1, 0, 0, _, _, _] => OPCode::cb_op_00100xxx(cpu, &opcode_bits),

                // SRA r 00101xxx
                [0, 0, 1, 0, 1, _, _, _] => OPCode::cb_op_00101xxx(cpu, &opcode_bits),

                // SWAP r 00110xxx
                [0, 0, 1, 1, 0, _, _, _] => OPCode::cb_op_00110xxx(cpu, &opcode_bits),

                // SRL r 00111xxx
                [0, 0, 1, 1, 1, _, _, _] => OPCode::cb_op_00111xxx(cpu, &opcode_bits),

                // BIT b, r 01xxxxxx
                [0, 1, _, _, _, _, _, _] => OPCode::cb_op_01xxxxxx(cpu, &opcode_bits),

                // RES b, r 10xxxxxx
                [1, 0, _, _, _, _, _, _] => OPCode::cb_op_10xxxxxx(cpu, &opcode_bits),

                // SET b, r 11xxxxxx
                [1, 1, _, _, _, _, _, _] => OPCode::cb_op_11xxxxxx(cpu, &opcode_bits),

                _ => panic!("Invalid opcode"),
            }
        }
    }

    // TODO: execute the stop instruction
    pub fn exec_stop(cpu: &mut CPU) -> u8 {
        unimplemented!()
    }
}

// macro_rules! set_16b_register_by_index {
//     ($index:expr, $cpu:expr, $value:expr) => {
//         match $index {
//             0b00 => $cpu.set_BC($value),
//             0b01 => $cpu.set_DE($value),
//             0b10 => $cpu.set_HL($value),
//             0b11 => $cpu.set_SP($value),
//             _ => panic!("Invalid 16-bit register index"),
//         }
//     };
// }

// macro_rules! get_16b_register_by_index {
//     ($index: expr, $cpu: expr) => {
//         match $index {
//             0b00 => $cpu.BC(),
//             0b01 => $cpu.DE(),
//             0b10 => $cpu.HL(),
//             0b11 => $cpu.SP,
//             _ => panic!("Invalid 16-bit register index"),
//         }
//     };
// }

// pub(crate) use get_16b_register_by_index;
// pub(crate) use set_16b_register_by_index;
