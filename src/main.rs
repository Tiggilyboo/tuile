#![no_main]
#![no_std]

extern crate alloc;
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

use core::ffi::{c_char, CStr};

use alloc::{boxed::Box, slice};
pub use alloc::{
    format,
    string::{FromUtf8Error, String},
    vec::Vec,
};
use io::{print, println};
use log::{debug, error, info, trace, warn};
use mimalloc::MiMalloc;
use path::{Path, PathBuf};
use rustix::{cstr, io::Errno};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub type Result<T> = core::result::Result<T, error::Error>;

#[no_mangle]
pub extern "C" fn main(argc: isize, argv: *const *const c_char) -> isize {
    if let Err(e) = logger::init() {
        print(&format!("{}", e)).ok();
        return -1;
    }

    let argv: Vec<_> = (0..argc)
        .map(|i| unsafe { CStr::from_ptr(*argv.add(i as usize)) })
        .collect();

    if let Some(path_arg) = argv.get(1) {
        if let Ok(path_str) = path_arg.to_str() {
            debug!("reading from {}", path_str);

            let path = Path::from(path_str);
            match io::read_bytes(path) {
                Ok(b) => {
                    if let Some(s) = String::from_utf8(b).ok() {
                        println(format!("Read {} characters from {}", s.len(), path).as_str()).ok();
                    }
                }
                Err(e) => error!("{}", e),
            }
        }
    }

    trace!("0 OK");
    0
}
