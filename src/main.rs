#![no_main]
#![no_std]

extern crate alloc;
extern crate libc;
extern crate log;
extern crate mimalloc;
extern crate mlir_sys as mlir;
extern crate rustix;

mod error;
mod io;
mod logger;

use error::*;

pub use alloc::{
    format,
    string::{FromUtf8Error, String},
    vec::Vec,
};
use log::debug;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub type Result<T> = core::result::Result<T, Error>;

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    logger::init();

    io::print("Why hello there! What's your name?\n> ").expect("Uh-oh!");
    let name = io::read_stdin().expect("Could not read stdin");
    debug!("Testing!");

    io::print(&name).expect("Unable to write back input buffer");
    log::error!("AAAAAh");

    0
}
