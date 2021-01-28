mod utils;

use wasm_bindgen::prelude::*;
use chip8::Chip8CPU; 
use chip8::dissassembler::disassemble; 

use js_sys::DataView; 

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-chip8!");
}

#[wasm_bindgen]
pub struct WasmChip8 { 
    cpu : Chip8CPU 
}

#[wasm_bindgen]
impl WasmChip8 { 
    pub fn new() -> WasmChip8 { 
        utils::set_panic_hook(); 
        
        WasmChip8 { 
            cpu : Chip8CPU::new() 
        }
    }

    pub fn cycle(&mut self) { 
        self.cpu.cycle() 
    }

    pub fn reset(&mut self) { 
        self.cpu.reset(); 
    }

    pub fn key_down(&mut self, key : u8) { 
        self.cpu.set_keyboard(key, 1); 
    }

    pub fn key_up(&mut self, key : u8) { 
        self.cpu.set_keyboard(key, 0); 
    }

    // get a pointer to the display buffer for use in JS
    pub fn get_display(&self) -> *const u8 { 
        self.cpu.clone_display_buffer().as_ptr() 
    }

    pub fn get_memory(&self) -> *const u8 { 
        self.cpu.clone_memory().as_ptr() 
    }

    pub fn load_rom_js(&mut self, data : DataView) { 
        let mut rom : Vec<u8>= Vec::with_capacity(data.byte_length());

        for i in 0..data.byte_length()  { 
            rom.push(data.get_uint8(i));
        };
        self.cpu.load_rom_from_bytes(rom.as_slice());
    }

    pub fn pc(&self) -> u16 { 
        self.cpu.pc() 
    }
}
