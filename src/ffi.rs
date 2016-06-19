#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]

#[link(name = "freefare", kind = "dylib")]

extern crate libc;
extern crate nfc_sys;

use libc::{uint8_t, uint16_t, uint32_t, int32_t, size_t, ssize_t, off_t};

use self::nfc_sys::{nfc_device, nfc_target, nfc_iso14443a_info};

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_freefare_tag_type {
    FELICA = 0,
    MIFARE_CLASSIC_1K = 1,
    MIFARE_CLASSIC_4K = 2,
    MIFARE_DESFIRE = 3,
    MIFARE_ULTRALIGHT = 4,
    MIFARE_ULTRALIGHT_C = 5,
}
pub enum Struct_freefare_tag { }
pub type FreefareTag = *mut Struct_freefare_tag;
pub type MifareTag = *mut Struct_freefare_tag;
pub enum Struct_mifare_desfire_key { }
pub type MifareDESFireKey = *mut Struct_mifare_desfire_key;
pub type MifareUltralightPageNumber = uint8_t;
pub type MifareUltralightPage = [::std::os::raw::c_uchar; 4usize];
pub type MifareClassicBlock = [::std::os::raw::c_uchar; 16usize];
pub type MifareClassicSectorNumber = uint8_t;
pub type MifareClassicBlockNumber = ::std::os::raw::c_uchar;
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed1 { MFC_KEY_A = 0, MFC_KEY_B = 1, }
pub type MifareClassicKeyType = Enum_Unnamed1;
pub type MifareClassicKey = [::std::os::raw::c_uchar; 6usize];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_mad_aid {
    pub application_code: uint8_t,
    pub function_cluster_code: uint8_t,
}
impl ::std::default::Default for Struct_mad_aid {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type MadAid = Struct_mad_aid;
pub enum Struct_mad { }
pub type Mad = *mut Struct_mad;
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_mifare_desfire_file_types {
    MDFT_STANDARD_DATA_FILE = 0,
    MDFT_BACKUP_DATA_FILE = 1,
    MDFT_VALUE_FILE_WITH_BACKUP = 2,
    MDFT_LINEAR_RECORD_FILE_WITH_BACKUP = 3,
    MDFT_CYCLIC_RECORD_FILE_WITH_BACKUP = 4,
}
pub enum Struct_mifare_desfire_aid { }
pub type MifareDESFireAID = *mut Struct_mifare_desfire_aid;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_mifare_desfire_df {
    pub aid: uint32_t,
    pub fid: uint16_t,
    pub df_name: [uint8_t; 16usize],
    pub df_name_len: size_t,
}
impl ::std::default::Default for Struct_mifare_desfire_df {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type MifareDESFireDF = Struct_mifare_desfire_df;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_mifare_desfire_version_info {
    pub hardware: Struct_Unnamed2,
    pub software: Struct_Unnamed3,
    pub uid: [uint8_t; 7usize],
    pub batch_number: [uint8_t; 5usize],
    pub production_week: uint8_t,
    pub production_year: uint8_t,
}
impl ::std::default::Default for Struct_mifare_desfire_version_info {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed2 {
    pub vendor_id: uint8_t,
    pub _type: uint8_t,
    pub subtype: uint8_t,
    pub version_major: uint8_t,
    pub version_minor: uint8_t,
    pub storage_size: uint8_t,
    pub protocol: uint8_t,
}
impl ::std::default::Default for Struct_Unnamed2 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed3 {
    pub vendor_id: uint8_t,
    pub _type: uint8_t,
    pub subtype: uint8_t,
    pub version_major: uint8_t,
    pub version_minor: uint8_t,
    pub storage_size: uint8_t,
    pub protocol: uint8_t,
}
impl ::std::default::Default for Struct_Unnamed3 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_mifare_desfire_file_settings {
    pub file_type: uint8_t,
    pub communication_settings: uint8_t,
    pub access_rights: uint16_t,
    pub settings: Union_Unnamed4,
}
impl ::std::default::Default for Struct_mifare_desfire_file_settings {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Union_Unnamed4 {
    pub _bindgen_data_: [u32; 4usize],
}
impl Union_Unnamed4 {
    pub unsafe fn standard_file(&mut self) -> *mut Struct_Unnamed5 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn value_file(&mut self) -> *mut Struct_Unnamed6 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn linear_record_file(&mut self) -> *mut Struct_Unnamed7 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::default::Default for Union_Unnamed4 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed5 {
    pub file_size: uint32_t,
}
impl ::std::default::Default for Struct_Unnamed5 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed6 {
    pub lower_limit: int32_t,
    pub upper_limit: int32_t,
    pub limited_credit_value: int32_t,
    pub limited_credit_enabled: uint8_t,
}
impl ::std::default::Default for Struct_Unnamed6 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed7 {
    pub record_size: uint32_t,
    pub max_number_of_records: uint32_t,
    pub current_number_of_records: uint32_t,
}
impl ::std::default::Default for Struct_Unnamed7 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[link(name = "freefare", kind = "dylib")]
extern "C" {
    pub static mifare_classic_nfcforum_public_key_a: MifareClassicKey;
    pub static mad_public_key_a: MifareClassicKey;
    pub static mad_free_aid: MadAid;
    pub static mad_defect_aid: MadAid;
    pub static mad_reserved_aid: MadAid;
    pub static mad_card_holder_aid: MadAid;
    pub static mad_not_applicable_aid: MadAid;
    pub static mad_nfcforum_aid: MadAid;
}
#[link(name = "freefare", kind = "dylib")]
extern "C" {
    pub fn freefare_get_tags(device: *mut nfc_device) -> *mut FreefareTag;
    pub fn freefare_tag_new(device: *mut nfc_device, target: nfc_target)
        -> FreefareTag;
    pub fn freefare_get_tag_type(tag: FreefareTag) -> Enum_freefare_tag_type;
    pub fn freefare_get_tag_friendly_name(tag: FreefareTag)
        -> *const ::std::os::raw::c_char;
    pub fn freefare_get_tag_uid(tag: FreefareTag)
        -> *mut ::std::os::raw::c_char;
    pub fn freefare_free_tag(tag: FreefareTag);
    pub fn freefare_free_tags(tags: *mut FreefareTag);
    pub fn freefare_selected_tag_is_present(device: *mut nfc_device) -> u8;
    pub fn freefare_strerror(tag: FreefareTag)
        -> *const ::std::os::raw::c_char;
    pub fn freefare_strerror_r(tag: FreefareTag,
                               buffer: *mut ::std::os::raw::c_char,
                               len: size_t) -> ::std::os::raw::c_int;
    pub fn freefare_perror(tag: FreefareTag,
                           string: *const ::std::os::raw::c_char);
    pub fn felica_taste(device: *mut nfc_device, target: nfc_target) -> u8;
    pub fn felica_tag_new(device: *mut nfc_device, target: nfc_target)
        -> FreefareTag;
    pub fn felica_tag_free(tag: FreefareTag);
    pub fn felica_read(tag: FreefareTag, service: uint16_t, block: uint8_t,
                       data: *mut uint8_t, length: size_t) -> ssize_t;
    pub fn felica_read_ex(tag: FreefareTag, service: uint16_t,
                          block_count: uint8_t, blocks: *mut uint8_t,
                          data: *mut uint8_t, length: size_t) -> ssize_t;
    pub fn felica_write(tag: FreefareTag, service: uint16_t, block: uint8_t,
                        data: *mut uint8_t, length: size_t) -> ssize_t;
    pub fn felica_write_ex(tag: FreefareTag, service: uint16_t,
                           block_count: uint8_t, blocks: *mut uint8_t,
                           data: *mut uint8_t, length: size_t) -> ssize_t;
    pub fn mifare_ultralight_taste(device: *mut nfc_device,
                                   target: nfc_target) -> u8;
    pub fn mifare_ultralightc_taste(device: *mut nfc_device,
                                    target: nfc_target) -> u8;
    pub fn mifare_ultralight_tag_new(device: *mut nfc_device,
                                     target: nfc_target) -> FreefareTag;
    pub fn mifare_ultralightc_tag_new(device: *mut nfc_device,
                                      target: nfc_target) -> FreefareTag;
    pub fn mifare_ultralight_tag_free(tag: FreefareTag);
    pub fn mifare_ultralightc_tag_free(tag: FreefareTag);
    pub fn mifare_ultralight_connect(tag: FreefareTag)
        -> ::std::os::raw::c_int;
    pub fn mifare_ultralight_disconnect(tag: FreefareTag)
        -> ::std::os::raw::c_int;
    pub fn mifare_ultralight_read(tag: FreefareTag,
                                  page: MifareUltralightPageNumber,
                                  data: *mut MifareUltralightPage)
                                  -> ::std::os::raw::c_int;
    pub fn mifare_ultralight_write(tag: FreefareTag,
                                   page: MifareUltralightPageNumber,
                                   data: MifareUltralightPage)
                                   -> ::std::os::raw::c_int;
    pub fn mifare_ultralightc_authenticate(tag: FreefareTag,
                                           key: MifareDESFireKey)
                                           -> ::std::os::raw::c_int;
    pub fn is_mifare_ultralight(tag: FreefareTag) -> u8;
    pub fn is_mifare_ultralightc(tag: FreefareTag) -> u8;
    pub fn is_mifare_ultralightc_on_reader(device: *mut nfc_device,
                                           nai: nfc_iso14443a_info) -> u8;
    pub fn mifare_classic1k_taste(device: *mut nfc_device, target: nfc_target)
        -> u8;
    pub fn mifare_classic4k_taste(device: *mut nfc_device, target: nfc_target)
        -> u8;
    pub fn mifare_classic1k_tag_new(device: *mut nfc_device,
                                    target: nfc_target) -> FreefareTag;
    pub fn mifare_classic4k_tag_new(device: *mut nfc_device,
                                    target: nfc_target) -> FreefareTag;
    pub fn mifare_classic_tag_free(tag: FreefareTag);
    pub fn mifare_classic_connect(tag: FreefareTag) -> ::std::os::raw::c_int;
    pub fn mifare_classic_disconnect(tag: FreefareTag)
        -> ::std::os::raw::c_int;
    pub fn mifare_classic_authenticate(tag: FreefareTag,
                                       block: MifareClassicBlockNumber,
                                       key: MifareClassicKey,
                                       key_type: MifareClassicKeyType)
                                       -> ::std::os::raw::c_int;
    pub fn mifare_classic_read(tag: FreefareTag,
                               block: MifareClassicBlockNumber,
                               data: *mut MifareClassicBlock)
                               -> ::std::os::raw::c_int;
    pub fn mifare_classic_init_value(tag: FreefareTag,
                                     block: MifareClassicBlockNumber,
                                     value: int32_t,
                                     adr: MifareClassicBlockNumber)
                                     -> ::std::os::raw::c_int;
    pub fn mifare_classic_read_value(tag: FreefareTag,
                                     block: MifareClassicBlockNumber,
                                     value: *mut int32_t,
                                     adr: *mut MifareClassicBlockNumber)
                                     -> ::std::os::raw::c_int;
    pub fn mifare_classic_write(tag: FreefareTag,
                                block: MifareClassicBlockNumber,
                                data: MifareClassicBlock)
                                -> ::std::os::raw::c_int;
    pub fn mifare_classic_increment(tag: FreefareTag,
                                    block: MifareClassicBlockNumber,
                                    amount: uint32_t)
                                    -> ::std::os::raw::c_int;
    pub fn mifare_classic_decrement(tag: FreefareTag,
                                    block: MifareClassicBlockNumber,
                                    amount: uint32_t)
                                    -> ::std::os::raw::c_int;
    pub fn mifare_classic_restore(tag: FreefareTag,
                                  block: MifareClassicBlockNumber)
                                  -> ::std::os::raw::c_int;
    pub fn mifare_classic_transfer(tag: FreefareTag,
                                   block: MifareClassicBlockNumber)
                                   -> ::std::os::raw::c_int;
    pub fn mifare_classic_get_trailer_block_permission(tag: FreefareTag,
                                                       block:
                                                       MifareClassicBlockNumber,
                                                       permission: uint16_t,
                                                       key_type:
                                                       MifareClassicKeyType)
                                                       -> ::std::os::raw::c_int;
    pub fn mifare_classic_get_data_block_permission(tag: FreefareTag,
                                                    block:
                                                    MifareClassicBlockNumber,
                                                    permission:
                                                    ::std::os::raw::c_uchar,
                                                    key_type:
                                                    MifareClassicKeyType)
                                                    -> ::std::os::raw::c_int;
    pub fn mifare_classic_format_sector(tag: FreefareTag,
                                        sector: MifareClassicSectorNumber)
                                        -> ::std::os::raw::c_int;
    pub fn mifare_classic_trailer_block(block: *mut MifareClassicBlock,
                                        key_a: MifareClassicKey,
                                        ab_0: uint8_t, ab_1: uint8_t,
                                        ab_2: uint8_t, ab_tb: uint8_t,
                                        gpb: uint8_t,
                                        key_b: MifareClassicKey);
    pub fn mifare_classic_block_sector(block: MifareClassicBlockNumber)
        -> MifareClassicSectorNumber;
    pub fn mifare_classic_sector_first_block(sector:
                                             MifareClassicSectorNumber)
                                             -> MifareClassicBlockNumber;
    pub fn mifare_classic_sector_block_count(sector:
                                             MifareClassicSectorNumber)
                                             -> size_t;
    pub fn mifare_classic_sector_last_block(sector: MifareClassicSectorNumber)
        -> MifareClassicBlockNumber;
    pub fn mad_new(version: uint8_t) -> Mad;
    pub fn mad_read(tag: FreefareTag) -> Mad;
    pub fn mad_write(tag: FreefareTag, mad: Mad,
                     key_b_sector_00: MifareClassicKey,
                     key_b_sector_10: MifareClassicKey)
                     -> ::std::os::raw::c_int;
    pub fn mad_get_version(mad: Mad) -> ::std::os::raw::c_int;
    pub fn mad_set_version(mad: Mad, version: uint8_t);
    pub fn mad_get_card_publisher_sector(mad: Mad)
        -> MifareClassicSectorNumber;
    pub fn mad_set_card_publisher_sector(mad: Mad,
                                         cps: MifareClassicSectorNumber)
                                         -> ::std::os::raw::c_int;
    pub fn mad_get_aid(mad: Mad, sector: MifareClassicSectorNumber,
                       aid: *mut MadAid) -> ::std::os::raw::c_int;
    pub fn mad_set_aid(mad: Mad, sector: MifareClassicSectorNumber,
                       aid: MadAid) -> ::std::os::raw::c_int;
    pub fn mad_sector_reserved(sector: MifareClassicSectorNumber) -> u8;
    pub fn mad_free(mad: Mad);
    pub fn mifare_application_alloc(mad: Mad, aid: MadAid, size: size_t)
        -> *mut MifareClassicSectorNumber;
    pub fn mifare_application_read(tag: FreefareTag, mad: Mad, aid: MadAid,
                                   buf: *mut ::std::os::raw::c_void,
                                   nbytes: size_t, key: MifareClassicKey,
                                   key_type: MifareClassicKeyType) -> ssize_t;
    pub fn mifare_application_write(tag: FreefareTag, mad: Mad, aid: MadAid,
                                    buf: *const ::std::os::raw::c_void,
                                    nbytes: size_t, key: MifareClassicKey,
                                    key_type: MifareClassicKeyType)
                                    -> ssize_t;
    pub fn mifare_application_free(mad: Mad, aid: MadAid)
        -> ::std::os::raw::c_int;
    pub fn mifare_application_find(mad: Mad, aid: MadAid)
        -> *mut MifareClassicSectorNumber;
    pub fn mifare_desfire_taste(device: *mut nfc_device, target: nfc_target)
        -> u8;
    pub fn mifare_desfire_aid_new(aid: uint32_t) -> MifareDESFireAID;
    pub fn mifare_desfire_aid_new_with_mad_aid(mad_aid: MadAid, n: uint8_t)
        -> MifareDESFireAID;
    pub fn mifare_desfire_aid_get_aid(aid: MifareDESFireAID) -> uint32_t;
    pub fn mifare_desfire_last_pcd_error(tag: FreefareTag) -> uint8_t;
    pub fn mifare_desfire_last_picc_error(tag: FreefareTag) -> uint8_t;
    pub fn mifare_desfire_tag_new(device: *mut nfc_device, target: nfc_target)
        -> FreefareTag;
    pub fn mifare_desfire_tag_free(tags: FreefareTag);
    pub fn mifare_desfire_connect(tag: FreefareTag) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_disconnect(tag: FreefareTag)
        -> ::std::os::raw::c_int;
    pub fn mifare_desfire_authenticate(tag: FreefareTag, key_no: uint8_t,
                                       key: MifareDESFireKey)
                                       -> ::std::os::raw::c_int;
    pub fn mifare_desfire_authenticate_iso(tag: FreefareTag, key_no: uint8_t,
                                           key: MifareDESFireKey)
                                           -> ::std::os::raw::c_int;
    pub fn mifare_desfire_authenticate_aes(tag: FreefareTag, key_no: uint8_t,
                                           key: MifareDESFireKey)
                                           -> ::std::os::raw::c_int;
    pub fn mifare_desfire_change_key_settings(tag: FreefareTag,
                                              settings: uint8_t)
                                              -> ::std::os::raw::c_int;
    pub fn mifare_desfire_get_key_settings(tag: FreefareTag,
                                           settings: *mut uint8_t,
                                           max_keys: *mut uint8_t)
                                           -> ::std::os::raw::c_int;
    pub fn mifare_desfire_change_key(tag: FreefareTag, key_no: uint8_t,
                                     new_key: MifareDESFireKey,
                                     old_key: MifareDESFireKey)
                                     -> ::std::os::raw::c_int;
    pub fn mifare_desfire_get_key_version(tag: FreefareTag, key_no: uint8_t,
                                          version: *mut uint8_t)
                                          -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_application(tag: FreefareTag,
                                             aid: MifareDESFireAID,
                                             settings: uint8_t,
                                             key_no: uint8_t)
                                             -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_application_3k3des(tag: FreefareTag,
                                                    aid: MifareDESFireAID,
                                                    settings: uint8_t,
                                                    key_no: uint8_t)
                                                    -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_application_aes(tag: FreefareTag,
                                                 aid: MifareDESFireAID,
                                                 settings: uint8_t,
                                                 key_no: uint8_t)
                                                 -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_application_iso(tag: FreefareTag,
                                                 aid: MifareDESFireAID,
                                                 settings: uint8_t,
                                                 key_no: uint8_t,
                                                 want_iso_file_identifiers:
                                                 ::std::os::raw::c_int,
                                                 iso_file_id: uint16_t,
                                                 iso_file_name: *mut uint8_t,
                                                 iso_file_name_len: size_t)
                                                 -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_application_3k3des_iso(tag: FreefareTag,
                                                        aid: MifareDESFireAID,
                                                        settings: uint8_t,
                                                        key_no: uint8_t,
                                                        want_iso_file_identifiers:
                                                        ::std::os::raw::c_int,
                                                        iso_file_id: uint16_t,
                                                        iso_file_name:
                                                        *mut uint8_t,
                                                        iso_file_name_len:
                                                        size_t)
                                                        -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_application_aes_iso(tag: FreefareTag,
                                                     aid: MifareDESFireAID,
                                                     settings: uint8_t,
                                                     key_no: uint8_t,
                                                     want_iso_file_identifiers:
                                                     ::std::os::raw::c_int,
                                                     iso_file_id: uint16_t,
                                                     iso_file_name:
                                                     *mut uint8_t,
                                                     iso_file_name_len:
                                                     size_t)
                                                     -> ::std::os::raw::c_int;
    pub fn mifare_desfire_delete_application(tag: FreefareTag,
                                             aid: MifareDESFireAID)
                                             -> ::std::os::raw::c_int;
    pub fn mifare_desfire_get_application_ids(tag: FreefareTag,
                                              aids:
                                              *mut *mut MifareDESFireAID,
                                              count: *mut size_t)
                                              -> ::std::os::raw::c_int;
    pub fn mifare_desfire_get_df_names(tag: FreefareTag,
                                       dfs: *mut *mut MifareDESFireDF,
                                       count: *mut size_t)
                                       -> ::std::os::raw::c_int;
    pub fn mifare_desfire_free_application_ids(aids: *mut MifareDESFireAID);
    pub fn mifare_desfire_select_application(tag: FreefareTag,
                                             aid: MifareDESFireAID)
                                             -> ::std::os::raw::c_int;
    pub fn mifare_desfire_format_picc(tag: FreefareTag)
        -> ::std::os::raw::c_int;
    pub fn mifare_desfire_get_version(tag: FreefareTag,
                                      version_info:
                                      *mut Struct_mifare_desfire_version_info)
                                      -> ::std::os::raw::c_int;
    pub fn mifare_desfire_free_mem(tag: FreefareTag, size: *mut uint32_t)
        -> ::std::os::raw::c_int;
    pub fn mifare_desfire_set_configuration(tag: FreefareTag,
                                            disable_format: u8,
                                            enable_random_uid: u8)
                                            -> ::std::os::raw::c_int;
    pub fn mifare_desfire_set_default_key(tag: FreefareTag,
                                          key: MifareDESFireKey)
                                          -> ::std::os::raw::c_int;
    pub fn mifare_desfire_set_ats(tag: FreefareTag, ats: *mut uint8_t)
        -> ::std::os::raw::c_int;
    pub fn mifare_desfire_get_card_uid(tag: FreefareTag,
                                       uid: *mut *mut ::std::os::raw::c_char)
                                       -> ::std::os::raw::c_int;
    pub fn mifare_desfire_get_file_ids(tag: FreefareTag,
                                       files: *mut *mut uint8_t,
                                       count: *mut size_t)
                                       -> ::std::os::raw::c_int;
    pub fn mifare_desfire_get_iso_file_ids(tag: FreefareTag,
                                           files: *mut *mut uint16_t,
                                           count: *mut size_t)
                                           -> ::std::os::raw::c_int;
    pub fn mifare_desfire_get_file_settings(tag: FreefareTag,
                                            file_no: uint8_t,
                                            settings:
                                            *mut Struct_mifare_desfire_file_settings)
                                            -> ::std::os::raw::c_int;
    pub fn mifare_desfire_change_file_settings(tag: FreefareTag,
                                               file_no: uint8_t,
                                               communication_settings:
                                               uint8_t,
                                               access_rights: uint16_t)
                                               -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_std_data_file(tag: FreefareTag,
                                               file_no: uint8_t,
                                               communication_settings:
                                               uint8_t,
                                               access_rights: uint16_t,
                                               file_size: uint32_t)
                                               -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_std_data_file_iso(tag: FreefareTag,
                                                   file_no: uint8_t,
                                                   communication_settings:
                                                   uint8_t,
                                                   access_rights: uint16_t,
                                                   file_size: uint32_t,
                                                   iso_file_id: uint16_t)
                                                   -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_backup_data_file(tag: FreefareTag,
                                                  file_no: uint8_t,
                                                  communication_settings:
                                                  uint8_t,
                                                  access_rights: uint16_t,
                                                  file_size: uint32_t)
                                                  -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_backup_data_file_iso(tag: FreefareTag,
                                                      file_no: uint8_t,
                                                      communication_settings:
                                                      uint8_t,
                                                      access_rights: uint16_t,
                                                      file_size: uint32_t,
                                                      iso_file_id: uint16_t)
                                                      -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_value_file(tag: FreefareTag,
                                            file_no: uint8_t,
                                            communication_settings: uint8_t,
                                            access_rights: uint16_t,
                                            lower_limit: int32_t,
                                            upper_limit: int32_t,
                                            value: int32_t,
                                            limited_credit_enable: uint8_t)
                                            -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_linear_record_file(tag: FreefareTag,
                                                    file_no: uint8_t,
                                                    communication_settings:
                                                    uint8_t,
                                                    access_rights: uint16_t,
                                                    record_size: uint32_t,
                                                    max_number_of_records:
                                                    uint32_t)
                                                    -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_linear_record_file_iso(tag: FreefareTag,
                                                        file_no: uint8_t,
                                                        communication_settings:
                                                        uint8_t,
                                                        access_rights:
                                                        uint16_t,
                                                        record_size: uint32_t,
                                                        max_number_of_records:
                                                        uint32_t,
                                                        iso_file_id: uint16_t)
                                                        -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_cyclic_record_file(tag: FreefareTag,
                                                    file_no: uint8_t,
                                                    communication_settings:
                                                    uint8_t,
                                                    access_rights: uint16_t,
                                                    record_size: uint32_t,
                                                    max_number_of_records:
                                                    uint32_t)
                                                    -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_cyclic_record_file_iso(tag: FreefareTag,
                                                        file_no: uint8_t,
                                                        communication_settings:
                                                        uint8_t,
                                                        access_rights:
                                                        uint16_t,
                                                        record_size: uint32_t,
                                                        max_number_of_records:
                                                        uint32_t,
                                                        iso_file_id: uint16_t)
                                                        -> ::std::os::raw::c_int;
    pub fn mifare_desfire_delete_file(tag: FreefareTag, file_no: uint8_t)
        -> ::std::os::raw::c_int;
    pub fn mifare_desfire_read_data(tag: FreefareTag, file_no: uint8_t,
                                    offset: off_t, length: size_t,
                                    data: *mut ::std::os::raw::c_void)
                                    -> ssize_t;
    pub fn mifare_desfire_read_data_ex(tag: FreefareTag, file_no: uint8_t,
                                       offset: off_t, length: size_t,
                                       data: *mut ::std::os::raw::c_void,
                                       cs: ::std::os::raw::c_int) -> ssize_t;
    pub fn mifare_desfire_write_data(tag: FreefareTag, file_no: uint8_t,
                                     offset: off_t, length: size_t,
                                     data: *const ::std::os::raw::c_void)
                                     -> ssize_t;
    pub fn mifare_desfire_write_data_ex(tag: FreefareTag, file_no: uint8_t,
                                        offset: off_t, length: size_t,
                                        data: *const ::std::os::raw::c_void,
                                        cs: ::std::os::raw::c_int) -> ssize_t;
    pub fn mifare_desfire_get_value(tag: FreefareTag, file_no: uint8_t,
                                    value: *mut int32_t)
                                    -> ::std::os::raw::c_int;
    pub fn mifare_desfire_get_value_ex(tag: FreefareTag, file_no: uint8_t,
                                       value: *mut int32_t,
                                       cs: ::std::os::raw::c_int)
                                       -> ::std::os::raw::c_int;
    pub fn mifare_desfire_credit(tag: FreefareTag, file_no: uint8_t,
                                 amount: int32_t) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_credit_ex(tag: FreefareTag, file_no: uint8_t,
                                    amount: int32_t,
                                    cs: ::std::os::raw::c_int)
                                    -> ::std::os::raw::c_int;
    pub fn mifare_desfire_debit(tag: FreefareTag, file_no: uint8_t,
                                amount: int32_t) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_debit_ex(tag: FreefareTag, file_no: uint8_t,
                                   amount: int32_t, cs: ::std::os::raw::c_int)
                                   -> ::std::os::raw::c_int;
    pub fn mifare_desfire_limited_credit(tag: FreefareTag, file_no: uint8_t,
                                         amount: int32_t)
                                         -> ::std::os::raw::c_int;
    pub fn mifare_desfire_limited_credit_ex(tag: FreefareTag,
                                            file_no: uint8_t, amount: int32_t,
                                            cs: ::std::os::raw::c_int)
                                            -> ::std::os::raw::c_int;
    pub fn mifare_desfire_write_record(tag: FreefareTag, file_no: uint8_t,
                                       offset: off_t, length: size_t,
                                       data: *mut ::std::os::raw::c_void)
                                       -> ssize_t;
    pub fn mifare_desfire_write_record_ex(tag: FreefareTag, file_no: uint8_t,
                                          offset: off_t, length: size_t,
                                          data: *mut ::std::os::raw::c_void,
                                          cs: ::std::os::raw::c_int)
                                          -> ssize_t;
    pub fn mifare_desfire_read_records(tag: FreefareTag, file_no: uint8_t,
                                       offset: off_t, length: size_t,
                                       data: *mut ::std::os::raw::c_void)
                                       -> ssize_t;
    pub fn mifare_desfire_read_records_ex(tag: FreefareTag, file_no: uint8_t,
                                          offset: off_t, length: size_t,
                                          data: *mut ::std::os::raw::c_void,
                                          cs: ::std::os::raw::c_int)
                                          -> ssize_t;
    pub fn mifare_desfire_clear_record_file(tag: FreefareTag,
                                            file_no: uint8_t)
                                            -> ::std::os::raw::c_int;
    pub fn mifare_desfire_commit_transaction(tag: FreefareTag)
        -> ::std::os::raw::c_int;
    pub fn mifare_desfire_abort_transaction(tag: FreefareTag)
        -> ::std::os::raw::c_int;
    pub fn mifare_desfire_des_key_new(value: *mut uint8_t)
        -> MifareDESFireKey;
    pub fn mifare_desfire_3des_key_new(value: *mut uint8_t)
        -> MifareDESFireKey;
    pub fn mifare_desfire_des_key_new_with_version(value: *mut uint8_t)
        -> MifareDESFireKey;
    pub fn mifare_desfire_3des_key_new_with_version(value: *mut uint8_t)
        -> MifareDESFireKey;
    pub fn mifare_desfire_3k3des_key_new(value: *mut uint8_t)
        -> MifareDESFireKey;
    pub fn mifare_desfire_3k3des_key_new_with_version(value: *mut uint8_t)
        -> MifareDESFireKey;
    pub fn mifare_desfire_aes_key_new(value: *mut uint8_t)
        -> MifareDESFireKey;
    pub fn mifare_desfire_aes_key_new_with_version(value: *mut uint8_t,
                                                   version: uint8_t)
                                                   -> MifareDESFireKey;
    pub fn mifare_desfire_key_get_version(key: MifareDESFireKey) -> uint8_t;
    pub fn mifare_desfire_key_set_version(key: MifareDESFireKey,
                                          version: uint8_t);
    pub fn mifare_desfire_key_free(key: MifareDESFireKey);
    pub fn tlv_encode(_type: uint8_t, istream: *const uint8_t,
                      isize: uint16_t, osize: *mut size_t) -> *mut uint8_t;
    pub fn tlv_decode(istream: *const uint8_t, _type: *mut uint8_t,
                      size: *mut uint16_t) -> *mut uint8_t;
    pub fn tlv_record_length(istream: *const uint8_t,
                             field_length_size: *mut size_t,
                             field_value_size: *mut size_t) -> size_t;
    pub fn tlv_append(a: *mut uint8_t, b: *mut uint8_t) -> *mut uint8_t;
}
