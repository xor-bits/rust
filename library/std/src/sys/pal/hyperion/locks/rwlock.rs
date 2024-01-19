use hyperion_syscall::{futex_wait, futex_wake};

use crate::sync::atomic::{AtomicUsize, Ordering};

//

pub struct RwLock {
    futex: AtomicUsize,
}

unsafe impl Send for RwLock {}
unsafe impl Sync for RwLock {} // no threads on this platform

impl RwLock {
    #[inline]
    #[rustc_const_stable(feature = "const_locks", since = "1.63.0")]
    pub const fn new() -> RwLock {
        RwLock { futex: AtomicUsize::new(0) }
    }

    #[inline]
    pub fn read(&self) {
        loop {
            let current = self.futex.load(Ordering::Acquire);

            if current == WRITE_LOCKED || current == WRITE_LOCKED - 1 {
                // write locked or too many readers
                futex_wait(&self.futex, current);
                continue;
            }

            if self
                .futex
                .compare_exchange(current, current + 1, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                return;
            }
        }
    }

    #[inline]
    pub fn try_read(&self) -> bool {
        let current = self.futex.load(Ordering::Acquire);

        if current == WRITE_LOCKED || current == WRITE_LOCKED - 1 {
            return false;
        }

        self.futex
            .compare_exchange(current, current + 1, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    #[inline]
    pub fn write(&self) {
        loop {
            let current = self.futex.load(Ordering::Acquire);

            if current != UNLOCKED {
                futex_wait(&self.futex, current);
                continue;
            }

            if self
                .futex
                .compare_exchange(UNLOCKED, WRITE_LOCKED, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                return;
            }
        }
    }

    #[inline]
    pub fn try_write(&self) -> bool {
        let current = self.futex.load(Ordering::Acquire);

        if current != UNLOCKED {
            return false;
        }

        self.futex
            .compare_exchange(UNLOCKED, WRITE_LOCKED, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    #[inline]
    pub unsafe fn read_unlock(&self) {
        let old = self.futex.fetch_sub(1, Ordering::Release);
        assert_ne!(old, WRITE_LOCKED);
        if old == 1 {
            // wake up a writer if the reader lock was completely unlocked
            futex_wake(&self.futex, 1);
        }
    }

    #[inline]
    pub unsafe fn write_unlock(&self) {
        let old = self.futex.swap(UNLOCKED, Ordering::Release);
        assert_eq!(old, WRITE_LOCKED);
    }
}

//

const UNLOCKED: usize = 0;
// const READ_LOCKED: usize = 1;
const WRITE_LOCKED: usize = usize::MAX;
