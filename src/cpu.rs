use crate::memory;

#[derive(FromPrimitive)]
enum Instruction {
    NOP = 0x00,
    LoadBB = 0x40,
    LoadBC = 0x41,
    LoadBD = 0x42,
    LoadBE = 0x43,
    LoadBH = 0x44,
    LoadBL = 0x45,
    LoadBA = 0x47,
    LoadDB = 0x50,
    LoadDC = 0x51,
    LoadDD = 0x52,
    LoadDE = 0x53,
    LoadDH = 0x54,
    LoadDL = 0x55,
    LoadDA = 0x57,
    LoadHB = 0x60,
    LoadHC = 0x61,
    LoadHD = 0x62,
    LoadHE = 0x63,
    LoadHH = 0x64,
    LoadHL = 0x65,
    LoadHA = 0x67,
}

pub struct Cpu<'a> {
    // General purpose registers
    a: u8,
    b: u8,
    d: u8,
    h: u8,
    f: u8,
    c: u8,
    e: u8,
    l: u8,
    // the program counter
    sp: u16,
    // the stack pointer
    pc: u16,
    memory: &'a mut memory::Memory,
}

impl<'a> Cpu<'a> {
    pub fn new(memory: &'a mut memory::Memory) -> Self {
        Cpu {
            a: 0,
            b: 0,
            d: 0,
            h: 0,
            f: 0,
            c: 0,
            e: 0,
            l: 0,
            sp: 0xfffe,
            pc: 0x100,
            memory: memory,
        }
    }

    fn get_instruction(self: &Self) -> Instruction {
        let data = self.memory.get_data(self.pc);
        num::FromPrimitive::from_u8(data).unwrap()
    }

    fn execute_instruction(self: &mut Self) {
        let instruction = self.get_instruction();
        self.pc += 1;
        match instruction {
            Instruction::NOP => {}
            Instruction::LoadBB => self.b = self.b,
            Instruction::LoadBC => self.b = self.c,
            Instruction::LoadBD => self.b = self.d,
            Instruction::LoadBE => self.b = self.e,
            Instruction::LoadBH => self.b = self.h,
            Instruction::LoadBL => self.b = self.l,
            Instruction::LoadBA => self.b = self.a,
            Instruction::LoadDB => self.d = self.b,
            Instruction::LoadDC => self.d = self.c,
            Instruction::LoadDD => self.d = self.d,
            Instruction::LoadDE => self.d = self.e,
            Instruction::LoadDH => self.d = self.h,
            Instruction::LoadDL => self.d = self.l,
            Instruction::LoadDA => self.d = self.a,
            Instruction::LoadHB => self.h = self.b,
            Instruction::LoadHC => self.h = self.c,
            Instruction::LoadHD => self.h = self.d,
            Instruction::LoadHE => self.h = self.e,
            Instruction::LoadHH => self.h = self.h,
            Instruction::LoadHL => self.h = self.l,
            Instruction::LoadHA => self.h = self.a,
        }
    }
}
