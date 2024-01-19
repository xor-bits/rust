use crate::boxed::Box;
use crate::ptr;

//

pub type Key = usize;

//

#[inline]
pub unsafe fn create(_dtor: Option<unsafe extern "C" fn(*mut u8)>) -> Key {
    // FIXME: this is unsafe with multiple threads

    let val = Box::new(ptr::null_mut::<*mut u8>());
    let key = Box::into_raw(val);

    key as usize

    // panic!("should not be used on this target");
}

#[inline]
pub unsafe fn set(_key: Key, _value: *mut u8) {
    let ptr: *mut *mut u8 = crate::ptr::from_exposed_addr_mut(_key);
    unsafe {
        *ptr = _value;
    }

    // panic!("should not be used on this target");
}

#[inline]
pub unsafe fn get(_key: Key) -> *mut u8 {
    let ptr: *mut *mut u8 = crate::ptr::from_exposed_addr_mut(_key);
    unsafe { *ptr }

    // panic!("should not be used on this target");
}

#[inline]
pub unsafe fn destroy(_key: Key) {

    // panic!("should not be used on this target");
}
