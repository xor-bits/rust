#![deny(unsafe_op_in_unsafe_fn)]

use crate::{process::ExitCode, ptr};
use process::ExitCodeExt;

//

pub mod alloc;
pub mod args;
pub mod env;
pub mod fs;
pub mod io;
pub mod locks;
pub mod net;
pub mod once;
pub mod os;
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
extern "sysv64" fn _start(hyperion_cli_args_ptr: usize, _a2: usize) -> ! {
    // rustc generates the real `main` function, that fn
    // simply calls `lang_start` with the correct args
    extern "C" {
        fn main(argc: isize, argv: *const *const u8) -> i32;
    }

    // init cli args from stack, move them to the heap
    unsafe { args::init_args(hyperion_cli_args_ptr) };

    // call `lang_start`
    let exit_code = unsafe { main(0, ptr::null()) };

    ExitCode::from_raw(exit_code as _).exit_process();
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn __rust_abort() {
    abort_internal();
}
