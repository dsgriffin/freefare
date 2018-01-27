use ::freefare_sys;

pub fn new(version: u8) -> freefare_sys::Mad {
    unsafe { freefare_sys::mad_new(version) }
}

pub fn read(tag: freefare_sys::FreefareTag) -> freefare_sys::Mad {
    unsafe { freefare_sys::mad_read(tag) }
}

pub fn write(tag: freefare_sys::FreefareTag, mad: freefare_sys::Mad, key_b_sector_00: freefare_sys::MifareClassicKey, key_b_sector_10: freefare_sys::MifareClassicKey) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mad_write(tag, mad, key_b_sector_00, key_b_sector_10) }
}

pub fn get_version(mad: freefare_sys::Mad) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mad_get_version(mad) }
}

pub fn set_version(mad: freefare_sys::Mad, version: u8) {
    unsafe { freefare_sys::mad_set_version(mad, version) }
}

pub fn get_card_publisher_sector(mad: freefare_sys::Mad) -> freefare_sys::MifareClassicSectorNumber {
    unsafe { freefare_sys::mad_get_card_publisher_sector(mad) }
}

pub fn set_card_publisher_sector(mad: freefare_sys::Mad, cps: freefare_sys::MifareClassicSectorNumber) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mad_set_card_publisher_sector(mad, cps) }
}

pub fn get_aid(mad: freefare_sys::Mad, sector: freefare_sys::MifareClassicSectorNumber, aid: *mut freefare_sys::MadAid) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mad_get_aid(mad, sector, aid) }
}

pub fn set_aid(mad: freefare_sys::Mad, sector: freefare_sys::MifareClassicSectorNumber, aid: freefare_sys::MadAid) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mad_set_aid(mad, sector, aid) }
}

pub fn sector_reserved(sector: freefare_sys::MifareClassicSectorNumber) -> u8 {
    unsafe { freefare_sys::mad_sector_reserved(sector) }
}

pub fn free(mad: freefare_sys::Mad) {
    unsafe { freefare_sys::mad_free(mad) }
}