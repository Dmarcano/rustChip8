use std::fs::File;
use std::io::prelude::*; 
use std::io;
use rand::Rng; 

mod tests;
mod opcodes; 

//TODO does usize break anything later on?
const START_ADDR : usize = 0x200; 

const FONTSET_SIZE : usize  = 80; 

const FONTSET : [u8; FONTSET_SIZE]  = [
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

const VIDEO_WIDTH : u8 = 64; 
const VIDEO_HEIGHT : u8 = 32; 
const SPRITE_WIDTH : u8 = 8; 

/// Chip-8 CPU capable of reading and processing instructions
/// more information on chip-8 can be found at http://devernay.free.fr/hacks/chip8/C8TECH10.HTM 
pub struct Chip8CPU { 

    /// general purpose registers
    v : [u8 ; 16], 

    /// chip-8 has 4Kb of memory
    memory : [u8; 4096], 

    /// index register stores memory addresses for use in operations.
    index : u16, 

    /// program counter keeps track of which memory address to fetch next for instruction decoding
    pc : u16,
    
    /// stack keeps track of memory addresses 
    stack : [u16 ; 16],

    /// stack pointer
    sp : u16,

    delay_timer : u8 ,

    sound_timer : u8,

    rng : rand::prelude::ThreadRng,

    /// the display buffer that is used to draw graphics
    disp_buf : [u8 ; 32*64], 

    keyboard : [u8; 16]
}

// public methods
impl  Chip8CPU{ 

    
    pub fn new() ->  Chip8CPU{

        let v : [u8; 16]= [0; 16];
        let mut memory : [u8; 4096] = [0; 4096]; 
        let stack = [0; 16];
        let disp_buf = [0; VIDEO_WIDTH as usize * VIDEO_HEIGHT as usize];
        let keyboard = [0; 16];


        let rng = rand::thread_rng();
        let pc : u16 = START_ADDR as u16; 
        let index = 0; 
        let sp = 0; 
        let delay_timer = 0; // both delay timers start at 0.
        let sound_timer = 0;

        // write the fontset into memory starting at 0x50
        memory[0x50..0x50+FONTSET.len()].copy_from_slice(&FONTSET) ;

        Chip8CPU { 
            v,
            memory, 
            stack, 
            sp, 
            pc, 
            index, 
            delay_timer, 
            sound_timer,
            rng,
            disp_buf,
            keyboard
        }
        
    }

    /// resets the memory, registers, stack and pc of the Chip-8
    pub fn reset(&mut self) { 

        self.v.iter_mut().for_each(|m| *m = 0); // clear out registers
        self.memory[START_ADDR..].iter_mut().for_each(|m| *m = 0);  // clear out any possibly loaded ROM
        self.stack.iter_mut().for_each(|m| *m = 0);
        self.disp_buf.iter_mut().for_each(|m| *m = 0);
        self.pc = START_ADDR as u16; 
        self.delay_timer = 0; 
        self.sound_timer = 0;
        self.sp = 0;
        self.index = 0;
    }

    pub fn load_rom_from_file(&mut self, filename : String) { 
        
        let file = std::fs::File::open(std::path::Path::new(&filename)).unwrap();
        self.load_rom_from_bytes(file);
    }

     /// given some iterable of bytes, loads the bytes onto Chip8's memory
     pub fn load_rom_from_bytes< U : std::io::Read>(&mut self, source : U) { 
        for (i, byte) in source.bytes().enumerate() { 
            self.memory[START_ADDR  + i] = byte.unwrap(); 
        }
    }

    /// Emulates a single CPU cycle for the Chip-8 CPU
    /// 
    /// 1. Fetches an opcode from memory,
    /// 2. Decodes the opcode into an instruction, 
    /// 3. Executes the instruction storing any results
    pub fn cycle(&mut self) { 

        let opcode = self.fetch_opcode(); 
        self.increment_pc();
        self.process_opcode(opcode); 

    }

}


// private helper functions
impl Chip8CPU { 

    fn fetch_opcode(&self) -> u16 { 
        (self.memory[self.pc as usize] as u16) << 8 | (self.memory[(self.pc + 1) as usize]) as u16
    }

    fn process_opcode(&mut self, opcode : u16 ) { 
        unimplemented!()
    }

    fn random_byte(&mut self) -> u8 { 
        self.rng.gen::<u8>()
    }

    // each opcode is 2 bytes and the PC is indexed by 1 byte.
    /// increments the program counter by 1 instruction
    fn increment_pc(&mut self) { 
        self.pc += 2; 
    }

    /// decrements the progam counter by 1 instruction
    fn decrement_pc(&mut self) { 
        self.pc -=2;
    }

}



pub trait Keypad { 

}

pub trait Display {  

}

