use ::freefare_sys;

pub fn encode(_type: u8, istream: *const u8, isize: u16, osize: *mut usize) -> *mut u8 {
    unsafe { freefare_sys::tlv_encode(_type, istream, isize, osize) }
}

pub fn decode(istream: *const u8, _type: *mut u8, size: *mut u16) -> *mut u8 {
    unsafe { freefare_sys::tlv_decode(istream, _type, size) }
}

pub fn record_length(istream: *const u8, field_length_size: *mut usize, field_value_size: *mut usize) -> usize {
    unsafe { freefare_sys::tlv_record_length(istream, field_length_size, field_value_size) }
}

pub fn append(a: *mut u8, b: *mut u8) -> *mut u8 {
    unsafe { freefare_sys::tlv_append(a, b) }
}