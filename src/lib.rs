#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

/// Error and result types used by the high-level wrapper API.
pub mod error;

/// Re-exports the raw `freefare-sys` API for lower-level interop.
pub mod ffi {
    pub use freefare_sys::*;
}

pub use crate::error::{Error, Result};

use libc::{c_char, c_void, free};
use std::ffi::CStr;

/// General freefare tag discovery and metadata helpers.
pub mod freefare;
/// Wrappers around the Mifare Application Directory (MAD) API.
pub mod mad;
/// Wrappers around Mifare Classic, Ultralight, and DESFire operations.
pub mod mifare;
/// Helpers for working with TLV-encoded data used by libfreefare.
pub mod tlv;

fn check_status(tag: ffi::FreefareTag, code: i32, fallback: &'static str) -> Result<()> {
    if code < 0 {
        Err(Error::from_tag(tag, code, fallback))
    } else {
        Ok(())
    }
}

fn copy_c_string(ptr: *const c_char) -> String {
    unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
}

fn copy_malloc_c_string(ptr: *mut c_char) -> Result<String> {
    if ptr.is_null() {
        return Err(Error::new("libfreefare returned a null C string"));
    }

    let result = copy_c_string(ptr);
    unsafe {
        free(ptr.cast::<c_void>());
    }
    Ok(result)
}

fn copy_malloc_array<T: Copy>(ptr: *mut T, len: usize) -> Result<Vec<T>> {
    if ptr.is_null() {
        return Err(Error::new("libfreefare returned a null allocation"));
    }

    let result = unsafe { std::slice::from_raw_parts(ptr, len).to_vec() };
    unsafe {
        free(ptr.cast::<c_void>());
    }
    Ok(result)
}
