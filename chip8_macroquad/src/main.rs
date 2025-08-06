use chip8::Chip8CPU;
use macroquad::color::{Color, colors};
use macroquad::prelude::*;

const VIDEO_WIDTH: u32 = 64;
const VIDEO_HEIGHT: u32 = 32;
// This is the pixel width in our screen that corresponds to one chip8 pixel
const PIXEL_WIDTH: u32 = 8;

const GRID_CELL_SIZE: (u32, u32) = (PIXEL_WIDTH, PIXEL_WIDTH);

const SCREEN_SIZE: (i32, i32) = (
    (VIDEO_WIDTH * GRID_CELL_SIZE.0) as i32,
    (VIDEO_HEIGHT * GRID_CELL_SIZE.1) as i32,
);

struct Chip8Emulator {
    cpu: Chip8CPU,
}

#[macroquad::main("Chip8 Emulator")]
async fn main() {
    let mut image = Image::gen_image_color(64, 32, colors::WHITE);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let file_name = "roms/3-corax+.ch8";

    let mut cpu = Chip8CPU::new();

    cpu.load_rom_from_file(String::from(file_name));

    let mut emulator = Chip8Emulator { cpu };

    loop {
        clear_background(BLACK);

        let screen_buffer = emulator.cpu.peek_display_buffer();

        for (i, &pixel) in screen_buffer.iter().enumerate() {
            let x = (i % 64) as u32;
            let y = (i / 64) as u32;
            let color = if pixel != 0 {
                colors::WHITE
            } else {
                colors::BLACK
            };
            image.set_pixel(x, y, color);
        }

        texture.update(&image);

        draw_texture_ex(
            &texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        let _ = emulator.cpu.cycle();

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
