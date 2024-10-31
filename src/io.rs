use crate::Result;
use crate::String;
use crate::Vec;

pub fn print(s: &str) -> Result<usize> {
    write_stdout(s.as_bytes())
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
