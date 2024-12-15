// Offsets for various pieces of gameboy memory
const ROM_BANK_0_START: u16 = 0x0000;
const ROM_BANK_N_START: u16 = 0x4000;
const TILE_RAM_START: u16 = 0x8000;
const BACKGROUND_MAP_START: u16 = 0x9800;
const CARTRIDGE_RAM_START: u16 = 0xA000;
const WORKING_RAM_START: u16 = 0xC000;
const ECHO_RAM_START: u16 = 0xE000;
const OAM_START: u16 = 0xFE00;
const UNUSED_START: u16 = 0xFEA0;
const IO_REGISTERS: u16 = 0xFF00;
const HIGH_RAM_START: u16 = 0xFF80;
const INTERRUPT_ENABLE_REGISTER: u16 = 0xFFFF;

pub struct Memory {
    // All of the data that exists in the gameboy
    rom_bank_0: [u8; (ROM_BANK_N_START - ROM_BANK_0_START) as usize],
    rom_bank_n: [u8; (TILE_RAM_START - ROM_BANK_N_START) as usize],
    tile_ram: [u8; (BACKGROUND_MAP_START - TILE_RAM_START) as usize],
    background_map: [u8; (CARTRIDGE_RAM_START - BACKGROUND_MAP_START) as usize],
    cartridge_ram: [u8; (WORKING_RAM_START - CARTRIDGE_RAM_START) as usize],
    working_ram: [u8; (ECHO_RAM_START - WORKING_RAM_START) as usize],
    echo_ram: [u8; (OAM_START - ECHO_RAM_START) as usize],
    object_attribute_memory: [u8; (UNUSED_START - OAM_START) as usize],
    unused: [u8; (IO_REGISTERS - UNUSED_START) as usize],
    io_registers: [u8; (HIGH_RAM_START - IO_REGISTERS) as usize],
    high_ram_start: [u8; (INTERRUPT_ENABLE_REGISTER - HIGH_RAM_START) as usize],
    interrupt_enable_register: [u8; 1],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            rom_bank_0: [0; (ROM_BANK_N_START - ROM_BANK_0_START) as usize],
            rom_bank_n: [0; (TILE_RAM_START - ROM_BANK_N_START) as usize],
            tile_ram: [0; (BACKGROUND_MAP_START - TILE_RAM_START) as usize],
            background_map: [0; (CARTRIDGE_RAM_START - BACKGROUND_MAP_START) as usize],
            cartridge_ram: [0; (WORKING_RAM_START - CARTRIDGE_RAM_START) as usize],
            working_ram: [0; (ECHO_RAM_START - WORKING_RAM_START) as usize],
            echo_ram: [0; (OAM_START - ECHO_RAM_START) as usize],
            object_attribute_memory: [0; (UNUSED_START - OAM_START) as usize],
            unused: [0; (IO_REGISTERS - UNUSED_START) as usize],
            io_registers: [0; (HIGH_RAM_START - IO_REGISTERS) as usize],
            high_ram_start: [0; (INTERRUPT_ENABLE_REGISTER - HIGH_RAM_START) as usize],
            interrupt_enable_register: [0; 1 as usize],
        }
    }

    pub fn get_data(self: &Self, address: u16) -> u8 {
        let offset;
        match address {
            ROM_BANK_0_START..ROM_BANK_N_START => {
                offset = 0;
                self.rom_bank_0[(address - offset) as usize]
            }
            _ => panic!(),
        }
    }

    pub fn set_byte(self: &mut Self, address: u16, data: u8) {
        let offset;
        match address {
            ROM_BANK_0_START..ROM_BANK_N_START => {
                offset = 0;
                self.rom_bank_0[(address - offset) as usize] = data;
            }
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_and_set_byte() {
        let pc = 0x100;
        let new_value = 10;
        let mut memory = Memory::new();
        assert_eq!(memory.get_data(pc), 0);
        memory.set_byte(pc, new_value);
        assert_eq!(memory.get_data(pc), new_value);
    }
}
