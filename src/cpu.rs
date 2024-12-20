use crate::memory;
use bitflags::bitflags;
use std::collections::VecDeque;

const INITIAL_PC: u16 = 0x100;
const INITIAL_SP: u16 = 0xFFFE;

enum EightBitRegister {
    A,
    B,
    D,
    H,
    F,
    C,
    E,
    L,
    S,
    P,
}

enum MicroOp {
    LoadImmediate { destination: EightBitRegister },
}

#[derive(FromPrimitive)]
enum Instruction {
    NOP = 0x00,
    // LD rr,nn instruction
    LoadBcTwoByteImmediate = 0x01,
    LoadDeTwoByteImmediate = 0x11,
    LoadHlTwoByteImmediate = 0x21,
    LoadSpTwoByteImmediate = 0x31,
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
    // the Adc A X instruction
    AdcAB = 0x88,
    AdcAC = 0x89,
    AdcAD = 0x8A,
    AdcAE = 0x8B,
    AdcAH = 0x8C,
    AdcAL = 0x8D,
    AdcAA = 0x8F,
    // the Sub A X instruction
    SubAB = 0x90,
    SubAC = 0x91,
    SubAD = 0x92,
    SubAE = 0x93,
    SubAH = 0x94,
    SubAL = 0x95,
    SubAA = 0x97,
    // the Xor A X instruction
    SbcAB = 0x98,
    SbcAC = 0x99,
    SbcAD = 0x9A,
    SbcAE = 0x9B,
    SbcAH = 0x9C,
    SbcAL = 0x9D,
    SbcAA = 0x9F,
    // the And A X instruction
    AndAB = 0xA0,
    AndAC = 0xA1,
    AndAD = 0xA2,
    AndAE = 0xA3,
    AndAH = 0xA4,
    AndAL = 0xA5,
    AndAA = 0xA7,
    // the Xor A X instruction
    XorAB = 0xA8,
    XorAC = 0xA9,
    XorAD = 0xAA,
    XorAE = 0xAB,
    XorAH = 0xAC,
    XorAL = 0xAD,
    XorAA = 0xAF,
    // the Or A X instruction
    OrAB = 0xB0,
    OrAC = 0xB1,
    OrAD = 0xB2,
    OrAE = 0xB3,
    OrAH = 0xB4,
    OrAL = 0xB5,
    OrAA = 0xB7,
    // the Cp A X instruction
    CpAB = 0xB8,
    CpAC = 0xB9,
    CpAD = 0xBA,
    CpAE = 0xBB,
    CpAH = 0xBC,
    CpAL = 0xBD,
    CpAA = 0xBF,
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
    // stores the micro ops that we need to execute
    micro_op_queue: VecDeque<MicroOp>,
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
            micro_op_queue: VecDeque::new(),
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

    pub fn execute_instruction(self: &mut Self) {
        match self.micro_op_queue.is_empty() {
            true => self.fetch_and_execute_instruction(),
            false => self.execute_micro_op(),
        }
    }

    fn execute_micro_op(self: &mut Self) {
        let micro_op = self.micro_op_queue.pop_front().unwrap();

        match micro_op {
            MicroOp::LoadImmediate { destination } => {
                let value = self.memory.get_data(self.pc);
                match destination {
                    EightBitRegister::A => self.a = value,
                    EightBitRegister::B => self.b = value,
                    EightBitRegister::D => self.d = value,
                    EightBitRegister::H => self.h = value,
                    EightBitRegister::F => self.f = value,
                    EightBitRegister::C => self.c = value,
                    EightBitRegister::E => self.e = value,
                    EightBitRegister::L => self.l = value,
                    EightBitRegister::S => self.sp = ((value as u16) << 8) + (self.sp & 0x00FF),
                    EightBitRegister::P => self.sp = (self.sp & 0xFF00) + value as u16,
                }
            }
        }
        self.pc += 1;
    }

    fn fetch_and_execute_instruction(self: &mut Self) {
        let instruction = self.get_instruction();
        self.pc += 1;
        match instruction {
            Instruction::NOP => {}
            // LD rr,nn instruction
            Instruction::LoadBcTwoByteImmediate => {
                self.load_eight_bit_register_with_immediate(EightBitRegister::C);
                self.load_eight_bit_register_with_immediate(EightBitRegister::B);
            }
            Instruction::LoadDeTwoByteImmediate => {
                self.load_eight_bit_register_with_immediate(EightBitRegister::E);
                self.load_eight_bit_register_with_immediate(EightBitRegister::D);
            }
            Instruction::LoadHlTwoByteImmediate => {
                self.load_eight_bit_register_with_immediate(EightBitRegister::L);
                self.load_eight_bit_register_with_immediate(EightBitRegister::H);
            }
            Instruction::LoadSpTwoByteImmediate => {
                self.load_eight_bit_register_with_immediate(EightBitRegister::P);
                self.load_eight_bit_register_with_immediate(EightBitRegister::S);
            }
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
            // Adc A X instruction
            Instruction::AdcAB => self.a = self.adc(self.a, self.b),
            Instruction::AdcAC => self.a = self.adc(self.a, self.c),
            Instruction::AdcAD => self.a = self.adc(self.a, self.d),
            Instruction::AdcAE => self.a = self.adc(self.a, self.e),
            Instruction::AdcAH => self.a = self.adc(self.a, self.h),
            Instruction::AdcAL => self.a = self.adc(self.a, self.l),
            Instruction::AdcAA => self.a = self.adc(self.a, self.a),
            // Sub A X instruction
            Instruction::SubAB => self.a = self.sub(self.a, self.b),
            Instruction::SubAC => self.a = self.sub(self.a, self.c),
            Instruction::SubAD => self.a = self.sub(self.a, self.d),
            Instruction::SubAE => self.a = self.sub(self.a, self.e),
            Instruction::SubAH => self.a = self.sub(self.a, self.h),
            Instruction::SubAL => self.a = self.sub(self.a, self.l),
            Instruction::SubAA => self.a = self.sub(self.a, self.a),
            // Sbc A X instruction
            Instruction::SbcAB => self.a = self.sbc(self.a, self.b),
            Instruction::SbcAC => self.a = self.sbc(self.a, self.c),
            Instruction::SbcAD => self.a = self.sbc(self.a, self.d),
            Instruction::SbcAE => self.a = self.sbc(self.a, self.e),
            Instruction::SbcAH => self.a = self.sbc(self.a, self.h),
            Instruction::SbcAL => self.a = self.sbc(self.a, self.l),
            Instruction::SbcAA => self.a = self.sbc(self.a, self.a),
            // And A X instruction
            Instruction::AndAB => self.a = self.and(self.a, self.b),
            Instruction::AndAC => self.a = self.and(self.a, self.c),
            Instruction::AndAD => self.a = self.and(self.a, self.d),
            Instruction::AndAE => self.a = self.and(self.a, self.e),
            Instruction::AndAH => self.a = self.and(self.a, self.h),
            Instruction::AndAL => self.a = self.and(self.a, self.l),
            Instruction::AndAA => self.a = self.and(self.a, self.a),
            // Or A X instruction
            Instruction::OrAB => self.a = self.or(self.a, self.b),
            Instruction::OrAC => self.a = self.or(self.a, self.c),
            Instruction::OrAD => self.a = self.or(self.a, self.d),
            Instruction::OrAE => self.a = self.or(self.a, self.e),
            Instruction::OrAH => self.a = self.or(self.a, self.h),
            Instruction::OrAL => self.a = self.or(self.a, self.l),
            Instruction::OrAA => self.a = self.or(self.a, self.a),
            // Xor A X instruction
            Instruction::XorAB => self.a = self.xor(self.a, self.b),
            Instruction::XorAC => self.a = self.xor(self.a, self.c),
            Instruction::XorAD => self.a = self.xor(self.a, self.d),
            Instruction::XorAE => self.a = self.xor(self.a, self.e),
            Instruction::XorAH => self.a = self.xor(self.a, self.h),
            Instruction::XorAL => self.a = self.xor(self.a, self.l),
            Instruction::XorAA => self.a = self.xor(self.a, self.a),
            // Cp A X instruction
            Instruction::CpAB => self.cp(self.a, self.b),
            Instruction::CpAC => self.cp(self.a, self.c),
            Instruction::CpAD => self.cp(self.a, self.d),
            Instruction::CpAE => self.cp(self.a, self.e),
            Instruction::CpAH => self.cp(self.a, self.h),
            Instruction::CpAL => self.cp(self.a, self.l),
            Instruction::CpAA => self.cp(self.a, self.a),
        }
    }

    fn load_eight_bit_register_with_immediate(self: &mut Self, register: EightBitRegister) {
        self.micro_op_queue.push_back(MicroOp::LoadImmediate {
            destination: register,
        });
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

    fn adc(self: &mut Self, value_one: u8, value_two: u8) -> u8 {
        let mut carry: u16 = 0;

        if self.flags.contains(CpuFlags::CARRY_FLAG) {
            carry = 1;
        }

        // this is ugly, but it's not something worth spending too long to make pretty
        let half_carry: bool = (((value_one & 0xF) + (value_two & 0xF)) + carry as u8) > 0x0F;
        let output: u16 = (value_one as u16) + (value_two as u16) + carry;

        self.clear_flags();

        if output as u8 == 0 {
            self.flags.set(CpuFlags::ZERO_FLAG, true);
        }

        self.flags.remove(CpuFlags::SUBTRACTION_FLAG);

        if half_carry {
            self.flags.set(CpuFlags::HALF_CARRY_FLAG, true);
        }

        if output > u8::MAX as u16 {
            self.flags.set(CpuFlags::CARRY_FLAG, true);
        }

        output as u8
    }

    fn sub(self: &mut Self, value_one: u8, value_two: u8) -> u8 {
        // this is ugly, but it's not something worth spending too long to make pretty
        let half_carry: bool = (((value_one & 0xF) - (value_two & 0xF)) & 0x10) == 0x10;
        let output: u8 = value_one.wrapping_sub(value_two);

        self.clear_flags();

        if output == 0 {
            self.flags.set(CpuFlags::ZERO_FLAG, true);
        }

        self.flags.set(CpuFlags::SUBTRACTION_FLAG, true);

        if half_carry {
            self.flags.set(CpuFlags::HALF_CARRY_FLAG, true);
        }

        if value_one < value_two {
            self.flags.set(CpuFlags::CARRY_FLAG, true);
        }

        output as u8
    }

    fn sbc(self: &mut Self, value_one: u8, value_two: u8) -> u8 {
        let mut carry: u8 = 0;

        if self.flags.contains(CpuFlags::CARRY_FLAG) {
            carry = 1;
        }

        let output: u8 = value_one.wrapping_sub(value_two).wrapping_sub(carry as u8);

        self.clear_flags();

        if output == 0 {
            self.flags.set(CpuFlags::ZERO_FLAG, true);
        }

        self.flags.set(CpuFlags::SUBTRACTION_FLAG, true);

        if value_one & 0x0F < (value_two.wrapping_sub(carry) & 0x0F) {
            self.flags.set(CpuFlags::HALF_CARRY_FLAG, true);
        }

        // only happens if wrap around, so we must have carried
        if output >= value_one {
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

    fn or(self: &mut Self, value_one: u8, value_two: u8) -> u8 {
        let output = value_one | value_two;

        self.clear_flags();

        if output == 0 {
            self.flags.set(CpuFlags::ZERO_FLAG, true);
        }

        output as u8
    }

    fn xor(self: &mut Self, value_one: u8, value_two: u8) -> u8 {
        let output = value_one ^ value_two;

        self.clear_flags();

        if output == 0 {
            self.flags.set(CpuFlags::ZERO_FLAG, true);
        }

        self.flags.remove(CpuFlags::SUBTRACTION_FLAG);
        self.flags.remove(CpuFlags::HALF_CARRY_FLAG);
        self.flags.remove(CpuFlags::CARRY_FLAG);

        output as u8
    }

    fn cp(self: &mut Self, value_one: u8, value_two: u8) {
        // cp is just subtraction without actually generating an output,
        // so we can just discard the result
        let _ = self.sub(value_one, value_two);
    }

    #[cfg(test)]
    fn set_byte_in_memory(self: &mut Self, address: u16, data: u8) {
        self.memory.set_byte(address, data);
    }
}

#[cfg(test)]
mod test_load_sixteen_bit_immediate {
    use super::*;

    #[test]
    fn test_load_bc() {
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);
        let lower_byte = 0x0F;
        let upper_byte = 0xF0;

        cpu.set_byte_in_memory(cpu.pc, Instruction::LoadBcTwoByteImmediate as u8);
        cpu.set_byte_in_memory(cpu.pc + 1, lower_byte);
        cpu.set_byte_in_memory(cpu.pc + 2, upper_byte);
        cpu.execute_instruction();
        cpu.execute_instruction();
        cpu.execute_instruction();

        assert_eq!(cpu.c, lower_byte);
        assert_eq!(cpu.b, upper_byte);
    }

    #[test]
    fn test_load_de() {
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);
        let lower_byte = 0x0F;
        let upper_byte = 0xF0;

        cpu.set_byte_in_memory(cpu.pc, Instruction::LoadDeTwoByteImmediate as u8);
        cpu.set_byte_in_memory(cpu.pc + 1, lower_byte);
        cpu.set_byte_in_memory(cpu.pc + 2, upper_byte);
        cpu.execute_instruction();
        cpu.execute_instruction();
        cpu.execute_instruction();

        assert_eq!(cpu.e, lower_byte);
        assert_eq!(cpu.d, upper_byte);
    }

    #[test]
    fn test_load_hl() {
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);
        let lower_byte = 0x0F;
        let upper_byte = 0xF0;

        cpu.set_byte_in_memory(cpu.pc, Instruction::LoadHlTwoByteImmediate as u8);
        cpu.set_byte_in_memory(cpu.pc + 1, lower_byte);
        cpu.set_byte_in_memory(cpu.pc + 2, upper_byte);
        cpu.execute_instruction();
        cpu.execute_instruction();
        cpu.execute_instruction();

        assert_eq!(cpu.l, lower_byte);
        assert_eq!(cpu.h, upper_byte);
    }

    #[test]
    fn test_load_sp() {
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);
        let lower_byte = 0x0F;
        let upper_byte = 0xF0;

        cpu.set_byte_in_memory(cpu.pc, Instruction::LoadSpTwoByteImmediate as u8);
        cpu.set_byte_in_memory(cpu.pc + 1, lower_byte);
        cpu.set_byte_in_memory(cpu.pc + 2, upper_byte);
        cpu.execute_instruction();
        cpu.execute_instruction();
        cpu.execute_instruction();

        assert_eq!(cpu.sp, ((upper_byte as u16) << 8) + (lower_byte as u16));
    }
}

mod test_adc {
    use super::*;

    #[test]
    fn test_adc_aa_no_overflow_no_carry() {
        let expected_value = 0x4;
        let expected_flags = CpuFlags::empty();
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0x02;
        cpu.set_byte_in_memory(cpu.pc, Instruction::AdcAA as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_adc_aa_overflow_no_carry() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::CARRY_FLAG | CpuFlags::ZERO_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0x80;
        cpu.set_byte_in_memory(cpu.pc, Instruction::AdcAA as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_adc_aa_no_overflow_with_carry() {
        let expected_value = 0x5;
        let expected_flags = CpuFlags::empty();
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);
        cpu.flags.set(CpuFlags::CARRY_FLAG, true);

        cpu.a = 0x02;
        cpu.set_byte_in_memory(cpu.pc, Instruction::AdcAA as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_adc_aa_overflow_with_carry() {
        let expected_value = 0x01;
        let expected_flags = CpuFlags::CARRY_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);
        cpu.flags.set(CpuFlags::CARRY_FLAG, true);

        cpu.a = 0x80;
        cpu.b = 0x80;
        cpu.set_byte_in_memory(cpu.pc, Instruction::AdcAA as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_adc_ab_no_overflow_no_carry() {
        let expected_value = 0x4;
        let expected_flags = CpuFlags::empty();
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0x02;
        cpu.b = 0x02;
        cpu.set_byte_in_memory(cpu.pc, Instruction::AdcAB as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_adc_ab_overflow_no_carry() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::CARRY_FLAG | CpuFlags::ZERO_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0x80;
        cpu.b = 0x80;
        cpu.set_byte_in_memory(cpu.pc, Instruction::AdcAB as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_adc_ab_no_overflow_with_carry() {
        let expected_value = 0x5;
        let expected_flags = CpuFlags::empty();
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);
        cpu.flags.set(CpuFlags::CARRY_FLAG, true);

        cpu.a = 0x02;
        cpu.b = 0x02;
        cpu.set_byte_in_memory(cpu.pc, Instruction::AdcAB as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_adc_ab_overflow_with_carry() {
        let expected_value = 0x01;
        let expected_flags = CpuFlags::CARRY_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);
        cpu.flags.set(CpuFlags::CARRY_FLAG, true);

        cpu.a = 0x80;
        cpu.b = 0x80;
        cpu.set_byte_in_memory(cpu.pc, Instruction::AdcAB as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }
}

#[cfg(test)]
mod test_sub {
    use super::*;

    #[test]
    fn test_sub_aa() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG | CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.set_byte_in_memory(cpu.pc, Instruction::SubAA as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_sub_ab_non_zero() {
        let expected_value = 0x0F;
        let expected_flags = CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.b = 0xF0;
        cpu.set_byte_in_memory(cpu.pc, Instruction::SubAB as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_sub_ab_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG | CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.b = cpu.a;
        cpu.set_byte_in_memory(cpu.pc, Instruction::SubAB as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_sub_ac_non_zero() {
        let expected_value = 0x0F;
        let expected_flags = CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.c = 0xF0;
        cpu.set_byte_in_memory(cpu.pc, Instruction::SubAC as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_sub_ac_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG | CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.c = cpu.a;
        cpu.set_byte_in_memory(cpu.pc, Instruction::SubAC as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_sub_ad_non_zero() {
        let expected_value = 0x0F;
        let expected_flags = CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.d = 0xF0;
        cpu.set_byte_in_memory(cpu.pc, Instruction::SubAD as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_sub_ad_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG | CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.d = cpu.a;
        cpu.set_byte_in_memory(cpu.pc, Instruction::SubAD as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_sub_ae_non_zero() {
        let expected_value = 0x0F;
        let expected_flags = CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.e = 0xF0;
        cpu.set_byte_in_memory(cpu.pc, Instruction::SubAE as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_sub_ae_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG | CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.e = cpu.a;
        cpu.set_byte_in_memory(cpu.pc, Instruction::SubAE as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_sub_ah_non_zero() {
        let expected_value = 0x0F;
        let expected_flags = CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.h = 0xF0;
        cpu.set_byte_in_memory(cpu.pc, Instruction::SubAH as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_sub_ah_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG | CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.h = cpu.a;
        cpu.set_byte_in_memory(cpu.pc, Instruction::SubAH as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_sub_al_non_zero() {
        let expected_value = 0x0F;
        let expected_flags = CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.l = 0xF0;
        cpu.set_byte_in_memory(cpu.pc, Instruction::SubAL as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_sub_al_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG | CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.l = cpu.a;
        cpu.set_byte_in_memory(cpu.pc, Instruction::SubAL as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }
}

#[cfg(test)]
mod test_sbc {
    use super::*;

    #[test]
    fn test_sbc_aa_no_carry() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG | CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.set_byte_in_memory(cpu.pc, Instruction::SbcAA as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_sbc_aa_with_carry() {
        let expected_value = 0xFF;
        let expected_flags = CpuFlags::SUBTRACTION_FLAG | CpuFlags::CARRY_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);
        cpu.flags.set(CpuFlags::CARRY_FLAG, true);

        cpu.a = 0xFF;
        cpu.set_byte_in_memory(cpu.pc, Instruction::SbcAA as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_sbc_ab_non_zero_no_carry() {
        let expected_value = 0x0F;
        let expected_flags = CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.b = 0xF0;
        cpu.set_byte_in_memory(cpu.pc, Instruction::SbcAB as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_sbc_ab_zero_no_carry() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG | CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.b = cpu.a;
        cpu.set_byte_in_memory(cpu.pc, Instruction::SbcAB as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_sbc_ab_non_zero_with_carry() {
        let expected_value = 0x0E;
        let expected_flags = CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);
        cpu.flags.set(CpuFlags::CARRY_FLAG, true);

        cpu.a = 0xFF;
        cpu.b = 0xF0;
        cpu.set_byte_in_memory(cpu.pc, Instruction::SbcAB as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_sbc_ab_zero_with_carry() {
        let expected_value = 0xFF;
        let expected_flags = CpuFlags::CARRY_FLAG | CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);
        cpu.flags.set(CpuFlags::CARRY_FLAG, true);

        cpu.a = 0xFF;
        cpu.b = cpu.a;
        cpu.set_byte_in_memory(cpu.pc, Instruction::SbcAB as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }
}

#[cfg(test)]
mod and_tests {
    use super::*;

    #[test]
    fn test_and_aa_non_zero() {
        let expected_value = 0xFF;
        let expected_flags = CpuFlags::HALF_CARRY_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = expected_value;
        cpu.set_byte_in_memory(cpu.pc, Instruction::AndAA as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_and_aa_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG | CpuFlags::HALF_CARRY_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = expected_value;
        cpu.set_byte_in_memory(cpu.pc, Instruction::AndAA as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_and_ab_non_zero() {
        let expected_value = 0x0F;
        let expected_flags = CpuFlags::HALF_CARRY_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.b = 0x0F;
        cpu.set_byte_in_memory(cpu.pc, Instruction::AndAB as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_and_ab_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG | CpuFlags::HALF_CARRY_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xF0;
        cpu.b = 0x0F;
        cpu.set_byte_in_memory(cpu.pc, Instruction::AndAB as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_and_ac_non_zero() {
        let expected_value = 0x0F;
        let expected_flags = CpuFlags::HALF_CARRY_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.c = 0x0F;
        cpu.set_byte_in_memory(cpu.pc, Instruction::AndAC as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_and_ac_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG | CpuFlags::HALF_CARRY_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xF0;
        cpu.c = 0x0F;
        cpu.set_byte_in_memory(cpu.pc, Instruction::AndAC as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_and_ad_non_zero() {
        let expected_value = 0x0F;
        let expected_flags = CpuFlags::HALF_CARRY_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.d = 0x0F;
        cpu.set_byte_in_memory(cpu.pc, Instruction::AndAD as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_and_ad_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG | CpuFlags::HALF_CARRY_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xF0;
        cpu.d = 0x0F;
        cpu.set_byte_in_memory(cpu.pc, Instruction::AndAD as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_and_ae_non_zero() {
        let expected_value = 0x0F;
        let expected_flags = CpuFlags::HALF_CARRY_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.e = 0x0F;
        cpu.set_byte_in_memory(cpu.pc, Instruction::AndAE as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_and_ae_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG | CpuFlags::HALF_CARRY_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xF0;
        cpu.e = 0x0F;
        cpu.set_byte_in_memory(cpu.pc, Instruction::AndAE as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_and_ah_non_zero() {
        let expected_value = 0x0F;
        let expected_flags = CpuFlags::HALF_CARRY_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.h = 0x0F;
        cpu.set_byte_in_memory(cpu.pc, Instruction::AndAH as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_and_ah_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG | CpuFlags::HALF_CARRY_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xF0;
        cpu.h = 0x0F;
        cpu.set_byte_in_memory(cpu.pc, Instruction::AndAH as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_and_al_non_zero() {
        let expected_value = 0x0F;
        let expected_flags = CpuFlags::HALF_CARRY_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.l = 0x0F;
        cpu.set_byte_in_memory(cpu.pc, Instruction::AndAL as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_and_al_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG | CpuFlags::HALF_CARRY_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xF0;
        cpu.l = 0x0F;
        cpu.set_byte_in_memory(cpu.pc, Instruction::AndAL as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }
}

#[cfg(test)]
mod or_test {
    use super::*;
    #[test]
    fn test_or_aa_non_zero() {
        let expected_value = 0xFF;
        let expected_flags = CpuFlags::empty();
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = expected_value;
        cpu.set_byte_in_memory(cpu.pc, Instruction::OrAA as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_or_aa_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = expected_value;
        cpu.set_byte_in_memory(cpu.pc, Instruction::OrAA as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_or_ab_non_zero() {
        let expected_value = 0xFF;
        let expected_flags = CpuFlags::empty();
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.b = 0x0F;
        cpu.set_byte_in_memory(cpu.pc, Instruction::OrAB as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_or_ab_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0x00;
        cpu.b = 0x00;
        cpu.set_byte_in_memory(cpu.pc, Instruction::OrAB as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_or_ac_non_zero() {
        let expected_value = 0xFF;
        let expected_flags = CpuFlags::empty();
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.c = 0x0F;
        cpu.set_byte_in_memory(cpu.pc, Instruction::OrAC as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_or_ac_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0x00;
        cpu.c = 0x00;
        cpu.set_byte_in_memory(cpu.pc, Instruction::OrAC as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_or_ad_non_zero() {
        let expected_value = 0xFF;
        let expected_flags = CpuFlags::empty();
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.d = 0x0F;
        cpu.set_byte_in_memory(cpu.pc, Instruction::OrAD as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_or_ad_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0x00;
        cpu.d = 0x00;
        cpu.set_byte_in_memory(cpu.pc, Instruction::OrAD as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_or_ae_non_zero() {
        let expected_value = 0xFF;
        let expected_flags = CpuFlags::empty();
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.e = 0x0F;
        cpu.set_byte_in_memory(cpu.pc, Instruction::OrAE as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_or_ae_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0x00;
        cpu.e = 0x00;
        cpu.set_byte_in_memory(cpu.pc, Instruction::OrAE as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_or_ah_non_zero() {
        let expected_value = 0xFF;
        let expected_flags = CpuFlags::empty();
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.h = 0x0F;
        cpu.set_byte_in_memory(cpu.pc, Instruction::OrAH as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_or_ah_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0x00;
        cpu.h = 0x00;
        cpu.set_byte_in_memory(cpu.pc, Instruction::OrAH as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_or_al_non_zero() {
        let expected_value = 0xFF;
        let expected_flags = CpuFlags::empty();
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.l = 0x0F;
        cpu.set_byte_in_memory(cpu.pc, Instruction::OrAL as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_or_al_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0x00;
        cpu.l = 0x00;
        cpu.set_byte_in_memory(cpu.pc, Instruction::OrAL as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }
}

#[cfg(test)]
mod xortest {
    use super::*;
    #[test]
    fn test_xor_aa_non_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0x01;
        cpu.set_byte_in_memory(cpu.pc, Instruction::XorAA as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_xor_aa_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = expected_value;
        cpu.set_byte_in_memory(cpu.pc, Instruction::XorAA as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_xor_ab_non_zero() {
        let expected_value = 0xF0;
        let expected_flags = CpuFlags::empty();
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.b = 0x0F;
        cpu.set_byte_in_memory(cpu.pc, Instruction::XorAB as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_xor_ab_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0x00;
        cpu.b = 0x00;
        cpu.set_byte_in_memory(cpu.pc, Instruction::XorAB as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_xor_ac_non_zero() {
        let expected_value = 0xF0;
        let expected_flags = CpuFlags::empty();
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.c = 0x0F;
        cpu.set_byte_in_memory(cpu.pc, Instruction::XorAC as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_xor_ac_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0x00;
        cpu.c = 0x00;
        cpu.set_byte_in_memory(cpu.pc, Instruction::XorAC as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_xor_ad_non_zero() {
        let expected_value = 0xF0;
        let expected_flags = CpuFlags::empty();
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.d = 0x0F;
        cpu.set_byte_in_memory(cpu.pc, Instruction::XorAD as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_xor_ad_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0x00;
        cpu.d = 0x00;
        cpu.set_byte_in_memory(cpu.pc, Instruction::XorAD as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_xor_ae_non_zero() {
        let expected_value = 0xF0;
        let expected_flags = CpuFlags::empty();
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.e = 0x0F;
        cpu.set_byte_in_memory(cpu.pc, Instruction::XorAE as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_xor_ae_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0x00;
        cpu.e = 0x00;
        cpu.set_byte_in_memory(cpu.pc, Instruction::XorAE as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_xor_ah_non_zero() {
        let expected_value = 0xF0;
        let expected_flags = CpuFlags::empty();
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.h = 0x0F;
        cpu.set_byte_in_memory(cpu.pc, Instruction::XorAH as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_xor_ah_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0x00;
        cpu.h = 0x00;
        cpu.set_byte_in_memory(cpu.pc, Instruction::XorAH as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_xor_al_non_zero() {
        let expected_value = 0xF0;
        let expected_flags = CpuFlags::empty();
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0xFF;
        cpu.l = 0x0F;
        cpu.set_byte_in_memory(cpu.pc, Instruction::XorAL as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_xor_al_zero() {
        let expected_value = 0x00;
        let expected_flags = CpuFlags::ZERO_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = 0x00;
        cpu.l = 0x00;
        cpu.set_byte_in_memory(cpu.pc, Instruction::XorAL as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }
}

mod test_cp {
    use super::*;

    #[test]
    fn test_cp_aa() {
        let expected_value = 0xFF;
        let expected_flags = CpuFlags::ZERO_FLAG | CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = expected_value;
        cpu.set_byte_in_memory(cpu.pc, Instruction::CpAA as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_cp_ab() {
        let expected_value = 0xFF;
        let expected_flags = CpuFlags::ZERO_FLAG | CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = expected_value;
        cpu.b = cpu.a;
        cpu.set_byte_in_memory(cpu.pc, Instruction::CpAB as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_cp_ac() {
        let expected_value = 0xFF;
        let expected_flags = CpuFlags::ZERO_FLAG | CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = expected_value;
        cpu.c = cpu.a;
        cpu.set_byte_in_memory(cpu.pc, Instruction::CpAC as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_cp_ad() {
        let expected_value = 0xFF;
        let expected_flags = CpuFlags::ZERO_FLAG | CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = expected_value;
        cpu.d = cpu.a;
        cpu.set_byte_in_memory(cpu.pc, Instruction::CpAD as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_cp_ae() {
        let expected_value = 0xFF;
        let expected_flags = CpuFlags::ZERO_FLAG | CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = expected_value;
        cpu.e = cpu.a;
        cpu.set_byte_in_memory(cpu.pc, Instruction::CpAE as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_cp_ah() {
        let expected_value = 0xFF;
        let expected_flags = CpuFlags::ZERO_FLAG | CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = expected_value;
        cpu.h = cpu.a;
        cpu.set_byte_in_memory(cpu.pc, Instruction::CpAH as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }

    #[test]
    fn test_cp_al() {
        let expected_value = 0xFF;
        let expected_flags = CpuFlags::ZERO_FLAG | CpuFlags::SUBTRACTION_FLAG;
        let mut memory = memory::Memory::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.a = expected_value;
        cpu.l = cpu.a;
        cpu.set_byte_in_memory(cpu.pc, Instruction::CpAL as u8);
        cpu.execute_instruction();

        assert_eq!(cpu.a, expected_value);
        assert_eq!(cpu.flags, expected_flags);
    }
}
