#![no_main]
#![no_std]

extern crate libc;
extern crate mimalloc;
extern crate mlir_sys as mlir;
extern crate rustix;

use mimalloc::MiMalloc;
use rustix::io;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn write(s: &str) -> io::Result<usize> {
    write_slice(s.as_bytes())
}

fn write_slice(buf: &[u8]) -> io::Result<usize> {
    let stdout = unsafe { rustix::stdio::stdout() };
    io::write(stdout, buf)
}

fn read(buf: &mut [u8]) -> io::Result<usize> {
    let stdin = unsafe { rustix::stdio::stdin() };
    io::read(stdin, buf)
}

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    write("Why hello there! What's your name?\n> ").expect("Uh-oh!");
    let mut buf: [u8; 1024] = [0u8; 1024];
    read(&mut buf).expect("Could not read stdin");

    write_slice(&buf).expect("Unable to write back input buffer");
    0
}
