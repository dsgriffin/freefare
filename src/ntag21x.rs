use nfc_sys;
use freefare_sys;

pub fn taste(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> bool {
    unsafe { freefare_sys::ntag21x_taste(device, target) }
}

pub fn tag_new(
    device: *mut nfc_sys::nfc_device,
    target: nfc_sys::nfc_target,
) -> freefare_sys::FreefareTag {
    unsafe { freefare_sys::ntag21x_tag_new(device, target) }
}

pub fn tag_reuse(tag: freefare_sys::FreefareTag) -> freefare_sys::FreefareTag {
    unsafe { freefare_sys::ntag21x_tag_reuse(tag) }
}

pub fn key_new(data: *const u8, pack: *const u8) -> freefare_sys::NTAG21xKey {
    unsafe { freefare_sys::ntag21x_key_new(data, pack) }
}

pub fn key_free(key: freefare_sys::NTAG21xKey) {
    unsafe { freefare_sys::ntag21x_key_free(key) }
}

pub fn tag_free(tag: freefare_sys::FreefareTag) {
    unsafe { freefare_sys::ntag21x_tag_free(tag) }
}

pub fn connect(tag: freefare_sys::FreefareTag) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::ntag21x_connect(tag) }
}

pub fn disconnect(tag: freefare_sys::FreefareTag) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::ntag21x_disconnect(tag) }
}

pub fn get_info(tag: freefare_sys::FreefareTag) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::ntag21x_get_info(tag) }
}

pub fn get_subtype(tag: freefare_sys::FreefareTag) -> freefare_sys::ntag_tag_subtype {
    unsafe { freefare_sys::ntag21x_get_subtype(tag) }
}

pub fn get_last_page(tag: freefare_sys::FreefareTag) -> u8 {
    unsafe { freefare_sys::ntag21x_get_last_page(tag) }
}

pub fn read_signature(tag: freefare_sys::FreefareTag, data: *mut u8) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::ntag21x_read_signature(tag, data) }
}

pub fn set_pwd(tag: freefare_sys::FreefareTag, data: *mut u8) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::ntag21x_set_pwd(tag, data) }
}

pub fn set_pack(tag: freefare_sys::FreefareTag, data: *mut u8) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::ntag21x_set_pack(tag, data) }
}

pub fn set_key(
    tag: freefare_sys::FreefareTag,
    key: freefare_sys::NTAG21xKey,
) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::ntag21x_set_key(tag, key) }
}

pub fn set_auth(tag: freefare_sys::FreefareTag, byte: u8) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::ntag21x_set_auth(tag, byte) }
}

pub fn get_auth(tag: freefare_sys::FreefareTag, byte: *mut u8) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::ntag21x_get_auth(tag, byte) }
}

pub fn access_enable(tag: freefare_sys::FreefareTag, byte: u8) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::ntag21x_access_enable(tag, byte) }
}

pub fn access_disable(tag: freefare_sys::FreefareTag, byte: u8) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::ntag21x_access_disable(tag, byte) }
}

pub fn get_access(tag: freefare_sys::FreefareTag, byte: *mut u8) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::ntag21x_get_access(tag, byte) }
}

pub fn check_access(
    tag: freefare_sys::FreefareTag,
    byte: u8,
    result: *mut bool,
) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::ntag21x_check_access(tag, byte, result) }
}

pub fn get_authentication_limit(
    tag: freefare_sys::FreefareTag,
    byte: *mut u8,
) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::ntag21x_get_authentication_limit(tag, byte) }
}

pub fn set_authentication_limit(tag: freefare_sys::FreefareTag, byte: u8) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::ntag21x_set_authentication_limit(tag, byte) }
}

pub fn read(tag: freefare_sys::FreefareTag, page: u8, data: *mut u8) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::ntag21x_read(tag, page, data) }
}

pub fn read4(tag: freefare_sys::FreefareTag, page: u8, data: *mut u8) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::ntag21x_read4(tag, page, data) }
}

pub fn fast_read(
    tag: freefare_sys::FreefareTag,
    start_page: u8,
    end_page: u8,
    data: *mut u8,
) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::ntag21x_fast_read(tag, start_page, end_page, data) }
}

pub fn fast_read4(
    tag: freefare_sys::FreefareTag,
    page: u8,
    data: *mut u8,
) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::ntag21x_fast_read4(tag, page, data) }
}

pub fn read_cnt(tag: freefare_sys::FreefareTag, data: *mut u8) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::ntag21x_read_cnt(tag, data) }
}

pub fn write(tag: freefare_sys::FreefareTag, page: u8, data: *mut u8) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::ntag21x_write(tag, page, data) }
}

pub fn compatibility_write(
    tag: freefare_sys::FreefareTag,
    page: u8,
    data: *mut u8,
) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::ntag21x_compatibility_write(tag, page, data) }
}

pub fn authenticate(
    tag: freefare_sys::FreefareTag,
    key: freefare_sys::NTAG21xKey,
) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::ntag21x_authenticate(tag, key) }
}

pub fn is_ntag21x(tag: freefare_sys::FreefareTag) -> bool {
    unsafe { freefare_sys::is_ntag21x(tag) }
}

pub fn is_auth_supported(
    device: *mut nfc_sys::nfc_device,
    nai: nfc_sys::nfc_iso14443a_info,
) -> bool {
    unsafe { freefare_sys::ntag21x_is_auth_supported(device, nai) }
}
