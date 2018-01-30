use ::nfc_sys;
use ::freefare_sys;

pub fn get_tags(device: *mut nfc_sys::nfc_device) -> *mut freefare_sys::FreefareTag {
    unsafe { freefare_sys::freefare_get_tags(device) }
}

pub fn tag_new(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> freefare_sys::FreefareTag {
    unsafe { freefare_sys::freefare_tag_new(device, target) }
}

pub fn get_tag_type(tag: freefare_sys::FreefareTag) -> freefare_sys::Enum_freefare_tag_type {
    unsafe { freefare_sys::freefare_get_tag_type(tag) }
}

pub fn get_tag_friendly_name(tag: freefare_sys::FreefareTag) -> *const ::std::os::raw::c_char {
    unsafe { freefare_sys::freefare_get_tag_friendly_name(tag) }
}

pub fn get_tag_uid(tag: freefare_sys::FreefareTag) -> *mut ::std::os::raw::c_char {
    unsafe { freefare_sys::freefare_get_tag_uid(tag) }
}

pub fn free_tag(tag: freefare_sys::FreefareTag) {
    unsafe { freefare_sys::freefare_free_tag(tag) }
}

pub fn free_tags(tags: *mut freefare_sys::FreefareTag) {
    unsafe { freefare_sys::freefare_free_tags(tags) }
}

pub fn selected_tag_is_present(device: *mut nfc_sys::nfc_device) -> u8 {
    unsafe { freefare_sys::freefare_selected_tag_is_present(device) }
}

pub fn strerror(tag: freefare_sys::FreefareTag) -> *const ::std::os::raw::c_char {
    unsafe { freefare_sys::freefare_strerror(tag) }
}

pub fn strerror_r(tag: freefare_sys::FreefareTag, buffer: *mut ::std::os::raw::c_char, len: usize) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::freefare_strerror_r(tag, buffer, len) }
}

pub fn perror(tag: freefare_sys::FreefareTag, string: *const ::std::os::raw::c_char) {
    unsafe { freefare_sys::freefare_perror(tag, string) }
}