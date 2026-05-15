use gameboy::cpu::CPU;

#[test]
fn new_initializes_pc_and_sp() {
    let cpu = CPU::new();
    assert_eq!(cpu.pc, 0x0100);
    assert_eq!(cpu.sp, 0xFFFE);
}

#[test]
fn new_initializes_registers_to_post_boot_state() {
    let cpu = CPU::new();
    assert_eq!(cpu.registers.a, 0x01);
    assert_eq!(cpu.registers.b, 0x00);
    assert_eq!(cpu.registers.c, 0x13);
    assert_eq!(cpu.registers.d, 0x00);
    assert_eq!(cpu.registers.e, 0xD8);
    assert_eq!(cpu.registers.h, 0x01);
    assert_eq!(cpu.registers.l, 0x4D);
}
