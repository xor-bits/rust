use hyperion_syscall::err::Error;

use crate::io;
use crate::mem;

#[derive(Copy, Clone)]
pub struct IoSlice<'a>(&'a [u8]);

impl<'a> IoSlice<'a> {
    #[inline]
    pub fn new(buf: &'a [u8]) -> IoSlice<'a> {
        IoSlice(buf)
    }

    #[inline]
    pub fn advance(&mut self, n: usize) {
        self.0 = &self.0[n..]
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.0
    }
}

pub struct IoSliceMut<'a>(&'a mut [u8]);

impl<'a> IoSliceMut<'a> {
    #[inline]
    pub fn new(buf: &'a mut [u8]) -> IoSliceMut<'a> {
        IoSliceMut(buf)
    }

    #[inline]
    pub fn advance(&mut self, n: usize) {
        let slice = mem::take(&mut self.0);
        let (_, remaining) = slice.split_at_mut(n);
        self.0 = remaining;
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.0
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.0
    }
}

pub fn is_terminal<T>(_: &T) -> bool {
    false
}

pub fn map_sys_err(err: Error) -> io::Error {
    io::Error::from_raw_os_error(err.0 as i32)
}

pub fn sys_err_kind(err: Error) -> io::ErrorKind {
    match err {
        Error::ALREADY_EXISTS => io::ErrorKind::AlreadyExists,
        Error::BAD_FILE_DESCRIPTOR => io::ErrorKind::NotFound,
        Error::CLOSED => io::ErrorKind::NotFound,
        Error::CONNECTION_REFUSED => io::ErrorKind::ConnectionRefused,
        Error::FILESYSTEM_ERROR => io::ErrorKind::Uncategorized,
        Error::INTERRUPTED => io::ErrorKind::Interrupted,
        Error::INVALID_ADDRESS => io::ErrorKind::InvalidInput,
        Error::INVALID_ALLOC => io::ErrorKind::InvalidInput,
        Error::INVALID_ARGUMENT => io::ErrorKind::InvalidInput,
        Error::INVALID_DOMAIN => io::ErrorKind::InvalidInput,
        Error::INVALID_FLAGS => io::ErrorKind::InvalidInput,
        Error::INVALID_TYPE => io::ErrorKind::InvalidInput,
        Error::INVALID_UTF8 => io::ErrorKind::InvalidData,
        Error::IS_A_PIPE => io::ErrorKind::InvalidInput,
        Error::NOT_A_DIRECTORY => io::ErrorKind::NotADirectory,
        Error::NOT_A_FILE => io::ErrorKind::IsADirectory,
        Error::NOT_A_SOCKET => io::ErrorKind::InvalidInput,
        Error::NOT_FOUND => io::ErrorKind::NotFound,
        Error::NO_SUCH_PROCESS => io::ErrorKind::NotFound,
        Error::OUT_OF_MEMORY => io::ErrorKind::OutOfMemory,
        Error::OUT_OF_VIRTUAL_MEMORY => io::ErrorKind::OutOfMemory,
        Error::PERMISSION_DENIED => io::ErrorKind::PermissionDenied,
        Error::UNEXPECTED_EOF => io::ErrorKind::UnexpectedEof,
        Error::UNKNOWN_PROTOCOL => io::ErrorKind::InvalidInput,
        Error::WRITE_ZERO => io::ErrorKind::WriteZero,
        _ => io::ErrorKind::Uncategorized,
    }
}
