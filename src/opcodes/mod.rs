use super::{Chip8CPU, SPRITE_WIDTH, START_ADDR, VIDEO_HEIGHT, VIDEO_WIDTH};

pub(crate) mod function_table;

pub mod dissassembler; 

// Op-Code implementations
impl Chip8CPU {
    /// Returns from subroutine using the stack to return to before the call was made
    ///
    /// for ```opcode => 0x00E0 ```
    fn clear_display(&mut self, _ : u16) {
        self.disp_buf.iter_mut().for_each(|m| *m = 0);
    }

    /// Returns from subroutine using the stack to return to before the call was made
    ///
    /// for ```opcode => 0x00EE```
    fn ret(&mut self, _ : u16) {
        // return from a subroutine
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    /// jumps to a specified address in the opcode
    ///
    /// for ```opcode => 0x1nnn```
    fn jmp_addr(&mut self, opcode: u16) {
        // jump to a given address
        let addr = opcode & 0x0FFF;
        self.pc = addr;
    }

    /// given an opcode representing a subroutine, it jumps the ```pc``` to the memory address while saving the previous
    /// address in the stack
    ///
    /// for ```opcode => 0x2nnn```
    fn call_addr(&mut self, opcode: u16) {
        // calls a function
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.jmp_addr(opcode);
    }

    /// given an opcode it compares Vx to kk in some way. Incrementing on a truthy Val
    /// 1. opcodes ```0x3xkk => skips if Vx == kk```
    /// 2. opcodes ```0x4xkk => skips if Vx != kk```
    fn skip_vx(&mut self, opcode: u16) {
        // both SNE and SE Vx byte

        let instruction = ((opcode & 0xF000) >> 12) as u8;
        let vx = ((opcode & 0x0F00) >> 8) as u8;
        let val = (opcode & 0x00FF) as u8;

        let equals = self.v[vx as usize] == val;

        match (instruction, equals) {
            (3, true) => self.increment_pc(),
            (3, false) => {}
            (4, true) => {}
            (4, false) => self.increment_pc(),
            (_, _) => {
                panic!("Given a bad opcode of value {}", opcode)
            }
        }
    }

    /// Skips next instruction based on ```Vx``` === ```Vy```
    ///
    /// 1. ```opcodes => 0x5xy0``` Skips if Vx == Vy
    fn skip_vx_vy_eq(&mut self, opcode: u16) {
        let vx = ((opcode & 0x0F00) >> 8) as u8;
        let vy = ((opcode & 0x00F0) >> 4) as u8;

        println!("vx: {}, vy {}", vx, vy);

        if self.v[vx as usize] == self.v[vy as usize] {
            self.increment_pc();
        }
    }

    /// Mutates Vx according to the opcode
    ///
    /// 1. ```opcodes => 0x6xkk``` sets ```Vx``` as the value ```kk```
    /// 2. ```opcodes => 0x7xkk``` adds the val ```kk``` to ```Vx```

    fn set_vx(&mut self, opcode: u16) {
        let instruction = (opcode & 0xF000) >> 12;
        let vx = ((opcode & 0x0F00) >> 8) as u8;
        let val = (opcode & 0x00FF) as u8;

        match instruction {
            6 => {
                self.v[vx as usize] = val;
            }
            7 => {
                self.v[vx as usize] += val;
            }
            _ => {
                panic!(
                    "Given a bad opcode of value {} expected some variant of 0x7... or ox6...",
                    opcode
                )
            }
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
    fn set_vx_vy(&mut self, opcode: u16) {
        let vx = ((opcode & 0x0F00) >> 8) as usize;
        let vy = ((opcode & 0x00F0) >> 4) as usize;

        let instruction = (opcode & 0x000F) as u8;

        match instruction {
            0 => self.v[vx] = self.v[vy],
            1 => self.v[vx] |= self.v[vy],
            2 => self.v[vx] &= self.v[vy],
            3 => self.v[vx] ^= self.v[vy],
            4 => {
                let (sum, of) = self.v[vx].overflowing_add(self.v[vy]);
                self.v[0xF] = of as u8;
                self.v[vx] = sum;
            }
            5 => {
                let (sum, of) = self.v[vx].overflowing_sub(self.v[vy]);
                self.v[0xF] = (!of) as u8; // set flag if vx > vy or no overflow occurs
                self.v[vx] = sum;
            }
            6 => {
                self.v[0xF] = self.v[vx] & 0x1;
                self.v[vx] >>= 1;
            }
            7 => {
                let (sum, of) = self.v[vx].overflowing_sub(self.v[vy]);
                self.v[0xF] = of as u8;
                self.v[vx] = sum;
            }
            0xE => {
                self.v[0xF] = (self.v[vx] & 0x80) >> 7;
                self.v[vx] <<= 1;
            }
            _ => {
                panic!(
                    "Given a bad opcode of value {}. Expected some variant of 0x8...",
                    opcode
                )
            }
        }
    }

    /// Skips next instruction based on ```Vx != Vy```
    ///
    /// 1. ```opcodes => 0x9xy0``` Skips if Vx ```!=``` Vy
    fn skip_vx_vy_ne(&mut self, opcode: u16) {
        let vx = ((opcode & 0x0F00) >> 8) as u8;
        let vy = ((opcode & 0x00F0) >> 4) as u8;

        if self.v[vx as usize] != self.v[vy as usize] {
            self.increment_pc();
        }
    }

    /// Set I = nnn.
    ///
    /// ```opcode => 0xAnnn```
    fn set_i(&mut self, opcode: u16) {
        self.index = opcode & 0x0FFF;
    }

    /// Jump to location V0 + addr
    ///
    /// ```opcode => 0xBnnn``` jumps to ```v[0] + 0xnnn```
    fn jmp_v0_addr(&mut self, opcode: u16) {
        let address = opcode & 0x0FFF;
        self.pc = self.v[0] as u16 + address;
    }

    /// Sets Vx to a random byte and input byte
    ///
    /// ```opcode => 0xCxkk``` sets v[x] = random byte & 0xkk
    fn rnd_vx_byte(&mut self, opcode: u16) {
        let vx = ((opcode & 0x0F00) >> 8) as usize;
        let val = (opcode & 0x00FF) as u8;

        self.v[vx] = self.random_byte() & val;
    }

    /// Display n-byte sprite starting in the index (vx, vy) and draws N bytes. sets VF to 0. If any two sprites collide then VF is set to 1.
    ///
    /// ```opcode => 0xDxyn```
    ///
    /// # Explanation
    ///
    /// A sprite byte is a byte read from memory starting at the address contained at the index register. Call this address I.
    /// The CPU will read each and every byte from address I up to address I + n
    ///
    /// A sprite byte is some collection of ```0bXXXX|XXXX``` byte where each ```X``` is a bit
    /// ( the | is a divisor between 4 bits or a "nibble" of a byte). Each byte corresponds to a row of our sprite and each bit corresponds to a single pixel.
    /// So given that there are 8 bits in a byte there are a total of 8 cols per row for a single sprite.
    ///
    /// This function then translates each and every byte into a set of pixel values. Pixels are ON or OFF and each bit in a sprite byte represents a pixel
    /// To find out if each pixel is ON or OFF one can AND the sprite byte with the value 0b1000|0000 and shifting. Should the AND ever be Non-zero then one knows that
    /// the pixel is ON.
    ///
    /// For Example take sprite 0xF6  which in binary is 0b1111|0110
    ///
    /// In the first iteration of the loop 0b1111|0110  is AND-ed with 0b1000|0000 this results in the value 0b1000|0000
    ///
    /// In the next iteration one shifts the bit down by 1. So one ANDs  0b1111|0110  with 0b0100|0000 resulting in 0b1000|0000
    ///
    /// In the 5th iteration one ANDs 0b1111|0110 with 0b0000|1000 which results in 0b0000|0000
    ///
    /// Any time any non-zero byte is found then the display buffer at location I + row is set to ON
    fn drw_vx_vy_n(&mut self, opcode: u16) {
        let vx = ((opcode & 0x0F00) >> 8) as usize;
        let vy = ((opcode & 0x00F0) >> 4) as usize;
        let sprite_len = (opcode & 0x000F) as usize;

        let x_pos = self.v[vx] % VIDEO_WIDTH;
        let y_pos = self.v[vy] % VIDEO_HEIGHT;

        // set collision register to 0 "no-collition"
        self.v[0xF] = 0;

        for row in 0..sprite_len {
            //grab a row of the spryte we are going to draw
            let sprite_byte = self.memory[self.index as usize + row];

            for col in 0..SPRITE_WIDTH as usize {
                let sprite_pixel = sprite_byte & (0x080 >> col);

                let screen_idx = ((y_pos as usize + row) * (VIDEO_WIDTH as usize)
                    + (x_pos as usize + col)) as usize;
                let mut screen_pixel = self.disp_buf[screen_idx];

                if sprite_pixel != 0x00 {
                    if screen_pixel != 0x0000 {
                        // collision occurs
                        self.v[0xF] = 1;
                    }

                    // XOR the screenPixel with the current spryte pixel
                    screen_pixel ^= 0xFF;
                    self.disp_buf[screen_idx] = screen_pixel;
                }
            }
        }
    }

    // TODO: Test functions below

    /// Skip the next instruction if key with the value of Vx is pressed.
    ///
    /// ```opcode => 0xEx9E```
    fn skip_vx_keypad(&mut self, opcode: u16) {
        let vx = ((opcode & 0x0F00) >> 8) as usize;
        let key = self.v[vx] as usize;

        if self.keyboard[key] != 0 {
            self.increment_pc();
        }
    }

    /// Skip the next instruction if key with the value of Vx is not pressed.
    ///
    /// ```opcode => 0xExA1```
    fn not_skip_vx_keypad(&mut self, opcode: u16) {
        let vx = ((opcode & 0x0F00) >> 8) as usize;
        let key = self.v[vx] as usize;

        if self.keyboard[key] == 0 {
            self.increment_pc();
        }
    }

    /// set Vx = delay timer val
    ///
    /// ```opcode => xFx07```
    fn set_vx_delay_timer(&mut self, opcode: u16) {
        let vx = ((opcode & 0x0F00) >> 8) as usize;
        self.v[vx] = self.delay_timer;
    }

    /// Wait for a key press, store the value of the key in Vx.
    ///
    /// ```opcode => 0xFx0A```
    fn load_keypress_vx(&mut self, opcode: u16) {
        let vx = ((opcode & 0x0F00) >> 8) as usize;
        for i in 0..self.keyboard.len() {
            if self.keyboard[i] != 0 {
                self.v[vx] = i as u8;
                return;
            }
        }
        self.decrement_pc(); // if a key is not pressed decrement the pc to rerun the instruction.
    }

    /// Set the delay timer equal to Vx
    ///
    /// ```opcode => 0xFx15```
    fn set_delay_timer_vx(&mut self, opcode: u16) {
        let vx = ((opcode & 0x0F00) >> 8) as usize;
        self.delay_timer = self.v[vx];
    }

    /// Set the sound timer equal to Vx
    ///
    /// ```opcode => 0xFx18```
    fn set_snd_timer_vx(&mut self, opcode: u16) {
        let vx = ((opcode & 0x0F00) >> 8) as usize;
        self.sound_timer = self.v[vx];
    }

    /// add vx to the index register. I = I + Vx
    ///
    /// ```opcode => 0xFx1E```
    fn add_idx_vx(&mut self, opcode: u16) {
        let vx = ((opcode & 0x0F00) >> 8) as usize;
        self.index += self.v[vx] as u16;
    }

    /// Sets the index register to the location of the start address of the Vx-th digit
    ///
    /// Here it is assumed that vx is bounded to between 0-15 since there are 15 Chip-8 Character Sprites
    ///
    /// ```opcode -> 0xFx29```
    fn set_idx_font_sprite_vx(&mut self, opcode: u16) {
        let vx = ((opcode & 0x0F00) >> 8) as usize;
        let digit = self.v[vx];

        self.index = (START_ADDR as u16) + ((5 * digit) as u16);
    }

    /// Store the BCD representation of Vx into the addresses I, I+1, I + 2
    ///
    /// ```opcode => 0xFx33```
    fn set_idx_bcd_vx(&mut self, opcode: u16) {
        let vx = ((opcode & 0x0F00) >> 8) as usize;
        let mut val = self.v[vx];

        // Ones-Place
        self.memory[self.index as usize + 2] = val % 10;
        val /= 10;

        // Tens-place
        self.memory[self.index as usize + 1] = val % 10;
        val /= 10;

        // Hundres Place
        self.memory[self.index as usize] = val % 10;
    }

    /// Load registers V0 through Vx in memory starting at memory address I up to I + X   
    ///
    /// ```opcode => 0xFx55```
    fn write_x_registers(&mut self, opcode: u16) {
        let vx = ((opcode & 0x0F00) >> 8) as usize;

        for i in 0..vx {
            self.memory[self.index as usize + i] = self.v[i];
        }
    }

    /// Load memory locations I to I + X to registers V0 through Vx
    ///
    /// ```opcode => 0xFx65```
    fn read_x_registers(&mut self, opcode: u16) {
        let vx = ((opcode & 0x0F00) >> 8) as usize;
        for i in 0..vx {
            self.v[i] = self.memory[self.index as usize + i];
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

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

        cpu.ret(opcode);
        assert_eq!(cpu.pc, 0xFAF); // return to previous address at 0xFAF

        cpu.reset();
        let v0_val = 0x020;
        set_registers(&mut cpu, &[(0, v0_val)]);
        opcode = 0xB111; // jump to V0 + 0x111
        cpu.jmp_v0_addr(opcode);
        assert_eq!(cpu.pc, v0_val as u16 + 0x111);
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
        set_registers(&mut cpu, &[(1, x_val), (2, y_val)]);

        // SET TEST
        let mut opcode = 0x8120; // set vx equal to vy
        cpu.set_vx_vy(opcode);
        assert_eq!(cpu.v[1], cpu.v[2]);
        // OR TEST
        set_registers(&mut cpu, &[(1, x_val), (2, y_val)]);
        opcode = 0x8121; // set vx to vx | vy
        cpu.set_vx_vy(opcode);
        assert_eq!(cpu.v[1], x_val | y_val);
        // AND TEST
        set_registers(&mut cpu, &[(1, x_val), (2, y_val)]);
        opcode = 0x8122; // set vx to vx & vy
        cpu.set_vx_vy(opcode);
        assert_eq!(cpu.v[1], x_val & y_val);
        // XOR TEST
        set_registers(&mut cpu, &[(1, x_val), (2, y_val)]);
        opcode = 0x8123; // set vx to vx ^ vy
        cpu.set_vx_vy(opcode);
        assert_eq!(cpu.v[1], x_val ^ y_val);
    }

    /// Testing of the Chip-8 CPU's ability to perform arithmetic instructions based on the state of two registers
    #[test]
    fn register_arithmetic_test() {
        let mut cpu = Chip8CPU::new();
        let x_val = 0x56;
        let y_val = 0x33;

        // Add registers no overflow
        set_registers(&mut cpu, &[(1, x_val), (2, y_val)]);
        let mut opcode = 0x8124; // add v[1] and v[2]
        cpu.set_vx_vy(opcode);
        let expected = x_val + y_val;
        assert_eq!(cpu.v[1], expected);
        assert_eq!(cpu.v[0xF], 0);
        // Add registers with overflow
        set_registers(&mut cpu, &[(1, x_val), (2, 0xFA)]);
        opcode = 0x8124; // add v[1] and v[2]
        cpu.set_vx_vy(opcode);
        let (expected, _) = x_val.overflowing_add(0xFA);
        assert_eq!(cpu.v[1], expected);
        assert_eq!(cpu.v[0xF], 1);

        // Subtract registers no overflow with VF expected to be set to 1 since X > Y
        set_registers(&mut cpu, &[(1, x_val), (2, y_val)]);
        opcode = 0x8125; // subtract borrow
        cpu.set_vx_vy(opcode);
        let expected = x_val - y_val;
        assert_eq!(cpu.v[1], expected);
        assert_eq!(cpu.v[0xF], 1);
        // Subtract registers with overflow with VF expected to be set to 0 since X <> Y
        set_registers(&mut cpu, &[(1, 0x01), (2, y_val)]);
        opcode = 0x8125;
        cpu.set_vx_vy(opcode);
        let (expected, _) = u8::overflowing_sub(0x01, y_val); // 0x01.overflowing_sub(y_val);
        assert_eq!(cpu.v[1], expected);
        assert_eq!(cpu.v[0xF], 0);

        // Subtract registers no overflow with VF expected to be set to 0 since X < Y
        set_registers(&mut cpu, &[(1, x_val), (2, y_val)]);
        opcode = 0x8127; // subtract no borrow
        cpu.set_vx_vy(opcode);
        let expected = x_val - y_val;
        assert_eq!(cpu.v[1], expected);
        assert_eq!(cpu.v[0xF], 0);
        // Subtract registers with overflow with VF expected to be set to 0 since X > Y
        set_registers(&mut cpu, &[(1, 0x01), (2, y_val)]);
        opcode = 0x8127;
        cpu.set_vx_vy(opcode);
        let (expected, _) = u8::overflowing_sub(0x01, y_val); // 0x01.overflowing_sub(y_val);
        assert_eq!(cpu.v[1], expected);
        assert_eq!(cpu.v[0xF], 1);
    }

    /// Testing of the Chip-8 CPU's ability to perform shifting instructions
    #[test]
    fn register_shifting_test() {
        let mut cpu = Chip8CPU::new();
        set_registers(&mut cpu, &[(1, 0xFF)]);

        // shift right with vf expected to be 1
        let mut opcode = 0x8126;
        let expected_vf = 1;
        cpu.set_vx_vy(opcode);
        assert_eq!(cpu.v[1], 0xFF >> 1);
        assert_eq!(cpu.v[0xF], expected_vf);
        // shift right with vf expected to be 0
        set_registers(&mut cpu, &[(1, 0xF0)]);
        opcode = 0x8126;
        let expected_vf = 0;
        cpu.set_vx_vy(opcode);
        assert_eq!(cpu.v[1], 0xF0 >> 1);
        assert_eq!(cpu.v[0xF], expected_vf);

        // shift left with vf expected to be 1
        set_registers(&mut cpu, &[(1, 0xFF)]);
        opcode = 0x812E;
        let expected_vf = 1;
        cpu.set_vx_vy(opcode);
        assert_eq!(cpu.v[1], 0xFF << 1);
        assert_eq!(cpu.v[0xF], expected_vf);
        // shift left with vf expected to be 0
        set_registers(&mut cpu, &[(1, 0x0F)]);
        opcode = 0x812E;
        let expected_vf = 0;
        cpu.set_vx_vy(opcode);
        assert_eq!(cpu.v[1], 0x0F << 1);
        assert_eq!(cpu.v[0xF], expected_vf);
    }

    /// Testing of the Chip-8 CPU's ability to perform jump instructions based on the state of two registers
    #[test]
    fn register_jump_test() {
        let mut cpu = Chip8CPU::new();
        let x_val = 0x56;
        let y_val = 0x33;

        // Jump if v[1] == v[2]
        set_registers(&mut cpu, &[(1, x_val), (2, y_val)]);
        let opcode = 0x5120; // jump
        cpu.skip_vx_vy_eq(opcode);
        assert_eq!(cpu.pc, START_ADDR as u16); // dont expect to jump

        println!("pc before {}", cpu.pc);
        set_registers(&mut cpu, &[(1, x_val), (2, x_val)]);
        let opcode = 0x5120; // jump
        cpu.skip_vx_vy_eq(opcode);
        println!("pc after {}", cpu.pc);

        assert_eq!(cpu.pc, (START_ADDR + 2) as u16); // expect to jump

        cpu.reset();

        // Jump if v[1] == v[2]
        set_registers(&mut cpu, &[(1, x_val), (2, x_val)]);
        let opcode = 0x9120; // jump
        cpu.skip_vx_vy_ne(opcode);
        assert_eq!(cpu.pc, START_ADDR as u16); // dont expect to jump

        set_registers(&mut cpu, &[(1, x_val), (2, y_val)]);
        let opcode = 0x9120; // jump
        cpu.skip_vx_vy_ne(opcode);
        assert_eq!(cpu.pc, (START_ADDR + 2) as u16); // expect to jump
    }

    #[test]
    fn index_register_test() {
        let mut cpu = Chip8CPU::new();
        let opcode = 0xA123;
        cpu.set_i(opcode);
        assert_eq!(cpu.index, 0x123)
    }

    /// test Chip8 CPU's drawing functions ability to draw sprites in expected coordinates
    /// next to one another without false collisions.
    #[test]
    fn display_test() {
        // sprites are 8-cols wide. and rely on the register values for starting x and y positions.
        let mut cpu = Chip8CPU::new();

        //sprite is a filled rectangle of two rows and 10 cols. Split into two sprites since each sprite is max 8 cols
        let rect: [u8; 4] = [0xFF, 0xFF, 0xC0, 0xC0];
        cpu.load_rom_from_bytes(rect.as_ref()); // load the sprites to the begining of memory
        cpu.index = START_ADDR as u16; //
                                       // the rectangle will start at
        set_registers(&mut cpu, &[(1, 1), (2, 1)]); // the start of the byte is at 1, 1
        let mut opcode = 0xD122; // read registers 1 and 2 and read 2 bytes from I to I + 1
        cpu.drw_vx_vy_n(opcode);
        cpu.index += 2; // increment index for next sprite
        opcode = 0xD122; // read registers 1 and 2 and read 2 bytes from I to I + 1
        set_registers(&mut cpu, &[(1, 9), (2, 1)]); // the start of the byte is at 1, 9. No collision expected here
        cpu.drw_vx_vy_n(opcode);
        // there should have been no collision
        assert_eq!(cpu.v[0xF], 0);
        assert_eq!([0; 64].as_ref(), cpu.disp_buf[0..64].as_ref()); // first row is empty

        let mut expected_col = [0; 64];
        for i in 0..10 {
            expected_col[i + 1] = 255;
        }

        assert_eq!(expected_col.as_ref(), cpu.disp_buf[64..128].as_ref());
        assert_eq!(expected_col.as_ref(), cpu.disp_buf[128..192].as_ref());
    }

    #[test]
    fn drw_collision_test() {
        let mut cpu = Chip8CPU::new();

        //sprite is a filled rectangle of two rows and 9 cols.
        // the first portion of the sprite is a filled 8 column rect, the next is a 2col square that collides with a side of the rectangle
        let rect: [u8; 4] = [0xFF, 0xFF, 0xC0, 0xC0];
        cpu.load_rom_from_bytes(rect.as_ref()); // load the sprites to the begining of memory
        cpu.index = START_ADDR as u16; //

        set_registers(&mut cpu, &[(1, 1), (2, 1)]);
        let mut opcode = 0xD122;
        cpu.drw_vx_vy_n(opcode);
        cpu.index += 2;
        opcode = 0xD122;
        set_registers(&mut cpu, &[(1, 8), (2, 1)]);
        cpu.drw_vx_vy_n(opcode);
        // there should have been no collision
        assert_eq!(cpu.v[0xF], 1);
        assert_eq!([0; 64].as_ref(), cpu.disp_buf[0..64].as_ref()); // first row is empty

        let mut expected_col = [0; 64];
        for i in 0..9 {
            expected_col[i + 1] = 255;
        }
        expected_col[8] = 0; // the XOR collision will cause one byte to be 0.

        assert_eq!(expected_col.as_ref(), cpu.disp_buf[64..128].as_ref());
        assert_eq!(expected_col.as_ref(), cpu.disp_buf[128..192].as_ref());
    }

    // uses array of (register idx, register val) to set register easily
    fn set_registers(cpu: &mut Chip8CPU, register_vals: &[(u8, u8)]) {
        for (register, val) in register_vals {
            let opcode = (0x6000 | (*register as u16) << 8) | (*val as u16);
            cpu.set_vx(opcode);
        }
    }
}
