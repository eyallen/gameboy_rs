use gameboy::cpu::CPU;

#[test]
fn new_initializes_pc_and_sp_to_zero() {
    let cpu = CPU::new();
    assert_eq!(cpu.pc, 0);
    assert_eq!(cpu.sp, 0);
}

#[test]
fn new_initializes_registers_to_zero() {
    let cpu = CPU::new();
    assert_eq!(cpu.registers.a, 0);
    assert_eq!(cpu.registers.b, 0);
    assert_eq!(cpu.registers.c, 0);
    assert_eq!(cpu.registers.d, 0);
    assert_eq!(cpu.registers.e, 0);
    assert_eq!(cpu.registers.h, 0);
    assert_eq!(cpu.registers.l, 0);
}
