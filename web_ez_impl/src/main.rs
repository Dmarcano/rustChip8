//! Basic hello world example.
extern crate good_web_game as ggez;
use good_web_game::mint;

use ggez::event;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::{Context, GameResult};
use std::env;
use std::path;

use chip8::Chip8CPU;

// Width and height match the resolution of the chip-8 specification
const VIDEO_WIDTH : u32 = 64; 
const VIDEO_HEIGHT : u32= 32; 
// This is the pixel width in our screen that corresponds to one chip8 pixel
const PIXEL_WIDTH : u32 = 8; 

const GRID_CELL_SIZE: (u32, u32) = (PIXEL_WIDTH, PIXEL_WIDTH);

const SCREEN_SIZE: (i32, i32) = (
    (VIDEO_WIDTH * GRID_CELL_SIZE.0) as i32,
    (VIDEO_HEIGHT * GRID_CELL_SIZE.1) as i32,
);

// set FPS to better control emulation
const DESIRED_FPS: u32 = 8;


// First we make a structure to contain the game's state
struct MainState {
    frames: usize,
    text: graphics::Text,
    cpu: Chip8CPU,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // The ttf file will be in your resources directory. Later, we
        // will mount that directory so we can omit it in the path here.
        let font = graphics::Font::new(ctx, "/LiberationMono-Regular.ttf")?;
        let text = graphics::Text::new(("Hello world!", font, 48.0));

        let file_name = "roms/test_opcode.ch8";
        
        let file_name = "roms/4-flags.ch8";
        
        
        let mut cpu = Chip8CPU::new();

        cpu.load_rom_from_file(String::from(file_name));

        let s = MainState {
            frames: 0,
            text,
            cpu,
        };
        Ok(s)
    }

    fn get_color(&self, pixel_val : u8) -> Color { 
        if pixel_val == 0 { 
            Color::BLACK
        }
        else { 
            Color::GREEN
        }
    }
}

// To Quickly go from GG-ez rectangle back to x,y coordinates of our screen
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct PixelPosition {
    x: u32,
    y: u32,
}

impl PixelPosition {
    /// We make a standard helper function so that we can create a new `GridPosition`
    /// more easily.
    pub fn new(x: u32, y: u32) -> Self {
        PixelPosition { x, y }
    }
}

impl From<PixelPosition> for graphics::Rect {
    fn from(pos: PixelPosition) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32  *GRID_CELL_SIZE.0 as i32,
            pos.y as i32 * GRID_CELL_SIZE.1 as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        )
    }
}

impl From<(u32, u32)> for PixelPosition {
    fn from(pos: (u32, u32)) -> Self {
        PixelPosition::new(pos.0, pos.1) 
    }
}

// Then we implement the `ggez:event::EventHandler` trait on it, which
// requires callbacks for updating and drawing the game state each frame.
//
// The `EventHandler` trait also contains callbacks for event handling
// that you can override if you wish, but the defaults are fine.
impl event::EventHandler for MainState {
    fn update(
        &mut self,
        _ctx: &mut Context,
        _quad_ctx: &mut miniquad::graphics::GraphicsContext,
    ) -> GameResult {
        let error = self.cpu.cycle();

        match error {
            Ok(_) => {},
            Err(cycle_error) => println!("{:?}", cycle_error),
        }
        Ok(())
    }

    fn draw(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::graphics::GraphicsContext,
    ) -> GameResult {
        graphics::clear(ctx, quad_ctx, Color::BLACK);

        let chip8_pixels = self.cpu.clone_display_buffer();


          for row in 0..VIDEO_HEIGHT { 
            for col in 0..VIDEO_WIDTH { 
                let pos = PixelPosition::from((col, row));

                let i = (col  + row * VIDEO_WIDTH ) as usize;
                let pixel_val = chip8_pixels[i];
                let color = self.get_color(pixel_val); 


                if color == Color::BLACK { 
                    continue; 
                }

            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                quad_ctx,
                graphics::DrawMode::fill(),
                pos.into(),
                color,
            )?;
             graphics::draw(
                ctx,
                quad_ctx,
                &rectangle,
                (mint::Point2 { x: 0.0, y: 0.0 },),
            )?;

                // let _ = 
                // self.canvas.fill_rect(Rect::new(col as i32 * PIXEL_WIDTH as i32, row as i32 * PIXEL_WIDTH as i32, PIXEL_WIDTH, PIXEL_WIDTH));
            }
        }
        // self.canva

        graphics::present(ctx, quad_ctx)?;

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::fps(ctx));
            println!("drawable size: {:?}", graphics::drawable_size(quad_ctx));
        }

        Ok(())
    }

}

// Now our main function:
pub fn main() -> GameResult {
    // We add the CARGO_MANIFEST_DIR/resources to the resource paths
    // so that ggez will look in our cargo project directory for files.
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    ggez::start(
        ggez::conf::Conf::default()
            // .cache(Some(include_bytes!("resources.tar")))
            .physical_root_dir(Some(resource_dir)),
        |mut context, mut _quad_ctx| Box::new(MainState::new(&mut context).unwrap()),
    )
}
