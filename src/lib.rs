#![crate_name = "freefare"]
#![crate_type = "dylib"]

extern crate libc;
extern crate nfc_sys;

pub mod ffi;

pub mod felica;
pub mod freefare;
pub mod mad;
pub mod mifare;
pub mod tlv;
