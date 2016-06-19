use ::nfc_sys;
use ::ffi;

pub fn get_tags(device: *mut nfc_sys::nfc_device) -> *mut ffi::FreefareTag {
    unsafe { ffi::freefare_get_tags(device) }
}

pub fn tag_new(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> ffi::FreefareTag {
    unsafe { ffi::freefare_tag_new(device, target) }
}

pub fn get_tag_type(tag: ffi::FreefareTag) -> ffi::Enum_freefare_tag_type {
    unsafe { ffi::freefare_get_tag_type(tag) }
}

pub fn get_tag_friendly_name(tag: ffi::FreefareTag) -> *const ::std::os::raw::c_char {
    unsafe { ffi::freefare_get_tag_friendly_name(tag) }
}

pub fn get_tag_uid(tag: ffi::FreefareTag) -> *mut ::std::os::raw::c_char {
    unsafe { ffi::freefare_get_tag_uid(tag) }
}

pub fn free_tag(tag: ffi::FreefareTag) {
    unsafe { ffi::freefare_free_tag(tag) }
}

pub fn free_tags(tags: *mut ffi::FreefareTag) {
    unsafe { ffi::freefare_free_tags(tags) }
}

pub fn selected_tag_is_present(device: *mut nfc_sys::nfc_device) -> u8 {
    unsafe { ffi::freefare_selected_tag_is_present(device) }
}

pub fn strerror(tag: ffi::FreefareTag) -> *const ::std::os::raw::c_char {
    unsafe { ffi::freefare_strerror(tag) }
}

pub fn strerror_r(tag: ffi::FreefareTag, buffer: *mut ::std::os::raw::c_char, len: usize) -> ::std::os::raw::c_int {
    unsafe { ffi::freefare_strerror_r(tag, buffer, len) }
}

pub fn perror(tag: ffi::FreefareTag, string: *const ::std::os::raw::c_char) {
    unsafe { ffi::freefare_perror(tag, string) }
}