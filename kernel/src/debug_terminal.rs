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

pub fn init(framebufferinfo: FrameBufferInfo, display: Display){
    let height = framebufferinfo.height;
    let (mut upper, mut lower) = display.split_at_line(height / 2);

    upper.clear(Rgb888::RED).unwrap_or_else(infallible);
    lower.clear(Rgb888::BLUE).unwrap_or_else(infallible);

    let style = PrimitiveStyle::with_fill(Rgb888::YELLOW);
    Circle::new(Point::new(50, 50), 300)
        .draw_styled(&style, &mut upper)
        .unwrap_or_else(infallible);

    let character_style = MonoTextStyle::new(&FONT_10X20, Rgb888::BLUE);
    let text = Text::new("Hello, world!", Point::new(140, 210), character_style);
    text.draw(&mut upper).unwrap_or_else(infallible);
}