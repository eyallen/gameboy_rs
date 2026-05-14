pub struct MMU {
    pub memory: [u8; 0x10000],
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            memory: [0; 0x10000],
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write_byte(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}
