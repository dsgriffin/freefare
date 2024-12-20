use ::nfc_sys;
use ::freefare_sys;
use libc::off_t;

pub fn ultralight_taste(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> u8 {
    unsafe { freefare_sys::mifare_ultralight_taste(device, target) }
}

pub fn ultralightc_taste(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> u8 {
    unsafe { freefare_sys::mifare_ultralightc_taste(device, target) }
}

pub fn ultralight_tag_new(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> freefare_sys::FreefareTag {
    unsafe { freefare_sys::mifare_ultralight_tag_new(device, target) }
}

pub fn ultralightc_tag_new(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> freefare_sys::FreefareTag {
    unsafe { freefare_sys::mifare_ultralightc_tag_new(device, target) }
}

pub fn ultralight_tag_free(tag: freefare_sys::FreefareTag) {
    unsafe { freefare_sys::mifare_ultralight_tag_free(tag) }
}

pub fn ultralightc_tag_free(tag: freefare_sys::FreefareTag) {
    unsafe { freefare_sys::mifare_ultralightc_tag_free(tag) }
}

pub fn ultralight_connect(tag: freefare_sys::FreefareTag) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_ultralight_connect(tag) }
}

pub fn ultralight_disconnect(tag: freefare_sys::FreefareTag) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_ultralight_disconnect(tag) }
}

pub fn ultralight_read(tag: freefare_sys::FreefareTag, page: freefare_sys::MifareUltralightPageNumber, data: *mut freefare_sys::MifareUltralightPage) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_ultralight_read(tag, page, data) }
}

pub fn ultralight_write(tag: freefare_sys::FreefareTag, page: freefare_sys::MifareUltralightPageNumber, data: freefare_sys::MifareUltralightPage) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_ultralight_write(tag, page, data) }
}

pub fn ultralightc_authenticate(tag: freefare_sys::FreefareTag, key: freefare_sys::MifareDESFireKey) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_ultralightc_authenticate(tag, key) }
}


pub fn is_mifare_ultralight(tag: freefare_sys::FreefareTag) -> u8 {
    unsafe { freefare_sys::is_mifare_ultralight(tag) }
}

pub fn is_mifare_ultralightc(tag: freefare_sys::FreefareTag) -> u8 {
    unsafe { freefare_sys::is_mifare_ultralightc(tag) }
}

pub fn is_mifare_ultralightc_on_reader(device: *mut nfc_sys::nfc_device, nai: nfc_sys::nfc_iso14443a_info) -> u8 {
    unsafe { freefare_sys::is_mifare_ultralightc_on_reader(device, nai) }
}


pub fn classic1k_taste(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> u8 {
    unsafe { freefare_sys::mifare_classic1k_taste(device, target) }
}

pub fn classic4k_taste(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> u8 {
    unsafe { freefare_sys::mifare_classic4k_taste(device, target) }
}

pub fn classic1k_tag_new(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> freefare_sys::FreefareTag {
    unsafe { freefare_sys::mifare_classic1k_tag_new(device, target) }
}

pub fn classic4k_tag_new(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> freefare_sys::FreefareTag {
    unsafe { freefare_sys::mifare_classic4k_tag_new(device, target) }
}

pub fn classic_tag_free(tag: freefare_sys::FreefareTag) {
    unsafe { freefare_sys::mifare_classic_tag_free(tag) }
}

pub fn classic_connect(tag: freefare_sys::FreefareTag) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_classic_connect(tag) }
}

pub fn classic_disconnect(tag: freefare_sys::FreefareTag) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_classic_disconnect(tag) }
}

pub fn classic_authenticate(tag: freefare_sys::FreefareTag, block: freefare_sys::MifareClassicBlockNumber, key: freefare_sys::MifareClassicKey, key_type: freefare_sys::MifareClassicKeyType) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_classic_authenticate(tag, block, key, key_type) }
}

pub fn classic_read(tag: freefare_sys::FreefareTag, block: freefare_sys::MifareClassicBlockNumber, data: *mut freefare_sys::MifareClassicBlock) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_classic_read(tag, block, data) }
}

pub fn classic_init_value(tag: freefare_sys::FreefareTag, block: freefare_sys::MifareClassicBlockNumber, value: i32, adr: freefare_sys::MifareClassicBlockNumber) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_classic_init_value(tag, block, value, adr) }
}

pub fn classic_read_value(tag: freefare_sys::FreefareTag, block: freefare_sys::MifareClassicBlockNumber, value: *mut i32, adr: *mut freefare_sys::MifareClassicBlockNumber) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_classic_read_value(tag, block, value, adr) }
}

pub fn classic_write(tag: freefare_sys::FreefareTag, block: freefare_sys::MifareClassicBlockNumber, data: freefare_sys::MifareClassicBlock) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_classic_write(tag, block, data) }
}

pub fn classic_increment(tag: freefare_sys::FreefareTag, block: freefare_sys::MifareClassicBlockNumber, amount: u32) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_classic_increment(tag, block, amount) }
}

pub fn classic_decrement(tag: freefare_sys::FreefareTag, block: freefare_sys::MifareClassicBlockNumber, amount: u32) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_classic_decrement(tag, block, amount) }
}

pub fn classic_restore(tag: freefare_sys::FreefareTag, block: freefare_sys::MifareClassicBlockNumber) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_classic_restore(tag, block) }
}

pub fn classic_transfer(tag: freefare_sys::FreefareTag, block: freefare_sys::MifareClassicBlockNumber) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_classic_transfer(tag, block) }
}

pub fn classic_get_trailer_block_permission(tag: freefare_sys::FreefareTag, block: freefare_sys::MifareClassicBlockNumber, permission: u16, key_type: freefare_sys::MifareClassicKeyType) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_classic_get_trailer_block_permission(tag, block, permission, key_type) }
}

pub fn classic_get_data_block_permission(tag: freefare_sys::FreefareTag, block: freefare_sys::MifareClassicBlockNumber, permission: ::std::os::raw::c_uchar, key_type: freefare_sys::MifareClassicKeyType) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_classic_get_data_block_permission(tag, block, permission, key_type) }
}

pub fn classic_format_sector(tag: freefare_sys::FreefareTag, sector: freefare_sys::MifareClassicSectorNumber) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_classic_format_sector(tag, sector) }
}

pub fn classic_trailer_block(block: *mut freefare_sys::MifareClassicBlock, key_a: freefare_sys::MifareClassicKey, ab_0: u8, ab_1: u8, ab_2: u8, ab_tb: u8, gpb: u8, key_b: freefare_sys::MifareClassicKey) {
    unsafe { freefare_sys::mifare_classic_trailer_block(block, key_a, ab_0, ab_1, ab_2, ab_tb, gpb, key_b) }
}

pub fn classic_block_sector(block: freefare_sys::MifareClassicBlockNumber) -> freefare_sys::MifareClassicSectorNumber {
    unsafe { freefare_sys::mifare_classic_block_sector(block) }
}

pub fn classic_sector_first_block(sector: freefare_sys::MifareClassicSectorNumber) -> freefare_sys::MifareClassicBlockNumber {
    unsafe { freefare_sys::mifare_classic_sector_first_block(sector) }
}

pub fn classic_sector_block_count(sector: freefare_sys::MifareClassicSectorNumber) -> usize {
    unsafe { freefare_sys::mifare_classic_sector_block_count(sector) }
}

pub fn classic_sector_last_block(sector: freefare_sys::MifareClassicSectorNumber) -> freefare_sys::MifareClassicBlockNumber {
    unsafe { freefare_sys::mifare_classic_sector_last_block(sector) }
}

pub fn application_alloc(mad: freefare_sys::Mad, aid: freefare_sys::MadAid, size: usize) -> *mut freefare_sys::MifareClassicSectorNumber {
    unsafe { freefare_sys::mifare_application_alloc(mad, aid, size) }
}

pub fn application_read(tag: freefare_sys::FreefareTag, mad: freefare_sys::Mad, aid: freefare_sys::MadAid, buf: *mut ::std::os::raw::c_void, nbytes: usize, key: freefare_sys::MifareClassicKey, key_type: freefare_sys::MifareClassicKeyType) -> isize {
    unsafe { freefare_sys::mifare_application_read(tag, mad, aid, buf, nbytes, key, key_type) }
}

pub fn application_write(tag: freefare_sys::FreefareTag, mad: freefare_sys::Mad, aid: freefare_sys::MadAid, buf: *const ::std::os::raw::c_void, nbytes: usize, key: freefare_sys::MifareClassicKey, key_type: freefare_sys::MifareClassicKeyType) -> isize {
    unsafe { freefare_sys::mifare_application_write(tag, mad, aid, buf, nbytes, key, key_type) }
}

pub fn application_free(mad: freefare_sys::Mad, aid: freefare_sys::MadAid) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_application_free(mad, aid) }
}

pub fn application_find(mad: freefare_sys::Mad, aid: freefare_sys::MadAid) -> *mut freefare_sys::MifareClassicSectorNumber {
    unsafe { freefare_sys::mifare_application_find(mad, aid) }
}




pub fn desfire_taste(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> u8 {
    unsafe { freefare_sys::mifare_desfire_taste(device, target) }
}

pub fn desfire_aid_new(aid: u32) -> freefare_sys::MifareDESFireAID {
    unsafe { freefare_sys::mifare_desfire_aid_new(aid) }
}

pub fn desfire_aid_new_with_mad_aid(mad_aid: freefare_sys::MadAid, n: u8) -> freefare_sys::MifareDESFireAID {
    unsafe { freefare_sys::mifare_desfire_aid_new_with_mad_aid(mad_aid, n) }
}

pub fn desfire_aid_get_aid(aid: freefare_sys::MifareDESFireAID) -> u32 {
    unsafe { freefare_sys::mifare_desfire_aid_get_aid(aid) }
}

pub fn desfire_last_pcd_error(tag: freefare_sys::FreefareTag) -> u8 {
    unsafe { freefare_sys::mifare_desfire_last_pcd_error(tag) }
}

pub fn desfire_last_picc_error(tag: freefare_sys::FreefareTag) -> u8 {
    unsafe { freefare_sys::mifare_desfire_last_picc_error(tag) }
}

pub fn desfire_tag_new(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> freefare_sys::FreefareTag {
    unsafe { freefare_sys::mifare_desfire_tag_new(device, target) }
}

pub fn desfire_tag_free(tags: freefare_sys::FreefareTag) {
    unsafe { freefare_sys::mifare_desfire_tag_free(tags) }
}

pub fn desfire_connect(tag: freefare_sys::FreefareTag) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_connect(tag) }
}

pub fn desfire_disconnect(tag: freefare_sys::FreefareTag) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_disconnect(tag) }
}

pub fn desfire_authenticate(tag: freefare_sys::FreefareTag, key_no: u8, key: freefare_sys::MifareDESFireKey) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_authenticate(tag, key_no, key) }
}

pub fn desfire_authenticate_iso(tag: freefare_sys::FreefareTag, key_no: u8, key: freefare_sys::MifareDESFireKey) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_authenticate_iso(tag, key_no, key) }
}

pub fn desfire_authenticate_aes(tag: freefare_sys::FreefareTag, key_no: u8, key: freefare_sys::MifareDESFireKey) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_authenticate_aes(tag, key_no, key) }
}

pub fn desfire_change_key_settings(tag: freefare_sys::FreefareTag, settings: u8) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_change_key_settings(tag, settings) }
}

pub fn desfire_get_key_settings(tag: freefare_sys::FreefareTag, settings: *mut u8, max_keys: *mut u8) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_get_key_settings(tag, settings, max_keys) }
}

pub fn desfire_change_key(tag: freefare_sys::FreefareTag, key_no: u8, new_key: freefare_sys::MifareDESFireKey, old_key: freefare_sys::MifareDESFireKey) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_change_key(tag, key_no, new_key, old_key) }
}

pub fn desfire_get_key_version(tag: freefare_sys::FreefareTag, key_no: u8, version: *mut u8) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_get_key_version(tag, key_no, version) }
}

pub fn desfire_create_application(tag: freefare_sys::FreefareTag, aid: freefare_sys::MifareDESFireAID, settings: u8, key_no: u8) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_create_application(tag, aid, settings, key_no) }
}

pub fn desfire_create_application_3k3des(tag: freefare_sys::FreefareTag, aid: freefare_sys::MifareDESFireAID, settings: u8, key_no: u8) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_create_application_3k3des(tag, aid, settings, key_no) }
}

pub fn desfire_create_application_aes(tag: freefare_sys::FreefareTag, aid: freefare_sys::MifareDESFireAID, settings: u8, key_no: u8) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_create_application_aes(tag, aid, settings, key_no) }
}

pub fn desfire_create_application_iso(tag: freefare_sys::FreefareTag, aid: freefare_sys::MifareDESFireAID, settings: u8, key_no: u8, want_iso_file_identifiers: ::std::os::raw::c_int, iso_file_id: u16, iso_file_name: *mut u8, iso_file_name_len: usize) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_create_application_iso(tag, aid, settings, key_no, want_iso_file_identifiers, iso_file_id, iso_file_name, iso_file_name_len) }
}

pub fn desfire_create_application_3k3des_iso(tag: freefare_sys::FreefareTag, aid: freefare_sys::MifareDESFireAID, settings: u8, key_no: u8, want_iso_file_identifiers: ::std::os::raw::c_int, iso_file_id: u16, iso_file_name: *mut u8, iso_file_name_len: usize) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_create_application_3k3des_iso(tag, aid, settings, key_no, want_iso_file_identifiers, iso_file_id, iso_file_name, iso_file_name_len) }
}

pub fn desfire_create_application_aes_iso(tag: freefare_sys::FreefareTag, aid: freefare_sys::MifareDESFireAID, settings: u8, key_no: u8, want_iso_file_identifiers: ::std::os::raw::c_int, iso_file_id: u16, iso_file_name: *mut u8, iso_file_name_len: usize) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_create_application_aes_iso(tag, aid, settings, key_no, want_iso_file_identifiers, iso_file_id, iso_file_name, iso_file_name_len) }
}

pub fn desfire_delete_application(tag: freefare_sys::FreefareTag, aid: freefare_sys::MifareDESFireAID) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_delete_application(tag, aid) }
}

pub fn desfire_get_application_ids(tag: freefare_sys::FreefareTag, aids: *mut *mut freefare_sys::MifareDESFireAID, count: *mut usize) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_get_application_ids(tag, aids, count) }
}

pub fn desfire_get_df_names(tag: freefare_sys::FreefareTag, dfs: *mut *mut freefare_sys::MifareDESFireDF, count: *mut usize) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_get_df_names(tag, dfs, count) }
}

pub fn desfire_free_application_ids(aids: *mut freefare_sys::MifareDESFireAID) {
    unsafe { freefare_sys::mifare_desfire_free_application_ids(aids) }
}

pub fn desfire_select_application(tag: freefare_sys::FreefareTag, aid: freefare_sys::MifareDESFireAID) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_select_application(tag, aid) }
}

pub fn desfire_format_picc(tag: freefare_sys::FreefareTag) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_format_picc(tag) }
}

pub fn desfire_get_version(tag: freefare_sys::FreefareTag, version_info: *mut freefare_sys::Struct_mifare_desfire_version_info) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_get_version(tag, version_info) }
}

pub fn desfire_free_mem(tag: freefare_sys::FreefareTag, size: *mut u32) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_free_mem(tag, size) }
}

pub fn desfire_set_configuration(tag: freefare_sys::FreefareTag, disable_format: u8, enable_random_uid: u8) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_set_configuration(tag, disable_format, enable_random_uid) }
}

pub fn desfire_set_default_key(tag: freefare_sys::FreefareTag, key: freefare_sys::MifareDESFireKey) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_set_default_key(tag, key) }
}

pub fn desfire_set_ats(tag: freefare_sys::FreefareTag, ats: *mut u8) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_set_ats(tag, ats) }
}

pub fn desfire_get_card_uid(tag: freefare_sys::FreefareTag, uid: *mut *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_get_card_uid(tag, uid) }
}

pub fn desfire_get_file_ids(tag: freefare_sys::FreefareTag, files: *mut *mut u8, count: *mut usize) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_get_file_ids(tag, files, count) }
}

pub fn desfire_get_iso_file_ids(tag: freefare_sys::FreefareTag, files: *mut *mut u16, count: *mut usize) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_get_iso_file_ids(tag, files, count) }
}

pub fn desfire_get_file_settings(tag: freefare_sys::FreefareTag, file_no: u8, settings: *mut freefare_sys::Struct_mifare_desfire_file_settings) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_get_file_settings(tag, file_no, settings) }
}

pub fn desfire_change_file_settings(tag: freefare_sys::FreefareTag, file_no: u8, communication_settings: u8, access_rights: u16) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_change_file_settings(tag, file_no, communication_settings, access_rights) }
}

pub fn desfire_create_std_data_file(tag: freefare_sys::FreefareTag, file_no: u8, communication_settings: u8, access_rights: u16, file_size: u32) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_create_std_data_file(tag, file_no, communication_settings, access_rights, file_size) }
}

pub fn desfire_create_std_data_file_iso(tag: freefare_sys::FreefareTag, file_no: u8, communication_settings: u8, access_rights: u16, file_size: u32, iso_file_id: u16) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_create_std_data_file_iso(tag, file_no, communication_settings, access_rights, file_size, iso_file_id) }
}

pub fn desfire_create_backup_data_file(tag: freefare_sys::FreefareTag, file_no: u8, communication_settings: u8, access_rights: u16, file_size: u32) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_create_backup_data_file(tag, file_no, communication_settings, access_rights, file_size) }
}

pub fn desfire_create_backup_data_file_iso(tag: freefare_sys::FreefareTag, file_no: u8, communication_settings:u8, access_rights: u16, file_size: u32,iso_file_id: u16) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_create_backup_data_file_iso(tag, file_no, communication_settings, access_rights, file_size, iso_file_id) }
}

pub fn desfire_create_value_file(tag: freefare_sys::FreefareTag, file_no: u8, communication_settings: u8, access_rights: u16, lower_limit: i32, upper_limit: i32, value: i32, limited_credit_enable: u8) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_create_value_file(tag, file_no, communication_settings, access_rights, lower_limit, upper_limit, value, limited_credit_enable) }
}

pub fn desfire_create_linear_record_file(tag: freefare_sys::FreefareTag, file_no: u8, communication_settings: u8, access_rights: u16, record_size: u32, max_number_of_records: u32) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_create_linear_record_file(tag, file_no, communication_settings, access_rights, record_size, max_number_of_records) }
}

pub fn desfire_create_linear_record_file_iso(tag: freefare_sys::FreefareTag, file_no: u8, communication_settings: u8, access_rights: u16, record_size: u32, max_number_of_records: u32, iso_file_id: u16) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_create_linear_record_file_iso(tag, file_no, communication_settings, access_rights, record_size, max_number_of_records, iso_file_id) }
}

pub fn desfire_create_cyclic_record_file(tag: freefare_sys::FreefareTag, file_no: u8, communication_settings: u8, access_rights: u16, record_size: u32, max_number_of_records: u32) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_create_cyclic_record_file(tag, file_no, communication_settings, access_rights, record_size, max_number_of_records) }
}

pub fn desfire_create_cyclic_record_file_iso(tag: freefare_sys::FreefareTag, file_no: u8, communication_settings: u8, access_rights: u16, record_size: u32, max_number_of_records: u32, iso_file_id: u16) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_create_cyclic_record_file_iso(tag, file_no, communication_settings, access_rights, record_size, max_number_of_records, iso_file_id) }
}

pub fn desfire_delete_file(tag: freefare_sys::FreefareTag, file_no: u8) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_delete_file(tag, file_no) }
}

pub fn desfire_read_data(tag: freefare_sys::FreefareTag, file_no: u8, offset: off_t, length: usize, data: *mut ::std::os::raw::c_void) -> isize {
    unsafe { freefare_sys::mifare_desfire_read_data(tag, file_no, offset, length, data) }
}

pub fn desfire_read_data_ex(tag: freefare_sys::FreefareTag, file_no: u8, offset: off_t, length: usize, data: *mut ::std::os::raw::c_void, cs: ::std::os::raw::c_int) -> isize {
    unsafe { freefare_sys::mifare_desfire_read_data_ex(tag, file_no, offset, length, data, cs) }
}

pub fn desfire_write_data(tag: freefare_sys::FreefareTag, file_no: u8, offset: off_t, length: usize, data: *const ::std::os::raw::c_void) -> isize {
    unsafe { freefare_sys::mifare_desfire_write_data(tag, file_no, offset, length, data) }
}

pub fn desfire_write_data_ex(tag: freefare_sys::FreefareTag, file_no: u8, offset: off_t, length: usize, data: *const ::std::os::raw::c_void, cs: ::std::os::raw::c_int) -> isize {
    unsafe { freefare_sys::mifare_desfire_write_data_ex(tag, file_no, offset, length, data, cs) }
}

pub fn desfire_get_value(tag: freefare_sys::FreefareTag, file_no: u8, value: *mut i32) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_get_value(tag, file_no, value) }
}

pub fn desfire_get_value_ex(tag: freefare_sys::FreefareTag, file_no: u8, value: *mut i32, cs: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_get_value_ex(tag, file_no, value, cs) }
}

pub fn desfire_credit(tag: freefare_sys::FreefareTag, file_no: u8, amount: i32) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_credit(tag, file_no, amount) }
}

pub fn desfire_credit_ex(tag: freefare_sys::FreefareTag, file_no: u8, amount: i32, cs: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_credit_ex(tag, file_no, amount, cs) }
}

pub fn desfire_debit(tag: freefare_sys::FreefareTag, file_no: u8, amount: i32) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_debit(tag, file_no, amount) }
}

pub fn desfire_debit_ex(tag: freefare_sys::FreefareTag, file_no: u8, amount: i32, cs: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_debit_ex(tag, file_no, amount, cs) }
}

pub fn desfire_limited_credit(tag: freefare_sys::FreefareTag, file_no: u8, amount: i32) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_limited_credit(tag, file_no, amount) }
}

pub fn desfire_limited_credit_ex(tag: freefare_sys::FreefareTag, file_no: u8, amount: i32, cs: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_limited_credit_ex(tag, file_no, amount, cs) }
}

pub fn desfire_write_record(tag: freefare_sys::FreefareTag, file_no: u8, offset: off_t, length: usize, data: *mut ::std::os::raw::c_void) -> isize {
    unsafe { freefare_sys::mifare_desfire_write_record(tag, file_no, offset, length, data) }
}

pub fn desfire_write_record_ex(tag: freefare_sys::FreefareTag, file_no: u8, offset: off_t, length: usize, data: *mut ::std::os::raw::c_void, cs: ::std::os::raw::c_int) -> isize {
    unsafe { freefare_sys::mifare_desfire_write_record_ex(tag, file_no, offset, length, data, cs) }
}

pub fn desfire_read_records(tag: freefare_sys::FreefareTag, file_no: u8, offset: off_t, length: usize, data: *mut ::std::os::raw::c_void) -> isize {
    unsafe { freefare_sys::mifare_desfire_read_records(tag, file_no, offset, length, data) }
}

pub fn desfire_read_records_ex(tag: freefare_sys::FreefareTag, file_no: u8, offset: off_t, length: usize, data: *mut ::std::os::raw::c_void, cs: ::std::os::raw::c_int) -> isize {
    unsafe { freefare_sys::mifare_desfire_read_records_ex(tag, file_no, offset, length, data, cs) }
}

pub fn desfire_clear_record_file(tag: freefare_sys::FreefareTag, file_no: u8) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_clear_record_file(tag, file_no) }
}

pub fn desfire_commit_transaction(tag: freefare_sys::FreefareTag) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_commit_transaction(tag) }
}

pub fn desfire_abort_transaction(tag: freefare_sys::FreefareTag) -> ::std::os::raw::c_int {
    unsafe { freefare_sys::mifare_desfire_abort_transaction(tag) }
}

pub fn desfire_des_key_new(value: *mut u8) -> freefare_sys::MifareDESFireKey {
    unsafe { freefare_sys::mifare_desfire_des_key_new(value) }
}

pub fn desfire_3des_key_new(value: *mut u8) -> freefare_sys::MifareDESFireKey {
    unsafe { freefare_sys::mifare_desfire_3des_key_new(value) }
}

pub fn desfire_des_key_new_with_version(value: *mut u8) -> freefare_sys::MifareDESFireKey {
    unsafe { freefare_sys::mifare_desfire_des_key_new_with_version(value) }
}

pub fn desfire_3des_key_new_with_version(value: *mut u8) -> freefare_sys::MifareDESFireKey {
    unsafe { freefare_sys::mifare_desfire_3des_key_new_with_version(value) }
}

pub fn desfire_3k3des_key_new(value: *mut u8) -> freefare_sys::MifareDESFireKey {
    unsafe { freefare_sys::mifare_desfire_3k3des_key_new(value) }
}

pub fn desfire_3k3des_key_new_with_version(value: *mut u8) -> freefare_sys::MifareDESFireKey {
    unsafe { freefare_sys::mifare_desfire_3k3des_key_new_with_version(value) }
}

pub fn desfire_aes_key_new(value: *mut u8) -> freefare_sys::MifareDESFireKey {
    unsafe { freefare_sys::mifare_desfire_aes_key_new(value) }
}

pub fn desfire_aes_key_new_with_version(value: *mut u8, version: u8) -> freefare_sys::MifareDESFireKey {
    unsafe { freefare_sys::mifare_desfire_aes_key_new_with_version(value, version) }
}

pub fn desfire_key_get_version(key: freefare_sys::MifareDESFireKey) -> u8 {
    unsafe { freefare_sys::mifare_desfire_key_get_version(key) }
}

pub fn desfire_key_set_version(key: freefare_sys::MifareDESFireKey, version: u8) {
    unsafe { freefare_sys::mifare_desfire_key_set_version(key, version) }
}

pub fn desfire_key_free(key: freefare_sys::MifareDESFireKey) {
    unsafe { freefare_sys::mifare_desfire_key_free(key) }
}
