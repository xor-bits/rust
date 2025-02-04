use hyperion_abi::sys::{err as sys, fs::FileDesc, read, read_uninit, write};

use crate::{
    io::{self, BorrowedCursor},
    os::hyperion::{map_sys_err, to_sys_err},
};

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
        read(FileDesc(0), buf).map_err(map_sys_err)
    }

    fn read_buf(&mut self, mut buf: BorrowedCursor<'_>) -> io::Result<()> {
        let ret: usize = read_uninit(FileDesc(0), buf.uninit_mut()).map_err(map_sys_err)?;

        // Safety: `ret` bytes were written to the initialized portion of the buffer
        unsafe { buf.advance(ret) };
        Ok(())
    }
}

impl Stdout {
    pub const fn new() -> Stdout {
        Stdout
    }
}

impl io::Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        write(FileDesc(1), buf).map_err(map_sys_err)
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
        write(FileDesc(2), buf).map_err(map_sys_err)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub const STDIN_BUF_SIZE: usize = 0;

//

pub fn is_ebadf(_err: &io::Error) -> bool {
    _err.raw_os_error().map_or(false, |err| to_sys_err(err) == sys::Error::BAD_FILE_DESCRIPTOR)
}

pub fn panic_output() -> Option<impl io::Write> {
    // struct Log;

    // impl io::Write for Log {
    //     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    //         Ok(hyperion_abi::sys::write(hyperion_abi::sys::fs::FileDesc(1), buf).unwrap())
    //     }

    //     fn flush(&mut self) -> io::Result<()> {
    //         Ok(())
    //     }
    // }

    // Some(Log)
    Some(Stderr::new())
}
