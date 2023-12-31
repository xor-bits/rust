use hyperion_syscall::{err as sys, fs::FileDesc, read, write};

use crate::io;

//

pub struct Stdin;
pub struct Stdout;
pub struct Stderr;

//

impl Stdin {
    pub const fn new() -> Stdin {
        Stdin
    }
}

impl io::Read for Stdin {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        read(FileDesc(0), buf).map_err(super::io::map_sys_err)
    }
}

impl Stdout {
    pub const fn new() -> Stdout {
        Stdout
    }
}

impl io::Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        write(FileDesc(1), buf).map_err(super::io::map_sys_err)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Stderr {
    pub const fn new() -> Stderr {
        Stderr
    }
}

impl io::Write for Stderr {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        write(FileDesc(1), buf).map_err(super::io::map_sys_err)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub const STDIN_BUF_SIZE: usize = 0;

//

pub fn is_ebadf(_err: &io::Error) -> bool {
    _err.raw_os_error()
        .and_then(|i: i32| Some(sys::Error(i as _) == sys::Error::BAD_FILE_DESCRIPTOR))
        .unwrap_or(false)
}

pub fn panic_output() -> Option<impl io::Write> {
    Some(Stderr::new())
}
