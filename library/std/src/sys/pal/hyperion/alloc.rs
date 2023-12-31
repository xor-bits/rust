use hyperion_syscall::{err::Result, palloc};

use crate::alloc::{GlobalAlloc, Layout, System};
use crate::ptr::{null_mut, NonNull};

//

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match palloc(layout.size().div_ceil(0x1000)) {
            Result::Ok(Some(ptr)) => ptr.as_ptr(),
            _ => null_mut(),
        }
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // it is always zeroed
        unsafe { self.alloc(layout) }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if let Some(ptr) = NonNull::new(ptr) {
            _ = hyperion_syscall::pfree(ptr, layout.size().div_ceil(0x1000));
        }
    }
}
