//! `yanix` stands for Yet Another Nix crate, and, well, it is simply
//! a yet another crate in the spirit of the [nix] crate. As such,
//! this crate is inspired by the original `nix` crate, however,
//! it takes a different approach, using lower-level interfaces with
//! less abstraction, so that it fits better with its main use case
//! which is our WASI implementation, [wasi-common].
//!
//! [nix]: https://github.com/nix-rust/nix
//! [wasi-common]: https://github.com/bytecodealliance/wasmtime/tree/main/crates/wasi-common
#![cfg(any(unix, target_os = "wasi"))]

#[cfg(not(target_os = "wasi"))] // not implemented for WASI in yanix yet
pub mod clock;
pub mod dir;
pub mod fcntl;
pub mod file;
pub mod filetime;
#[cfg(not(target_os = "wasi"))] // not implemented for WASI in yanix yet
pub mod poll;
#[cfg(not(target_os = "wasi"))] // not supported in WASI yet
pub mod socket;

mod sys;

pub mod fadvise {
    pub use super::sys::fadvise::*;
}

use std::ffi::CString;
use std::io::{Error, Result};
use std::path::Path;

fn from_success_code<T: IsZero>(t: T) -> Result<()> {
    if t.is_zero() {
        Ok(())
    } else {
        Err(Error::last_os_error())
    }
}

fn from_result<T: IsMinusOne>(t: T) -> Result<T> {
    if t.is_minus_one() {
        Err(Error::last_os_error())
    } else {
        Ok(t)
    }
}

trait IsZero {
    fn is_zero(&self) -> bool;
}

macro_rules! impl_is_zero {
    ($($t:ident)*) => ($(impl IsZero for $t {
        fn is_zero(&self) -> bool {
            *self == 0
        }
    })*)
}

impl_is_zero! { i32 i64 isize }

trait IsMinusOne {
    fn is_minus_one(&self) -> bool;
}

macro_rules! impl_is_minus_one {
    ($($t:ident)*) => ($(impl IsMinusOne for $t {
        fn is_minus_one(&self) -> bool {
            *self == -1
        }
    })*)
}

impl_is_minus_one! { i32 i64 isize }

/// Convert an `AsRef<Path>` into a `CString`.
fn cstr<P: AsRef<Path>>(path: P) -> Result<CString> {
    #[cfg(target_os = "hermit")]
    use std::os::hermit::ext::ffi::OsStrExt;
    #[cfg(unix)]
    use std::os::unix::ffi::OsStrExt;
    #[cfg(target_os = "vxworks")]
    use std::os::vxworks::ext::ffi::OsStrExt;
    #[cfg(target_os = "wasi")]
    use std::os::wasi::ffi::OsStrExt;

    Ok(CString::new(path.as_ref().as_os_str().as_bytes())?)
}
