pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    f: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16
}

pub enum CpuFlag {
    C = 0b00010000,
    H = 0b00100000,
    N = 0b01000000,
    Z = 0b10000000,
}

impl Registers {
    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | self.f
    }

    pub fn set_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f = (val & 0x00FF) as u8;
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | self.c
    }

    pub fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = (val & 0x00FF) as u8;
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | self.e
    }

    pub fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = (val & 0x00FF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | self.l
    }

    pub fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = (val & 0x00FF) as u8;
    }

    pub fn set_flag(&mut self, flag: CpuFlag, set: bool) {
        let mask = flag as u8;
        match set {
            true => self.f |= mask,
            false => self.f &= !mask,
        }
        self.f &= 0xF0;
    }

    pub fn get_flag(&mut self, flag: CpuFlag) -> bool {
        let mask = flag as u8;
        self.f & mask > 0
    }
}
