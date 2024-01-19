use hyperion_syscall::{done, nanosleep, rename, spawn, yield_now};

use super::unsupported;
use crate::ffi::CStr;
use crate::io;
use crate::num::NonZeroUsize;
use crate::ptr;
use crate::time::Duration;

pub struct Thread();

pub const DEFAULT_MIN_STACK_SIZE: usize = 4096;

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(_stack: usize, p: Box<dyn FnOnce()>) -> io::Result<Thread> {
        let p = Box::into_raw(Box::new(p));
        spawn(_thread_entry, p.expose_addr());

        extern "C" fn _thread_entry(_stack_ptr: usize, arg: usize) -> ! {
            let main = unsafe {
                Box::from_raw(ptr::from_exposed_addr::<Box<dyn FnOnce()>>(arg).cast_mut())
            };
            main();
            done(0);
        }

        Ok(Thread())
    }

    pub fn yield_now() {
        yield_now();
    }

    pub fn set_name(name: &CStr) {
        rename(name.to_str().expect("name should be UTF-8")).unwrap();
    }

    pub fn sleep(dur: Duration) {
        nanosleep(dur.as_nanos().try_into().unwrap());
    }

    pub fn join(self) {
        unsupported().unwrap()
    }
}

pub fn available_parallelism() -> io::Result<NonZeroUsize> {
    unsupported()
}

pub mod guard {
    pub type Guard = !;
    pub unsafe fn current() -> Option<Guard> {
        None
    }
    pub unsafe fn init() -> Option<Guard> {
        None
    }
}
