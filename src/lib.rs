//! The Chip8CPU crate provides a ready Chip-8 Cpu interpreter that implements all of the Chip8's 35 opcodes. 
//! One can use this CPU and implement a method to display its graphics 
//! 
//! 

use rand::Rng;

mod opcodes;
use opcodes::function_table::*;
use cycle_error::CycleError; 
pub mod dissassembler; 
pub mod cycle_error;


const START_ADDR: usize = 0x200;

const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

const VIDEO_WIDTH: u8 = 64;
const VIDEO_HEIGHT: u8 = 32;
const SPRITE_WIDTH: u8 = 8;

/// Chip-8 CPU capable of reading and processing instructions
/// more information on chip-8 can be found at http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
/// 
/// ## Examples
/// 
/// ```no_run
///     use chip8::Chip8CPU; 
///     let mut cpu = Chip8CPU::new();
///     // load a ROM file from path
///     cpu.load_rom_from_file(String::from("path/filename"));
///     // 
///     cpu.cycle();
/// 
/// ```
/// 
pub struct Chip8CPU {
    /// general purpose registers
    v: [u8; 16],

    /// chip-8 has 4Kb of memory
    memory: [u8; 4096],

    /// index register stores memory addresses for use in operations.
    index: u16,

    /// program counter keeps track of which memory address to fetch next for instruction decoding
    pc: u16,

    /// stack keeps track of memory addresses
    stack: [u16; 16],

    /// stack pointer
    sp: u16,

    delay_timer: u8,

    sound_timer: u8,

    rng: rand::prelude::ThreadRng,

    /// the display buffer that is used to draw graphics
    disp_buf: [u8; 32 * 64],

    /// The keyboard buffer that holds values for the Chip-8's keyboard
    keyboard: [u8; 16],

    opcode_table : [OpcodeFnGetter; 16]
}

// public methods
impl Chip8CPU {

    /// Create a brand new Chip-8 CPU
    pub fn new() -> Chip8CPU {
        let v: [u8; 16] = [0; 16];
        let mut memory: [u8; 4096] = [0; 4096];
        let stack = [0; 16];
        let disp_buf = [0; VIDEO_WIDTH as usize * VIDEO_HEIGHT as usize];
        let keyboard = [0; 16];

        let rng = rand::thread_rng();
        let pc: u16 = START_ADDR as u16;
        let index = 0;
        let sp = 0;
        let delay_timer = 0; // both delay timers start at 0.
        let sound_timer = 0;

        // write the fontset into memory starting at 0x50
        memory[0x50..0x50 + FONTSET.len()].copy_from_slice(&FONTSET);

        let opcode_table = Chip8CPU::create_function_table(); 

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
            keyboard,
            opcode_table
        }
    }

    /// resets the memory, registers, stack and pc of the Chip-8
    pub fn reset(&mut self) {
        self.v.iter_mut().for_each(|m| *m = 0); // clear out registers
        self.memory[START_ADDR..].iter_mut().for_each(|m| *m = 0); // clear out any possibly loaded ROM
        self.stack.iter_mut().for_each(|m| *m = 0);
        self.disp_buf.iter_mut().for_each(|m| *m = 0);
        self.pc = START_ADDR as u16;
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.sp = 0;
        self.index = 0;
    }

    /// Load a ROM from a valid path given that a filesystem is available
    pub fn load_rom_from_file(&mut self, filename: String) {
        let file = std::fs::File::open(std::path::Path::new(&filename)).unwrap();
        self.load_rom_from_bytes(file);
    }

    /// Load a ROM in the form of some iterable (eg a Vec<u8>)
    pub fn load_rom_from_bytes(&mut self, source: impl std::io::Read) {
        for (i, byte) in source.bytes().enumerate() {
            self.memory[START_ADDR + i] = byte.unwrap();
        }
    }

    /// Emulates a single CPU cycle for the Chip-8 CPU
    ///
    /// 1. Fetches an opcode from memory,
    /// 2. Decodes the opcode into an instruction,
    /// 3. Executes the instruction storing any results
    pub fn cycle(&mut self) ->Result<(), CycleError>{
        let opcode = self.fetch_opcode();
        self.increment_pc();
        self.process_opcode(opcode)?;

        if self.sound_timer > 0 { 
            self.sound_timer -= 1; 
        }

        if self.delay_timer > 0 { 
            self.delay_timer -= 1; 
        }

        Ok(())
    }

    /// Sets the keyboard value at the given idx of the CHIP8 to a value
    ///
    /// Expects to be given indeces between 0 and 15. Panics otherwise
    pub fn set_keyboard(&mut self, idx: u8, val: u8) {
        self.keyboard[idx as usize] = val;
    }

    /// Return a reference to the display buffer. Meant to be used for the sole purpose of displaying graphics.
    pub fn peek_display_buffer(&self) -> &[u8] { 
        &self.disp_buf
    }    

    /// Returns a reference to the main meory. Meant to be used for debugging the CPU
    /// or displaying state without the overhead of cloning.
    pub fn peek_memory<(&self) -> &[u8] { 
        &self.memory
    }

    /// Clones the display buffer to use in graphics
    pub fn clone_display_buffer(&self) -> [u8; 32*64] { 
        self.disp_buf.clone()
    }

    /// Clones the cpu memory for either debugging or display purposes.
    pub fn clone_memory(&self) -> [u8; 4096] { 
        self.memory.clone()
    }

    /// clones the Chip8's 16 general purpose registers should their state be needed for display or debugging purposes
    pub fn clone_registers(&self) -> [u8; 16] { 
        self.v.clone()
    }

    /// get a reference of the Chip8's 16 general purpose registers should their state be needed for display or debugging purposes
    pub fn peek_register(&self) -> &[u8] { 
        &self.v
    }

    /// get the value of the Chip8's index register should its state be needed for display or debugging pruposes
    pub fn get_index_register(&self) -> u16 { 
        self.index
    }

    /// get the value of the Chip8's delay timer should it be needed for display or debugging purpses 
    pub fn get_delay_timer(&self) -> u8 { 
        self.delay_timer
    }

    /// get the value of the Chip8's delay timer should it be needed for display or debugging purpses 
    pub fn get_sound_timer(&self) ->u8 { 
        self.sound_timer
    }


    /// clones the cpu's keyboard should their state be needed for display or debugging
    pub fn clone_keyboard(&self) -> [u8; 16] { 
        self.keyboard.clone()   
    }

    /// get the value of the Chip8's program counter should it be needed for display or debugging purposes
    pub fn pc(&self) -> u16 { 
        self.pc 
    }

    
}

// private helper functions
impl Chip8CPU {

    fn fetch_opcode(&self) -> u16 {
        (self.memory[self.pc as usize] as u16) << 8 | (self.memory[(self.pc + 1) as usize]) as u16
    }

    fn process_opcode(&mut self, opcode: u16) ->Result<(), CycleError> {

        let table_idx = ((opcode & 0xF000) >> 12) as usize; 
        let opcode_func = self.opcode_table[table_idx](opcode); 
        opcode_func(self, opcode)?;

        Ok(())

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
        self.pc -= 2;
    }
}

pub trait Keypad {}

pub trait Display {}

#[cfg(test)]
mod tests {

    use super::Chip8CPU;
    use super::FONTSET;
    use super::*;

    #[test]
    fn init_test() {
        let mut cpu = Chip8CPU::new();

        let exp_v: [u8; 16] = [0; 16];
        let exp_pc = START_ADDR as u16;
        let exp_stack = [0; 16];
        let exp_keyboard = [0; 16];

        check_fontset(&cpu.memory);
        assert_eq!(cpu.v, exp_v);
        assert_eq!(exp_pc, cpu.pc);
        assert_eq!(exp_stack, cpu.stack);
        // assert_eq!(exp_disp_buf, cpu.disp_buf);
        assert_eq!(exp_keyboard, cpu.keyboard);

        cpu.memory[START_ADDR + 1] = 10;
        cpu.stack[0] = 0x23;
        cpu.pc = START_ADDR as u16 + 1;
        cpu.v[3] = 100;

        cpu.reset();

        check_fontset(&cpu.memory);
        assert_eq!(cpu.v, exp_v);
        assert_eq!(exp_pc, cpu.pc);
        assert_eq!(exp_stack, cpu.stack);
        // assert_eq!(exp_disp_buf, cpu.disp_buf);
        assert_eq!(exp_keyboard, cpu.keyboard);
    }

    fn check_fontset(arr: &[u8]) {
        assert_eq!(&arr[0x50..0x50 + FONTSET.len()], &FONTSET[..])
    }
}
