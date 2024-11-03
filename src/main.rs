#![no_main]
#![no_std]

extern crate alloc;
extern crate libc;
extern crate log;
extern crate mimalloc;
extern crate mlir_sys as mlir;
extern crate rustix;

mod error;
mod fs;
mod io;
mod logger;
mod parser;
mod path;
mod time;

use error::*;

pub use alloc::{
    format,
    string::{FromUtf8Error, String},
    vec::Vec,
};
use io::{print, println};
use log::debug;
use mimalloc::MiMalloc;
use path::{Path, PathBuf};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub type Result<T> = core::result::Result<T, error::Error>;

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    logger::init();
    debug!("Started");
    return 0;

    let path = PathBuf::from(String::from("test.mlir"));
    match io::read_bytes(Path::from(&path)) {
        Ok(b) => {
            if let Some(s) = String::from_utf8(b).ok() {
                println(format!("Read {} characters from {}", s.len(), path).as_str()).ok();
            }
        }
        Err(e) => log::debug!("{}", e),
    }

    0
}
