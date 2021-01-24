use chip8::Chip8CPU;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;


const VIDEO_WIDTH : u32 = 64; 
const VIDEO_HEIGHT : u32= 32; 
const PIXEL_WIDTH : u32 = 12; 


struct SdlDisplay { 
    canvas : Canvas<Window>
}

impl SdlDisplay { 

    pub fn new_default(ctx : &sdl2::Sdl) -> SdlDisplay{ 

       
        let video_subsystem = ctx.video().unwrap();
    
        let window = video_subsystem.window("rust-sdl2 demo", VIDEO_WIDTH * PIXEL_WIDTH, VIDEO_HEIGHT * PIXEL_WIDTH)
            .position_centered()
            .build()
            .unwrap();
    
        let mut canvas = window.into_canvas().build().unwrap();
    
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.clear();
        canvas.present();
        
        SdlDisplay { 
            canvas
        }
    }

    pub fn draw(&mut self, chip8 : &Chip8CPU) { 
        let chip8_pixels = chip8.clone_display_buffer();

        // 
        for row in 0..VIDEO_HEIGHT { 
            for col in 0..VIDEO_WIDTH { 
                let i = (col  + row * VIDEO_WIDTH ) as usize;
                let pixel_val = chip8_pixels[i];
                let color = self.get_color(pixel_val); 
                self.canvas.set_draw_color(color);

                let _ = 
                self.canvas.fill_rect(Rect::new(col as i32 * PIXEL_WIDTH as i32, row as i32 * PIXEL_WIDTH as i32, PIXEL_WIDTH, PIXEL_WIDTH));
            }
        }
        self.canvas.present();
    }

    fn get_color(&self, pixel_val : u8) -> Color { 
        if pixel_val == 0 { 
            Color::RGB(0, 0, 0)
        }
        else { 
            Color::RGB(0, 255, 0)
        }
    }
}

fn main() {
    let file_name = "roms/test_opcode.ch8";
    let mut cpu = Chip8CPU::new();
    cpu.load_rom_from_file(String::from(file_name));

    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut display = SdlDisplay::new_default(&sdl_context);

    'running: loop {

        let memory = cpu.clone_memory(); 

        for _ in 0..1 { // emulate a 500Hz cpu clockrate 
            let pc = cpu.pc() as usize; 
        //(self.memory[self.pc as usize] as u16) << 8 | (self.memory[(self.pc + 1) as usize]) as u16 
            let opcode = (memory[pc]  as u16 ) << 8| (memory[pc+ 1]as u16) ;
            let instr = chip8::dissassembler::disassemble(opcode);

            println!("pc at {} opcode {:#04X?} instruction: {}", pc, opcode, instr);

            cpu.cycle();
        };
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        display.draw(&mut cpu);

        
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }


}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}