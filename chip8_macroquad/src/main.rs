use std::collections::HashSet;

use chip8::Chip8CPU;
use macroquad::color::colors;
use macroquad::ui::{hash, widgets};
use macroquad::{prelude::*, ui};

struct Chip8Emulator {
    cpu: Chip8CPU,
    texture: Texture2D,
    image: Image,
}

struct Chip8Keyboard {}

// CHIP-8 Keypad    User Keyboard
// +-+-+-+-+        +-+-+-+-+
// |1|2|3|C|        |1|2|3|4|
// +-+-+-+-+        +-+-+-+-+
// |4|5|6|D|        |Q|W|E|R|
// +-+-+-+-+   <=   +-+-+-+-+
// |7|8|9|E|        |A|S|D|F|
// +-+-+-+-+        +-+-+-+-+
// |A|0|B|F|        |Z|X|C|V|
// +-+-+-+-+        +-+-+-+-+

// const CHIP8_KEYBOARD = {
//   // KEYBOARD
//   1: 0x1,
//   2: 0x2,
//   3: 0x3,
//   4: 0xc,

//   q: 0x4,
//   w: 0x5,
//   e: 0x6,
//   r: 0xd,

//   a: 0x7,
//   s: 0x8,
//   d: 0x9,
//   f: 0xe,

//   z: 0xa,
//   x: 0x0,
//   c: 0xb,
//   v: 0xf,
// };

impl Chip8Emulator {
    pub fn draw(&mut self) {
        let emulator = self;
        let screen_buffer = emulator.cpu.peek_display_buffer();

        for (i, &pixel) in screen_buffer.iter().enumerate() {
            let x = (i % 64) as u32;
            let y = (i / 64) as u32;
            let color = if pixel != 0 {
                colors::WHITE
            } else {
                colors::BLACK
            };
            emulator.image.set_pixel(x, y, color);
        }

        emulator.texture.update(&emulator.image);

        draw_texture_ex(
            &emulator.texture,
            screen_width() / 4.0,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width() / 2.0, screen_height() / 2.0)),
                ..Default::default()
            },
        );
    }

    pub fn key_presses(&mut self, keys: HashSet<KeyCode>) {
        // Clear all keys first
        for i in 0..16 {
            self.cpu.set_keyboard(i, 0);
        }

        // Set pressed keys
        for key in keys {
            self.key_press(key);
        }
    }

    pub fn key_press(&mut self, key: KeyCode) {
        if let Some(chip8_key) = self.keycode_to_chip8(key) {
            self.cpu.set_keyboard(chip8_key, 1);
        }
    }

    /// Maps Macroquad KeyCode to CHIP-8 keypad values
    /// Based on the standard CHIP-8 keyboard layout:
    /// CHIP-8 Keypad    User Keyboard
    /// +-+-+-+-+        +-+-+-+-+
    /// |1|2|3|C|        |1|2|3|4|
    /// +-+-+-+-+        +-+-+-+-+
    /// |4|5|6|D|        |Q|W|E|R|
    /// +-+-+-+-+   <=   +-+-+-+-+
    /// |7|8|9|E|        |A|S|D|F|
    /// +-+-+-+-+        +-+-+-+-+
    /// |A|0|B|F|        |Z|X|C|V|
    /// +-+-+-+-+        +-+-+-+-+
    fn keycode_to_chip8(&self, key: KeyCode) -> Option<u8> {
        match key {
            // Top row: 1,2,3,C
            KeyCode::Key1 => Some(0x1),
            KeyCode::Key2 => Some(0x2),
            KeyCode::Key3 => Some(0x3),
            KeyCode::Key4 => Some(0xC),

            // Second row: 4,5,6,D
            KeyCode::Q => Some(0x4),
            KeyCode::W => Some(0x5),
            KeyCode::E => Some(0x6),
            KeyCode::R => Some(0xD),

            // Third row: 7,8,9,E
            KeyCode::A => Some(0x7),
            KeyCode::S => Some(0x8),
            KeyCode::D => Some(0x9),
            KeyCode::F => Some(0xE),

            // Bottom row: A,0,B,F
            KeyCode::Z => Some(0xA),
            KeyCode::X => Some(0x0),
            KeyCode::C => Some(0xB),
            KeyCode::V => Some(0xF),

            _ => None,
        }
    }
}

#[macroquad::main("Chip8 Emulator")]
async fn main() {
    let image = Image::gen_image_color(64, 32, colors::WHITE);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let file_name = "roms/snake.ch8";
    // let file_name = "roms/3-corax+.ch8";

    let mut cpu = Chip8CPU::new();

    cpu.load_rom_from_file(String::from(file_name));

    let mut emulator = Chip8Emulator {
        cpu,
        texture,
        image,
    };

    loop {
        clear_background(BLUE);

        emulator.draw();
        // let frame = get_frame_time();

        // println!("frame time: {}", frame);

        draw_fps();

        ui::root_ui().window(
            hash!(),
            Vec2::new(screen_width() / 4., screen_height() / 2.),
            Vec2::new(screen_width() / 2., screen_height() / 4.),
            |ui| {
                ui.button(None, "1");
                ui.button(Vec2::new(screen_width() / 8., 0.0), "2");
                ui.button(Vec2::new(screen_width() / 4., 0.0), "3");
                ui.button(Vec2::new(3.*screen_width() / 8., 0.0), "C");
            },
        );

        // Handle keyboard input - get all currently pressed keys
        let pressed_keys = get_keys_down();
        emulator.key_presses(pressed_keys);

        for _ in 0..8 {
            let _ = emulator.cpu.cycle();
        }

        next_frame().await
    }
}
