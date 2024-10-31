use crate::io;

pub struct Logger();

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        match record.level() {
            log::Level::Error => {
                let b = crate::format!("{:?}", record);
                io::write_stderr(b.as_bytes()).ok();
            }
            _ => (),
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Logger {
    Logger {}
}
