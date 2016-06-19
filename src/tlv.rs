use ::ffi;

pub fn encode(_type: u8, istream: *const u8, isize: u16, osize: *mut usize) -> *mut u8 {
    unsafe { ffi::tlv_encode(_type, istream, isize, osize) }
}

pub fn decode(istream: *const u8, _type: *mut u8, size: *mut u16) -> *mut u8 {
    unsafe { ffi::tlv_decode(istream, _type, size) }
}

pub fn record_length(istream: *const u8, field_length_size: *mut usize, field_value_size: *mut usize) -> usize {
    unsafe { ffi::tlv_record_length(istream, field_length_size, field_value_size) }
}

pub fn append(a: *mut u8, b: *mut u8) -> *mut u8 {
    unsafe { ffi::tlv_append(a, b) }
}