use std::fs::File;
use std::io::prelude::*; 
use std::io;

//TODO does usize break anything later on?
const START_ADDR : usize = 0x200; 

const FONTSET_SIZE : usize  = 80; 

const fontset : [u8; FONTSET_SIZE]  = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
	0x20, 0x60, 0x20, 0x20, 0x70, // 1
	0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
	0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
	0x90, 0x90, 0xF0, 0x10, 0x10, // 4
	0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
	0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
	0xF0, 0x10, 0x20, 0x40, 0x40, // 7
	0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
	0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
	0xF0, 0x90, 0xF0, 0x90, 0x90, // A
	0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
	0xF0, 0x80, 0x80, 0x80, 0xF0, // C
	0xE0, 0x90, 0x90, 0x90, 0xE0, // D
	0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
	0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

// Chip-8 CPU capable of reading and processing instructions
pub struct Chip8CPU { 

    // general purpose registers
    v : [u8 ; 16], 

    // chip-8 has 4Kb of memory
    memory : [u8; 4096], 

    // index register stores memory addresses for use in operations.
    index : u16, 

    // program counter keeps track of which memory address to fetch next for instruction decoding
    pc : u16,
    
    // stack keeps track of memory addresses 
    stack : [u16 ; 16],

    // stack pointer
    sp : u16,

    delay_timer : u8 ,

    sound_timer : u8,
}


impl  Chip8CPU{ 

    pub fn new() ->  Chip8CPU{
        
        unimplemented!()

    }

    pub fn load_rom_from_file(&mut self, filename : String) { 
        
        let file = std::fs::File::open(std::path::Path::new(&filename)).unwrap();
        self.load_rom_from_bytes(file);
    }

    fn load_rom_from_bytes< U : std::io::Read>(&mut self, source : U) { 

        for (i, byte) in source.bytes().enumerate() { 
            self.memory[START_ADDR  + i] = byte.unwrap(); 
        }
    }

}


pub trait Keypad { 

}

pub trait Display {  

}



