use hyperion_abi::sys::futex_wait;

use crate::cell::Cell;
use crate::sync as public;
use crate::sync::{
    atomic::{AtomicUsize, Ordering},
    once::ExclusiveState,
};

//

pub struct Once {
    state: AtomicUsize,
}

pub struct OnceState {
    poisoned: bool,
    set_state_to: Cell<usize>,
}

const INCOMPLETE: usize = 0;
const POISONED: usize = 1;
const RUNNING: usize = 2;
const COMPLETE: usize = 3;

struct CompletionGuard<'a> {
    state: &'a AtomicUsize,
    set_state_on_drop_to: usize,
}

impl<'a> Drop for CompletionGuard<'a> {
    fn drop(&mut self) {
        self.state.store(self.set_state_on_drop_to, Ordering::Release);
    }
}

// Safety: threads are not supported on this platform.
unsafe impl Sync for Once {}

impl Once {
    #[inline]
    #[rustc_const_stable(feature = "const_once_new", since = "1.32.0")]
    pub const fn new() -> Once {
        Once { state: AtomicUsize::new(INCOMPLETE) }
    }

    #[inline]
    pub fn is_completed(&self) -> bool {
        self.state.load(Ordering::Acquire) == COMPLETE
    }

    #[inline]
    pub(crate) fn state(&mut self) -> ExclusiveState {
        match self.state.load(Ordering::Acquire) {
            INCOMPLETE => ExclusiveState::Incomplete,
            POISONED => ExclusiveState::Poisoned,
            // RUNNING => ExclusiveState::Incomplete,
            COMPLETE => ExclusiveState::Complete,
            _ => unreachable!("invalid Once state"),
        }
    }

    #[cold]
    #[track_caller]
    pub fn call(&self, ignore_poisoning: bool, f: &mut impl FnMut(&public::OnceState)) {
        loop {
            let state = self.state.load(Ordering::Acquire);
            match state {
                POISONED if !ignore_poisoning => {
                    // Panic to propagate the poison.
                    panic!("Once instance has previously been poisoned");
                }
                RUNNING => {
                    futex_wait(&self.state, RUNNING);
                    // panic!("one-time initialization may not be performed recursively");
                }
                COMPLETE => return,
                INCOMPLETE | POISONED => {
                    // acquire the 'run lock'
                    if self
                        .state
                        .compare_exchange(state, RUNNING, Ordering::Acquire, Ordering::Relaxed)
                        .is_err()
                    {
                        continue;
                    }
                    // `guard` will set the new state on drop.
                    let mut guard =
                        CompletionGuard { state: &self.state, set_state_on_drop_to: POISONED };
                    // Run the function, letting it know if we're poisoned or not.
                    let f_state = public::OnceState {
                        inner: OnceState {
                            poisoned: state == POISONED,
                            set_state_to: Cell::new(COMPLETE),
                        },
                    };
                    f(&f_state);
                    guard.set_state_on_drop_to = f_state.inner.set_state_to.get();
                }
                _ => unreachable!("state should always be one of the constants"),
            }
        }
    }
}

impl OnceState {
    #[inline]
    pub fn is_poisoned(&self) -> bool {
        self.poisoned
    }

    #[inline]
    pub fn poison(&self) {
        self.set_state_to.set(POISONED)
    }
}
