mod utils;

use wasm_bindgen::prelude::*;
use chip8::Chip8CPU; 
use chip8::dissassembler::disassemble; 

use js_sys::DataView; 
use js_sys::Array;
use web_sys::console;

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

    pub fn cycle(&mut self) -> bool{ 
        match self.cpu.cycle() { 
            Ok(_) => {
                true 
            }
            Err(err) => { 
                console::error_1(&JsValue::from_str(err.message.as_str()));
                console::log(&self.disassemble_memory());
                false
            }
        }
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

    pub fn disassemble_memory(&self) -> Array { 

        let start = self.pc() as usize; 
        let mut end = start + 20; // grab at most 10 instructions
        let memory = self.cpu.peek_memory(); 
        // at most want to avoid overflowing the buffer
        if end > memory.len() { 
            end = memory.len();
        }

        // create an Array in JS to avoid serializing using Serde
        let memory_arr = Array::new(); 

        for i in (start..end).step_by(2) { 
            let opcode : u16= ((memory[i] as u16) << 8 ) | memory[i+1] as u16; 
            let instruction_str = format!("{:X}: {}", opcode, disassemble(opcode)); 
            memory_arr.push(&JsValue::from_str(instruction_str.as_str()));
            
        }    

        memory_arr
    }
}
