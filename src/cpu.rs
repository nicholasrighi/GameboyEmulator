use crate::memory;

#[derive(FromPrimitive)]
enum Instruction {
    NOP = 0x00,
    // Add of the LD B X instructions
    LoadBB = 0x40,
    LoadBC = 0x41,
    LoadBD = 0x42,
    LoadBE = 0x43,
    LoadBH = 0x44,
    LoadBL = 0x45,
    LoadBA = 0x47,
    // Add of the LD C X instructions
    LoadCB = 0x48,
    LoadCC = 0x49,
    LoadCD = 0x4A,
    LoadCE = 0x4B,
    LoadCH = 0x4C,
    LoadCL = 0x4D,
    LoadCA = 0x4F,
    // Add of the LD D X instructions
    LoadDB = 0x50,
    LoadDC = 0x51,
    LoadDD = 0x52,
    LoadDE = 0x53,
    LoadDH = 0x54,
    LoadDL = 0x55,
    LoadDA = 0x57,
    // Add of the LD E X instructions
    LoadEB = 0x58,
    LoadEC = 0x59,
    LoadED = 0x5A,
    LoadEE = 0x5B,
    LoadEH = 0x5C,
    LoadEL = 0x5D,
    LoadEA = 0x5F,
    // Add of the LD H X instructions
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
            // Add of the LD B X instructions
            Instruction::LoadBB => self.b = self.b,
            Instruction::LoadBC => self.b = self.c,
            Instruction::LoadBD => self.b = self.d,
            Instruction::LoadBE => self.b = self.e,
            Instruction::LoadBH => self.b = self.h,
            Instruction::LoadBL => self.b = self.l,
            Instruction::LoadBA => self.b = self.a,
            // Add of the LD C X instructions
            Instruction::LoadCB => self.c = self.b,
            Instruction::LoadCC => self.c = self.c,
            Instruction::LoadCD => self.c = self.d,
            Instruction::LoadCE => self.c = self.e,
            Instruction::LoadCH => self.c = self.h,
            Instruction::LoadCL => self.c = self.l,
            Instruction::LoadCA => self.c = self.a,
            // Add of the LD D X instructions
            Instruction::LoadDB => self.d = self.b,
            Instruction::LoadDC => self.d = self.c,
            Instruction::LoadDD => self.d = self.d,
            Instruction::LoadDE => self.d = self.e,
            Instruction::LoadDH => self.d = self.h,
            Instruction::LoadDL => self.d = self.l,
            Instruction::LoadDA => self.d = self.a,
            // Add of the LD E X instructions
            Instruction::LoadEB => self.e = self.b,
            Instruction::LoadEC => self.e = self.c,
            Instruction::LoadED => self.e = self.d,
            Instruction::LoadEE => self.e = self.e,
            Instruction::LoadEH => self.e = self.h,
            Instruction::LoadEL => self.e = self.l,
            Instruction::LoadEA => self.e = self.a,
            // Add of the LD H X instructions
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
