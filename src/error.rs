extern crate alloc;
use core::{
    ffi::{self, FromBytesWithNulError},
    fmt::{Debug, Display},
    str::Utf8Error,
};
use alloc::{ffi::NulError, string::ToString};
use log::SetLoggerError;
use rustix::io::Errno;

#[derive(Debug)]
pub struct Error(pub ErrorCode);

#[derive(Debug)]
pub enum ErrorCode {
    Rustix(rustix::io::Errno),
    Utf8Error(Utf8Error),
    FromBytesWithNulError(FromBytesWithNulError),
    LoggerError(SetLoggerError),
    NulError(NulError),
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ErrorCode::Rustix(error_no) => write!(f, "{}", rustix::io::Errno::to_string(error_no)),
            _ => write!(f, "{:?}", self)
        }
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.0)
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
impl From<log::SetLoggerError> for Error {
    fn from(value: log::SetLoggerError) -> Self {
        Error(ErrorCode::LoggerError(value))
    }
}
impl From<NulError> for Error {
    fn from(value: NulError) -> Self {
        Error(ErrorCode::NulError(value))
    }
}

impl core::error::Error for Error {}
