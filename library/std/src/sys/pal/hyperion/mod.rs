#![deny(unsafe_op_in_unsafe_fn)]

use core::borrow::BorrowMut;

use crate::ptr;

//

pub mod alloc;
pub mod args;
#[path = "../unix/cmath.rs"]
pub mod cmath;
pub mod env;
pub mod fs;
pub mod io;
pub mod locks;
pub mod net;
pub mod once;
pub mod os;
#[path = "../unix/os_str.rs"]
pub mod os_str;
#[path = "../unix/path.rs"]
pub mod path;
pub mod pipe;
pub mod process;
pub mod stdio;
pub mod thread;
#[cfg(target_thread_local)]
pub mod thread_local_dtor;
#[path = "../unsupported/thread_local_key.rs"]
pub mod thread_local_key;
pub mod thread_parking;
pub mod time;

mod common;
pub use common::*;

//

#[no_mangle]
extern "C" fn _start(hyperion_cli_args_ptr: usize, _a2: usize) -> ! {
    // rustc generates the real `main` function, that fn
    // simply calls `lang_start` with the correct args
    extern "C" {
        fn main(argc: isize, argv: *const *const u8) -> i32;
    }

    // init cli args from stack, move them to the heap
    hyperion_syscall::write(hyperion_syscall::fs::FileDesc(1), b"testing1\n");
    let raw = stdio::Stdout::new();
    hyperion_syscall::write(hyperion_syscall::fs::FileDesc(1), b"testing2\n");
    let lines = crate::io::LineWriter::new(raw);
    hyperion_syscall::write(hyperion_syscall::fs::FileDesc(1), b"testing3\n");
    let rc = core::cell::RefCell::new(lines);
    hyperion_syscall::write(hyperion_syscall::fs::FileDesc(1), b"testing4\n");
    let mutex = crate::sync::ReentrantMutex::new(rc);
    hyperion_syscall::write(hyperion_syscall::fs::FileDesc(1), b"testing5\n");
    // thread_local! {
    //     static X: u8 = 0;
    // }
    let x = mutex.lock();
    hyperion_syscall::write(hyperion_syscall::fs::FileDesc(1), b"testing6\n");
    // X.with(|v| {
    //     hyperion_syscall::write(hyperion_syscall::fs::FileDesc(1), b"testing7\n");
    // });
    drop(x);
    hyperion_syscall::write(hyperion_syscall::fs::FileDesc(1), b"testing\n");
    println!("testing");

    // unsafe { env::init_args(hyperion_cli_args_ptr) };

    // call `lang_start`
    let exit_code = unsafe { main(0, ptr::null()) };

    hyperion_syscall::exit(exit_code as i64);

    // ExitCode::from_raw(exit_code as _).exit_process();
}
