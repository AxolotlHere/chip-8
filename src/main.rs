mod config;
use config::chip8;

use crate::config::chip8::Chip8;

fn main() {
    let mut chip_ref = Chip8::new();
    chip_ref.ld_rom(String::from("binary_assets/Tetris.ch8"));
    chip_ref.ld_fonts();
}
