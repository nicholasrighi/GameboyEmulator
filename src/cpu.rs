use crate::memory;
use bitflags::bitflags;

const INITIAL_PC: u16 = 0x100;
const INITIAL_SP: u16 = 0xFFFE;

#[derive(FromPrimitive)]
enum Instruction {
    NOP = 0x00,
    // the LD B X instructions
    LoadBB = 0x40,
    LoadBC = 0x41,
    LoadBD = 0x42,
    LoadBE = 0x43,
    LoadBH = 0x44,
    LoadBL = 0x45,
    LoadBA = 0x47,
    // the LD C X instructions
    LoadCB = 0x48,
    LoadCC = 0x49,
    LoadCD = 0x4A,
    LoadCE = 0x4B,
    LoadCH = 0x4C,
    LoadCL = 0x4D,
    LoadCA = 0x4F,
    // the LD D X instructions
    LoadDB = 0x50,
    LoadDC = 0x51,
    LoadDD = 0x52,
    LoadDE = 0x53,
    LoadDH = 0x54,
    LoadDL = 0x55,
    LoadDA = 0x57,
    // the LD E X instructions
    LoadEB = 0x58,
    LoadEC = 0x59,
    LoadED = 0x5A,
    LoadEE = 0x5B,
    LoadEH = 0x5C,
    LoadEL = 0x5D,
    LoadEA = 0x5F,
    // the LD H X instructions
    LoadHB = 0x60,
    LoadHC = 0x61,
    LoadHD = 0x62,
    LoadHE = 0x63,
    LoadHH = 0x64,
    LoadHL = 0x65,
    LoadHA = 0x67,
    // the LD L X instructions
    LoadLB = 0x68,
    LoadLC = 0x69,
    LoadLD = 0x6A,
    LoadLE = 0x6B,
    LoadLH = 0x6C,
    LoadLL = 0x6D,
    LoadLA = 0x6F,
    // the Add A X instruction
    AddAB = 0x80,
    AddAC = 0x81,
    AddAD = 0x82,
    AddAE = 0x83,
    AddAH = 0x84,
    AddAL = 0x85,
    AddAA = 0x87,
    // the And A X instruction
    AndAB = 0xA0,
    AndAC = 0xA1,
    AndAD = 0xA2,
    AndAE = 0xA3,
    AndAH = 0xA4,
    AndAL = 0xA5,
    AndAA = 0xA7,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct CpuFlags: u8 {
        const ZERO_FLAG = 0b10000000;
        const SUBTRACTION_FLAG = 0b01000000;
        const HALF_CARRY_FLAG = 0b00100000;
        const CARRY_FLAG = 0b00010000;
    }
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
    // this is the f register
    flags: CpuFlags,
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
            flags: CpuFlags::empty(),
            sp: INITIAL_SP,
            pc: INITIAL_PC,
            memory: memory,
        }
    }

    fn get_instruction(self: &Self) -> Instruction {
        let data = self.memory.get_data(self.pc);
        num::FromPrimitive::from_u8(data).unwrap()
    }

    fn clear_flags(self: &mut Self) {
        self.flags = CpuFlags::empty();
    }

    fn execute_instruction(self: &mut Self) {
        let instruction = self.get_instruction();
        self.pc += 1;
        match instruction {
            Instruction::NOP => {}
            // Implement the LD B X instructions
            Instruction::LoadBB => self.b = self.b,
            Instruction::LoadBC => self.b = self.c,
            Instruction::LoadBD => self.b = self.d,
            Instruction::LoadBE => self.b = self.e,
            Instruction::LoadBH => self.b = self.h,
            Instruction::LoadBL => self.b = self.l,
            Instruction::LoadBA => self.b = self.a,
            // Implement the LD C X instructions
            Instruction::LoadCB => self.c = self.b,
            Instruction::LoadCC => self.c = self.c,
            Instruction::LoadCD => self.c = self.d,
            Instruction::LoadCE => self.c = self.e,
            Instruction::LoadCH => self.c = self.h,
            Instruction::LoadCL => self.c = self.l,
            Instruction::LoadCA => self.c = self.a,
            // Implement the LD D X instructions
            Instruction::LoadDB => self.d = self.b,
            Instruction::LoadDC => self.d = self.c,
            Instruction::LoadDD => self.d = self.d,
            Instruction::LoadDE => self.d = self.e,
            Instruction::LoadDH => self.d = self.h,
            Instruction::LoadDL => self.d = self.l,
            Instruction::LoadDA => self.d = self.a,
            // Implement the LD E X instructions
            Instruction::LoadEB => self.e = self.b,
            Instruction::LoadEC => self.e = self.c,
            Instruction::LoadED => self.e = self.d,
            Instruction::LoadEE => self.e = self.e,
            Instruction::LoadEH => self.e = self.h,
            Instruction::LoadEL => self.e = self.l,
            Instruction::LoadEA => self.e = self.a,
            // Implement the LD H X instructions
            Instruction::LoadHB => self.h = self.b,
            Instruction::LoadHC => self.h = self.c,
            Instruction::LoadHD => self.h = self.d,
            Instruction::LoadHE => self.h = self.e,
            Instruction::LoadHH => self.h = self.h,
            Instruction::LoadHL => self.h = self.l,
            Instruction::LoadHA => self.h = self.a,
            // Implement the LD L X instructions
            Instruction::LoadLB => self.l = self.b,
            Instruction::LoadLC => self.l = self.c,
            Instruction::LoadLD => self.l = self.d,
            Instruction::LoadLE => self.l = self.e,
            Instruction::LoadLH => self.l = self.h,
            Instruction::LoadLL => self.l = self.l,
            Instruction::LoadLA => self.l = self.a,
            // Add A X instruction
            Instruction::AddAB => self.a = self.add(self.a, self.b),
            Instruction::AddAC => self.a = self.add(self.a, self.c),
            Instruction::AddAD => self.a = self.add(self.a, self.d),
            Instruction::AddAE => self.a = self.add(self.a, self.e),
            Instruction::AddAH => self.a = self.add(self.a, self.h),
            Instruction::AddAL => self.a = self.add(self.a, self.l),
            Instruction::AddAA => self.a = self.add(self.a, self.a),
            // And A X instruction
            Instruction::AndAB => self.a = self.and(self.a, self.b),
            Instruction::AndAC => self.a = self.and(self.a, self.c),
            Instruction::AndAD => self.a = self.and(self.a, self.d),
            Instruction::AndAE => self.a = self.and(self.a, self.e),
            Instruction::AndAH => self.a = self.and(self.a, self.h),
            Instruction::AndAL => self.a = self.and(self.a, self.l),
            Instruction::AndAA => self.a = self.and(self.a, self.a),
        }
    }

    fn add(self: &mut Self, value_one: u8, value_two: u8) -> u8 {
        // this is ugly, but it's not something worth spending too long to make pretty
        let half_carry: bool = (((value_one & 0xF) + (value_two & 0xF)) & 0x10) == 0x10;
        let output: u16 = (value_one as u16) + (value_two as u16);

        self.clear_flags();

        if output as u8 == 0 {
            self.flags.set(CpuFlags::ZERO_FLAG, true);
        }

        if half_carry {
            self.flags.set(CpuFlags::HALF_CARRY_FLAG, true);
        }

        if output > u8::MAX as u16 {
            self.flags.set(CpuFlags::CARRY_FLAG, true);
        }

        output as u8
    }

    fn and(self: &mut Self, value_one: u8, value_two: u8) -> u8 {
        let output = value_one & value_two;

        self.clear_flags();

        if output == 0 {
            self.flags.set(CpuFlags::ZERO_FLAG, true);
        }

        self.flags.set(CpuFlags::HALF_CARRY_FLAG, true);

        output as u8
    }

    #[cfg(test)]
    fn set_byte_in_memory(self: &mut Self, address: u16, data: u8) {
        self.memory.set_byte(address, data);
    }
}
