use hyperion_abi::sys::err::Error;

use crate::{io as std_io, os::hyperion::sys_err_kind};

//

pub mod memchr {
    pub use core::slice::memchr::{memchr, memrchr};
}

// SAFETY: must be called only once during runtime initialization.
// NOTE: this is not guaranteed to run, for example when Rust code is called externally.
pub unsafe fn init(_argc: isize, _argv: *const *const u8, _sigpipe: u8) {}

// SAFETY: must be called only once during runtime cleanup.
// NOTE: this is not guaranteed to run, for example when the program aborts.
pub unsafe fn cleanup() {}

pub fn unsupported<T>() -> std_io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> std_io::Error {
    std_io::const_io_error!(
        std_io::ErrorKind::Unsupported,
        "operation not supported on this platform",
    )
}

pub fn is_interrupted(_code: i32) -> bool {
    false
}

pub fn decode_error_kind(_code: i32) -> crate::io::ErrorKind {
    sys_err_kind(Error(_code as _))
}

pub fn abort_internal() -> ! {
    hyperion_abi::sys::exit(-1);
}

pub fn hashmap_random_keys() -> (u64, u64) {
    (1, 2)
}
