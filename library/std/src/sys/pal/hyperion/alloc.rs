use hyperion_abi::alloc::{PageAlloc, Pages, SlabAllocator};
use hyperion_abi::sys::{palloc, pfree};

use crate::alloc::{GlobalAlloc, Layout, System};
use crate::ptr::NonNull;

//

static SLAB: SlabAllocator<BaseAlloc> = SlabAllocator::new();

//

struct BaseAlloc;

unsafe impl PageAlloc for BaseAlloc {
    unsafe fn alloc(pages: usize) -> Pages {
        let alloc = palloc(pages).unwrap().unwrap();
        unsafe { Pages::new(alloc.as_ptr(), pages) }
    }

    unsafe fn dealloc(frames: Pages) {
        pfree(NonNull::new(frames.as_ptr()).unwrap(), frames.len()).unwrap();
    }
}

//

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { GlobalAlloc::alloc(&SLAB, layout) }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { GlobalAlloc::dealloc(&SLAB, ptr, layout) }
    }
}
