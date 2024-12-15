pub struct Memory {
    // All of the data that exists in the gameboy
    data: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Memory { data: Vec::new() }
    }

    pub fn get_data(self: &Self, address: u16) -> u8 {
        self.data[usize::from(address)]
    }

    pub fn set_byte(self: &mut Self, address: u16, data: u8) {
        self.data[usize::from(address)] = data;
    }
}
