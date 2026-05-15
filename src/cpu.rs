use crate::mmu::MMU;
use crate::registers::Registers;

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
            sp: 0xFFFE,
            pc: 0x0100,

            mmu: MMU::new(),
        }
    }

    pub fn step(&mut self) {
        let instr = self.mmu.read_byte(self.pc);
        let block = (instr & 0b1100_0000) >> 6;
        match block {
            0x00 => self.execute_block0(instr),
            _ => todo!(),
        }
    }

    fn execute_block0(&mut self, instr: u8) {
        match instr {
            0x00 => {
                self.pc += 1;
            } // NOP
            0x10 => {} // STOP
            0x01 | 0x11 | 0x21 | 0x31 => {
                let dest = (0b0011_0000 & instr) >> 4;
                let lo_byte = self.mmu.read_byte(self.pc + 1);
                let hi_byte = self.mmu.read_byte(self.pc + 2);
                let imm16 = (hi_byte as u16) << 8 | lo_byte as u16;

                match dest {
                    0 => self.registers.set_bc(imm16),
                    1 => self.registers.set_de(imm16),
                    2 => self.registers.set_hl(imm16),
                    3 => self.sp = imm16,
                    _ => unreachable!(),
                }

                self.pc += 3;
            } // LD r16, imm16
            0x02 | 0x12 | 0x22 | 0x32 => {} // LD [r16mem], A
            0x0A | 0x1A | 0x2A | 0x3A => {} // LD A, [r16mem]
            0x08 => {} // LD [imm16], SP
            0x03 | 0x13 | 0x23 | 0x33 => {} // INC r16
            0x0B | 0x1B | 0x2B | 0x3B => {} // DEC r16
            0x09 | 0x19 | 0x29 | 0x39 => {} // ADD HL, r16
            0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C => {} // INC r8
            0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D => {} // DEC r8
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => {} // LD r8, imm8
            0x07 => {} // RLCA
            0x0F => {} // RRCA
            0x17 => {} // RLA
            0x1F => {} // RRA
            0x27 => {} // DAA
            0x2F => {} // CPL
            0x37 => {} // SCF
            0x3F => {} // CCF
            0x18 => {} // JR imm8
            0x20 | 0x28 | 0x30 | 0x38 => {} // JR cond, imm8
            _ => panic!("Unknown block 0 instruction: {:#04X}", instr),
        }
    }
}
