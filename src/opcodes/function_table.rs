//! This
//! 

use super::*;

/* the function table keeps track of what function to use based on the opcode there is a first table that maps the digits
    0x0-0xF to their valid opcode functions. 
*/


pub(crate) type opcode_fn = fn (&mut Chip8CPU, u16); 

pub(crate) type opcode_function_getter = fn (u16) -> opcode_fn; 




impl Chip8CPU {

    pub(crate) fn create_function_table() -> [opcode_function_getter; 16] { 
        [
            Chip8CPU::table_0, 
            Chip8CPU::table_1,
            Chip8CPU::table_2, 
            Chip8CPU::table_3, 
            Chip8CPU::table_4, 
            Chip8CPU::table_5, 
            Chip8CPU::table_6, 
            Chip8CPU::table_7, 
            Chip8CPU::table_8, 
            Chip8CPU::table_9, 
            Chip8CPU::table_a, 
            Chip8CPU::table_b, 
            Chip8CPU::table_c, 
            Chip8CPU::table_d, 
            Chip8CPU::table_e, 
            Chip8CPU::table_f, 
            
        ]
    }
    
    pub(crate) fn table_0(opcode : u16) -> opcode_fn { 

        let idx = opcode & 0x000F;
        
        match idx {
            0x0 => {Chip8CPU::clear_display},
            0xE => {Chip8CPU::ret},
            _ => {panic!("Wrong opcode used for function table!")}
        }

        // unimplemented!()
    } 

    pub(crate) fn table_1(_opcode : u16) -> opcode_fn { 
        return Chip8CPU::jmp_addr; 
    }

    pub(crate) fn table_2(_opcode : u16) -> opcode_fn { 
        return Chip8CPU::call_addr; 
    } 

    pub(crate) fn table_3(_opcode : u16) -> opcode_fn { 
        return Chip8CPU::skip_vx; 
    } 

    pub(crate) fn table_4(_opcode : u16) -> opcode_fn { 
        return Chip8CPU::skip_vx; 
    } 

    pub(crate) fn table_5(_opcode : u16) -> opcode_fn { 
        return Chip8CPU::skip_vx_vy_eq;
    } 

    pub(crate) fn table_6(_opcode : u16) -> opcode_fn { 
        return Chip8CPU::set_vx;
    } 

    pub(crate) fn table_7(_opcode : u16) -> opcode_fn { 
        return Chip8CPU::set_vx;
    } 

    pub(crate) fn table_8(_opcode : u16) -> opcode_fn { 
        return Chip8CPU::set_vx_vy;
    } 

    pub(crate) fn table_9(_opcode : u16) -> opcode_fn { 
        return Chip8CPU::skip_vx_vy_ne;
    } 

    pub(crate) fn table_a(_opcode : u16) -> opcode_fn { 
        return Chip8CPU::set_i;
    } 

    pub(crate) fn table_b(_opcode : u16) -> opcode_fn { 
        return Chip8CPU::jmp_v0_addr;
    } 

    pub(crate) fn table_c(_opcode : u16) -> opcode_fn { 
        return Chip8CPU::rnd_vx_byte;
    } 

    pub(crate) fn table_d(_opcode : u16) -> opcode_fn { 
        return Chip8CPU::drw_vx_vy_n;
    } 

    pub(crate) fn table_e(opcode : u16) -> opcode_fn { 
        unimplemented!()
    } 

    pub(crate) fn table_f(opcode : u16) -> opcode_fn { 
        unimplemented!()
    } 
}