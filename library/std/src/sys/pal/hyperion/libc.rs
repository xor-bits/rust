use core::ffi;

// some libc things that Rust expects

// ffi::c_size_t
#[allow(non_camel_case_types)]
type size_t = usize;

// void* memcpy( void *dest, const void *src, size_t count );
#[no_mangle]
extern "C" fn memcpy(dest: *mut ffi::c_void, src: *const ffi::c_void, count: size_t) {
    let mut dest = dest.cast::<u8>();
    let mut src = src.cast::<u8>();

    for _ in 0..count {
        unsafe {
            *dest = *src;
            dest = dest.add(1);
            src = src.add(1);
        }
    }
}

// int memcmp( const void* lhs, const void* rhs, size_t count );
#[no_mangle]
extern "C" fn memcmp(
    lhs: *const ffi::c_void,
    rhs: *const ffi::c_void,
    count: size_t,
) -> ffi::c_int {
    let lhs = lhs.cast::<u8>();
    let rhs = rhs.cast::<u8>();

    for i in 0..count {
        let l = unsafe { *lhs.add(i) };
        let r = unsafe { *rhs.add(i) };

        if l != r {
            return l as ffi::c_int - r as ffi::c_int;
        }
    }

    0
}

// void* memmove( void* dest, const void* src, size_t count );
#[no_mangle]
extern "C" fn memmove(
    dest: *mut ffi::c_void,
    src: *const ffi::c_void,
    count: size_t,
) -> *mut ffi::c_void {
    let dest = dest.cast::<u8>();
    let src = src.cast::<u8>();

    if dest.cast_const() < src {
        for i in 0..count {
            unsafe { *dest.add(i) = *src.add(i) };
        }
    } else {
        for i in (0..count).rev() {
            unsafe { *dest.add(i) = *src.add(i) };
        }
    }

    dest.cast()
}

// void* memset( void* dest, int ch, std::size_t count );
#[no_mangle]
extern "C" fn memset(dest: *mut ffi::c_void, ch: ffi::c_int, count: size_t) {
    let dest = dest.cast::<u8>();

    for i in 0..count {
        unsafe { *dest.add(i) = ch as u8 }
    }
}

#[no_mangle]
extern "C" fn __libc_start_main() {}
