use crate::ffi::OsString;
use crate::fmt;
use crate::mem;
use crate::ptr;
use crate::slice;
use crate::str;
use crate::sync::atomic::{fence, Ordering};

//

pub fn args() -> Args {
    Args { top: unsafe { ARGS }.iter() }
}

//

#[derive(Clone)]
pub struct Args {
    top: slice::Iter<'static, &'static str>,
}

//

impl fmt::Debug for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl Iterator for Args {
    type Item = OsString;

    fn next(&mut self) -> Option<OsString> {
        self.top.next().copied().map(|s| OsString::from(s))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.top.size_hint()
    }
}

impl ExactSizeIterator for Args {
    fn len(&self) -> usize {
        self.top.len()
    }
}

impl DoubleEndedIterator for Args {
    fn next_back(&mut self) -> Option<OsString> {
        self.top.next_back().copied().map(|s| OsString::from(s))
    }
}

//

pub(crate) unsafe fn init_args(hyperion_cli_args_ptr: usize) {
    let stack_args = CliArgs { hyperion_cli_args_ptr };

    let args = stack_args.iter().map(|arg| &*String::from(arg).leak()).collect::<Vec<_>>().leak();

    unsafe { ARGS = args };
    fence(Ordering::SeqCst);
}

//

static mut ARGS: &[&str] = &[];

#[derive(Clone, Copy)]
struct CliArgs {
    hyperion_cli_args_ptr: usize,
}

impl CliArgs {
    fn iter(self) -> impl DoubleEndedIterator<Item = &'static str> + Clone {
        let mut ptr = self.hyperion_cli_args_ptr;

        let argc: usize = Self::pop(&mut ptr);
        let mut arg_lengths = ptr;
        let mut arg_strings = ptr + argc * mem::size_of::<usize>();

        (0..argc).map(move |_| {
            let len: usize = Self::pop(&mut arg_lengths);
            let str: &[u8] = unsafe {
                slice::from_raw_parts(ptr::from_exposed_addr(arg_strings as _), len as _)
            };
            arg_strings += len;

            unsafe { str::from_utf8_unchecked(str) }
        })
    }

    fn pop<T: Sized>(top: &mut usize) -> T {
        let v = unsafe { (ptr::from_exposed_addr::<T>(*top)).read() };
        *top += mem::size_of::<T>();
        v
    }
}
