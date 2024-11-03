use core::mem::MaybeUninit;

use rustix::fs::Mode;
use rustix::fs::OFlags;

use crate::path::Path;
use crate::Result;
use crate::String;
use crate::Vec;

#[cfg(windows)]
const LINE_ENDING: &'static [u8] = "\r\n".as_bytes();
#[cfg(not(windows))]
const LINE_ENDING: &'static [u8] = "\n".as_bytes();

const READ_BUFFER_SIZE: usize = 4096;

pub fn print(s: &str) -> Result<usize> {
    write_stdout(s.as_bytes())
}
pub fn println(s: &str) -> Result<usize> {
    let mut buf = Vec::from(s);
    buf.extend(LINE_ENDING);
    write_stdout(&buf)
}

pub fn write_stderr(buf: &[u8]) -> Result<usize> {
    let stderr = unsafe { rustix::stdio::stderr() };
    let count = rustix::io::write(stderr, buf)?;
    Ok(count)
}

pub fn write_stdout(buf: &[u8]) -> Result<usize> {
    let stdout = unsafe { rustix::stdio::stdout() };
    let count = rustix::io::write(stdout, buf)?;
    Ok(count)
}

pub fn read_stdin() -> Result<String> {
    let stdin = unsafe { rustix::stdio::stdin() };
    let mut buf = Vec::new();
    buf.resize(64, 0u8);

    rustix::io::read(stdin, &mut buf)?;
    let s = String::from_utf8(buf)?;

    Ok(s)
}

pub fn read_bytes(path: Path) -> Result<Vec<u8>> {
    let flags = OFlags::RDONLY;
    let fd = rustix::fs::open(path, flags, Mode::empty())?;
    let mut buf: [u8; READ_BUFFER_SIZE] = [0u8; READ_BUFFER_SIZE];
    let mut data = Vec::new();

    loop {
        let count = rustix::io::read(&fd, &mut buf)?;
        if count == 0 {
            break;
        }

        data.extend_from_slice(&buf[..count]);
    }

    Ok(data)
}
