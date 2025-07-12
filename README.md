# CHIP-8 Emulator in Rust

A simple CHIP-8 emulator written in Rust using SDL2 for graphics and input.

## Features

- Full CPU emulation for all CHIP-8 opcodes
- SDL2-based rendering (64x32 resolution)
- Keyboard input handling
- Timer support (delay and sound)
- ROM loading from a directory
- Accurate fetch-decode-execute cycle

## ROM Directory

CHIP-8 ROMs (`.ch8` files) should be placed inside the `binary_assets/` directory.

Example:

.
├── binary_assets
│   ├── Blitz.ch8
│   ├── Invaders.ch8
│   ├── Pong.ch8
│   ├── Rocket.ch8
│   ├── Tank.ch8
│   └── Tetris.ch8
├── Cargo.lock
├── Cargo.toml
├── README.md
├── src
│   ├── config
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    └── debug

## Building and Running

### Requirements

- Rust (edition 2021+)
- SDL2 development libraries installed

For Debian/Ubuntu:
```sudo apt install libsdl2-dev```

For Arch Linux:
```sudo pacman -S sdl2```

To build and run the emulator:
```cargo run```

Currently running game files to be changed at ```src/main.rs```
