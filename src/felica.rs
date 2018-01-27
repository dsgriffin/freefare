use ::nfc_sys;
use ::freefare_sys;

use libc::size_t;

pub fn taste(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> u8 {
    unsafe { freefare_sys::felica_taste(device, target) }
}

pub fn tag_new(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> freefare_sys::FreefareTag {
    unsafe { freefare_sys::felica_tag_new(device, target) }
}

pub fn tag_free(tag: freefare_sys::FreefareTag) {
    unsafe { freefare_sys::felica_tag_free(tag) }
}

pub fn read(tag: freefare_sys::FreefareTag, service: u16, block: u8, data: *mut u8, length: size_t) -> isize {
    unsafe { freefare_sys::felica_read(tag, service, block, data, length) }
}

pub fn read_ex(tag: freefare_sys::FreefareTag, service: u16, block_count: u8, blocks: *mut u8, data: *mut u8, length: size_t) -> isize {
    unsafe { freefare_sys::felica_read_ex(tag, service, block_count, blocks, data, length) }
}

pub fn write(tag: freefare_sys::FreefareTag, service: u16, block: u8, data: *mut u8, length: size_t) -> isize {
    unsafe { freefare_sys::felica_write(tag, service, block, data, length) }
}

pub fn write_ex(tag: freefare_sys::FreefareTag, service: u16, block_count: u8, blocks: *mut u8, data: *mut u8, length: size_t) -> isize {
    unsafe { freefare_sys::felica_write_ex(tag, service, block_count, blocks, data, length) }
}