use core::fmt::{Debug, Display};
use core::{fmt::Formatter, time::Duration};

#[derive(Debug)]
pub struct Instant(rustix::time::Timespec);

impl Display for Instant {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let secs = self.0.tv_sec;
        let hrs = (secs / 3600) % 24;
        let min = (secs % 3600) / 60;
        let sec = secs % 60;
        let ss = (secs % 1_000_000) / 1_000;

        write!(f, "{:02}:{:02}:{:02}.{:03}", hrs, min, sec, ss)
    }
}

impl Instant {
    pub fn now() -> Instant {
        Instant(rustix::time::clock_gettime(
            rustix::time::ClockId::Monotonic,
        ))
    }

    pub fn system() -> Instant {
        Instant(rustix::time::clock_gettime(rustix::time::ClockId::Realtime))
    }

    pub fn elapsed(&self, before: &Instant) -> core::time::Duration {
        let ns = self.0.tv_nsec - before.0.tv_nsec;
        Duration::from_nanos(ns as u64)
    }
}

pub fn now() -> Instant {
    Instant::now()
}

pub fn system() -> Instant {
    Instant::system()
}
