mod config;
use crate::config::chip8;
use crate::config::chip8::Chip8;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};

pub fn keymaps(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}

fn main() {
    const E_WIDTH: u8 = 64;
    const E_HEIGHT: u8 = 32;
    const SCALE: u32 = 15;
    const WIN_WIDTH: u32 = E_WIDTH as u32 * SCALE;
    const WIN_HEIGHT: u32 = E_HEIGHT as u32 * SCALE;
    let emulator_ = sdl2::init().unwrap();
    let vid_sys = emulator_.video().unwrap();
    let emulator_window = vid_sys
        .window("Tetris", WIN_WIDTH, WIN_HEIGHT)
        .build()
        .unwrap();
    let mut canvas = emulator_window
        .into_canvas()
        .present_vsync()
        .build()
        .unwrap();
    let mut event_pool = emulator_.event_pump().unwrap();

    let mut chip_ref = Chip8::new();
    chip_ref.ld_rom(String::from("binary_assets/Pong.ch8"));
    chip_ref.ld_fonts();

    let mut start_timer = std::time::Instant::now();
    let timer_rate = std::time::Duration::from_secs_f64(1.0 / 60.0); //To anyone reviewing, don't change
    let mut exit_flag = false;

    //the clock speed
    while !exit_flag {
        for e in event_pool.poll_iter() {
            match e {
                Event::Quit { .. } => {
                    exit_flag = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    exit_flag = true;
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = keymaps(key) {
                        chip_ref.keypad[k] = 1;
                        println!("Pressed {:X}", k);
                    }
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = keymaps(key) {
                        chip_ref.keypad[k] = 0;
                    }
                }
                _ => {}
            }
        }
        chip_ref.cycle();
        if start_timer.elapsed() >= timer_rate {
            if chip_ref.delay_timer > 0 {
                chip_ref.delay_timer -= 1;
            }
            start_timer = std::time::Instant::now();
        }
        canvas.set_draw_color(Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        });
        canvas.clear();
        canvas.set_draw_color(Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        });
        for i in 0..E_HEIGHT {
            for j in 0..E_WIDTH {
                let index = j as usize + i as usize * E_WIDTH as usize;
                if chip_ref.video[index] != 0 {
                    let rect = Rect::new(
                        (j as u32 * SCALE) as i32,
                        (i as u32 * SCALE) as i32,
                        SCALE,
                        SCALE,
                    );
                    canvas.fill_rect(rect).unwrap();
                }
            }
        }
        canvas.present();
        std::thread::sleep(std::time::Duration::from_micros(1));
    }
}
