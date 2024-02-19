#![stable(feature = "rust1", since = "1.0.0")]

//

use hyperion_abi::sys::err::Error;

use crate::{fs, io, sys_common::AsInner};

#[stable(feature = "rust1", since = "1.0.0")]
pub mod net;

//

#[stable(feature = "rust1", since = "1.0.0")]
pub trait AsRawFd {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn as_raw_fd(&self) -> usize;
}

#[stable(feature = "rust1", since = "1.0.0")]
impl AsRawFd for fs::File {
    #[inline]
    fn as_raw_fd(&self) -> usize {
        self.as_inner().as_inner().0
    }
}

//

pub(crate) fn to_sys_err(err: i32) -> Error {
    Error(usize::try_from(err).unwrap_or(usize::MAX))
}

pub(crate) fn map_sys_err(err: Error) -> io::Error {
    io::Error::from_raw_os_error(err.0 as i32)
}

pub(crate) fn sys_err_kind(err: Error) -> io::ErrorKind {
    match err {
        Error::ALREADY_EXISTS => io::ErrorKind::AlreadyExists,
        Error::BAD_FILE_DESCRIPTOR => io::ErrorKind::NotFound,
        Error::CLOSED => io::ErrorKind::UnexpectedEof,
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
