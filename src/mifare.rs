#![allow(clippy::missing_safety_doc)]

use crate::{check_status, copy_malloc_array, copy_malloc_c_string, Error, Result};
use ::freefare_sys;
use ::nfc_sys;
use libc::off_t;

/// Namespace for Mifare-related libfreefare helpers.
pub struct Mifare;

impl Mifare {
    /// Connects to a Mifare Ultralight tag
    pub unsafe fn ultralight_connect(tag: freefare_sys::FreefareTag) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot connect to Mifare Ultralight tag.",
            ));
        }

        let result = unsafe { freefare_sys::mifare_ultralight_connect(tag) };
        if result < 0 {
            Err(Error::new("Failed to connect to Mifare Ultralight tag."))
        } else {
            Ok(())
        }
    }

    /// Disconnects from a Mifare Ultralight tag.
    pub unsafe fn ultralight_disconnect(tag: freefare_sys::FreefareTag) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot disconnect from Mifare Ultralight tag.",
            ));
        }

        let result = unsafe { freefare_sys::mifare_ultralight_disconnect(tag) };
        if result < 0 {
            Err(Error::new(
                "Failed to disconnect from Mifare Ultralight tag.",
            ))
        } else {
            Ok(())
        }
    }

    /// Reads data from a page of a Mifare Ultralight tag.
    pub unsafe fn ultralight_read(
        tag: freefare_sys::FreefareTag,
        page: freefare_sys::MifareUltralightPageNumber,
        buffer: &mut freefare_sys::MifareUltralightPage,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot read from Mifare Ultralight tag.",
            ));
        }

        let result = unsafe { freefare_sys::mifare_ultralight_read(tag, page, buffer as *mut _) };
        if result < 0 {
            Err(Error::new("Failed to read from Mifare Ultralight tag."))
        } else {
            Ok(())
        }
    }

    /// Writes data to a page of a Mifare Ultralight tag.
    pub unsafe fn ultralight_write(
        tag: freefare_sys::FreefareTag,
        page: freefare_sys::MifareUltralightPageNumber,
        data: &freefare_sys::MifareUltralightPage,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot write to Mifare Ultralight tag.",
            ));
        }

        let result = unsafe { freefare_sys::mifare_ultralight_write(tag, page, data as *const _) };
        if result < 0 {
            Err(Error::new("Failed to write to Mifare Ultralight tag."))
        } else {
            Ok(())
        }
    }

    /// Authenticates a Mifare Ultralight C tag using a DESFire key.
    pub unsafe fn ultralightc_authenticate(
        tag: freefare_sys::FreefareTag,
        key: freefare_sys::MifareDESFireKey,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot authenticate Mifare Ultralight C tag.",
            ));
        }

        if key.is_null() {
            return Err(Error::new(
                "Key is null. Cannot authenticate Mifare Ultralight C tag.",
            ));
        }

        let result = unsafe { freefare_sys::mifare_ultralightc_authenticate(tag, key) };
        if result < 0 {
            Err(Error::new(
                "Failed to authenticate Mifare Ultralight C tag.",
            ))
        } else {
            Ok(())
        }
    }

    /// Checks if a Mifare Ultralight C tag is present on the NFC reader.
    pub unsafe fn is_mifare_ultralightc_on_reader(
        device: *mut nfc_sys::nfc_device,
        nai: nfc_sys::nfc_iso14443a_info,
    ) -> bool {
        if device.is_null() {
            return false;
        }

        unsafe { freefare_sys::is_mifare_ultralightc_on_reader(device, nai) }
    }

    /// Connects to a Mifare Classic tag.
    pub unsafe fn classic_connect(tag: freefare_sys::FreefareTag) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot connect to Mifare Classic tag.",
            ));
        }

        let result = unsafe { freefare_sys::mifare_classic_connect(tag) };
        if result < 0 {
            Err(Error::new("Failed to connect to Mifare Classic tag."))
        } else {
            Ok(())
        }
    }

    /// Disconnects from a Mifare Classic tag.
    pub unsafe fn classic_disconnect(tag: freefare_sys::FreefareTag) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot disconnect from Mifare Classic tag.",
            ));
        }

        let result = unsafe { freefare_sys::mifare_classic_disconnect(tag) };
        if result < 0 {
            Err(Error::new("Failed to disconnect from Mifare Classic tag."))
        } else {
            Ok(())
        }
    }

    /// Authenticates a Mifare Classic block using a specified key and key type.
    pub unsafe fn classic_authenticate(
        tag: freefare_sys::FreefareTag,
        block: freefare_sys::MifareClassicBlockNumber,
        key: &freefare_sys::MifareClassicKey,
        key_type: freefare_sys::MifareClassicKeyType,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot authenticate Mifare Classic tag.",
            ));
        }

        let result = unsafe {
            freefare_sys::mifare_classic_authenticate(tag, block, key as *const _, key_type)
        };
        if result < 0 {
            Err(Error::new("Failed to authenticate Mifare Classic tag."))
        } else {
            Ok(())
        }
    }

    /// Reads data from a Mifare Classic block.
    pub unsafe fn classic_read(
        tag: freefare_sys::FreefareTag,
        block: freefare_sys::MifareClassicBlockNumber,
        buffer: &mut freefare_sys::MifareClassicBlock,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot read from Mifare Classic block.",
            ));
        }

        let result = unsafe { freefare_sys::mifare_classic_read(tag, block, buffer as *mut _) };
        if result < 0 {
            Err(Error::new("Failed to read from Mifare Classic block."))
        } else {
            Ok(())
        }
    }

    /// Initializes a Mifare Classic block as a value block.
    pub unsafe fn classic_init_value(
        tag: freefare_sys::FreefareTag,
        block: freefare_sys::MifareClassicBlockNumber,
        value: i32,
        adr: freefare_sys::MifareClassicBlockNumber,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot initialize Mifare Classic block as value block.",
            ));
        }

        let result = unsafe { freefare_sys::mifare_classic_init_value(tag, block, value, adr) };
        check_status(
            tag,
            result,
            "Failed to initialize Mifare Classic block as value block.",
        )
    }

    /// Reads a value from a Mifare Classic block.
    pub unsafe fn classic_read_value(
        tag: freefare_sys::FreefareTag,
        block: freefare_sys::MifareClassicBlockNumber,
    ) -> Result<(i32, freefare_sys::MifareClassicBlockNumber)> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot read value from Mifare Classic block.",
            ));
        }

        let mut value: i32 = 0;
        let mut adr: freefare_sys::MifareClassicBlockNumber = 0;
        let result = unsafe {
            freefare_sys::mifare_classic_read_value(tag, block, &mut value as *mut _, &mut adr)
        };

        if result < 0 {
            Err(Error::new(
                "Failed to read value from Mifare Classic block.",
            ))
        } else {
            Ok((value, adr))
        }
    }

    /// Writes data to a Mifare Classic block.
    pub unsafe fn classic_write(
        tag: freefare_sys::FreefareTag,
        block: freefare_sys::MifareClassicBlockNumber,
        data: &freefare_sys::MifareClassicBlock,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot write to Mifare Classic block.",
            ));
        }

        let result = unsafe { freefare_sys::mifare_classic_write(tag, block, data as *const _) };
        if result < 0 {
            Err(Error::new("Failed to write to Mifare Classic block."))
        } else {
            Ok(())
        }
    }

    /// Increments a value in a Mifare Classic block.
    pub unsafe fn classic_increment(
        tag: freefare_sys::FreefareTag,
        block: freefare_sys::MifareClassicBlockNumber,
        amount: u32,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot increment Mifare Classic block.",
            ));
        }

        let result = unsafe { freefare_sys::mifare_classic_increment(tag, block, amount) };
        if result < 0 {
            Err(Error::new("Failed to increment Mifare Classic block."))
        } else {
            Ok(())
        }
    }

    /// Decrements a value in a Mifare Classic block.
    pub unsafe fn classic_decrement(
        tag: freefare_sys::FreefareTag,
        block: freefare_sys::MifareClassicBlockNumber,
        amount: u32,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot decrement Mifare Classic block.",
            ));
        }

        let result = unsafe { freefare_sys::mifare_classic_decrement(tag, block, amount) };
        if result < 0 {
            Err(Error::new("Failed to decrement Mifare Classic block."))
        } else {
            Ok(())
        }
    }

    /// Restores a value from a Mifare Classic block.
    pub unsafe fn classic_restore(
        tag: freefare_sys::FreefareTag,
        block: freefare_sys::MifareClassicBlockNumber,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot restore Mifare Classic block.",
            ));
        }

        let result = unsafe { freefare_sys::mifare_classic_restore(tag, block) };
        if result < 0 {
            Err(Error::new("Failed to restore Mifare Classic block."))
        } else {
            Ok(())
        }
    }

    /// Transfers a value to a Mifare Classic block.
    pub unsafe fn classic_transfer(
        tag: freefare_sys::FreefareTag,
        block: freefare_sys::MifareClassicBlockNumber,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot transfer to Mifare Classic block.",
            ));
        }

        let result = unsafe { freefare_sys::mifare_classic_transfer(tag, block) };
        if result < 0 {
            Err(Error::new("Failed to transfer to Mifare Classic block."))
        } else {
            Ok(())
        }
    }

    /// Gets permissions for a trailer block in a Mifare Classic tag.
    pub unsafe fn classic_get_trailer_block_permission(
        tag: freefare_sys::FreefareTag,
        block: freefare_sys::MifareClassicBlockNumber,
        permission: u16,
        key_type: freefare_sys::MifareClassicKeyType,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot get trailer block permissions.",
            ));
        }

        let result = unsafe {
            freefare_sys::mifare_classic_get_trailer_block_permission(
                tag, block, permission, key_type,
            )
        };
        if result < 0 {
            Err(Error::new("Failed to get trailer block permissions."))
        } else {
            Ok(())
        }
    }

    /// Formats a sector of a Mifare Classic tag.
    pub unsafe fn classic_format_sector(
        tag: freefare_sys::FreefareTag,
        sector: freefare_sys::MifareClassicSectorNumber,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot format sector."));
        }

        let result = unsafe { freefare_sys::mifare_classic_format_sector(tag, sector) };
        if result < 0 {
            Err(Error::new("Failed to format sector."))
        } else {
            Ok(())
        }
    }

    /// Creates a trailer block for a Mifare Classic tag.
    #[allow(clippy::too_many_arguments)]
    pub fn classic_trailer_block(
        block: &mut freefare_sys::MifareClassicBlock,
        key_a: &freefare_sys::MifareClassicKey,
        ab_0: u8,
        ab_1: u8,
        ab_2: u8,
        ab_tb: u8,
        gpb: u8,
        key_b: &freefare_sys::MifareClassicKey,
    ) {
        unsafe {
            freefare_sys::mifare_classic_trailer_block(
                block as *mut _,
                key_a,
                ab_0,
                ab_1,
                ab_2,
                ab_tb,
                gpb,
                key_b,
            )
        }
    }

    /// Gets permissions for a data block in a Mifare Classic tag.
    pub unsafe fn classic_get_data_block_permission(
        tag: freefare_sys::FreefareTag,
        block: freefare_sys::MifareClassicBlockNumber,
        permission: u8,
        key_type: freefare_sys::MifareClassicKeyType,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot get data block permissions.",
            ));
        }

        let result = unsafe {
            freefare_sys::mifare_classic_get_data_block_permission(tag, block, permission, key_type)
        };
        if result < 0 {
            Err(Error::new("Failed to get data block permissions."))
        } else {
            Ok(())
        }
    }

    /// Gets the sector of a Mifare Classic block.
    pub fn classic_block_sector(
        block: freefare_sys::MifareClassicBlockNumber,
    ) -> freefare_sys::MifareClassicSectorNumber {
        unsafe { freefare_sys::mifare_classic_block_sector(block) }
    }

    /// Gets the first block of a sector in a Mifare Classic tag.
    pub fn classic_sector_first_block(
        sector: freefare_sys::MifareClassicSectorNumber,
    ) -> freefare_sys::MifareClassicBlockNumber {
        unsafe { freefare_sys::mifare_classic_sector_first_block(sector) }
    }

    /// Gets the number of blocks in a sector of a Mifare Classic tag.
    pub fn classic_sector_block_count(sector: freefare_sys::MifareClassicSectorNumber) -> usize {
        unsafe { freefare_sys::mifare_classic_sector_block_count(sector) }
    }

    /// Gets the last block of a sector in a Mifare Classic tag.
    pub fn classic_sector_last_block(
        sector: freefare_sys::MifareClassicSectorNumber,
    ) -> freefare_sys::MifareClassicBlockNumber {
        unsafe { freefare_sys::mifare_classic_sector_last_block(sector) }
    }

    /// Reads application data from a Mifare Classic tag.
    pub unsafe fn application_read(
        tag: freefare_sys::FreefareTag,
        mad: freefare_sys::Mad,
        aid: freefare_sys::MadAid,
        buffer: &mut [u8],
        key: &freefare_sys::MifareClassicKey,
        key_type: freefare_sys::MifareClassicKeyType,
    ) -> Result<usize> {
        if tag.is_null() || mad.is_null() {
            return Err(Error::new(
                "Tag or MAD structure is null. Cannot read application.",
            ));
        }

        let result = unsafe {
            freefare_sys::mifare_application_read(
                tag,
                mad,
                aid,
                buffer.as_mut_ptr() as *mut _,
                buffer.len(),
                key,
                key_type,
            )
        };

        if result < 0 {
            Err(Error::new("Failed to read application data."))
        } else {
            Ok(result as usize)
        }
    }

    /// Writes application data to a Mifare Classic tag.
    pub unsafe fn application_write(
        tag: freefare_sys::FreefareTag,
        mad: freefare_sys::Mad,
        aid: freefare_sys::MadAid,
        buffer: &[u8],
        key: &freefare_sys::MifareClassicKey,
        key_type: freefare_sys::MifareClassicKeyType,
    ) -> Result<usize> {
        if tag.is_null() || mad.is_null() {
            return Err(Error::new(
                "Tag or MAD structure is null. Cannot write application.",
            ));
        }

        let result = unsafe {
            freefare_sys::mifare_application_write(
                tag,
                mad,
                aid,
                buffer.as_ptr() as *const _,
                buffer.len(),
                key,
                key_type,
            )
        };

        if result < 0 {
            Err(Error::new("Failed to write application data."))
        } else {
            Ok(result as usize)
        }
    }

    /// Frees an application in a Mifare Classic tag.
    pub unsafe fn application_free(
        mad: freefare_sys::Mad,
        aid: freefare_sys::MadAid,
    ) -> Result<()> {
        if mad.is_null() {
            return Err(Error::new(
                "MAD structure is null. Cannot free application.",
            ));
        }

        unsafe { freefare_sys::mifare_application_free(mad, aid) };
        Ok(())
    }

    /// Creates a new Mifare DESFire AID.
    pub fn desfire_aid_new(aid: u32) -> freefare_sys::MifareDESFireAID {
        unsafe { freefare_sys::mifare_desfire_aid_new(aid) }
    }

    /// Creates a new Mifare DESFire AID with MAD AID.
    pub unsafe fn desfire_aid_new_with_mad_aid(
        mad_aid: freefare_sys::MadAid,
        n: u8,
    ) -> freefare_sys::MifareDESFireAID {
        unsafe { freefare_sys::mifare_desfire_aid_new_with_mad_aid(mad_aid, n) }
    }

    /// Gets the AID value from a Mifare DESFire AID.
    pub unsafe fn desfire_aid_get_aid(aid: freefare_sys::MifareDESFireAID) -> u32 {
        unsafe { freefare_sys::mifare_desfire_aid_get_aid(aid) }
    }

    /// Gets the last PCD error from a Mifare DESFire tag.
    pub unsafe fn desfire_last_pcd_error(tag: freefare_sys::FreefareTag) -> Result<u8> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot retrieve last PCD error."));
        }

        Ok(unsafe { freefare_sys::mifare_desfire_last_pcd_error(tag) })
    }

    /// Gets the last PICC error from a Mifare DESFire tag.
    pub unsafe fn desfire_last_picc_error(tag: freefare_sys::FreefareTag) -> Result<u8> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot retrieve last PICC error."));
        }

        Ok(unsafe { freefare_sys::mifare_desfire_last_picc_error(tag) })
    }

    /// Connects to a Mifare DESFire tag.
    pub unsafe fn desfire_connect(tag: freefare_sys::FreefareTag) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot connect to Mifare DESFire tag.",
            ));
        }

        let result = unsafe { freefare_sys::mifare_desfire_connect(tag) };
        if result < 0 {
            Err(Error::new("Failed to connect to Mifare DESFire tag."))
        } else {
            Ok(())
        }
    }

    /// Disconnects from a Mifare DESFire tag.
    pub unsafe fn desfire_disconnect(tag: freefare_sys::FreefareTag) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot disconnect from Mifare DESFire tag.",
            ));
        }

        let result = unsafe { freefare_sys::mifare_desfire_disconnect(tag) };
        if result < 0 {
            Err(Error::new("Failed to disconnect from Mifare DESFire tag."))
        } else {
            Ok(())
        }
    }

    /// Authenticates with a Mifare DESFire tag using a key.
    pub unsafe fn desfire_authenticate(
        tag: freefare_sys::FreefareTag,
        key_no: u8,
        key: freefare_sys::MifareDESFireKey,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot authenticate with Mifare DESFire tag.",
            ));
        }

        if key.is_null() {
            return Err(Error::new(
                "Key is null. Cannot authenticate with Mifare DESFire tag.",
            ));
        }

        let result = unsafe { freefare_sys::mifare_desfire_authenticate(tag, key_no, key) };
        if result < 0 {
            Err(Error::new(
                "Failed to authenticate with Mifare DESFire tag.",
            ))
        } else {
            Ok(())
        }
    }

    /// Authenticates with a Mifare DESFire tag using ISO authentication.
    pub unsafe fn desfire_authenticate_iso(
        tag: freefare_sys::FreefareTag,
        key_no: u8,
        key: freefare_sys::MifareDESFireKey,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot authenticate with Mifare DESFire tag.",
            ));
        }

        if key.is_null() {
            return Err(Error::new(
                "Key is null. Cannot authenticate with Mifare DESFire tag.",
            ));
        }

        let result = unsafe { freefare_sys::mifare_desfire_authenticate_iso(tag, key_no, key) };
        if result < 0 {
            Err(Error::new(
                "Failed to authenticate with Mifare DESFire tag using ISO.",
            ))
        } else {
            Ok(())
        }
    }

    /// Authenticates with a Mifare DESFire tag using AES.
    pub unsafe fn desfire_authenticate_aes(
        tag: freefare_sys::FreefareTag,
        key_no: u8,
        key: freefare_sys::MifareDESFireKey,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot authenticate with Mifare DESFire tag using AES.",
            ));
        }

        if key.is_null() {
            return Err(Error::new(
                "Key is null. Cannot authenticate with Mifare DESFire tag using AES.",
            ));
        }

        let result = unsafe { freefare_sys::mifare_desfire_authenticate_aes(tag, key_no, key) };
        check_status(
            tag,
            result,
            "Failed to authenticate with Mifare DESFire tag using AES.",
        )
    }

    /// Changes the key settings of a Mifare DESFire tag.
    pub unsafe fn desfire_change_key_settings(
        tag: freefare_sys::FreefareTag,
        settings: u8,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot change key settings."));
        }

        let result = unsafe { freefare_sys::mifare_desfire_change_key_settings(tag, settings) };
        if result < 0 {
            Err(Error::new("Failed to change key settings."))
        } else {
            Ok(())
        }
    }

    /// Retrieves the key settings of a Mifare DESFire tag.
    pub unsafe fn desfire_get_key_settings(tag: freefare_sys::FreefareTag) -> Result<(u8, u8)> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot retrieve key settings."));
        }

        let mut settings: u8 = 0;
        let mut max_keys: u8 = 0;

        let result = unsafe {
            freefare_sys::mifare_desfire_get_key_settings(
                tag,
                &mut settings as *mut _,
                &mut max_keys as *mut _,
            )
        };

        if result < 0 {
            Err(Error::new("Failed to retrieve key settings."))
        } else {
            Ok((settings, max_keys))
        }
    }

    /// Changes a key on a Mifare DESFire tag.
    pub unsafe fn desfire_change_key(
        tag: freefare_sys::FreefareTag,
        key_no: u8,
        new_key: freefare_sys::MifareDESFireKey,
        old_key: freefare_sys::MifareDESFireKey,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot change key."));
        }

        let result =
            unsafe { freefare_sys::mifare_desfire_change_key(tag, key_no, new_key, old_key) };

        if result < 0 {
            Err(Error::new("Failed to change key."))
        } else {
            Ok(())
        }
    }

    /// Retrieves the version of a key on a Mifare DESFire tag.
    pub unsafe fn desfire_get_key_version(
        tag: freefare_sys::FreefareTag,
        key_no: u8,
    ) -> Result<u8> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot retrieve key version."));
        }

        let mut version: u8 = 0;

        let result = unsafe {
            freefare_sys::mifare_desfire_get_key_version(tag, key_no, &mut version as *mut _)
        };

        if result < 0 {
            Err(Error::new("Failed to retrieve key version."))
        } else {
            Ok(version)
        }
    }

    /// Creates an application on a Mifare DESFire tag.
    pub unsafe fn desfire_create_application(
        tag: freefare_sys::FreefareTag,
        aid: freefare_sys::MifareDESFireAID,
        settings: u8,
        key_no: u8,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot create application."));
        }

        let result =
            unsafe { freefare_sys::mifare_desfire_create_application(tag, aid, settings, key_no) };

        if result < 0 {
            Err(Error::new("Failed to create application."))
        } else {
            Ok(())
        }
    }

    /// Deletes an application on a Mifare DESFire tag.
    pub unsafe fn desfire_delete_application(
        tag: freefare_sys::FreefareTag,
        aid: freefare_sys::MifareDESFireAID,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot delete application."));
        }

        let result = unsafe { freefare_sys::mifare_desfire_delete_application(tag, aid) };

        if result < 0 {
            Err(Error::new("Failed to delete application."))
        } else {
            Ok(())
        }
    }

    /// Retrieves the application IDs on a Mifare DESFire tag.
    pub unsafe fn desfire_get_application_ids(
        tag: freefare_sys::FreefareTag,
    ) -> Result<Vec<freefare_sys::MifareDESFireAID>> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot retrieve application IDs."));
        }

        let mut aids: *mut freefare_sys::MifareDESFireAID = std::ptr::null_mut();
        let mut count: usize = 0;

        let result = unsafe {
            freefare_sys::mifare_desfire_get_application_ids(
                tag,
                &mut aids as *mut _,
                &mut count as *mut _,
            )
        };

        if result < 0 {
            return Err(Error::new("Failed to retrieve application IDs."));
        }

        let aid_vec = unsafe { std::slice::from_raw_parts(aids, count).to_vec() };
        unsafe { freefare_sys::mifare_desfire_free_application_ids(aids) };
        Ok(aid_vec)
    }

    /// Creates a Mifare DESFire application with 3K3DES encryption.
    pub unsafe fn desfire_create_application_3k3des(
        tag: freefare_sys::FreefareTag,
        aid: freefare_sys::MifareDESFireAID,
        settings: u8,
        key_no: u8,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot create 3K3DES application."));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_create_application_3k3des(tag, aid, settings, key_no)
        };

        if result < 0 {
            Err(Error::new("Failed to create 3K3DES application."))
        } else {
            Ok(())
        }
    }

    /// Creates a Mifare DESFire application with AES encryption.
    pub unsafe fn desfire_create_application_aes(
        tag: freefare_sys::FreefareTag,
        aid: freefare_sys::MifareDESFireAID,
        settings: u8,
        key_no: u8,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot create AES application."));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_create_application_aes(tag, aid, settings, key_no)
        };

        if result < 0 {
            Err(Error::new("Failed to create AES application."))
        } else {
            Ok(())
        }
    }

    /// Creates a Mifare DESFire application with ISO parameters.
    pub unsafe fn desfire_create_application_iso(
        tag: freefare_sys::FreefareTag,
        aid: freefare_sys::MifareDESFireAID,
        settings: u8,
        key_no: u8,
        want_iso_file_identifiers: bool,
        iso_file_id: u16,
        iso_file_name: &mut [u8],
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot create ISO application."));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_create_application_iso(
                tag,
                aid,
                settings,
                key_no,
                want_iso_file_identifiers as ::std::os::raw::c_int,
                iso_file_id,
                iso_file_name.as_mut_ptr(),
                iso_file_name.len(),
            )
        };

        if result < 0 {
            Err(Error::new("Failed to create ISO application."))
        } else {
            Ok(())
        }
    }

    /// Gets the data file names of a Mifare DESFire tag.
    pub unsafe fn desfire_get_df_names(
        tag: freefare_sys::FreefareTag,
    ) -> Result<Vec<freefare_sys::MifareDESFireDF>> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot retrieve data file names."));
        }

        let mut dfs_ptr: *mut freefare_sys::MifareDESFireDF = std::ptr::null_mut();
        let mut count: usize = 0;

        let result = unsafe {
            freefare_sys::mifare_desfire_get_df_names(
                tag,
                &mut dfs_ptr as *mut _,
                &mut count as *mut _,
            )
        };

        if result < 0 {
            return Err(Error::new("Failed to retrieve data file names."));
        }

        copy_malloc_array(dfs_ptr, count)
    }

    /// Selects an application on a Mifare DESFire tag.
    pub unsafe fn desfire_select_application(
        tag: freefare_sys::FreefareTag,
        aid: freefare_sys::MifareDESFireAID,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot select application."));
        }

        let result = unsafe { freefare_sys::mifare_desfire_select_application(tag, aid) };

        if result < 0 {
            Err(Error::new("Failed to select application."))
        } else {
            Ok(())
        }
    }

    /// Formats the PICC of a Mifare DESFire tag.
    pub unsafe fn desfire_format_picc(tag: freefare_sys::FreefareTag) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot format PICC."));
        }

        let result = unsafe { freefare_sys::mifare_desfire_format_picc(tag) };

        if result < 0 {
            Err(Error::new("Failed to format PICC."))
        } else {
            Ok(())
        }
    }

    /// Gets the version information of a Mifare DESFire tag
    pub unsafe fn desfire_get_version(
        tag: freefare_sys::FreefareTag,
    ) -> Result<freefare_sys::MifareDESFireVersionInfo> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot retrieve version information.",
            ));
        }

        let mut version_info =
            std::mem::MaybeUninit::<freefare_sys::MifareDESFireVersionInfo>::uninit();

        let result =
            unsafe { freefare_sys::mifare_desfire_get_version(tag, version_info.as_mut_ptr()) };

        if result < 0 {
            Err(Error::new("Failed to retrieve version information."))
        } else {
            Ok(unsafe { version_info.assume_init() })
        }
    }

    /// Creates a new 3DES key for Mifare DESFire
    pub fn desfire_3des_key_new(value: &mut [u8]) -> Result<freefare_sys::MifareDESFireKey> {
        if value.len() != 16 {
            return Err(Error::new("3DES key must be 16 bytes."));
        }

        let key = unsafe { freefare_sys::mifare_desfire_3des_key_new(value.as_mut_ptr()) };

        if key.is_null() {
            Err(Error::new("Failed to create 3DES key."))
        } else {
            Ok(key)
        }
    }

    /// Creates a new DES key with a version for Mifare DESFire
    pub fn desfire_des_key_new_with_version(
        value: &mut [u8],
    ) -> Result<freefare_sys::MifareDESFireKey> {
        if value.len() != 8 {
            return Err(Error::new("DES key must be 8 bytes."));
        }

        let key =
            unsafe { freefare_sys::mifare_desfire_des_key_new_with_version(value.as_mut_ptr()) };

        if key.is_null() {
            Err(Error::new("Failed to create DES key with version."))
        } else {
            Ok(key)
        }
    }

    /// Creates a new 3DES key with a version for Mifare DESFire
    pub fn desfire_3des_key_new_with_version(
        value: &mut [u8],
    ) -> Result<freefare_sys::MifareDESFireKey> {
        if value.len() != 16 {
            return Err(Error::new("3DES key must be 16 bytes."));
        }

        let key =
            unsafe { freefare_sys::mifare_desfire_3des_key_new_with_version(value.as_mut_ptr()) };

        if key.is_null() {
            Err(Error::new("Failed to create 3DES key with version."))
        } else {
            Ok(key)
        }
    }

    /// Creates a new 3K3DES key for Mifare DESFire
    pub fn desfire_3k3des_key_new(value: &mut [u8]) -> Result<freefare_sys::MifareDESFireKey> {
        if value.len() != 24 {
            return Err(Error::new("3K3DES key must be 24 bytes."));
        }

        let key = unsafe { freefare_sys::mifare_desfire_3k3des_key_new(value.as_mut_ptr()) };

        if key.is_null() {
            Err(Error::new("Failed to create 3K3DES key."))
        } else {
            Ok(key)
        }
    }

    /// Creates a new 3K3DES key with a version for Mifare DESFire
    pub fn desfire_3k3des_key_new_with_version(
        value: &mut [u8],
    ) -> Result<freefare_sys::MifareDESFireKey> {
        if value.len() != 24 {
            return Err(Error::new("3K3DES key must be 24 bytes."));
        }

        let key =
            unsafe { freefare_sys::mifare_desfire_3k3des_key_new_with_version(value.as_mut_ptr()) };

        if key.is_null() {
            Err(Error::new("Failed to create 3K3DES key with version."))
        } else {
            Ok(key)
        }
    }

    /// Creates a new AES key for Mifare DESFire
    pub fn desfire_aes_key_new(value: &mut [u8]) -> Result<freefare_sys::MifareDESFireKey> {
        if value.len() != 16 {
            return Err(Error::new("AES key must be 16 bytes."));
        }

        let key = unsafe { freefare_sys::mifare_desfire_aes_key_new(value.as_mut_ptr()) };

        if key.is_null() {
            Err(Error::new("Failed to create AES key."))
        } else {
            Ok(key)
        }
    }

    /// Creates a new AES key with a version for Mifare DESFire
    pub fn desfire_aes_key_new_with_version(
        value: &mut [u8],
        version: u8,
    ) -> Result<freefare_sys::MifareDESFireKey> {
        if value.len() != 16 {
            return Err(Error::new("AES key must be 16 bytes."));
        }

        let key = unsafe {
            freefare_sys::mifare_desfire_aes_key_new_with_version(value.as_mut_ptr(), version)
        };

        if key.is_null() {
            Err(Error::new("Failed to create AES key with version."))
        } else {
            Ok(key)
        }
    }

    /// Gets the version of a Mifare DESFire key
    pub unsafe fn desfire_key_get_version(key: freefare_sys::MifareDESFireKey) -> Result<u8> {
        if key.is_null() {
            return Err(Error::new("Key is null. Cannot get version."));
        }

        Ok(unsafe { freefare_sys::mifare_desfire_key_get_version(key) })
    }

    /// Sets the version of a Mifare DESFire key
    pub unsafe fn desfire_key_set_version(
        key: freefare_sys::MifareDESFireKey,
        version: u8,
    ) -> Result<()> {
        if key.is_null() {
            return Err(Error::new("Key is null. Cannot set version."));
        }

        unsafe { freefare_sys::mifare_desfire_key_set_version(key, version) };

        Ok(())
    }

    /// Frees a Mifare DESFire key.
    pub unsafe fn desfire_key_free(key: freefare_sys::MifareDESFireKey) -> Result<()> {
        if key.is_null() {
            return Err(Error::new("Key is null. Cannot free key."));
        }

        unsafe { freefare_sys::mifare_desfire_key_free(key) };

        Ok(())
    }

    /// Debits an amount from a file with communication settings
    pub unsafe fn desfire_debit_ex(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        amount: i32,
        cs: ::std::os::raw::c_int,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot perform debit."));
        }

        let result = unsafe { freefare_sys::mifare_desfire_debit_ex(tag, file_no, amount, cs) };

        if result < 0 {
            Err(Error::new("Failed to debit the file."))
        } else {
            Ok(())
        }
    }

    /// Performs a limited credit operation on a file
    pub unsafe fn desfire_limited_credit(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        amount: i32,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot perform limited credit."));
        }

        let result = unsafe { freefare_sys::mifare_desfire_limited_credit(tag, file_no, amount) };

        if result < 0 {
            Err(Error::new("Failed to perform limited credit."))
        } else {
            Ok(())
        }
    }

    /// Performs a limited credit operation on a file with communication settings
    pub unsafe fn desfire_limited_credit_ex(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        amount: i32,
        cs: ::std::os::raw::c_int,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot perform limited credit."));
        }

        let result =
            unsafe { freefare_sys::mifare_desfire_limited_credit_ex(tag, file_no, amount, cs) };

        if result < 0 {
            Err(Error::new(
                "Failed to perform limited credit with communication settings.",
            ))
        } else {
            Ok(())
        }
    }

    /// Writes a record to a file
    pub unsafe fn desfire_write_record(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        offset: off_t,
        length: usize,
        data: &[u8],
    ) -> Result<isize> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot write record."));
        }

        if data.len() < length {
            return Err(Error::new(
                "Provided data length is smaller than specified.",
            ));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_write_record(
                tag,
                file_no,
                offset,
                length,
                data.as_ptr() as *mut _,
            )
        };

        if result < 0 {
            Err(Error::new("Failed to write record."))
        } else {
            Ok(result)
        }
    }

    /// Writes a record to a file with communication settings
    pub unsafe fn desfire_write_record_ex(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        offset: off_t,
        length: usize,
        data: &[u8],
        cs: ::std::os::raw::c_int,
    ) -> Result<isize> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot write record."));
        }

        if data.len() < length {
            return Err(Error::new(
                "Provided data length is smaller than specified.",
            ));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_write_record_ex(
                tag,
                file_no,
                offset,
                length,
                data.as_ptr() as *mut _,
                cs,
            )
        };

        if result < 0 {
            Err(Error::new(
                "Failed to write record with communication settings.",
            ))
        } else {
            Ok(result)
        }
    }

    /// Reads records from a file
    pub unsafe fn desfire_read_records(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        offset: off_t,
        length: usize,
        buffer: &mut [u8],
    ) -> Result<isize> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot read records."));
        }

        if buffer.len() < length {
            return Err(Error::new("Buffer is too small for the specified length."));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_read_records(
                tag,
                file_no,
                offset,
                length,
                buffer.as_mut_ptr() as *mut _,
            )
        };

        if result < 0 {
            Err(Error::new("Failed to read records."))
        } else {
            Ok(result)
        }
    }

    /// Reads records from a file with communication settings
    pub unsafe fn desfire_read_records_ex(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        offset: off_t,
        length: usize,
        buffer: &mut [u8],
        cs: ::std::os::raw::c_int,
    ) -> Result<isize> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot read records."));
        }

        if buffer.len() < length {
            return Err(Error::new("Buffer is too small for the specified length."));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_read_records_ex(
                tag,
                file_no,
                offset,
                length,
                buffer.as_mut_ptr() as *mut _,
                cs,
            )
        };

        if result < 0 {
            Err(Error::new(
                "Failed to read records with communication settings.",
            ))
        } else {
            Ok(result)
        }
    }

    /// Clears a record file
    pub unsafe fn desfire_clear_record_file(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot clear record file."));
        }

        let result = unsafe { freefare_sys::mifare_desfire_clear_record_file(tag, file_no) };

        if result < 0 {
            Err(Error::new("Failed to clear record file."))
        } else {
            Ok(())
        }
    }

    /// Commits a transaction on a Mifare DESFire tag
    pub unsafe fn desfire_commit_transaction(tag: freefare_sys::FreefareTag) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot commit transaction."));
        }

        let result = unsafe { freefare_sys::mifare_desfire_commit_transaction(tag) };

        if result < 0 {
            Err(Error::new("Failed to commit transaction."))
        } else {
            Ok(())
        }
    }

    /// Aborts a transaction on a Mifare DESFire tag
    pub unsafe fn desfire_abort_transaction(tag: freefare_sys::FreefareTag) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot abort transaction."));
        }

        let result = unsafe { freefare_sys::mifare_desfire_abort_transaction(tag) };

        if result < 0 {
            Err(Error::new("Failed to abort transaction."))
        } else {
            Ok(())
        }
    }

    /// Creates a new DES key for Mifare DESFire
    pub fn desfire_des_key_new(value: &mut [u8]) -> Result<freefare_sys::MifareDESFireKey> {
        if value.len() != 8 {
            return Err(Error::new("DES key must be 8 bytes."));
        }

        let key = unsafe { freefare_sys::mifare_desfire_des_key_new(value.as_mut_ptr()) };

        if key.is_null() {
            Err(Error::new("Failed to create DES key."))
        } else {
            Ok(key)
        }
    }

    /// Creates a linear record file with ISO parameters
    pub unsafe fn desfire_create_linear_record_file_iso(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        record_size: u32,
        max_number_of_records: u32,
        iso_file_id: u16,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot create linear record file."));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_create_linear_record_file_iso(
                tag,
                file_no,
                communication_settings,
                access_rights,
                record_size,
                max_number_of_records,
                iso_file_id,
            )
        };

        if result < 0 {
            Err(Error::new(
                "Failed to create linear record file with ISO parameters.",
            ))
        } else {
            Ok(())
        }
    }

    /// Creates a cyclic record file
    pub unsafe fn desfire_create_cyclic_record_file(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        record_size: u32,
        max_number_of_records: u32,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot create cyclic record file."));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_create_cyclic_record_file(
                tag,
                file_no,
                communication_settings,
                access_rights,
                record_size,
                max_number_of_records,
            )
        };

        if result < 0 {
            Err(Error::new("Failed to create cyclic record file."))
        } else {
            Ok(())
        }
    }

    /// Deletes a file on a Mifare DESFire tag
    pub unsafe fn desfire_delete_file(tag: freefare_sys::FreefareTag, file_no: u8) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot delete file."));
        }

        let result = unsafe { freefare_sys::mifare_desfire_delete_file(tag, file_no) };

        if result < 0 {
            Err(Error::new("Failed to delete file."))
        } else {
            Ok(())
        }
    }

    /// Reads data from a file
    pub unsafe fn desfire_read_data(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        offset: off_t,
        length: usize,
        buffer: &mut [u8],
    ) -> Result<isize> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot read data."));
        }

        if buffer.len() < length {
            return Err(Error::new("Buffer is too small for the specified length."));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_read_data(
                tag,
                file_no,
                offset,
                length,
                buffer.as_mut_ptr() as *mut _,
            )
        };

        if result < 0 {
            Err(Error::new("Failed to read data."))
        } else {
            Ok(result)
        }
    }

    /// Writes data to a file
    pub unsafe fn desfire_write_data(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        offset: off_t,
        length: usize,
        data: &[u8],
    ) -> Result<isize> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot write data."));
        }

        if data.len() < length {
            return Err(Error::new(
                "Provided data length is smaller than specified.",
            ));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_write_data(
                tag,
                file_no,
                offset,
                length,
                data.as_ptr() as *const _,
            )
        };

        if result < 0 {
            Err(Error::new("Failed to write data."))
        } else {
            Ok(result)
        }
    }

    /// Gets a value from a file
    pub unsafe fn desfire_get_value(tag: freefare_sys::FreefareTag, file_no: u8) -> Result<i32> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot get value."));
        }

        let mut value: i32 = 0;

        let result =
            unsafe { freefare_sys::mifare_desfire_get_value(tag, file_no, &mut value as *mut _) };

        if result < 0 {
            Err(Error::new("Failed to get value."))
        } else {
            Ok(value)
        }
    }

    /// Credits an amount to a file
    pub unsafe fn desfire_credit(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        amount: i32,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot credit amount."));
        }

        let result = unsafe { freefare_sys::mifare_desfire_credit(tag, file_no, amount) };

        if result < 0 {
            Err(Error::new("Failed to credit amount."))
        } else {
            Ok(())
        }
    }

    /// Debits an amount from a file
    pub unsafe fn desfire_debit(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        amount: i32,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot debit amount."));
        }

        let result = unsafe { freefare_sys::mifare_desfire_debit(tag, file_no, amount) };

        if result < 0 {
            Err(Error::new("Failed to debit amount."))
        } else {
            Ok(())
        }
    }

    /// Creates a 3K3DES application with ISO parameters
    pub unsafe fn desfire_create_application_3k3des_iso(
        tag: freefare_sys::FreefareTag,
        aid: freefare_sys::MifareDESFireAID,
        settings: u8,
        key_no: u8,
        want_iso_file_identifiers: bool,
        iso_file_id: u16,
        iso_file_name: &mut [u8],
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot create 3K3DES application with ISO parameters.",
            ));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_create_application_3k3des_iso(
                tag,
                aid,
                settings,
                key_no,
                want_iso_file_identifiers as ::std::os::raw::c_int,
                iso_file_id,
                iso_file_name.as_mut_ptr(),
                iso_file_name.len(),
            )
        };

        if result < 0 {
            Err(Error::new(
                "Failed to create 3K3DES application with ISO parameters.",
            ))
        } else {
            Ok(())
        }
    }

    /// Creates an AES application with ISO parameters
    pub unsafe fn desfire_create_application_aes_iso(
        tag: freefare_sys::FreefareTag,
        aid: freefare_sys::MifareDESFireAID,
        settings: u8,
        key_no: u8,
        want_iso_file_identifiers: bool,
        iso_file_id: u16,
        iso_file_name: &mut [u8],
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot create AES application with ISO parameters.",
            ));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_create_application_aes_iso(
                tag,
                aid,
                settings,
                key_no,
                want_iso_file_identifiers as ::std::os::raw::c_int,
                iso_file_id,
                iso_file_name.as_mut_ptr(),
                iso_file_name.len(),
            )
        };

        if result < 0 {
            Err(Error::new(
                "Failed to create AES application with ISO parameters.",
            ))
        } else {
            Ok(())
        }
    }

    /// Frees a list of application IDs
    pub unsafe fn desfire_free_application_ids(
        aids: *mut freefare_sys::MifareDESFireAID,
    ) -> Result<()> {
        if aids.is_null() {
            return Err(Error::new("Application IDs are null. Cannot free."));
        }

        unsafe { freefare_sys::mifare_desfire_free_application_ids(aids) };

        Ok(())
    }

    /// Gets the amount of free memory on the tag
    pub unsafe fn desfire_free_mem(tag: freefare_sys::FreefareTag) -> Result<u32> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot retrieve free memory."));
        }

        let mut size: u32 = 0;

        let result = unsafe { freefare_sys::mifare_desfire_free_mem(tag, &mut size as *mut _) };

        if result < 0 {
            Err(Error::new("Failed to retrieve free memory."))
        } else {
            Ok(size)
        }
    }

    /// Sets configuration options on the tag.
    pub unsafe fn desfire_set_configuration(
        tag: freefare_sys::FreefareTag,
        disable_format: bool,
        enable_random_uid: bool,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot set configuration."));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_set_configuration(tag, disable_format, enable_random_uid)
        };

        if result < 0 {
            Err(Error::new("Failed to set configuration."))
        } else {
            Ok(())
        }
    }

    /// Sets the default key for the tag.
    pub unsafe fn desfire_set_default_key(
        tag: freefare_sys::FreefareTag,
        key: freefare_sys::MifareDESFireKey,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot set default key."));
        }

        if key.is_null() {
            return Err(Error::new("Key is null. Cannot set default key."));
        }

        let result = unsafe { freefare_sys::mifare_desfire_set_default_key(tag, key) };

        if result < 0 {
            Err(Error::new("Failed to set default key."))
        } else {
            Ok(())
        }
    }

    /// Sets the ATS (Answer To Select) string for the tag
    pub unsafe fn desfire_set_ats(tag: freefare_sys::FreefareTag, ats: &mut [u8]) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot set ATS."));
        }

        let result = unsafe { freefare_sys::mifare_desfire_set_ats(tag, ats.as_mut_ptr()) };

        if result < 0 {
            Err(Error::new("Failed to set ATS."))
        } else {
            Ok(())
        }
    }

    /// Gets the card UID
    pub unsafe fn desfire_get_card_uid(tag: freefare_sys::FreefareTag) -> Result<String> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot retrieve card UID."));
        }

        let mut uid_ptr: *mut ::std::os::raw::c_char = std::ptr::null_mut();

        let result =
            unsafe { freefare_sys::mifare_desfire_get_card_uid(tag, &mut uid_ptr as *mut _) };

        check_status(tag, result, "Failed to retrieve card UID.")?;
        copy_malloc_c_string(uid_ptr)
    }

    /// Gets the file IDs on the tag
    pub unsafe fn desfire_get_file_ids(tag: freefare_sys::FreefareTag) -> Result<Vec<u8>> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot retrieve file IDs."));
        }

        let mut files_ptr: *mut u8 = std::ptr::null_mut();
        let mut count: usize = 0;

        let result = unsafe {
            freefare_sys::mifare_desfire_get_file_ids(
                tag,
                &mut files_ptr as *mut _,
                &mut count as *mut _,
            )
        };

        if result < 0 {
            return Err(Error::new("Failed to retrieve file IDs."));
        }

        copy_malloc_array(files_ptr, count)
    }

    /// Gets the ISO file IDs on the tag
    pub unsafe fn desfire_get_iso_file_ids(tag: freefare_sys::FreefareTag) -> Result<Vec<u16>> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot retrieve ISO file IDs."));
        }

        let mut files_ptr: *mut u16 = std::ptr::null_mut();
        let mut count: usize = 0;

        let result = unsafe {
            freefare_sys::mifare_desfire_get_iso_file_ids(
                tag,
                &mut files_ptr as *mut _,
                &mut count as *mut _,
            )
        };

        if result < 0 {
            return Err(Error::new("Failed to retrieve ISO file IDs."));
        }

        copy_malloc_array(files_ptr, count)
    }

    /// Gets the file settings of a file
    pub unsafe fn desfire_get_file_settings(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
    ) -> Result<freefare_sys::MifareDESFireFileSettings> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot retrieve file settings."));
        }

        let mut settings =
            std::mem::MaybeUninit::<freefare_sys::MifareDESFireFileSettings>::uninit();

        let result = unsafe {
            freefare_sys::mifare_desfire_get_file_settings(tag, file_no, settings.as_mut_ptr())
        };

        if result < 0 {
            Err(Error::new("Failed to retrieve file settings."))
        } else {
            Ok(unsafe { settings.assume_init() })
        }
    }

    /// Changes the settings of a file
    pub unsafe fn desfire_change_file_settings(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot change file settings."));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_change_file_settings(
                tag,
                file_no,
                communication_settings,
                access_rights,
            )
        };

        if result < 0 {
            Err(Error::new("Failed to change file settings."))
        } else {
            Ok(())
        }
    }

    /// Creates a standard data file.
    pub unsafe fn desfire_create_std_data_file(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        file_size: u32,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot create standard data file."));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_create_std_data_file(
                tag,
                file_no,
                communication_settings,
                access_rights,
                file_size,
            )
        };

        if result < 0 {
            Err(Error::new("Failed to create standard data file."))
        } else {
            Ok(())
        }
    }

    /// Creates a standard data file with ISO parameters
    pub unsafe fn desfire_create_std_data_file_iso(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        file_size: u32,
        iso_file_id: u16,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot create standard data file with ISO parameters.",
            ));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_create_std_data_file_iso(
                tag,
                file_no,
                communication_settings,
                access_rights,
                file_size,
                iso_file_id,
            )
        };

        if result < 0 {
            Err(Error::new(
                "Failed to create standard data file with ISO parameters.",
            ))
        } else {
            Ok(())
        }
    }

    /// Creates a backup data file
    pub unsafe fn desfire_create_backup_data_file(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        file_size: u32,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot create backup data file."));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_create_backup_data_file(
                tag,
                file_no,
                communication_settings,
                access_rights,
                file_size,
            )
        };

        if result < 0 {
            Err(Error::new("Failed to create backup data file."))
        } else {
            Ok(())
        }
    }

    /// Creates a backup data file with ISO parameters
    pub unsafe fn desfire_create_backup_data_file_iso(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        file_size: u32,
        iso_file_id: u16,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot create backup data file with ISO parameters.",
            ));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_create_backup_data_file_iso(
                tag,
                file_no,
                communication_settings,
                access_rights,
                file_size,
                iso_file_id,
            )
        };

        if result < 0 {
            Err(Error::new(
                "Failed to create backup data file with ISO parameters.",
            ))
        } else {
            Ok(())
        }
    }

    /// Creates a value file
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn desfire_create_value_file(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        lower_limit: i32,
        upper_limit: i32,
        value: i32,
        limited_credit_enable: bool,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot create value file."));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_create_value_file(
                tag,
                file_no,
                communication_settings,
                access_rights,
                lower_limit,
                upper_limit,
                value,
                limited_credit_enable as u8,
            )
        };

        if result < 0 {
            Err(Error::new("Failed to create value file."))
        } else {
            Ok(())
        }
    }

    /// Creates a linear record file
    pub unsafe fn desfire_create_linear_record_file(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        record_size: u32,
        max_number_of_records: u32,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot create linear record file."));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_create_linear_record_file(
                tag,
                file_no,
                communication_settings,
                access_rights,
                record_size,
                max_number_of_records,
            )
        };

        if result < 0 {
            Err(Error::new("Failed to create linear record file."))
        } else {
            Ok(())
        }
    }

    /// Creates a cyclic record file with ISO parameters
    pub unsafe fn desfire_create_cyclic_record_file_iso(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        record_size: u32,
        max_number_of_records: u32,
        iso_file_id: u16,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new(
                "Tag is null. Cannot create cyclic record file with ISO parameters.",
            ));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_create_cyclic_record_file_iso(
                tag,
                file_no,
                communication_settings,
                access_rights,
                record_size,
                max_number_of_records,
                iso_file_id,
            )
        };

        if result < 0 {
            Err(Error::new(
                "Failed to create cyclic record file with ISO parameters.",
            ))
        } else {
            Ok(())
        }
    }

    /// Reads data from a file with communication settings
    pub unsafe fn desfire_read_data_ex(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        offset: off_t,
        length: usize,
        buffer: &mut [u8],
        communication_settings: ::std::os::raw::c_int,
    ) -> Result<isize> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot read data."));
        }

        if buffer.len() < length {
            return Err(Error::new("Buffer is too small for the specified length."));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_read_data_ex(
                tag,
                file_no,
                offset,
                length,
                buffer.as_mut_ptr() as *mut _,
                communication_settings,
            )
        };

        if result < 0 {
            Err(Error::new(
                "Failed to read data with communication settings.",
            ))
        } else {
            Ok(result)
        }
    }

    /// Writes data to a file with communication settings
    pub unsafe fn desfire_write_data_ex(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        offset: off_t,
        length: usize,
        data: &[u8],
        communication_settings: ::std::os::raw::c_int,
    ) -> Result<isize> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot write data."));
        }

        if data.len() < length {
            return Err(Error::new(
                "Provided data length is smaller than specified.",
            ));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_write_data_ex(
                tag,
                file_no,
                offset,
                length,
                data.as_ptr() as *const _,
                communication_settings,
            )
        };

        if result < 0 {
            Err(Error::new(
                "Failed to write data with communication settings.",
            ))
        } else {
            Ok(result)
        }
    }

    /// Gets a value from a file with communication settings
    pub unsafe fn desfire_get_value_ex(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        communication_settings: ::std::os::raw::c_int,
    ) -> Result<i32> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot get value."));
        }

        let mut value: i32 = 0;

        let result = unsafe {
            freefare_sys::mifare_desfire_get_value_ex(
                tag,
                file_no,
                &mut value as *mut _,
                communication_settings,
            )
        };

        if result < 0 {
            Err(Error::new(
                "Failed to get value with communication settings.",
            ))
        } else {
            Ok(value)
        }
    }

    /// Credits an amount to a file with communication settings
    pub unsafe fn desfire_credit_ex(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        amount: i32,
        communication_settings: ::std::os::raw::c_int,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot credit amount."));
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_credit_ex(tag, file_no, amount, communication_settings)
        };

        if result < 0 {
            Err(Error::new(
                "Failed to credit amount with communication settings.",
            ))
        } else {
            Ok(())
        }
    }
}
