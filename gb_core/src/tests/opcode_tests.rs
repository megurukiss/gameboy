use crate::cpu::CPU;
use crate::opcodes::opcode::OPCode;

#[test]
fn test_op_0b11111000() {
    let mut cpu = CPU::new();
    cpu.sp = 0b00000000_11111111;
    cpu.pc = 0xC000;
    cpu.memory_bus.write_byte(cpu.pc, 0b00000001);
    // LD HL, both flags set
    OPCode::exec(&mut cpu, 0b11111000, false);
    assert_eq!(cpu.HL(), 0b00000001_00000000);
    assert_eq!(cpu.f, 0b00110000);

    cpu.sp = 0b00000000_11111111;
    cpu.pc = 0xC000;
    cpu.memory_bus.write_byte(cpu.pc, 0);
    // LD HL, both flags unset
    OPCode::exec(&mut cpu, 0b11111000, false);
    assert_eq!(cpu.HL(), 0b00000000_11111111);
    assert_eq!(cpu.f, 0b00000000);

    cpu.sp = 0b00000000_11101111;
    cpu.pc = 0xC000;
    cpu.memory_bus.write_byte(cpu.pc, 0b00000001);
    // LD HL, H flag set, N flag unset
    OPCode::exec(&mut cpu, 0b11111000, false);
    assert_eq!(cpu.HL(), 0b00000000_11110000);
    assert_eq!(cpu.f, 0b00100000);

    cpu.sp = 0b00000000_11110000;
    cpu.pc = 0xC000;
    cpu.memory_bus.write_byte(cpu.pc, 0b00010000);
    // LD HL, H flag unset, N flag
    OPCode::exec(&mut cpu, 0b11111000, false);
    assert_eq!(cpu.HL(), 0b00000001_00000000);
    assert_eq!(cpu.f, 0b00010000);
}
