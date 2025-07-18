use crate::core::CPU;
use crate::opcodes::opcode::OPCode;

#[test]
fn test_set_register_macro() {
    let mut cpu = CPU::new();
    OPCode::set_16b_register_by_index(0b00, &mut cpu, 0x1234);
    assert_eq!(cpu.b, 0x12);
    assert_eq!(cpu.c, 0x34);

    OPCode::set_16b_register_by_index(0b01, &mut cpu, 0x5678);
    assert_eq!(cpu.d, 0x56);
    assert_eq!(cpu.e, 0x78);

    OPCode::set_16b_register_by_index(0b10, &mut cpu, 0x9ABC);
    assert_eq!(cpu.h, 0x9A);
    assert_eq!(cpu.l, 0xBC);

    OPCode::set_16b_register_by_index(0b11, &mut cpu, 0xDEF0);
    assert_eq!(cpu.sp, 0xDEF0);
}

#[test]
fn test_get_register_macro() {
    let mut cpu = CPU::new();
    cpu.b = 0x12;
    cpu.c = 0x34;
    assert_eq!(OPCode::get_16b_register_by_index(0b00, &cpu), 0x1234);

    cpu.d = 0x56;
    cpu.e = 0x78;
    assert_eq!(OPCode::get_16b_register_by_index(0b01, &cpu), 0x5678);

    cpu.h = 0x9A;
    cpu.l = 0xBC;
    assert_eq!(OPCode::get_16b_register_by_index(0b10, &cpu), 0x9ABC);

    cpu.sp = 0xDEF0;
    assert_eq!(OPCode::get_16b_register_by_index(0b11, &cpu), 0xDEF0);
}

#[test]
fn test_flags() {
    let mut cpu = CPU::new();
    cpu.set_z(true);
    assert!(cpu.z());
    cpu.set_z(false);
    assert!(!cpu.z());
    cpu.set_n(true);
    assert!(cpu.n());

    cpu.set_n(false);
    assert!(!cpu.n());
    cpu.set_h(true);
    assert!(cpu.h());
    cpu.set_h(false);
    assert!(!cpu.h());
    cpu.set_c(true);
    assert!(cpu.c());
    cpu.set_c(false);
    assert!(!cpu.c());
}
