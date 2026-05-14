use crate::registers::Registers;
use crate::mmu::MMU;

pub struct CPU {
    pub registers: Registers,
    pub sp: u16,
    pub pc: u16,

    pub mmu: MMU,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            sp: 0,
            pc: 0,

            mmu: MMU::new(),
        }
    }
}
