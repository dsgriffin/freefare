use ::ffi;

pub fn new(version: u8) -> ffi::Mad {
    unsafe { ffi::mad_new(version) }
}

pub fn read(tag: ffi::FreefareTag) -> ffi::Mad {
    unsafe { ffi::mad_read(tag) }
}

pub fn write(tag: ffi::FreefareTag, mad: ffi::Mad, key_b_sector_00: ffi::MifareClassicKey, key_b_sector_10: ffi::MifareClassicKey) -> ::std::os::raw::c_int {
    unsafe { ffi::mad_write(tag, mad, key_b_sector_00, key_b_sector_10) }
}

pub fn get_version(mad: ffi::Mad) -> ::std::os::raw::c_int {
    unsafe { ffi::mad_get_version(mad) }
}

pub fn set_version(mad: ffi::Mad, version: u8) {
    unsafe { ffi::mad_set_version(mad, version) }
}

pub fn get_card_publisher_sector(mad: ffi::Mad) -> ffi::MifareClassicSectorNumber {
    unsafe { ffi::mad_get_card_publisher_sector(mad) }
}

pub fn set_card_publisher_sector(mad: ffi::Mad, cps: ffi::MifareClassicSectorNumber) -> ::std::os::raw::c_int {
    unsafe { ffi::mad_set_card_publisher_sector(mad, cps) }
}

pub fn get_aid(mad: ffi::Mad, sector: ffi::MifareClassicSectorNumber, aid: *mut ffi::MadAid) -> ::std::os::raw::c_int {
    unsafe { ffi::mad_get_aid(mad, sector, aid) }
}

pub fn set_aid(mad: ffi::Mad, sector: ffi::MifareClassicSectorNumber, aid: ffi::MadAid) -> ::std::os::raw::c_int {
    unsafe { ffi::mad_set_aid(mad, sector, aid) }
}

pub fn sector_reserved(sector: ffi::MifareClassicSectorNumber) -> u8 {
    unsafe { ffi::mad_sector_reserved(sector) }
}

pub fn free(mad: ffi::Mad) {
    unsafe { ffi::mad_free(mad) }
}