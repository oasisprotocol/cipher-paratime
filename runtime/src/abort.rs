//! Abort shim for the Fortanix SGX target.
//!
//! Why this exists:
//!
//! Some native libraries assume the standard C function `abort()` is
//! available. In our case, the `wasm3` runtime (via `wasm3-sys`) calls
//! `abort()` from C code (`m3_core.c` -> `m3_Abort`).
//!
//! When linking for `x86_64-fortanix-unknown-sgx`, there is no libc
//! providing the `abort` symbol, so linking fails with:
//!
//!     rust-lld: error: undefined symbol: abort
//!
//! This shim exports a C-compatible symbol named `abort` and forwards
//! execution to Rust's internal abort routine (`__rust_abort`), which
//! terminates the enclave/process immediately and never returns.

#[no_mangle]
pub extern "C" fn abort() -> ! {
    extern "C" {
        fn __rust_abort() -> !;
    }

    unsafe { __rust_abort() }
}
