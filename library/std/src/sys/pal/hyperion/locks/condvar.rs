use hyperion_syscall::{futex_wait, futex_wake};

use crate::sys::locks::Mutex;
use crate::time::Duration;

use crate::sync::atomic::{AtomicUsize, Ordering};

//

pub struct Condvar {
    futex: AtomicUsize,
}

impl Condvar {
    #[inline]
    #[rustc_const_stable(feature = "const_locks", since = "1.63.0")]
    pub const fn new() -> Condvar {
        Condvar { futex: AtomicUsize::new(0) }
    }

    #[inline]
    pub fn notify_one(&self) {
        self.futex.fetch_add(1, Ordering::Relaxed);
        futex_wake(&self.futex, 1)
    }

    #[inline]
    pub fn notify_all(&self) {
        self.futex.fetch_add(1, Ordering::Relaxed);
        futex_wake(&self.futex, usize::MAX)
    }

    pub unsafe fn wait(&self, mutex: &Mutex) {
        unsafe { self.wait_optional_timeout(mutex, None) };
    }

    pub unsafe fn wait_timeout(&self, mutex: &Mutex, dur: Duration) -> bool {
        unsafe { self.wait_optional_timeout(mutex, Some(dur)) }
    }

    unsafe fn wait_optional_timeout(&self, mutex: &Mutex, timeout: Option<Duration>) -> bool {
        // Examine the notification counter _before_ we unlock the mutex.
        let futex_value = self.futex.load(Ordering::Relaxed);

        // Unlock the mutex before going to sleep.
        unsafe { mutex.unlock() };

        // Wait, but only if there hasn't been any
        // notification since we unlocked the mutex.
        // let r = ..
        futex_wait(&self.futex, futex_value); // FIXME: timeout

        _ = timeout;
        let r = false;

        // Lock the mutex again.
        mutex.lock();

        r
    }
}
