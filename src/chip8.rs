use crate::cpu;
use crate::cpu::Cpu;
use crate::ram::Ram;

pub struct Chip8 {
    ram: Ram,
    cpu: Cpu,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            ram: Ram::new(),
            cpu: Cpu::new(),
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        for i in 0..data.len() {
            self.ram
                .write_byte(cpu::PROGRAM_START + (i as u16), data[i]);
        }
    }

    pub fn run_instruction(&mut self) {
        self.cpu.run_instruction(&mut self.ram);
        println!("Cpu state: {:#?}", self.cpu);
    }
}
