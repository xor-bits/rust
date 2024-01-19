use hyperion_syscall::{futex_wait, futex_wake};

use crate::sync::atomic::{AtomicUsize, Ordering};

//

pub struct Mutex {
    futex: AtomicUsize,
}

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}

impl Mutex {
    #[inline]
    #[rustc_const_stable(feature = "const_locks", since = "1.63.0")]
    pub const fn new() -> Mutex {
        Mutex { futex: AtomicUsize::new(UNLOCKED) }
    }

    #[inline]
    pub fn lock(&self) {
        while self
            .futex
            .compare_exchange(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            futex_wait(&self.futex, LOCKED);
        }
    }

    #[inline]
    pub unsafe fn unlock(&self) {
        // unlock the mutex
        self.futex.store(UNLOCKED, Ordering::Release);

        // and THEN wake up waiting threads
        futex_wake(&self.futex, 1);
    }

    #[inline]
    pub fn try_lock(&self) -> bool {
        self.futex
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }
}

//

const UNLOCKED: usize = 0;
const LOCKED: usize = 1;
