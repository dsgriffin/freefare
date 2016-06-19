use ::nfc_sys;
use ::ffi;

use libc::size_t;

pub fn taste(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> u8 {
    unsafe { ffi::felica_taste(device, target) }
}

pub fn tag_new(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> ffi::FreefareTag {
    unsafe { ffi::felica_tag_new(device, target) }
}

pub fn tag_free(tag: ffi::FreefareTag) {
    unsafe { ffi::felica_tag_free(tag) }
}

pub fn read(tag: ffi::FreefareTag, service: u16, block: u8, data: *mut u8, length: size_t) -> isize {
    unsafe { ffi::felica_read(tag, service, block, data, length) }
}

pub fn read_ex(tag: ffi::FreefareTag, service: u16, block_count: u8, blocks: *mut u8, data: *mut u8, length: size_t) -> isize {
    unsafe { ffi::felica_read_ex(tag, service, block_count, blocks, data, length) }
}

pub fn write(tag: ffi::FreefareTag, service: u16, block: u8, data: *mut u8, length: size_t) -> isize {
    unsafe { ffi::felica_write(tag, service, block, data, length) }
}

pub fn write_ex(tag: ffi::FreefareTag, service: u16, block_count: u8, blocks: *mut u8, data: *mut u8, length: size_t) -> isize {
    unsafe { ffi::felica_write_ex(tag, service, block_count, blocks, data, length) }
}