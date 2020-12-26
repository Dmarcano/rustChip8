#[cfg(test)]

use super::*;
use super::FONTSET; 
use super::Chip8CPU;

#[test]
fn init_test() {
    let mut cpu = Chip8CPU::new(); 

    let exp_v : [u8; 16]= [0; 16];
    let exp_pc = START_ADDR as u16; 
    let exp_stack = [0; 16];
    let exp_disp_buf = [0; 32 * 64]; 
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



fn check_fontset(arr : &[u8]) { 
    assert_eq!(&arr[0x50..0x50+FONTSET.len()], &FONTSET[..])
}




