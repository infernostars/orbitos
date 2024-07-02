use bootloader_api::info::FrameBufferInfo;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::Drawable;
use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use embedded_graphics::primitives::{Circle, PrimitiveStyle, StyledDrawable};
use embedded_graphics::text::Text;

use crate::framebuffer::Display;
use crate::infallible;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScreenChar {
    ascii_character: u8
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorPair {
    fg: Rgb888,
    bg: Rgb888,
}

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; 512usize]; 256usize], // If a screen is larger than 5120x5120 I will die in real life.
}

pub struct Writer {
    column_position: usize,
    buffer: &'static mut Buffer,
    color: ColorPair,
    screen_width: usize,
    screen_height: usize,
    display: Display
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= self.screen_width {
                    self.new_line();
                }

                let row = self.screen_height - 1;
                let col = self.column_position;

                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                };
                self.column_position += 1;
            }
        }
    }

    fn render(&mut self) {
        self.display.clear(self.color.bg).unwrap();
        let character_style = MonoTextStyle::new(&FONT_10X20, self.color.fg);

    }

    fn new_line(&mut self) {/* TODO */}
}


pub fn init(framebufferinfo: FrameBufferInfo, mut display: Display){
    let characters_vertical = framebufferinfo.height / 20;
    let characters_horizontal = framebufferinfo.width / 10;

    display.clear(Rgb888::BLACK).unwrap_or_else(infallible);

    let temp_writer = Writer {
        column_position: 0,
        buffer: &mut Buffer { chars: [[ScreenChar{ascii_character: 0}; 512]; 256] },
        color: ColorPair { fg: Rgb888::WHITE, bg: Rgb888::BLACK},
        screen_width: characters_horizontal,
        screen_height: characters_vertical,
        display,
    };


}