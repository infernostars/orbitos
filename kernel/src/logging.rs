use crate::debug_terminal::FrameBufferWriter;
use bootloader_api::info::FrameBufferInfo;
use conquer_once::spin::OnceCell;
use core::fmt::Write;
use spinning_top::Spinlock;

/// The global logger instance used for the `log` crate.
pub static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();

/// A logger instance protected by a spinlock.
pub struct LockedLogger {
    framebuffer: Option<Spinlock<FrameBufferWriter>>,
}

impl LockedLogger {
    /// Create a new instance that logs to the given framebuffer.
    pub fn new(
        framebuffer: &'static mut [u8],
        info: FrameBufferInfo,
        frame_buffer_logger_status: bool,
    ) -> Self {
        let framebuffer = match frame_buffer_logger_status {
            true => Some(Spinlock::new(FrameBufferWriter::new(framebuffer, info))),
            false => None,
        };
        LockedLogger { framebuffer }
    }

    /// Force-unlocks the logger to prevent a deadlock.
    ///
    /// ## Safety
    /// This method is not memory safe and should be only used when absolutely necessary.
    pub unsafe fn force_unlock(&self) {
        if let Some(framebuffer) = &self.framebuffer {
            unsafe { framebuffer.force_unlock() };
        }
    }
}

impl log::Log for LockedLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }
    fn log(&self, record: &log::Record) {
        if let Some(framebuffer) = &self.framebuffer {
            let mut framebuffer = framebuffer.lock();
            writeln!(framebuffer, "{:5}: {}", record.level(), record.args()).unwrap();
        }
    }
    fn flush(&self) {}
}

/// Initialize a text-based logger using the given pixel-based framebuffer as output.
pub fn init_logger(
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    log_level: log::LevelFilter,
    frame_buffer_logger_status: bool,
) {
    let logger = LOGGER
        .get_or_init(move || LockedLogger::new(framebuffer, info, frame_buffer_logger_status));
    log::set_logger(logger).expect("logger already set");
    log::set_max_level(log_level);
    log::info!("Framebuffer info: {:?}", info);
}
