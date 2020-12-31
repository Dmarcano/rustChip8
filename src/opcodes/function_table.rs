//! This
//! 

use super::*;

/* the function table keeps track of what function to use based on the opcode there is a first table that maps the digits
    0x0-0xF to their valid opcode functions. 
*/


pub(crate) type OpcodeFn = fn (&mut Chip8CPU, u16); 

pub(crate) type OpcodeFnGetter = fn (u16) -> OpcodeFn; 


impl Chip8CPU {

    pub(crate) fn create_function_table() -> [OpcodeFnGetter; 16] { 
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
    
    pub(crate) fn table_0(opcode : u16) -> OpcodeFn { 

        let idx = opcode & 0x000F;
        
        match idx {
            0x0 => {Chip8CPU::clear_display},
            0xE => {Chip8CPU::ret},
            _ => {panic!("Unexpected opcode for table e! got opcode: {}", opcode)}
        }

        // unimplemented!()
    } 

    fn table_1(_opcode : u16) -> OpcodeFn { 
        return Chip8CPU::jmp_addr; 
    }

    fn table_2(_opcode : u16) -> OpcodeFn { 
        return Chip8CPU::call_addr; 
    } 

    fn table_3(_opcode : u16) -> OpcodeFn { 
        return Chip8CPU::skip_vx; 
    } 

    fn table_4(_opcode : u16) -> OpcodeFn { 
        return Chip8CPU::skip_vx; 
    } 

   fn table_5(_opcode : u16) -> OpcodeFn { 
        return Chip8CPU::skip_vx_vy_eq;
    } 

    fn table_6(_opcode : u16) -> OpcodeFn { 
        return Chip8CPU::set_vx;
    } 

    fn table_7(_opcode : u16) -> OpcodeFn { 
        return Chip8CPU::set_vx;
    } 

   fn table_8(_opcode : u16) -> OpcodeFn { 
        return Chip8CPU::set_vx_vy;
    } 

   fn table_9(_opcode : u16) -> OpcodeFn { 
        return Chip8CPU::skip_vx_vy_ne;
    } 

   fn table_a(_opcode : u16) -> OpcodeFn { 
        return Chip8CPU::set_i;
    } 

    fn table_b(_opcode : u16) -> OpcodeFn { 
        return Chip8CPU::jmp_v0_addr;
    } 

    fn table_c(_opcode : u16) -> OpcodeFn { 
        return Chip8CPU::rnd_vx_byte;
    } 

    fn table_d(_opcode : u16) -> OpcodeFn { 
        return Chip8CPU::drw_vx_vy_n;
    } 

    fn table_e(opcode : u16) -> OpcodeFn { 

        let idx = opcode & 0x00FF;

        match idx { 
            0x9E => {Chip8CPU::skip_vx_keypad},
            0xA1 => {Chip8CPU::not_skip_vx_keypad},
            _ => {panic!("Unexpected opcode for table e! got opcode: {}", opcode)}
        }
    } 

    fn table_f(opcode : u16) -> OpcodeFn { 

        let idx = opcode & 0x00FF;

        match idx { 
            0x07 => {Chip8CPU::set_vx_delay_timer},
            0x0A => {Chip8CPU::load_keypress_vx},
            0x15 => {Chip8CPU::set_delay_timer_vx},
            0x18 => {Chip8CPU::set_snd_timer_vx},
            0x1E => {Chip8CPU::add_idx_vx},
            0x29 => {Chip8CPU::set_idx_font_sprite_vx},
            0x33 => {Chip8CPU::set_idx_bcd_vx},
            0x55=> {Chip8CPU::write_x_registers},
            0x65 => {Chip8CPU::read_x_registers},
            _ => {panic!("Unexpected opcode for table e! got opcode: {}", opcode)}
        }
    } 
}