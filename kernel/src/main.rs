#![no_std]
#![no_main]

use core::{convert::Infallible, panic::PanicInfo};

use bootloader_api::BootInfo;
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::{RgbColor},
    primitives::{StyledDrawable},
    Drawable,
};

mod framebuffer;
mod debug_terminal;

bootloader_api::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let fbinfo = framebuffer.info();
        let display = framebuffer::Display::new(framebuffer);
        debug_terminal::init(fbinfo, display)
    }
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn infallible<T>(v: Infallible) -> T {
    match v {}
}