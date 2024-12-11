use crate::{
    error::{Error, ErrorCode},
    io, time, Result,
};

static LOGGER: Logger = Logger;
struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let now = time::now();
        let line = crate::format!("[{}] {:6} {}\n", now, record.level(), record.args());

        match record.level() {
            log::Level::Error => {
                io::write_stderr(line.as_bytes()).ok();
            }
            _ => {
                io::println(&line).ok();
            }
        }
    }

    fn flush(&self) {}
}

pub fn init_with_level(level: log::LevelFilter) -> Result<()> {
    match log::set_logger(&LOGGER).map(|_| log::set_max_level(level)) {
        Err(e) => Err(Error(ErrorCode::LoggerError(e))),
        Ok(_) => Ok(()),
    }
}

pub fn init() -> Result<()> {
    init_with_level(log::LevelFilter::Error)
}