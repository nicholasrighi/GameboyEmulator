mod cpu;
mod memory;

extern crate num;
#[macro_use]
extern crate num_derive;

fn main() {
    let mut memory = memory::Memory::new();
    let cpu = cpu::Cpu::new(&mut memory);
}
