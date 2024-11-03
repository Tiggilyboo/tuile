use core::{ffi::FromBytesWithNulError, fmt::Display};

use alloc::string::String;
use rustix::io::Errno;

#[cfg(target_os = "windows")]
pub const MAIN_SEPARATOR: char = '\\';
#[cfg(not(target_os = "windows"))]
pub const MAIN_SEPARATOR: char = '/';

#[derive(Debug)]
pub struct PathBuf {
    inner: String,
}

impl Display for PathBuf {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl PathBuf {
    pub fn new() -> Self {
        Self {
            inner: String::new(),
        }
    }
}

impl From<String> for PathBuf {
    fn from(s: String) -> PathBuf {
        PathBuf { inner: s }
    }
}

impl<'a> From<&'a PathBuf> for Path<'a> {
    fn from(buf: &'a PathBuf) -> Path {
        Path { inner: &buf.inner }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Path<'a> {
    inner: &'a str,
}

impl<'a> Display for Path<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<'a> From<&'a str> for Path<'a> {
    fn from(value: &'a str) -> Self {
        Path { inner: value }
    }
}

impl<'a> rustix::path::Arg for Path<'a> {
    fn as_str(&self) -> rustix::io::Result<&str> {
        Ok(self.inner)
    }

    fn into_with_c_str<T, F>(self, f: F) -> rustix::io::Result<T>
    where
        Self: Sized,
        F: FnOnce(&core::ffi::CStr) -> rustix::io::Result<T>,
    {
        let b = self.inner.as_bytes();
        match core::ffi::CStr::from_bytes_with_nul(b) {
            Ok(s) => f(s),
            // TODO: What's a better way to do this?
            Err(_) => Err(Errno::NOSTR),
        }
    }
}

impl Default for PathBuf {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for PathBuf {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Eq for PathBuf {}
