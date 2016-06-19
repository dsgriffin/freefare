use ::nfc_sys;
use ::ffi;

use libc::off_t;

pub fn ultralight_taste(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> u8 {
    unsafe { ffi::mifare_ultralight_taste(device, target) }
}

pub fn ultralightc_taste(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> u8 {
    unsafe { ffi::mifare_ultralightc_taste(device, target) }
}

pub fn ultralight_tag_new(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> ffi::FreefareTag {
    unsafe { ffi::mifare_ultralight_tag_new(device, target) }
}

pub fn ultralightc_tag_new(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> ffi::FreefareTag {
    unsafe { ffi::mifare_ultralightc_tag_new(device, target) }
}

pub fn ultralight_tag_free(tag: ffi::FreefareTag) {
    unsafe { ffi::mifare_ultralight_tag_free(tag) }
}

pub fn ultralightc_tag_free(tag: ffi::FreefareTag) {
    unsafe { ffi::mifare_ultralightc_tag_free(tag) }
}

pub fn ultralight_connect(tag: ffi::FreefareTag) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_ultralight_connect(tag) }
}

pub fn ultralight_disconnect(tag: ffi::FreefareTag) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_ultralight_disconnect(tag) }
}

pub fn ultralight_read(tag: ffi::FreefareTag, page: ffi::MifareUltralightPageNumber, data: *mut ffi::MifareUltralightPage) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_ultralight_read(tag, page, data) }
}

pub fn ultralight_write(tag: ffi::FreefareTag, page: ffi::MifareUltralightPageNumber, data: ffi::MifareUltralightPage) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_ultralight_write(tag, page, data) }
}

pub fn ultralightc_authenticate(tag: ffi::FreefareTag, key: ffi::MifareDESFireKey) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_ultralightc_authenticate(tag, key) }
}


pub fn is_mifare_ultralight(tag: ffi::FreefareTag) -> u8 {
    unsafe { ffi::is_mifare_ultralight(tag) }
}

pub fn is_mifare_ultralightc(tag: ffi::FreefareTag) -> u8 {
    unsafe { ffi::is_mifare_ultralightc(tag) }
}

pub fn is_mifare_ultralightc_on_reader(device: *mut nfc_sys::nfc_device, nai: nfc_sys::nfc_iso14443a_info) -> u8 {
    unsafe { ffi::is_mifare_ultralightc_on_reader(device, nai) }
}


pub fn classic1k_taste(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> u8 {
    unsafe { ffi::mifare_classic1k_taste(device, target) }
}

pub fn classic4k_taste(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> u8 {
    unsafe { ffi::mifare_classic4k_taste(device, target) }
}

pub fn classic1k_tag_new(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> ffi::FreefareTag {
    unsafe { ffi::mifare_classic1k_tag_new(device, target) }
}

pub fn classic4k_tag_new(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> ffi::FreefareTag {
    unsafe { ffi::mifare_classic4k_tag_new(device, target) }
}

pub fn classic_tag_free(tag: ffi::FreefareTag) {
    unsafe { ffi::mifare_classic_tag_free(tag) }
}

pub fn classic_connect(tag: ffi::FreefareTag) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_classic_connect(tag) }
}

pub fn classic_disconnect(tag: ffi::FreefareTag) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_classic_disconnect(tag) }
}

pub fn classic_authenticate(tag: ffi::FreefareTag, block: ffi::MifareClassicBlockNumber, key: ffi::MifareClassicKey, key_type: ffi::MifareClassicKeyType) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_classic_authenticate(tag, block, key, key_type) }
}

pub fn classic_read(tag: ffi::FreefareTag, block: ffi::MifareClassicBlockNumber, data: *mut ffi::MifareClassicBlock) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_classic_read(tag, block, data) }
}

pub fn classic_init_value(tag: ffi::FreefareTag, block: ffi::MifareClassicBlockNumber, value: i32, adr: ffi::MifareClassicBlockNumber) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_classic_init_value(tag, block, value, adr) }
}

pub fn classic_read_value(tag: ffi::FreefareTag, block: ffi::MifareClassicBlockNumber, value: *mut i32, adr: *mut ffi::MifareClassicBlockNumber) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_classic_read_value(tag, block, value, adr) }
}

pub fn classic_write(tag: ffi::FreefareTag, block: ffi::MifareClassicBlockNumber, data: ffi::MifareClassicBlock) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_classic_write(tag, block, data) }
}

pub fn classic_increment(tag: ffi::FreefareTag, block: ffi::MifareClassicBlockNumber, amount: u32) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_classic_increment(tag, block, amount) }
}

pub fn classic_decrement(tag: ffi::FreefareTag, block: ffi::MifareClassicBlockNumber, amount: u32) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_classic_decrement(tag, block, amount) }
}

pub fn classic_restore(tag: ffi::FreefareTag, block: ffi::MifareClassicBlockNumber) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_classic_restore(tag, block) }
}

pub fn classic_transfer(tag: ffi::FreefareTag, block: ffi::MifareClassicBlockNumber) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_classic_transfer(tag, block) }
}

pub fn classic_get_trailer_block_permission(tag: ffi::FreefareTag, block: ffi::MifareClassicBlockNumber, permission: u16, key_type: ffi::MifareClassicKeyType) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_classic_get_trailer_block_permission(tag, block, permission, key_type) }
}

pub fn classic_get_data_block_permission(tag: ffi::FreefareTag, block: ffi::MifareClassicBlockNumber, permission: ::std::os::raw::c_uchar, key_type: ffi::MifareClassicKeyType) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_classic_get_data_block_permission(tag, block, permission, key_type) }
}

pub fn classic_format_sector(tag: ffi::FreefareTag, sector: ffi::MifareClassicSectorNumber) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_classic_format_sector(tag, sector) }
}

pub fn classic_trailer_block(block: *mut ffi::MifareClassicBlock, key_a: ffi::MifareClassicKey, ab_0: u8, ab_1: u8, ab_2: u8, ab_tb: u8, gpb: u8, key_b: ffi::MifareClassicKey) {
    unsafe { ffi::mifare_classic_trailer_block(block, key_a, ab_0, ab_1, ab_2, ab_tb, gpb, key_b) }
}

pub fn classic_block_sector(block: ffi::MifareClassicBlockNumber) -> ffi::MifareClassicSectorNumber {
    unsafe { ffi::mifare_classic_block_sector(block) }
}

pub fn classic_sector_first_block(sector: ffi::MifareClassicSectorNumber) -> ffi::MifareClassicBlockNumber {
    unsafe { ffi::mifare_classic_sector_first_block(sector) }
}

pub fn classic_sector_block_count(sector: ffi::MifareClassicSectorNumber) -> usize {
    unsafe { ffi::mifare_classic_sector_block_count(sector) }
}

pub fn classic_sector_last_block(sector: ffi::MifareClassicSectorNumber) -> ffi::MifareClassicBlockNumber {
    unsafe { ffi::mifare_classic_sector_last_block(sector) }
}

pub fn application_alloc(mad: ffi::Mad, aid: ffi::MadAid, size: usize) -> *mut ffi::MifareClassicSectorNumber {
    unsafe { ffi::mifare_application_alloc(mad, aid, size) }
}

pub fn application_read(tag: ffi::FreefareTag, mad: ffi::Mad, aid: ffi::MadAid, buf: *mut ::std::os::raw::c_void, nbytes: usize, key: ffi::MifareClassicKey, key_type: ffi::MifareClassicKeyType) -> isize {
    unsafe { ffi::mifare_application_read(tag, mad, aid, buf, nbytes, key, key_type) }
}

pub fn application_write(tag: ffi::FreefareTag, mad: ffi::Mad, aid: ffi::MadAid, buf: *const ::std::os::raw::c_void, nbytes: usize, key: ffi::MifareClassicKey, key_type: ffi::MifareClassicKeyType) -> isize {
    unsafe { ffi::mifare_application_write(tag, mad, aid, buf, nbytes, key, key_type) }
}

pub fn application_free(mad: ffi::Mad, aid: ffi::MadAid) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_application_free(mad, aid) }
}

pub fn application_find(mad: ffi::Mad, aid: ffi::MadAid) -> *mut ffi::MifareClassicSectorNumber {
    unsafe { ffi::mifare_application_find(mad, aid) }
}




pub fn desfire_taste(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> u8 {
    unsafe { ffi::mifare_desfire_taste(device, target) }
}

pub fn desfire_aid_new(aid: u32) -> ffi::MifareDESFireAID {
    unsafe { ffi::mifare_desfire_aid_new(aid) }
}

pub fn desfire_aid_new_with_mad_aid(mad_aid: ffi::MadAid, n: u8) -> ffi::MifareDESFireAID {
    unsafe { ffi::mifare_desfire_aid_new_with_mad_aid(mad_aid, n) }
}

pub fn desfire_aid_get_aid(aid: ffi::MifareDESFireAID) -> u32 {
    unsafe { ffi::mifare_desfire_aid_get_aid(aid) }
}

pub fn desfire_last_pcd_error(tag: ffi::FreefareTag) -> u8 {
    unsafe { ffi::mifare_desfire_last_pcd_error(tag) }
}

pub fn desfire_last_picc_error(tag: ffi::FreefareTag) -> u8 {
    unsafe { ffi::mifare_desfire_last_picc_error(tag) }
}

pub fn desfire_tag_new(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> ffi::FreefareTag {
    unsafe { ffi::mifare_desfire_tag_new(device, target) }
}

pub fn desfire_tag_free(tags: ffi::FreefareTag) {
    unsafe { ffi::mifare_desfire_tag_free(tags) }
}

pub fn desfire_connect(tag: ffi::FreefareTag) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_connect(tag) }
}

pub fn desfire_disconnect(tag: ffi::FreefareTag) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_disconnect(tag) }
}

pub fn desfire_authenticate(tag: ffi::FreefareTag, key_no: u8, key: ffi::MifareDESFireKey) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_authenticate(tag, key_no, key) }
}

pub fn desfire_authenticate_iso(tag: ffi::FreefareTag, key_no: u8, key: ffi::MifareDESFireKey) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_authenticate_iso(tag, key_no, key) }
}

pub fn desfire_authenticate_aes(tag: ffi::FreefareTag, key_no: u8, key: ffi::MifareDESFireKey) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_authenticate_aes(tag, key_no, key) }
}

pub fn desfire_change_key_settings(tag: ffi::FreefareTag, settings: u8) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_change_key_settings(tag, settings) }
}

pub fn desfire_get_key_settings(tag: ffi::FreefareTag, settings: *mut u8, max_keys: *mut u8) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_get_key_settings(tag, settings, max_keys) }
}

pub fn desfire_change_key(tag: ffi::FreefareTag, key_no: u8, new_key: ffi::MifareDESFireKey, old_key: ffi::MifareDESFireKey) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_change_key(tag, key_no, new_key, old_key) }
}

pub fn desfire_get_key_version(tag: ffi::FreefareTag, key_no: u8, version: *mut u8) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_get_key_version(tag, key_no, version) }
}

pub fn desfire_create_application(tag: ffi::FreefareTag, aid: ffi::MifareDESFireAID, settings: u8, key_no: u8) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_create_application(tag, aid, settings, key_no) }
}

pub fn desfire_create_application_3k3des(tag: ffi::FreefareTag, aid: ffi::MifareDESFireAID, settings: u8, key_no: u8) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_create_application_3k3des(tag, aid, settings, key_no) }
}

pub fn desfire_create_application_aes(tag: ffi::FreefareTag, aid: ffi::MifareDESFireAID, settings: u8, key_no: u8) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_create_application_aes(tag, aid, settings, key_no) }
}

pub fn desfire_create_application_iso(tag: ffi::FreefareTag, aid: ffi::MifareDESFireAID, settings: u8, key_no: u8, want_iso_file_identifiers: ::std::os::raw::c_int, iso_file_id: u16, iso_file_name: *mut u8, iso_file_name_len: usize) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_create_application_iso(tag, aid, settings, key_no, want_iso_file_identifiers, iso_file_id, iso_file_name, iso_file_name_len) }
}

pub fn desfire_create_application_3k3des_iso(tag: ffi::FreefareTag, aid: ffi::MifareDESFireAID, settings: u8, key_no: u8, want_iso_file_identifiers: ::std::os::raw::c_int, iso_file_id: u16, iso_file_name: *mut u8, iso_file_name_len: usize) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_create_application_3k3des_iso(tag, aid, settings, key_no, want_iso_file_identifiers, iso_file_id, iso_file_name, iso_file_name_len) }
}

pub fn desfire_create_application_aes_iso(tag: ffi::FreefareTag, aid: ffi::MifareDESFireAID, settings: u8, key_no: u8, want_iso_file_identifiers: ::std::os::raw::c_int, iso_file_id: u16, iso_file_name: *mut u8, iso_file_name_len: usize) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_create_application_aes_iso(tag, aid, settings, key_no, want_iso_file_identifiers, iso_file_id, iso_file_name, iso_file_name_len) }
}

pub fn desfire_delete_application(tag: ffi::FreefareTag, aid: ffi::MifareDESFireAID) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_delete_application(tag, aid) }
}

pub fn desfire_get_application_ids(tag: ffi::FreefareTag, aids: *mut *mut ffi::MifareDESFireAID, count: *mut usize) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_get_application_ids(tag, aids, count) }
}

pub fn desfire_get_df_names(tag: ffi::FreefareTag, dfs: *mut *mut ffi::MifareDESFireDF, count: *mut usize) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_get_df_names(tag, dfs, count) }
}

pub fn desfire_free_application_ids(aids: *mut ffi::MifareDESFireAID) {
    unsafe { ffi::mifare_desfire_free_application_ids(aids) }
}

pub fn desfire_select_application(tag: ffi::FreefareTag, aid: ffi::MifareDESFireAID) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_select_application(tag, aid) }
}

pub fn desfire_format_picc(tag: ffi::FreefareTag) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_format_picc(tag) }
}

pub fn desfire_get_version(tag: ffi::FreefareTag, version_info: *mut ffi::Struct_mifare_desfire_version_info) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_get_version(tag, version_info) }
}

pub fn desfire_free_mem(tag: ffi::FreefareTag, size: *mut u32) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_free_mem(tag, size) }
}

pub fn desfire_set_configuration(tag: ffi::FreefareTag, disable_format: u8, enable_random_uid: u8) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_set_configuration(tag, disable_format, enable_random_uid) }
}

pub fn desfire_set_default_key(tag: ffi::FreefareTag, key: ffi::MifareDESFireKey) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_set_default_key(tag, key) }
}

pub fn desfire_set_ats(tag: ffi::FreefareTag, ats: *mut u8) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_set_ats(tag, ats) }
}

pub fn desfire_get_card_uid(tag: ffi::FreefareTag, uid: *mut *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_get_card_uid(tag, uid) }
}

pub fn desfire_get_file_ids(tag: ffi::FreefareTag, files: *mut *mut u8, count: *mut usize) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_get_file_ids(tag, files, count) }
}

pub fn desfire_get_iso_file_ids(tag: ffi::FreefareTag, files: *mut *mut u16, count: *mut usize) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_get_iso_file_ids(tag, files, count) }
}

pub fn desfire_get_file_settings(tag: ffi::FreefareTag, file_no: u8, settings: *mut ffi::Struct_mifare_desfire_file_settings) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_get_file_settings(tag, file_no, settings) }
}

pub fn desfire_change_file_settings(tag: ffi::FreefareTag, file_no: u8, communication_settings: u8, access_rights: u16) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_change_file_settings(tag, file_no, communication_settings, access_rights) }
}

pub fn desfire_create_std_data_file(tag: ffi::FreefareTag, file_no: u8, communication_settings: u8, access_rights: u16, file_size: u32) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_create_std_data_file(tag, file_no, communication_settings, access_rights, file_size) }
}

pub fn desfire_create_std_data_file_iso(tag: ffi::FreefareTag, file_no: u8, communication_settings: u8, access_rights: u16, file_size: u32, iso_file_id: u16) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_create_std_data_file_iso(tag, file_no, communication_settings, access_rights, file_size, iso_file_id) }
}

pub fn desfire_create_backup_data_file(tag: ffi::FreefareTag, file_no: u8, communication_settings: u8, access_rights: u16, file_size: u32) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_create_backup_data_file(tag, file_no, communication_settings, access_rights, file_size) }
}

pub fn desfire_create_backup_data_file_iso(tag: ffi::FreefareTag, file_no: u8, communication_settings:u8, access_rights: u16, file_size: u32,iso_file_id: u16) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_create_backup_data_file_iso(tag, file_no, communication_settings, access_rights, file_size, iso_file_id) }
}

pub fn desfire_create_value_file(tag: ffi::FreefareTag, file_no: u8, communication_settings: u8, access_rights: u16, lower_limit: i32, upper_limit: i32, value: i32, limited_credit_enable: u8) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_create_value_file(tag, file_no, communication_settings, access_rights, lower_limit, upper_limit, value, limited_credit_enable) }
}

pub fn desfire_create_linear_record_file(tag: ffi::FreefareTag, file_no: u8, communication_settings: u8, access_rights: u16, record_size: u32, max_number_of_records: u32) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_create_linear_record_file(tag, file_no, communication_settings, access_rights, record_size, max_number_of_records) }
}

pub fn desfire_create_linear_record_file_iso(tag: ffi::FreefareTag, file_no: u8, communication_settings: u8, access_rights: u16, record_size: u32, max_number_of_records: u32, iso_file_id: u16) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_create_linear_record_file_iso(tag, file_no, communication_settings, access_rights, record_size, max_number_of_records, iso_file_id) }
}

pub fn desfire_create_cyclic_record_file(tag: ffi::FreefareTag, file_no: u8, communication_settings: u8, access_rights: u16, record_size: u32, max_number_of_records: u32) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_create_cyclic_record_file(tag, file_no, communication_settings, access_rights, record_size, max_number_of_records) }
}

pub fn desfire_create_cyclic_record_file_iso(tag: ffi::FreefareTag, file_no: u8, communication_settings: u8, access_rights: u16, record_size: u32, max_number_of_records: u32, iso_file_id: u16) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_create_cyclic_record_file_iso(tag, file_no, communication_settings, access_rights, record_size, max_number_of_records, iso_file_id) }
}

pub fn desfire_delete_file(tag: ffi::FreefareTag, file_no: u8) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_delete_file(tag, file_no) }
}

pub fn desfire_read_data(tag: ffi::FreefareTag, file_no: u8, offset: off_t, length: usize, data: *mut ::std::os::raw::c_void) -> isize {
    unsafe { ffi::mifare_desfire_read_data(tag, file_no, offset, length, data) }
}

pub fn desfire_read_data_ex(tag: ffi::FreefareTag, file_no: u8, offset: off_t, length: usize, data: *mut ::std::os::raw::c_void, cs: ::std::os::raw::c_int) -> isize {
    unsafe { ffi::mifare_desfire_read_data_ex(tag, file_no, offset, length, data, cs) }
}

pub fn desfire_write_data(tag: ffi::FreefareTag, file_no: u8, offset: off_t, length: usize, data: *const ::std::os::raw::c_void) -> isize {
    unsafe { ffi::mifare_desfire_write_data(tag, file_no, offset, length, data) }
}

pub fn desfire_write_data_ex(tag: ffi::FreefareTag, file_no: u8, offset: off_t, length: usize, data: *const ::std::os::raw::c_void, cs: ::std::os::raw::c_int) -> isize {
    unsafe { ffi::mifare_desfire_write_data_ex(tag, file_no, offset, length, data, cs) }
}

pub fn desfire_get_value(tag: ffi::FreefareTag, file_no: u8, value: *mut i32) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_get_value(tag, file_no, value) }
}

pub fn desfire_get_value_ex(tag: ffi::FreefareTag, file_no: u8, value: *mut i32, cs: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_get_value_ex(tag, file_no, value, cs) }
}

pub fn desfire_credit(tag: ffi::FreefareTag, file_no: u8, amount: i32) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_credit(tag, file_no, amount) }
}

pub fn desfire_credit_ex(tag: ffi::FreefareTag, file_no: u8, amount: i32, cs: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_credit_ex(tag, file_no, amount, cs) }
}

pub fn desfire_debit(tag: ffi::FreefareTag, file_no: u8, amount: i32) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_debit(tag, file_no, amount) }
}

pub fn desfire_debit_ex(tag: ffi::FreefareTag, file_no: u8, amount: i32, cs: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_debit_ex(tag, file_no, amount, cs) }
}

pub fn desfire_limited_credit(tag: ffi::FreefareTag, file_no: u8, amount: i32) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_limited_credit(tag, file_no, amount) }
}

pub fn desfire_limited_credit_ex(tag: ffi::FreefareTag, file_no: u8, amount: i32, cs: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_limited_credit_ex(tag, file_no, amount, cs) }
}

pub fn desfire_write_record(tag: ffi::FreefareTag, file_no: u8, offset: off_t, length: usize, data: *mut ::std::os::raw::c_void) -> isize {
    unsafe { ffi::mifare_desfire_write_record(tag, file_no, offset, length, data) }
}

pub fn desfire_write_record_ex(tag: ffi::FreefareTag, file_no: u8, offset: off_t, length: usize, data: *mut ::std::os::raw::c_void, cs: ::std::os::raw::c_int) -> isize {
    unsafe { ffi::mifare_desfire_write_record_ex(tag, file_no, offset, length, data, cs) }
}

pub fn desfire_read_records(tag: ffi::FreefareTag, file_no: u8, offset: off_t, length: usize, data: *mut ::std::os::raw::c_void) -> isize {
    unsafe { ffi::mifare_desfire_read_records(tag, file_no, offset, length, data) }
}

pub fn desfire_read_records_ex(tag: ffi::FreefareTag, file_no: u8, offset: off_t, length: usize, data: *mut ::std::os::raw::c_void, cs: ::std::os::raw::c_int) -> isize {
    unsafe { ffi::mifare_desfire_read_records_ex(tag, file_no, offset, length, data, cs) }
}

pub fn desfire_clear_record_file(tag: ffi::FreefareTag, file_no: u8) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_clear_record_file(tag, file_no) }
}

pub fn desfire_commit_transaction(tag: ffi::FreefareTag) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_commit_transaction(tag) }
}

pub fn desfire_abort_transaction(tag: ffi::FreefareTag) -> ::std::os::raw::c_int {
    unsafe { ffi::mifare_desfire_abort_transaction(tag) }
}

pub fn desfire_des_key_new(value: *mut u8) -> ffi::MifareDESFireKey {
    unsafe { ffi::mifare_desfire_des_key_new(value) }
}

pub fn desfire_3des_key_new(value: *mut u8) -> ffi::MifareDESFireKey {
    unsafe { ffi::mifare_desfire_3des_key_new(value) }
}

pub fn desfire_des_key_new_with_version(value: *mut u8) -> ffi::MifareDESFireKey {
    unsafe { ffi::mifare_desfire_des_key_new_with_version(value) }
}

pub fn desfire_3des_key_new_with_version(value: *mut u8) -> ffi::MifareDESFireKey {
    unsafe { ffi::mifare_desfire_3des_key_new_with_version(value) }
}

pub fn desfire_3k3des_key_new(value: *mut u8) -> ffi::MifareDESFireKey {
    unsafe { ffi::mifare_desfire_3k3des_key_new(value) }
}

pub fn desfire_3k3des_key_new_with_version(value: *mut u8) -> ffi::MifareDESFireKey {
    unsafe { ffi::mifare_desfire_3k3des_key_new_with_version(value) }
}

pub fn desfire_aes_key_new(value: *mut u8) -> ffi::MifareDESFireKey {
    unsafe { ffi::mifare_desfire_aes_key_new(value) }
}

pub fn desfire_aes_key_new_with_version(value: *mut u8, version: u8) -> ffi::MifareDESFireKey {
    unsafe { ffi::mifare_desfire_aes_key_new_with_version(value, version) }
}

pub fn desfire_key_get_version(key: ffi::MifareDESFireKey) -> u8 {
    unsafe { ffi::mifare_desfire_key_get_version(key) }
}

pub fn desfire_key_set_version(key: ffi::MifareDESFireKey, version: u8) {
    unsafe { ffi::mifare_desfire_key_set_version(key, version) }
}

pub fn desfire_key_free(key: ffi::MifareDESFireKey) {
    unsafe { ffi::mifare_desfire_key_free(key) }
}