use crate::{
    io,
    time::{self, Instant},
};

pub struct Logger {
    start: Instant,
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let now = time::now().elapsed(&self.start);
        let sec = now.as_secs();
        let hrs = sec / 3600;
        let min = (sec / 60) % 60;
        let sec = sec % 60;
        let ms = now.subsec_nanos() / 1_000_000;

        let line = crate::format!(
            "[{:02}:{:02}:{:02}.{:03}] {:6} {}\n",
            hrs,
            min,
            sec,
            ms,
            record.level(),
            record.args()
        );

        match record.level() {
            log::Level::Error => {
                io::write_stderr(line.as_bytes()).ok();
            }
            log::Level::Debug => {
                io::write_stdout(line.as_bytes()).ok();
            }
            _ => (),
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Logger {
    let start = time::now();
    Logger { start }
}
