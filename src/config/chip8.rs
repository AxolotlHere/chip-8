use std::{
    fs::File,
    io::{BufReader, Read},
    time::{SystemTime, UNIX_EPOCH},
};

use rand::{SeedableRng, rngs::StdRng};

static START_ADDR: u16 = 0x200;
static FONT_ADDR: u16 = 0x50;
static FONT_8_5: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];
static E_WIDTH: u8 = 64;
static E_HEIGHT: u8 = 32;
pub struct Chip8 {
    pub gr: [u8; 16],
    pub memory: [u8; 4096],
    pub index: u16,
    pub pc: u16,
    pub stk: [u16; 16],
    pub sp: u8,
    pub delay_timer: u8,
    pub snd_timer: u8,
    pub keypad: [u8; 16],
    pub video: [u32; 64 * 32],
    pub rng: StdRng,
}

impl Chip8 {
    pub fn new() -> Self {
        let seed: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("GND build failed, exiting process")
            .as_nanos() as u64;
        let rng_: StdRng = StdRng::seed_from_u64(seed);
        Self {
            gr: [0; 16],
            memory: [0; 4096],
            index: 0,
            pc: START_ADDR,
            stk: [0; 16],
            sp: 0,
            delay_timer: 0,
            snd_timer: 0,
            keypad: [0; 16],
            video: [0; 2048],
            rng: rng_,
        }
    }
    pub fn ld_fonts(&mut self) {
        for i in 1..FONT_8_5.len() {
            self.memory[(FONT_ADDR + i as u16) as usize] = FONT_8_5[i];
        }
    }
    pub fn ld_rom(&mut self, filepath: String) {
        let mut file_ref = File::open(filepath);
        match file_ref {
            Ok(msg) => {
                let mut read_buffer: BufReader<_> = BufReader::new(msg);
                let mut buf = vec![];
                read_buffer.read_to_end(&mut buf).unwrap();
                let count: u32 = 0;
                for i in 0..buf.len() {
                    self.memory[(START_ADDR + i as u16) as usize] = buf[i];
                }
                println!("{:?}", self.memory);
                drop(buf);
            }
            Err(e) => {
                println!("{e}");
            }
        }
    }
    pub fn op_00e0(&mut self) {
        self.video.fill(0);
    }
    pub fn op_fx0a(&mut self, opcode: u16) {
        let vx = ((opcode & 0x0F00) >> 8) as usize;
        let mut key_found = false;
        for i in 0..self.keypad.len() {
            if self.keypad[i] != 0 {
                self.gr[vx] = i as u8;
                key_found = true;
                break;
            }
        }
        if !key_found {
            self.pc -= 2;
        }
    }
    pub fn op_00ee(&mut self) {
        if (self.sp != 0) {
            self.sp -= 1;
            self.pc = self.stk[self.sp as usize];
        }
    }

    pub fn op_1nnn(&mut self, opcode: u16) {
        self.pc = opcode & 0x0FFF;
    }
    pub fn op_4xkk(&mut self, opcode: u16) {
        let Vx: u8 = ((opcode & 0x0F00) >> 8) as u8;
        let byte_val: u8 = (opcode & 0x00FF) as u8;
        if (self.gr[Vx as usize] != byte_val) {
            self.pc += 2;
        }
    }
    pub fn op_2nnn(&mut self, opcode: u16) {
        let addr_: u16 = opcode & 0x0FFF;
        self.stk[self.sp as usize] = addr_;
        self.sp += 1;
        self.pc = addr_;
    }
    pub fn op_dxyn(&mut self, opcode: u16) {
        let Vx: u16 = (opcode & 0x0F00) >> 8;
        let Vy: u16 = (opcode & 0x00F0) >> 4;
        let height: u16 = (opcode & 0x000F);

        let x_pos: u8 = self.gr[Vx as usize]; //x start value stored at register Vx
        let y_pos: u8 = self.gr[Vy as usize]; //y start value stored at register Vy

        for i in 0..height - 1 {
            let sprite_byte = self.memory[(self.index + i) as usize];
            for j in 0..7 {
                let draw_pixel: u8 = sprite_byte & (0x80 >> j);
                let screen_pixel: &mut u32 =
                    &mut self.video[((x_pos + i as u8) + (y_pos + j as u8) * E_WIDTH) as usize];
                if (draw_pixel == 1 && *screen_pixel == 1) {
                    //collision case
                    if (*screen_pixel == 1) {
                        self.gr[15] = 1;
                    }
                    *screen_pixel ^= 0xFFFFFFFF;
                }
            }
        }
    }
}
