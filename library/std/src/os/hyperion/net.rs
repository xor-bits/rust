use crate::{
    io::{Read, Write},
    mem::forget,
    sync::Arc,
};

use hyperion_abi::sys::{
    accept, bind, close, connect,
    net::{Protocol, SocketDomain, SocketType},
    recv, send, socket,
};

use crate::io;

use super::map_sys_err;

//

#[stable(feature = "rust1", since = "1.0.0")]
pub use hyperion_abi::sys::fs::FileDesc;

//

#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Debug)]
pub struct LocalListener {
    fd: OwnedFd,
}

#[stable(feature = "rust1", since = "1.0.0")]
impl LocalListener {
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn bind(addr: &str) -> io::Result<Self> {
        let fd = socket(SocketDomain::LOCAL, SocketType::STREAM, Protocol::LOCAL)
            .map_err(map_sys_err)?;
        bind(fd, addr).map_err(map_sys_err)?;

        Ok(Self { fd: OwnedFd(fd) })
    }

    /// the file descriptor won't be closed automatically
    #[stable(feature = "rust1", since = "1.0.0")]
    #[must_use]
    pub fn leak_fd(self) -> FileDesc {
        self.fd.leak_fd()
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn accept(&self) -> io::Result<LocalStream> {
        let fd = accept(self.fd.0).map_err(map_sys_err)?;
        Ok(LocalStream { fd: OwnedFd(fd) })
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn close(self) -> io::Result<()> {
        close(self.leak_fd()).map_err(map_sys_err)?;
        Ok(())
    }
}

//

#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Debug)]
pub struct LocalStream {
    fd: OwnedFd,
}

#[stable(feature = "rust1", since = "1.0.0")]
impl LocalStream {
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn connect(addr: &str) -> io::Result<Self> {
        let fd = socket(SocketDomain::LOCAL, SocketType::STREAM, Protocol::LOCAL)
            .map_err(map_sys_err)?;
        connect(fd, addr).map_err(map_sys_err)?;

        Ok(Self { fd: OwnedFd(fd) })
    }

    /// the file descriptor won't be closed automatically
    #[stable(feature = "rust1", since = "1.0.0")]
    #[must_use]
    pub fn leak_fd(self) -> FileDesc {
        self.fd.leak_fd()
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn close(self) -> io::Result<()> {
        close(self.leak_fd()).map_err(map_sys_err)?;
        Ok(())
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Read for Arc<LocalStream> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (&self.fd).read(buf)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Write for Arc<LocalStream> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (&self.fd).write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        (&self.fd).flush()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Read for &LocalStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (&self.fd).read(buf)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Write for &LocalStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (&self.fd).write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        (&self.fd).flush()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Read for LocalStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.fd.read(buf)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Write for LocalStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.fd.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.fd.flush()
    }
}

//

#[derive(Debug)]
struct OwnedFd(FileDesc);

impl OwnedFd {
    fn leak_fd(self) -> FileDesc {
        let fd = self.0;
        forget(self);
        fd
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Read for &OwnedFd {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        recv(self.0, buf, 0).map_err(map_sys_err)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Write for &OwnedFd {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        send(self.0, buf, 0).map_err(map_sys_err)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Read for OwnedFd {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (&*self).read(buf)
    }
}

impl Write for OwnedFd {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (&*self).write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        (&*self).flush()
    }
}

impl Drop for OwnedFd {
    fn drop(&mut self) {
        close(self.0).unwrap()
    }
}
