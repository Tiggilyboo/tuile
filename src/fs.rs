use crate::{path::Path, Result};
use alloc::vec::Vec;
use rustix::fs::{stat, OFlags};

pub struct Metadata(rustix::fs::Stat);

pub fn metadata(path: Path) -> Result<Metadata> {
    let stat64 = stat(path)?;
    Ok(Metadata(stat64))
}

pub fn exists(path: Path) -> bool {
    metadata(path).is_ok()
}
