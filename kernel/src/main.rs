#![no_std]
#![no_main]

use core::{convert::Infallible, panic::PanicInfo};

use bootloader_api::BootInfo;
use conquer_once::spin::OnceCell;
use embedded_graphics::{
    draw_target::DrawTarget, pixelcolor::RgbColor, primitives::StyledDrawable, Drawable,
};

use crate::logging::init_logger;
use log::{error, info, warn};
mod debug_terminal;
mod framebuffer;
mod logging;

bootloader_api::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        init_logger(framebuffer.buffer_mut(), info, log::LevelFilter::Info, true)
    }
    info!("test logger");
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
