
// Chip-8 CPU capable of reading and processing instructions
pub struct Chip8<T, V> where T : Keypad, V : Display{ 

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

    // A type that handles the input and decodes to a keypad input for chip-8
    keypad : T, 

    // a type that implements reading from the display buffer and displaying the chip-8 pixels
    display : V, 
}


impl<T: Keypad,V : Display>  Chip8<T,V>{ 

    pub fn load_rom(&mut self, filename : String) { 

    }

}


pub trait Keypad { 

}

pub trait Display {  

}



