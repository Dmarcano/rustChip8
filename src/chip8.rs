use std::fs::File;
use std::io::prelude::*; 
use std::io;
use rand::Rng; 

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

    rng : rand::prelude::ThreadRng
}

// public methods
impl  Chip8CPU{ 

    
    pub fn new() ->  Chip8CPU{

        let v : [u8; 16]= [0; 16];
        let mut memory : [u8; 4096] = [0; 4096]; 
        let stack = [0; 16];

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
            rng
        }
        
    }

    /// resets the memory, registers, stack and pc of the Chip-8
    pub fn reset(&mut self) { 

        self.v.iter_mut().for_each(|m| *m = 0); // clear out registers
        self.memory[START_ADDR..].iter_mut().for_each(|m| *m = 0);  // clear out any possibly loaded ROM
        self.stack.iter_mut().for_each(|m| *m = 0);
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

    /// given some iterable of bytes, loads the bytes onto Chip8's memory
    fn load_rom_from_bytes< U : std::io::Read>(&mut self, source : U) { 
        for (i, byte) in source.bytes().enumerate() { 
            self.memory[START_ADDR  + i] = byte.unwrap(); 
        }
    }

}

// Op-Code implementations
impl Chip8CPU { 

    fn clear_display(&mut self) { 
        // TODO Clear Display Buffer
    }

    /// Returns from subroutine using the stack to return to before the call was made
    /// 
    /// for ```opcode => 00EE```
    fn ret(&mut self) { 
        // return from a subroutine
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize]; 
    }

    /// jumps to a specified address in the opcode
    /// 
    /// for ```opcode => 0x1nnn```
    fn jmp_addr(&mut self, opcode : u16) { 
        // jump to a given address
        let addr = opcode & 0x0FFF;
        self.pc = addr;
    }

    /// given an opcode representing a subroutine, it jumps the ```pc``` to the memory address while saving the previous 
    /// address in the stack
    /// 
    /// for ```opcode => 0x2nnn```
    fn call_addr(&mut self, opcode : u16) {
        // calls a function 
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.jmp_addr(opcode); 

    }

    /// given an opcode it compares Vx to kk in some way. Incrementing on a truthy Val 
    /// 1. opcodes ```0x3xkk => skips if Vx == kk```
    /// 2. opcodes ```0x4xkk => skips if Vx != kk```
    fn skip_vx(&mut self, opcode : u16) { 
        // both SNE and SE Vx byte
       
        let instruction = ((opcode & 0xF000) >> 12) as u8;
        let vx = ((opcode & 0x0F00) >> 8) as u8; 
        let val = (opcode & 0x00FF) as u8; 

        let equals =  self.v[vx as usize] == val;

        match (instruction, equals) { 
            (3, true) => {self.increment_pc()},
            (3, false) => {},
            (4, true) => {}, 
            (4, false) => {self.increment_pc()}, 
            (_,_) => {panic!("Given a bad opcode of value {}", opcode)}
        }
    }

    /// Skips next instruction based on ```Vx``` === ```Vy```
    /// 
    /// 1. ```opcodes => 0x5xy0``` Skips if Vx == Vy
    fn skip_vx_vy_eq(&mut self, opcode : u16) { 
        let vx = ((opcode & 0x0F00) >> 8) as u8;
        let vy = ((opcode & 0x00F0) >> 8) as u8; 

        if self.v[vx as usize] == self.v[vy as usize] { 
            self.increment_pc();
        }

    }

    /// Mutates Vx according to the opcode
    /// 
    /// 1. ```opcodes => 0x6xkk``` sets ```Vx``` as the value ```kk```
    /// 2. ```opcodes => 0x7xkk``` adds the val ```kk``` to ```Vx```

    fn set_vx(&mut self, opcode : u16) { 
        let instruction = (opcode & 0xF000) >> 12; 
        let vx = ((opcode & 0x0F00) >> 8) as u8; 
        let val = (opcode & 0x00FF) as u8; 

        match instruction {
            6 => {
                self.v[vx as usize] = val; 
            },
            7 => {
                self.v[vx as usize] += val; 
            },
            _ => {panic!("Given a bad opcode of value {} expected some variant of 0x7... or ox6...", opcode)}
        }
    }

    /// Mutates Vx according to VY and the opcode
    /// 
    /// 1. ```opcode => 0x8xy0``` Sets Vx ```=``` Vy
    /// 2. ```opcode => 0x8xy1``` ```OR```'s Vx and Vy 
    /// 3. ```opcode => 0x8xy2``` ```AND```'s Vx and Vy 
    /// 4. ```opcode => 0x8xy3``` ```XOR```'s Vx and Vy 
    /// 5. ```opcode => 0x8xy4``` ```Adds```'s Vx and Vy and sets VF to 1 if the addition overflows
    /// 6. ```opcode => 0x8xy5``` Sets VF to 1 if Vx > Vy and ```Subs```'s Vx and Vy.
    /// 7. ```opcode => 0x8xy6``` saves the least significant bit in Vx in VF and Right shifts Vx by 1
    /// 8. ```opcode => 0x8xy7``` ```Subs```'s Vx and Vy and sets VF to 1 if the subtraction overflows
    /// 9. ```opcode => 0x8xyE``` saves the most significant bit in Vx in VF and left shifts Vx by 1
    fn set_vx_vy(&mut self, opcode : u16) { 
        let vx = ((opcode & 0x0F00) >> 8) as usize; 
        let vy = ((opcode & 0x00F0) >> 4) as usize; 

        let instruction = (opcode & 0x000F) as u8;

        match instruction { 
            0 => {self.v[vx] = self.v[vy]}, 
            1 => {self.v[vx] |= self.v[vy] },
            2 => {self.v[vx] &= self.v[vy]},
            3 => {self.v[vx] ^= self.v[vy]},
            4 => {
                let (sum, of)  = self.v[vx].overflowing_add(self.v[vy]) ;
                self.v[0xF] =  of as u8; 
                self.v[vx] = sum; 
            }
            5 => {
                let (sum, of)  = self.v[vx].overflowing_sub(self.v[vy]);
                self.v[0xF] = (!of) as u8; // set flag if vx > vy or no overflow occurs
                self.v[vx]= sum;  
            }
            6 => {
                self.v[0xF] = self.v[vx] & 0x1; 
                self.v[vx] >>= 1; 
            }
            7 => {
                let (sum, of)  = self.v[vx].overflowing_sub(self.v[vy]);
                self.v[0xF] = of as u8;
                self.v[vx]= sum;  
            }
            0xE => {
                self.v[0xF] = (self.v[vx] & 0x80 ) >> 7 ;
                self.v[vx] <<= 1; 
            }
            _ => {panic!("Given a bad opcode of value {}. Expected some variant of 0x8...", opcode)}
        }
        
    }

    /// Skips next instruction based on ```Vx != Vy```
    /// 
    /// 1. ```opcodes => 0x9xy0``` Skips if Vx ```!=``` Vy
    fn skip_vx_vy_ne(&mut self, opcode : u16) { 
        let vx = ((opcode & 0x0F00) >> 8) as u8;
        let vy = ((opcode & 0x00F0) >> 8) as u8; 

        if self.v[vx as usize] != self.v[vy as usize] { 
            self.increment_pc();
        }
    }

    /// Set I = nnn.
    /// 
    /// ```opcode => 0xAnnn```
    fn set_i(&mut self, opcode : u16) { 
        self.index = opcode & 0x0FFF;
    }

    /// Set I = nnn.
    /// 
    /// ```opcode => 0xAnnn```
    fn jmp_v0_addr(&mut self, opcode : u16) { 

    }
}


pub trait Keypad { 

}

pub trait Display {  

}

#[cfg(test)]
mod tests{ 

    use super::*;

    #[test]
    fn init_test() {
        let mut cpu = Chip8CPU::new(); 

        let exp_v : [u8; 16]= [0; 16];
        let exp_pc = START_ADDR as u16; 
        let exp_stack = [0; 16];
        
        check_fontset(&cpu.memory);
        assert_eq!(cpu.v, exp_v);
        assert_eq!(exp_pc, cpu.pc);
        assert_eq!(exp_stack, cpu.stack);

        cpu.memory[START_ADDR + 1] = 10;
        cpu.stack[0] = 0x23;
        cpu.pc = START_ADDR as u16 + 1;
        cpu.v[3] = 100;

        cpu.reset();

        check_fontset(&cpu.memory);
        assert_eq!(cpu.v, exp_v);
        assert_eq!(exp_pc, cpu.pc);
        assert_eq!(exp_stack, cpu.stack);


    }

    /// tests for simple setting and mutation of registers
    #[test]
    fn register_set_tests() {

        let mut cpu = Chip8CPU::new(); 

        let mut opcode = 0x6123; // sets register v[1] to 0x23
        cpu.set_vx(opcode);

        opcode = 0x7101; // sets v[1] += 1
        assert_eq!(cpu.v[1], 0x23); 
        
        cpu.set_vx(opcode); 
        assert_eq!(cpu.v[1], 0x24); 
    }

    /// Testing of the Chip-8 CPU's ability to properly handle jump direct jump, call, and ret commands
    #[test]
    fn jumping_tests() {

        let mut cpu = Chip8CPU::new(); 
        let mut opcode = 0x1FAF; // opcode calls JMP to address 0xFAF
        cpu.jmp_addr(opcode);   
        assert_eq!(cpu.pc, 0xFAF); 

        opcode = 0x2250; // opcode calls CALL to address 0x250
        cpu.call_addr(opcode); 
        assert_eq!(cpu.pc, 0x250); 

        cpu.ret(); 
        assert_eq!(cpu.pc, 0xFAF); // return to previous address at 0xFAF

    }

    /// Testing of the Chip-8 CPU's ability skip instructions based on values in registers
    #[test]
    fn skip_byte_tests() { 
        let mut cpu = Chip8CPU::new(); 
        let mut opcode = 0x6123; // set register v[1] to 0x23 
        cpu.set_vx(opcode); 


        opcode = 0x3123; // compare v[1] to 0x23 and Skip next instruction if they are equal
        cpu.skip_vx(opcode); 
        assert_eq!(cpu.pc, (START_ADDR + 2) as u16); 

        opcode = 0x4123; // compare v[1] to 0x23 and Skip next instruction if they are NOT equal
        cpu.skip_vx(opcode);
        assert_eq!(cpu.pc, (START_ADDR + 2) as u16); 

        opcode = 0x4124; // compare v[1] to 0x24 and Skip next instruction if they are NOT equal
        cpu.skip_vx(opcode);
        assert_eq!(cpu.pc, (START_ADDR + 4) as u16); 

        opcode = 0x3124; // compare v[1] to 0x24 and Skip next instruction if they are equal
        cpu.skip_vx(opcode);
        assert_eq!(cpu.pc, (START_ADDR + 4) as u16); 
    }

    /// Testing of the Chip-8 CPU's ability to perform logical instructions based on the state of two registers
    #[test]
    fn register_logical_ops_test() {
        let mut cpu = Chip8CPU::new(); 
        let x_val = 0x56; 
        let y_val = 0x33;
        set_registers(&mut cpu, &[(1, x_val), (2,y_val)]);

        // SET TEST
        let mut opcode = 0x8120; // set vx equal to vy 
        cpu.set_vx_vy(opcode); 
        assert_eq!(cpu.v[1], cpu.v[2]);
        // OR TEST
        set_registers(&mut cpu, &[(1, x_val), (2,y_val)]);
        opcode = 0x8121; // set vx to vx | vy 
        cpu.set_vx_vy(opcode); 
        assert_eq!(cpu.v[1], x_val | y_val);
        // AND TEST 
        set_registers(&mut cpu, &[(1, x_val), (2,y_val)]);
        opcode = 0x8122; // set vx to vx & vy 
        cpu.set_vx_vy(opcode); 
        assert_eq!(cpu.v[1], x_val & y_val);
         // XOR TEST 
        set_registers(&mut cpu, &[(1, x_val), (2,y_val)]);
        opcode = 0x8123; // set vx to vx ^ vy 
        cpu.set_vx_vy(opcode); 
        assert_eq!(cpu.v[1], x_val ^ y_val);
    }

    // uses array of (register idx, register val) to set register easily
    fn set_registers( cpu : &mut Chip8CPU, register_vals : &[(u8, u8)]) { 

        for (register, val) in register_vals { 
            let opcode =( 0x6000 | (*register as u16)<< 8 )| (*val as u16); 
            cpu.set_vx(opcode); 
        }
    }

    fn check_fontset(arr : &[u8]) { 
        assert_eq!(&arr[0x50..0x50+FONTSET.len()], &FONTSET[..])
    }
}



