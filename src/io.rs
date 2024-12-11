use rustix::fs::Mode;
use rustix::fs::OFlags;
use rustix::io::Errno;

use crate::error::ErrorCode;
use crate::path::Path;
use crate::Result;
use crate::String;
use crate::Vec;
use log::trace;

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

pub fn read_bytes<'a>(path: Path<'a>) -> Result<Vec<u8>> {
    if !path.exists() {
        return Err(crate::error::Error(ErrorCode::Rustix(Errno::NODEV)));
    }

    let flags = OFlags::RDONLY | OFlags::CLOEXEC | OFlags::NOATIME;
    let fd = if path.is_absolute() {
        rustix::fs::open(path, flags, Mode::empty())?
    } else {
        rustix::fs::openat(rustix::fs::CWD, path, flags, Mode::empty())?
    };

    let mut buf: [u8; READ_BUFFER_SIZE] = [0u8; READ_BUFFER_SIZE];
    let mut data = Vec::new();

    loop {
        trace!("Reading fd {:?}", fd);
        let count = rustix::io::read(&fd, &mut buf)?;
        if count == 0 {
            break;
        }
        trace!("Read {} bytes", count);

        data.extend_from_slice(&buf[..count]);
    }
    trace!("Finished reading");

    Ok(data)
}

pub fn read_string<'a>(path: Path<'a>) -> Result<String> {
    let bytes = read_bytes(path)?;
    String::from_utf8(bytes).map_err(|e| crate::error::Error::from(e))
}
