use hyperion_abi::sys::fs::FileDesc;

use crate::io::{self, BorrowedCursor, IoSlice, IoSliceMut};

use super::fs::File;

//

pub struct AnonPipe(File);

//

impl AnonPipe {
    pub fn from_fd(fd: FileDesc) -> Self {
        Self(File::from_inner(fd))
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }

    pub fn read_buf(&self, buf: BorrowedCursor<'_>) -> io::Result<()> {
        self.0.read_buf(buf)
    }

    pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        self.0.read_vectored(bufs)
    }

    pub fn is_read_vectored(&self) -> bool {
        self.0.is_read_vectored()
    }

    pub fn read_to_end(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        let mut bbuf = [0u8; 256];
        let mut nn = 0usize;
        loop {
            let n: usize = self.read(&mut bbuf)?;
            nn += n;
            if n == 0 {
                break;
            }

            buf.extend(&bbuf[..n]);
        }
        Ok(nn)
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        self.0.write_vectored(bufs)
    }

    pub fn is_write_vectored(&self) -> bool {
        self.0.is_write_vectored()
    }

    pub fn diverge(&self) -> ! {
        todo!()
    }
}

pub fn read2(p1: AnonPipe, v1: &mut Vec<u8>, p2: AnonPipe, v2: &mut Vec<u8>) -> io::Result<()> {
    // ??
    p1.read_to_end(v1)?;
    p2.read_to_end(v2)?;
    Ok(())
}
