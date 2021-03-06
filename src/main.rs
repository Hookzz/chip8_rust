use std::fs::File;
use std::io::Read;
use chip8::Chip8;

mod ram;
mod cpu;
mod chip8;

fn main() {
    let mut file = File::open("data/INVADERS").unwrap();
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data);

    // print!("Data: {:?}", data);
    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

    loop {
        chip8.run_instruction();
    }
}
