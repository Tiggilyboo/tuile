use alloc::{ffi::CString, string::String};
use core::fmt::Display;
use rustix::io::Errno;

use crate::error::Error;

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

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Path<'a> {
    inner: &'a str,
}

impl<'a> Path<'a> {
    pub fn is_relative(&self) -> bool {
        !self.is_absolute()
    }

    pub fn is_absolute(&self) -> bool {
        #[cfg(target_os = "windows")]
        if self.inner.nth(1).eq(':') {
            return true;
        }
        #[cfg(not(target_os = "windows"))]
        if self.inner.starts_with(MAIN_SEPARATOR) {
            return true;
        }

        return false;
    }

    pub fn exists(&self) -> bool {
        if self.is_absolute() {
            crate::fs::exists(self.inner)
        } else {
            crate::fs::exists_at(rustix::fs::CWD, self.inner)
        }
    }
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
        match CString::new(self.inner) {
            Ok(cstr) => f(&cstr),
            // TODO: Better error code?
            Err(e) => Err(Errno::FAULT),
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
