use crate::Result;
use rustix::fs::{stat, AtFlags};

pub struct Metadata(rustix::fs::Stat);

pub fn metadata<'a, P: rustix::path::Arg>(path: P) -> Result<Metadata> {
    let info = stat(path)?;
    Ok(Metadata(info))
}

pub fn exists<P: rustix::path::Arg>(path: P) -> bool {
    metadata(path).is_ok()
}

pub fn exists_at<D: rustix::fd::AsFd, P: rustix::path::Arg>(dir: D, path: P) -> bool {
    rustix::fs::statat(dir, path, AtFlags::empty()).is_ok()
}
