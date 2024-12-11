use core::fmt::{Debug, Display};
use core::{fmt::Formatter, time::Duration};

const NANOS_PER_SEC: u32 = 1_000_000_000;
const NANOS_PER_MILLI: u32 = 1_000_000;
const NANOS_PER_MICRO: u32 = 1_000;
const MILLIS_PER_SEC: u64 = 1_000;
const MICROS_PER_SEC: u64 = 1_000_000;
const SECS_PER_MINUTE: u64 = 60;
const MINS_PER_HOUR: u64 = 60;
const HOURS_PER_DAY: u64 = 24;
const DAYS_PER_WEEK: u64 = 7;

#[derive(Debug, Clone, Copy)]
pub struct Instant(rustix::time::Timespec);

impl Display for Instant {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let secs = self.0.tv_sec;
        let hrs = (secs / 3600) % 24;
        let min = (secs % 3600) / 60;
        let sec = secs % 60;
        let ss = self.as_nanos() % NANOS_PER_SEC as u64;

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

    pub fn as_secs(&self) -> u64 {
        self.0.tv_sec as u64
    }

    pub fn as_nanos(&self) -> u64 {
        self.0.tv_nsec as u64
    }
}

pub fn now() -> Instant {
    Instant::now()
}

pub fn system() -> Instant {
    Instant::system()
}
