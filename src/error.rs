extern crate alloc;
use alloc::string::ToString;
use core::{
    ffi::FromBytesWithNulError,
    fmt::{Debug, Display},
    str::Utf8Error,
};
use rustix::io::Errno;

#[derive(Debug)]
pub struct Error(ErrorCode);

#[derive(Debug)]
pub enum ErrorCode {
    Rustix(rustix::io::Errno),
    Utf8Error(Utf8Error),
    FromBytesWithNulError(FromBytesWithNulError),
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::Rustix(e) => write!(f, "{}", e),
            Self::Utf8Error(e) => write!(f, "{}", e),
            Self::FromBytesWithNulError(e) => write!(f, "{}", e),
        }
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Utf8Error> for Error {
    fn from(value: Utf8Error) -> Self {
        Error(ErrorCode::Utf8Error(value))
    }
}
impl From<Errno> for Error {
    fn from(value: Errno) -> Self {
        Error(ErrorCode::Rustix(value))
    }
}
impl From<crate::FromUtf8Error> for Error {
    fn from(value: crate::FromUtf8Error) -> Self {
        Error(ErrorCode::Utf8Error(value.utf8_error()))
    }
}
impl From<rustix::ffi::FromBytesWithNulError> for Error {
    fn from(value: rustix::ffi::FromBytesWithNulError) -> Self {
        Error(ErrorCode::FromBytesWithNulError(value))
    }
}

impl core::error::Error for Error {}
