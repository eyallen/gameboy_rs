use gameboy::registers::{CpuFlag, Registers};

fn make() -> Registers {
    Registers::new()
}

#[test]
fn get_set_af() {
    let mut r = make();
    r.set_af(0xABF0);
    assert_eq!(r.get_af(), 0xABF0);
    assert_eq!(r.a, 0xAB);
}

#[test]
fn get_set_bc() {
    let mut r = make();
    r.set_bc(0x1234);
    assert_eq!(r.get_bc(), 0x1234);
    assert_eq!(r.b, 0x12);
    assert_eq!(r.c, 0x34);
}

#[test]
fn get_set_de() {
    let mut r = make();
    r.set_de(0x5678);
    assert_eq!(r.get_de(), 0x5678);
    assert_eq!(r.d, 0x56);
    assert_eq!(r.e, 0x78);
}

#[test]
fn get_set_hl() {
    let mut r = make();
    r.set_hl(0x9ABC);
    assert_eq!(r.get_hl(), 0x9ABC);
    assert_eq!(r.h, 0x9A);
    assert_eq!(r.l, 0xBC);
}

#[test]
fn flags_set_and_get() {
    let mut r = make();
    r.set_flag(CpuFlag::Z, true);
    assert!(r.get_flag(CpuFlag::Z));
    assert!(!r.get_flag(CpuFlag::C));
}

#[test]
fn flags_clear() {
    let mut r = make();
    r.set_flag(CpuFlag::Z, true);
    r.set_flag(CpuFlag::Z, false);
    assert!(!r.get_flag(CpuFlag::Z));
}

#[test]
fn flags_lower_nibble_masked() {
    let mut r = make();
    // The F register lower nibble must always read as zero
    r.set_af(0x00FF);
    assert_eq!(r.get_af() & 0x000F, 0);
}
