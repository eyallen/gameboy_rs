use gameboy::mmu::MMU;

fn make() -> MMU {
    MMU::new()
}

#[test]
fn initial_memory_is_zero() {
    let mmu = make();
    assert_eq!(mmu.read_byte(0x0000), 0);
    assert_eq!(mmu.read_byte(0xFFFF), 0);
}

#[test]
fn write_then_read() {
    let mut mmu = make();
    mmu.write_byte(0x0100, 0xAB);
    assert_eq!(mmu.read_byte(0x0100), 0xAB);
}

#[test]
fn write_does_not_affect_neighbors() {
    let mut mmu = make();
    mmu.write_byte(0x0100, 0xFF);
    assert_eq!(mmu.read_byte(0x00FF), 0);
    assert_eq!(mmu.read_byte(0x0101), 0);
}

#[test]
fn write_boundary_addresses() {
    let mut mmu = make();
    mmu.write_byte(0x0000, 0x01);
    mmu.write_byte(0xFFFF, 0x02);
    assert_eq!(mmu.read_byte(0x0000), 0x01);
    assert_eq!(mmu.read_byte(0xFFFF), 0x02);
}
