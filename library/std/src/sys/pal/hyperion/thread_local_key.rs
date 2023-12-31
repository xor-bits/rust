use crate::boxed::Box;
use crate::ptr;

//

pub type Key = usize;

//

#[inline]
pub unsafe fn create(_dtor: Option<unsafe extern "C" fn(*mut u8)>) -> Key {
    hyperion_syscall::write(hyperion_syscall::fs::FileDesc(1), b"TLS create\n");

    let val = Box::new(ptr::null_mut());
    let key = Box::into_raw(val);

    key as usize

    // panic!("should not be used on this target");
}

#[inline]
pub unsafe fn set(_key: Key, _value: *mut u8) {
    hyperion_syscall::write(hyperion_syscall::fs::FileDesc(1), b"TLS set\n");

    let ptr = _key as *mut *mut u8;
    unsafe {
        *ptr = _value;
    }
    // panic!("should not be used on this target");
}

#[inline]
pub unsafe fn get(_key: Key) -> *mut u8 {
    hyperion_syscall::write(hyperion_syscall::fs::FileDesc(1), b"TLS get\n");

    let ptr = _key as *mut *mut u8;
    unsafe { *ptr }
    // panic!("should not be used on this target");
}

#[inline]
pub unsafe fn destroy(_key: Key) {
    hyperion_syscall::write(hyperion_syscall::fs::FileDesc(1), b"TLS destroy\n");
    // panic!("should not be used on this target");
}
