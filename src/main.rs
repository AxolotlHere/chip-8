mod config;
use crate::config::chip8;
use crate::config::chip8::Chip8;
use config::chip8;

fn main() {
    let mut chip_ref = Chip8::new();
    chip_ref.ld_rom(String::from("binary_assets/Tetris.ch8"));
    chip_ref.ld_fonts();
    const SCALE: u32 = 1;

    let mut start_timer = std::time::Instant::now();
    let timer_rate = std::time::Duration::from_secs_f64(1.0 / 60.0); //To anyone reviewing, don't change
    //the clock speed
    loop {
        chip_ref.cycle();
        if start_timer.elapsed() >= timer_rate {
            if chip_ref.delay_timer > 0 {
                chip_ref.delay_timer -= 1;
            }
            start_timer = std::time::Instant::now();
        }
        std::thread::sleep(std::time::Duration::from_micros(1200));
    }
}
