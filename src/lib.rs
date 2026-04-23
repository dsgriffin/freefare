#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

/// Error and result types used by the high-level wrapper API.
pub mod error;

/// Re-exports the raw `freefare-sys` API for lower-level interop.
pub mod ffi {
    pub use freefare_sys::*;
}

pub use crate::error::{Error, Result};
pub use crate::freefare::Freefare;
pub use crate::mad::Mad;
pub use crate::tlv::TLV;

use libc::{c_char, c_void, free};
use std::ffi::CStr;
use std::marker::PhantomData;
use std::ptr::{self, NonNull};
use std::rc::Rc;

/// Legacy compatibility module for tag discovery helpers.
#[doc(hidden)]
pub mod freefare;
/// Wrappers around the Mifare Application Directory (MAD) API.
pub mod mad;
/// Lower-level compatibility wrappers around Mifare Classic, Ultralight, and
/// DESFire operations.
#[doc(hidden)]
pub mod mifare;
/// Helpers for working with TLV-encoded data used by libfreefare.
pub mod tlv;

/// Owns a discovered libfreefare tag and releases it with `freefare_free_tag`.
///
/// A `Tag` borrows the parent [`nfc::Device`] it came from so callers cannot
/// accidentally outlive the underlying libnfc device session.
pub struct Tag<'device> {
    raw: NonNull<ffi::mifare_tag>,
    _device: PhantomData<&'device ()>,
    _not_send_sync: PhantomData<Rc<()>>,
}

/// Owns a DESFire key allocated by libfreefare.
pub struct DesfireKey {
    raw: NonNull<ffi::mifare_desfire_key>,
    _not_send_sync: PhantomData<Rc<()>>,
}

impl Tag<'_> {
    pub(crate) fn from_raw(raw: ffi::FreefareTag) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self {
            raw,
            _device: PhantomData,
            _not_send_sync: PhantomData,
        })
    }

    /// Returns the borrowed raw libfreefare tag pointer for advanced FFI interop.
    pub fn as_ptr(&self) -> ffi::FreefareTag {
        self.raw.as_ptr()
    }

    /// Returns the native libfreefare tag type.
    pub fn tag_type(&self) -> ffi::FreefareTagType {
        unsafe { ffi::freefare_get_tag_type(self.raw.as_ptr()) }
    }

    /// Returns the human-readable tag name reported by libfreefare.
    pub fn friendly_name(&self) -> Result<String> {
        let name_ptr = unsafe { ffi::freefare_get_tag_friendly_name(self.raw.as_ptr()) };
        if name_ptr.is_null() {
            Err(Error::new("Failed to get tag friendly name."))
        } else {
            Ok(copy_c_string(name_ptr))
        }
    }

    /// Returns the tag UID as an owned string.
    pub fn uid(&self) -> Result<String> {
        let uid_ptr = unsafe { ffi::freefare_get_tag_uid(self.raw.as_ptr()) };
        if uid_ptr.is_null() {
            Err(Error::new("Failed to get tag UID."))
        } else {
            copy_malloc_c_string(uid_ptr)
        }
    }

    /// Returns the most recent libfreefare error message for this tag.
    pub fn error_message(&self) -> Result<String> {
        let err_ptr = unsafe { ffi::freefare_strerror(self.raw.as_ptr()) };
        if err_ptr.is_null() {
            Err(Error::new("No error message available."))
        } else {
            Ok(copy_c_string(err_ptr))
        }
    }

    /// Writes the most recent libfreefare error string into `buffer`.
    pub fn write_error_message(&self, buffer: &mut [u8]) -> Result<()> {
        let result = unsafe {
            ffi::freefare_strerror_r(
                self.raw.as_ptr(),
                buffer.as_mut_ptr().cast::<c_char>(),
                buffer.len(),
            )
        };

        check_status(
            self.raw.as_ptr(),
            result,
            "Failed to write error string to buffer.",
        )
    }

    /// Prints the most recent libfreefare error with additional context.
    pub fn perror(&self, message: &str) {
        if let Ok(message) = std::ffi::CString::new(message) {
            unsafe { ffi::freefare_perror(self.raw.as_ptr(), message.as_ptr()) };
        }
    }

    /// Connects the tag for Mifare Ultralight operations.
    pub fn ultralight_connect(&self) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_ultralight_connect(self.raw.as_ptr()) },
            "Failed to connect to Mifare Ultralight tag.",
        )
    }

    /// Disconnects the tag from Mifare Ultralight operations.
    pub fn ultralight_disconnect(&self) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_ultralight_disconnect(self.raw.as_ptr()) },
            "Failed to disconnect from Mifare Ultralight tag.",
        )
    }

    /// Reads one Ultralight page into `buffer`.
    pub fn ultralight_read(
        &self,
        page: ffi::MifareUltralightPageNumber,
        buffer: &mut ffi::MifareUltralightPage,
    ) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_ultralight_read(self.raw.as_ptr(), page, buffer as *mut _) },
            "Failed to read from Mifare Ultralight tag.",
        )
    }

    /// Writes one Ultralight page from `data`.
    pub fn ultralight_write(
        &self,
        page: ffi::MifareUltralightPageNumber,
        data: &ffi::MifareUltralightPage,
    ) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_ultralight_write(self.raw.as_ptr(), page, data as *const _) },
            "Failed to write to Mifare Ultralight tag.",
        )
    }

    /// Authenticates an Ultralight C tag using a DESFire key.
    pub fn ultralightc_authenticate(&self, key: &DesfireKey) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_ultralightc_authenticate(self.raw.as_ptr(), key.as_ptr()) },
            "Failed to authenticate Mifare Ultralight C tag.",
        )
    }

    /// Connects the tag for Mifare Classic operations.
    pub fn classic_connect(&self) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_classic_connect(self.raw.as_ptr()) },
            "Failed to connect to Mifare Classic tag.",
        )
    }

    /// Disconnects the tag from Mifare Classic operations.
    pub fn classic_disconnect(&self) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_classic_disconnect(self.raw.as_ptr()) },
            "Failed to disconnect from Mifare Classic tag.",
        )
    }

    /// Authenticates a Classic block.
    pub fn classic_authenticate(
        &self,
        block: ffi::MifareClassicBlockNumber,
        key: &ffi::MifareClassicKey,
        key_type: ffi::MifareClassicKeyType,
    ) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe {
                ffi::mifare_classic_authenticate(
                    self.raw.as_ptr(),
                    block,
                    key as *const _,
                    key_type,
                )
            },
            "Failed to authenticate Mifare Classic tag.",
        )
    }

    /// Reads one Classic block into `buffer`.
    pub fn classic_read(
        &self,
        block: ffi::MifareClassicBlockNumber,
        buffer: &mut ffi::MifareClassicBlock,
    ) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_classic_read(self.raw.as_ptr(), block, buffer as *mut _) },
            "Failed to read from Mifare Classic block.",
        )
    }

    /// Writes one Classic block from `data`.
    pub fn classic_write(
        &self,
        block: ffi::MifareClassicBlockNumber,
        data: &ffi::MifareClassicBlock,
    ) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_classic_write(self.raw.as_ptr(), block, data as *const _) },
            "Failed to write to Mifare Classic block.",
        )
    }

    /// Initializes a Classic value block.
    pub fn classic_init_value(
        &self,
        block: ffi::MifareClassicBlockNumber,
        value: i32,
        adr: ffi::MifareClassicBlockNumber,
    ) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_classic_init_value(self.raw.as_ptr(), block, value, adr) },
            "Failed to initialize Mifare Classic block as value block.",
        )
    }

    /// Reads a Classic value block.
    pub fn classic_read_value(
        &self,
        block: ffi::MifareClassicBlockNumber,
    ) -> Result<(i32, ffi::MifareClassicBlockNumber)> {
        let mut value = 0;
        let mut adr = 0;
        check_status(
            self.raw.as_ptr(),
            unsafe {
                ffi::mifare_classic_read_value(self.raw.as_ptr(), block, &mut value, &mut adr)
            },
            "Failed to read value from Mifare Classic block.",
        )?;
        Ok((value, adr))
    }

    /// Increments a Classic value block.
    pub fn classic_increment(
        &self,
        block: ffi::MifareClassicBlockNumber,
        amount: u32,
    ) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_classic_increment(self.raw.as_ptr(), block, amount) },
            "Failed to increment Mifare Classic block.",
        )
    }

    /// Decrements a Classic value block.
    pub fn classic_decrement(
        &self,
        block: ffi::MifareClassicBlockNumber,
        amount: u32,
    ) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_classic_decrement(self.raw.as_ptr(), block, amount) },
            "Failed to decrement Mifare Classic block.",
        )
    }

    /// Restores a Classic value block.
    pub fn classic_restore(&self, block: ffi::MifareClassicBlockNumber) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_classic_restore(self.raw.as_ptr(), block) },
            "Failed to restore Mifare Classic block.",
        )
    }

    /// Transfers a Classic value block.
    pub fn classic_transfer(&self, block: ffi::MifareClassicBlockNumber) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_classic_transfer(self.raw.as_ptr(), block) },
            "Failed to transfer to Mifare Classic block.",
        )
    }

    /// Formats a Classic sector.
    pub fn classic_format_sector(&self, sector: ffi::MifareClassicSectorNumber) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_classic_format_sector(self.raw.as_ptr(), sector) },
            "Failed to format sector.",
        )
    }

    /// Reads application data through MAD.
    pub fn application_read(
        &self,
        mad: &mad::Mad,
        aid: ffi::MadAid,
        buffer: &mut [u8],
        key: &ffi::MifareClassicKey,
        key_type: ffi::MifareClassicKeyType,
    ) -> Result<usize> {
        let result = unsafe {
            ffi::mifare_application_read(
                self.raw.as_ptr(),
                mad.as_ptr(),
                aid,
                buffer.as_mut_ptr().cast::<c_void>(),
                buffer.len(),
                key as *const _,
                key_type,
            )
        };
        if result < 0 {
            Err(Error::from_tag(
                self.raw.as_ptr(),
                result as i32,
                "Failed to read application data.",
            ))
        } else {
            Ok(result as usize)
        }
    }

    /// Writes application data through MAD.
    pub fn application_write(
        &self,
        mad: &mad::Mad,
        aid: ffi::MadAid,
        buffer: &[u8],
        key: &ffi::MifareClassicKey,
        key_type: ffi::MifareClassicKeyType,
    ) -> Result<usize> {
        let result = unsafe {
            ffi::mifare_application_write(
                self.raw.as_ptr(),
                mad.as_ptr(),
                aid,
                buffer.as_ptr().cast::<c_void>(),
                buffer.len(),
                key as *const _,
                key_type,
            )
        };
        if result < 0 {
            Err(Error::from_tag(
                self.raw.as_ptr(),
                result as i32,
                "Failed to write application data.",
            ))
        } else {
            Ok(result as usize)
        }
    }

    /// Connects the tag for Mifare DESFire operations.
    pub fn desfire_connect(&self) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_desfire_connect(self.raw.as_ptr()) },
            "Failed to connect to Mifare DESFire tag.",
        )
    }

    /// Disconnects the tag from Mifare DESFire operations.
    pub fn desfire_disconnect(&self) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_desfire_disconnect(self.raw.as_ptr()) },
            "Failed to disconnect from Mifare DESFire tag.",
        )
    }

    /// Authenticates with a DESFire tag using a legacy key mode.
    pub fn desfire_authenticate(&self, key_no: u8, key: &DesfireKey) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_desfire_authenticate(self.raw.as_ptr(), key_no, key.as_ptr()) },
            "Failed to authenticate with Mifare DESFire tag.",
        )
    }

    /// Authenticates with a DESFire tag using ISO authentication.
    pub fn desfire_authenticate_iso(&self, key_no: u8, key: &DesfireKey) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe {
                ffi::mifare_desfire_authenticate_iso(self.raw.as_ptr(), key_no, key.as_ptr())
            },
            "Failed to authenticate with Mifare DESFire tag using ISO.",
        )
    }

    /// Authenticates with a DESFire tag using AES authentication.
    pub fn desfire_authenticate_aes(&self, key_no: u8, key: &DesfireKey) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe {
                ffi::mifare_desfire_authenticate_aes(self.raw.as_ptr(), key_no, key.as_ptr())
            },
            "Failed to authenticate with Mifare DESFire tag using AES.",
        )
    }

    /// Returns the most recent DESFire PCD error byte.
    pub fn desfire_last_pcd_error(&self) -> u8 {
        unsafe { ffi::mifare_desfire_last_pcd_error(self.raw.as_ptr()) }
    }

    /// Returns the most recent DESFire PICC error byte.
    pub fn desfire_last_picc_error(&self) -> u8 {
        unsafe { ffi::mifare_desfire_last_picc_error(self.raw.as_ptr()) }
    }

    /// Returns DESFire key settings and maximum key count.
    pub fn desfire_key_settings(&self) -> Result<(u8, u8)> {
        let mut settings = 0;
        let mut max_keys = 0;
        check_status(
            self.raw.as_ptr(),
            unsafe {
                ffi::mifare_desfire_get_key_settings(
                    self.raw.as_ptr(),
                    &mut settings,
                    &mut max_keys,
                )
            },
            "Failed to retrieve key settings.",
        )?;
        Ok((settings, max_keys))
    }

    /// Changes DESFire key settings.
    pub fn desfire_change_key_settings(&self, settings: u8) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_desfire_change_key_settings(self.raw.as_ptr(), settings) },
            "Failed to change key settings.",
        )
    }

    /// Returns the version byte for a DESFire key number.
    pub fn desfire_get_key_version(&self, key_no: u8) -> Result<u8> {
        let mut version = 0;
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_desfire_get_key_version(self.raw.as_ptr(), key_no, &mut version) },
            "Failed to retrieve key version.",
        )?;
        Ok(version)
    }

    /// Changes a DESFire key.
    pub fn desfire_change_key(
        &self,
        key_no: u8,
        new_key: &DesfireKey,
        old_key: &DesfireKey,
    ) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe {
                ffi::mifare_desfire_change_key(
                    self.raw.as_ptr(),
                    key_no,
                    new_key.as_ptr(),
                    old_key.as_ptr(),
                )
            },
            "Failed to change key.",
        )
    }

    /// Formats the DESFire PICC.
    pub fn desfire_format_picc(&self) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_desfire_format_picc(self.raw.as_ptr()) },
            "Failed to format PICC.",
        )
    }

    /// Returns DESFire version information.
    pub fn desfire_version(&self) -> Result<ffi::MifareDESFireVersionInfo> {
        let mut version = std::mem::MaybeUninit::uninit();
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_desfire_get_version(self.raw.as_ptr(), version.as_mut_ptr()) },
            "Failed to retrieve version information.",
        )?;
        Ok(unsafe { version.assume_init() })
    }

    /// Returns the available free DESFire memory in bytes.
    pub fn desfire_free_mem(&self) -> Result<u32> {
        let mut size = 0;
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_desfire_free_mem(self.raw.as_ptr(), &mut size) },
            "Failed to retrieve free memory.",
        )?;
        Ok(size)
    }

    /// Sets DESFire PICC configuration.
    pub fn desfire_set_configuration(
        &self,
        disable_format: bool,
        enable_random_uid: bool,
    ) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe {
                ffi::mifare_desfire_set_configuration(
                    self.raw.as_ptr(),
                    disable_format,
                    enable_random_uid,
                )
            },
            "Failed to set configuration.",
        )
    }

    /// Sets the default DESFire key.
    pub fn desfire_set_default_key(&self, key: &DesfireKey) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_desfire_set_default_key(self.raw.as_ptr(), key.as_ptr()) },
            "Failed to set default key.",
        )
    }

    /// Sets the DESFire ATS bytes.
    pub fn desfire_set_ats(&self, ats: &mut [u8]) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_desfire_set_ats(self.raw.as_ptr(), ats.as_mut_ptr()) },
            "Failed to set ATS.",
        )
    }

    /// Returns the DESFire card UID.
    pub fn desfire_card_uid(&self) -> Result<String> {
        let mut uid_ptr = ptr::null_mut();
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_desfire_get_card_uid(self.raw.as_ptr(), &mut uid_ptr) },
            "Failed to retrieve card UID.",
        )?;
        copy_malloc_c_string(uid_ptr)
    }

    /// Returns the DESFire file IDs on the selected application.
    pub fn desfire_file_ids(&self) -> Result<Vec<u8>> {
        let mut files_ptr = ptr::null_mut();
        let mut count = 0;
        check_status(
            self.raw.as_ptr(),
            unsafe {
                ffi::mifare_desfire_get_file_ids(self.raw.as_ptr(), &mut files_ptr, &mut count)
            },
            "Failed to retrieve file IDs.",
        )?;
        copy_malloc_array(files_ptr, count)
    }

    /// Returns the DESFire ISO file IDs on the selected application.
    pub fn desfire_iso_file_ids(&self) -> Result<Vec<u16>> {
        let mut files_ptr = ptr::null_mut();
        let mut count = 0;
        check_status(
            self.raw.as_ptr(),
            unsafe {
                ffi::mifare_desfire_get_iso_file_ids(self.raw.as_ptr(), &mut files_ptr, &mut count)
            },
            "Failed to retrieve ISO file IDs.",
        )?;
        copy_malloc_array(files_ptr, count)
    }

    /// Returns DESFire file settings.
    pub fn desfire_file_settings(&self, file_no: u8) -> Result<ffi::MifareDESFireFileSettings> {
        let mut settings = std::mem::MaybeUninit::uninit();
        check_status(
            self.raw.as_ptr(),
            unsafe {
                ffi::mifare_desfire_get_file_settings(
                    self.raw.as_ptr(),
                    file_no,
                    settings.as_mut_ptr(),
                )
            },
            "Failed to retrieve file settings.",
        )?;
        Ok(unsafe { settings.assume_init() })
    }

    /// Reads data from a DESFire file.
    pub fn desfire_read_data(
        &self,
        file_no: u8,
        offset: libc::off_t,
        length: usize,
        buffer: &mut [u8],
    ) -> Result<isize> {
        if buffer.len() < length {
            return Err(Error::new("Buffer is too small for the specified length."));
        }

        let result = unsafe {
            ffi::mifare_desfire_read_data(
                self.raw.as_ptr(),
                file_no,
                offset,
                length,
                buffer.as_mut_ptr().cast::<c_void>(),
            )
        };

        if result < 0 {
            Err(Error::from_tag(
                self.raw.as_ptr(),
                result as i32,
                "Failed to read data.",
            ))
        } else {
            Ok(result)
        }
    }

    /// Writes data to a DESFire file.
    pub fn desfire_write_data(
        &self,
        file_no: u8,
        offset: libc::off_t,
        length: usize,
        data: &[u8],
    ) -> Result<isize> {
        if data.len() < length {
            return Err(Error::new(
                "Provided data length is smaller than specified.",
            ));
        }

        let result = unsafe {
            ffi::mifare_desfire_write_data(
                self.raw.as_ptr(),
                file_no,
                offset,
                length,
                data.as_ptr().cast::<c_void>(),
            )
        };

        if result < 0 {
            Err(Error::from_tag(
                self.raw.as_ptr(),
                result as i32,
                "Failed to write data.",
            ))
        } else {
            Ok(result)
        }
    }

    /// Returns the current value stored in a DESFire value file.
    pub fn desfire_get_value(&self, file_no: u8) -> Result<i32> {
        let mut value = 0;
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_desfire_get_value(self.raw.as_ptr(), file_no, &mut value) },
            "Failed to get value.",
        )?;
        Ok(value)
    }

    /// Credits a DESFire value file.
    pub fn desfire_credit(&self, file_no: u8, amount: i32) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_desfire_credit(self.raw.as_ptr(), file_no, amount) },
            "Failed to credit amount.",
        )
    }

    /// Debits a DESFire value file.
    pub fn desfire_debit(&self, file_no: u8, amount: i32) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_desfire_debit(self.raw.as_ptr(), file_no, amount) },
            "Failed to debit amount.",
        )
    }

    /// Commits a DESFire transaction.
    pub fn desfire_commit_transaction(&self) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_desfire_commit_transaction(self.raw.as_ptr()) },
            "Failed to commit transaction.",
        )
    }

    /// Aborts a DESFire transaction.
    pub fn desfire_abort_transaction(&self) -> Result<()> {
        check_status(
            self.raw.as_ptr(),
            unsafe { ffi::mifare_desfire_abort_transaction(self.raw.as_ptr()) },
            "Failed to abort transaction.",
        )
    }
}

impl Drop for Tag<'_> {
    fn drop(&mut self) {
        unsafe {
            ffi::freefare_free_tag(self.raw.as_ptr());
        }
    }
}

impl DesfireKey {
    /// Returns the borrowed raw DESFire key pointer for advanced FFI interop.
    pub fn as_ptr(&self) -> ffi::MifareDESFireKey {
        self.raw.as_ptr()
    }

    fn from_raw(raw: ffi::MifareDESFireKey, failure: &'static str) -> Result<Self> {
        let raw = NonNull::new(raw).ok_or_else(|| Error::new(failure))?;
        Ok(Self {
            raw,
            _not_send_sync: PhantomData,
        })
    }

    /// Creates a DES key.
    pub fn new_des(mut value: [u8; 8]) -> Result<Self> {
        Self::from_raw(
            unsafe { ffi::mifare_desfire_des_key_new(value.as_mut_ptr()) },
            "Failed to create DES key.",
        )
    }

    /// Creates a DES key with embedded version bits.
    pub fn new_des_with_version(mut value: [u8; 8]) -> Result<Self> {
        Self::from_raw(
            unsafe { ffi::mifare_desfire_des_key_new_with_version(value.as_mut_ptr()) },
            "Failed to create DES key with version.",
        )
    }

    /// Creates a 3DES key.
    pub fn new_3des(mut value: [u8; 16]) -> Result<Self> {
        Self::from_raw(
            unsafe { ffi::mifare_desfire_3des_key_new(value.as_mut_ptr()) },
            "Failed to create 3DES key.",
        )
    }

    /// Creates a 3DES key with embedded version bits.
    pub fn new_3des_with_version(mut value: [u8; 16]) -> Result<Self> {
        Self::from_raw(
            unsafe { ffi::mifare_desfire_3des_key_new_with_version(value.as_mut_ptr()) },
            "Failed to create 3DES key with version.",
        )
    }

    /// Creates a 3K3DES key.
    pub fn new_3k3des(mut value: [u8; 24]) -> Result<Self> {
        Self::from_raw(
            unsafe { ffi::mifare_desfire_3k3des_key_new(value.as_mut_ptr()) },
            "Failed to create 3K3DES key.",
        )
    }

    /// Creates a 3K3DES key with embedded version bits.
    pub fn new_3k3des_with_version(mut value: [u8; 24]) -> Result<Self> {
        Self::from_raw(
            unsafe { ffi::mifare_desfire_3k3des_key_new_with_version(value.as_mut_ptr()) },
            "Failed to create 3K3DES key with version.",
        )
    }

    /// Creates an AES key.
    pub fn new_aes(mut value: [u8; 16]) -> Result<Self> {
        Self::from_raw(
            unsafe { ffi::mifare_desfire_aes_key_new(value.as_mut_ptr()) },
            "Failed to create AES key.",
        )
    }

    /// Creates an AES key with an explicit version byte.
    pub fn new_aes_with_version(mut value: [u8; 16], version: u8) -> Result<Self> {
        Self::from_raw(
            unsafe { ffi::mifare_desfire_aes_key_new_with_version(value.as_mut_ptr(), version) },
            "Failed to create AES key with version.",
        )
    }

    /// Returns the DESFire key version byte.
    pub fn version(&self) -> u8 {
        unsafe { ffi::mifare_desfire_key_get_version(self.raw.as_ptr()) }
    }

    /// Sets the DESFire key version byte.
    pub fn set_version(&mut self, version: u8) {
        unsafe { ffi::mifare_desfire_key_set_version(self.raw.as_ptr(), version) };
    }
}

impl Drop for DesfireKey {
    fn drop(&mut self) {
        unsafe {
            ffi::mifare_desfire_key_free(self.raw.as_ptr());
        }
    }
}

fn check_status(tag: ffi::FreefareTag, code: i32, fallback: &'static str) -> Result<()> {
    if code < 0 {
        Err(Error::from_tag(tag, code, fallback))
    } else {
        Ok(())
    }
}

fn copy_c_string(ptr: *const c_char) -> String {
    unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
}

fn copy_malloc_c_string(ptr: *mut c_char) -> Result<String> {
    if ptr.is_null() {
        return Err(Error::new("libfreefare returned a null C string"));
    }

    let result = copy_c_string(ptr);
    unsafe {
        free(ptr.cast::<c_void>());
    }
    Ok(result)
}

fn copy_malloc_array<T: Copy>(ptr: *mut T, len: usize) -> Result<Vec<T>> {
    if ptr.is_null() {
        return Err(Error::new("libfreefare returned a null allocation"));
    }

    let result = unsafe { std::slice::from_raw_parts(ptr, len).to_vec() };
    unsafe {
        free(ptr.cast::<c_void>());
    }
    Ok(result)
}
