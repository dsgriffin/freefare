use ::nfc_sys;
use ::freefare_sys;
use libc::off_t;

pub struct Mifare;

impl Mifare {
    /// Checks if the NFC target is a Mifare Ultralight card
    pub fn ultralight_taste(
        device: *mut nfc_sys::nfc_device,
        target: nfc_sys::nfc_target,
    ) -> Result<bool, String> {
        if device.is_null() {
            return Err("Device is null. Cannot check Mifare Ultralight target.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_ultralight_taste(device, target) };
        Ok(result != 0)
    }

    /// Checks if the NFC target is a Mifare Ultralight C card
    pub fn ultralightc_taste(
        device: *mut nfc_sys::nfc_device,
        target: nfc_sys::nfc_target,
    ) -> Result<bool, String> {
        if device.is_null() {
            return Err("Device is null. Cannot check Mifare Ultralight C target.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_ultralightc_taste(device, target) };
        Ok(result != 0)
    }

    /// Creates a new Mifare Ultralight tag object
    pub fn ultralight_tag_new(
        device: *mut nfc_sys::nfc_device,
        target: nfc_sys::nfc_target,
    ) -> Result<freefare_sys::FreefareTag, String> {
        if device.is_null() {
            return Err("Device is null. Cannot create Mifare Ultralight tag.".to_string());
        }

        let tag = unsafe { freefare_sys::mifare_ultralight_tag_new(device, target) };
        if tag.is_null() {
            Err("Failed to create Mifare Ultralight tag.".to_string())
        } else {
            Ok(tag)
        }
    }

    /// Creates a new Mifare Ultralight C tag object
    pub fn ultralightc_tag_new(
        device: *mut nfc_sys::nfc_device,
        target: nfc_sys::nfc_target,
    ) -> Result<freefare_sys::FreefareTag, String> {
        if device.is_null() {
            return Err("Device is null. Cannot create Mifare Ultralight C tag.".to_string());
        }

        let tag = unsafe { freefare_sys::mifare_ultralightc_tag_new(device, target) };
        if tag.is_null() {
            Err("Failed to create Mifare Ultralight C tag.".to_string())
        } else {
            Ok(tag)
        }
    }

    /// Frees a Mifare Ultralight tag object
    pub fn ultralight_tag_free(tag: freefare_sys::FreefareTag) {
        if !tag.is_null() {
            unsafe { freefare_sys::mifare_ultralight_tag_free(tag) };
        }
    }

    /// Frees a Mifare Ultralight C tag object
    pub fn ultralightc_tag_free(tag: freefare_sys::FreefareTag) {
        if !tag.is_null() {
            unsafe { freefare_sys::mifare_ultralightc_tag_free(tag) };
        }
    }

    /// Connects to a Mifare Ultralight tag
    pub fn ultralight_connect(tag: freefare_sys::FreefareTag) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot connect to Mifare Ultralight tag.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_ultralight_connect(tag) };
        if result < 0 {
            Err("Failed to connect to Mifare Ultralight tag.".to_string())
        } else {
            Ok(())
        }
    }

    /// Disconnects from a Mifare Ultralight tag.
    pub fn ultralight_disconnect(tag: freefare_sys::FreefareTag) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot disconnect from Mifare Ultralight tag.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_ultralight_disconnect(tag) };
        if result < 0 {
            Err("Failed to disconnect from Mifare Ultralight tag.".to_string())
        } else {
            Ok(())
        }
    }
    
    /// Reads data from a page of a Mifare Ultralight tag.
    pub fn ultralight_read(
        tag: freefare_sys::FreefareTag,
        page: freefare_sys::MifareUltralightPageNumber,
        buffer: &mut freefare_sys::MifareUltralightPage,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot read from Mifare Ultralight tag.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_ultralight_read(tag, page, buffer as *mut _) };
        if result < 0 {
            Err("Failed to read from Mifare Ultralight tag.".to_string())
        } else {
            Ok(())
        }
    }
    
    /// Writes data to a page of a Mifare Ultralight tag.
    pub fn ultralight_write(
        tag: freefare_sys::FreefareTag,
        page: freefare_sys::MifareUltralightPageNumber,
        data: &freefare_sys::MifareUltralightPage,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot write to Mifare Ultralight tag.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_ultralight_write(tag, page, data as *const _) };
        if result < 0 {
            Err("Failed to write to Mifare Ultralight tag.".to_string())
        } else {
            Ok(())
        }
    }
    
    /// Authenticates a Mifare Ultralight C tag using a DESFire key.
    pub fn ultralightc_authenticate(
        tag: freefare_sys::FreefareTag,
        key: freefare_sys::MifareDESFireKey,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot authenticate Mifare Ultralight C tag.".to_string());
        }

        if key.is_null() {
            return Err("Key is null. Cannot authenticate Mifare Ultralight C tag.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_ultralightc_authenticate(tag, key) };
        if result < 0 {
            Err("Failed to authenticate Mifare Ultralight C tag.".to_string())
        } else {
            Ok(())
        }
    }
    
    /// Checks if the tag is a Mifare Ultralight card.
    pub fn is_mifare_ultralight(tag: freefare_sys::FreefareTag) -> bool {
        if tag.is_null() {
            return false;
        }

        unsafe { freefare_sys::is_mifare_ultralight(tag) != 0 }
    }

    /// Checks if the tag is a Mifare Ultralight C card.
    pub fn is_mifare_ultralightc(tag: freefare_sys::FreefareTag) -> bool {
        if tag.is_null() {
            return false;
        }

        unsafe { freefare_sys::is_mifare_ultralightc(tag) != 0 }
    }
    
    /// Checks if a Mifare Ultralight C tag is present on the NFC reader.
    pub fn is_mifare_ultralightc_on_reader(
        device: *mut nfc_sys::nfc_device,
        nai: nfc_sys::nfc_iso14443a_info,
    ) -> bool {
        if device.is_null() {
            return false;
        }

        unsafe { freefare_sys::is_mifare_ultralightc_on_reader(device, nai) != 0 }
    }

    /// Checks if the NFC target is a Mifare Classic 1K card.
    pub fn classic1k_taste(
        device: *mut nfc_sys::nfc_device,
        target: nfc_sys::nfc_target,
    ) -> Result<bool, String> {
        if device.is_null() {
            return Err("Device is null. Cannot check Mifare Classic 1K target.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_classic1k_taste(device, target) };
        Ok(result != 0)
    }

    /// Checks if the NFC target is a Mifare Classic 4K card.
    pub fn classic4k_taste(
        device: *mut nfc_sys::nfc_device,
        target: nfc_sys::nfc_target,
    ) -> Result<bool, String> {
        if device.is_null() {
            return Err("Device is null. Cannot check Mifare Classic 4K target.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_classic4k_taste(device, target) };
        Ok(result != 0)
    }

    /// Creates a new Mifare Classic 1K tag object.
    pub fn classic1k_tag_new(
        device: *mut nfc_sys::nfc_device,
        target: nfc_sys::nfc_target,
    ) -> Result<freefare_sys::FreefareTag, String> {
        if device.is_null() {
            return Err("Device is null. Cannot create Mifare Classic 1K tag.".to_string());
        }

        let tag = unsafe { freefare_sys::mifare_classic1k_tag_new(device, target) };
        if tag.is_null() {
            Err("Failed to create Mifare Classic 1K tag.".to_string())
        } else {
            Ok(tag)
        }
    }

    /// Creates a new Mifare Classic 4K tag object.
    pub fn classic4k_tag_new(
        device: *mut nfc_sys::nfc_device,
        target: nfc_sys::nfc_target,
    ) -> Result<freefare_sys::FreefareTag, String> {
        if device.is_null() {
            return Err("Device is null. Cannot create Mifare Classic 4K tag.".to_string());
        }

        let tag = unsafe { freefare_sys::mifare_classic4k_tag_new(device, target) };
        if tag.is_null() {
            Err("Failed to create Mifare Classic 4K tag.".to_string())
        } else {
            Ok(tag)
        }
    }

    /// Frees a Mifare Classic tag object.
    pub fn classic_tag_free(tag: freefare_sys::FreefareTag) {
        if !tag.is_null() {
            unsafe { freefare_sys::mifare_classic_tag_free(tag) };
        }
    }

    /// Connects to a Mifare Classic tag.
    pub fn classic_connect(tag: freefare_sys::FreefareTag) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot connect to Mifare Classic tag.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_classic_connect(tag) };
        if result < 0 {
            Err("Failed to connect to Mifare Classic tag.".to_string())
        } else {
            Ok(())
        }
    }

    /// Disconnects from a Mifare Classic tag.
    pub fn classic_disconnect(tag: freefare_sys::FreefareTag) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot disconnect from Mifare Classic tag.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_classic_disconnect(tag) };
        if result < 0 {
            Err("Failed to disconnect from Mifare Classic tag.".to_string())
        } else {
            Ok(())
        }
    }

    /// Authenticates a Mifare Classic block using a specified key and key type.
    pub fn classic_authenticate(
        tag: freefare_sys::FreefareTag,
        block: freefare_sys::MifareClassicBlockNumber,
        key: &freefare_sys::MifareClassicKey,
        key_type: freefare_sys::MifareClassicKeyType,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot authenticate Mifare Classic tag.".to_string());
        }

        let result = unsafe {
            freefare_sys::mifare_classic_authenticate(tag, block, key as *const _, key_type)
        };
        if result < 0 {
            Err("Failed to authenticate Mifare Classic tag.".to_string())
        } else {
            Ok(())
        }
    }

    /// Reads data from a Mifare Classic block.
    pub fn classic_read(
        tag: freefare_sys::FreefareTag,
        block: freefare_sys::MifareClassicBlockNumber,
        buffer: &mut freefare_sys::MifareClassicBlock,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot read from Mifare Classic block.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_classic_read(tag, block, buffer as *mut _) };
        if result < 0 {
            Err("Failed to read from Mifare Classic block.".to_string())
        } else {
            Ok(())
        }
    }

    /// Initializes a Mifare Classic block as a value block.
    pub fn classic_init_value(
        tag: freefare_sys::FreefareTag,
        block: freefare_sys::MifareClassicBlockNumber,
        value: i32,
        adr: freefare_sys::MifareClassicBlockNumber,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot initialize Mifare Classic block as value block.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_classic_init_value(tag, block, value, adr) };
        if result < 0 {
            Err("Failed to initialize Mifare Classic block as value block.".to_string())
        } else {
            Ok(())
        }
    }

    /// Reads a value from a Mifare Classic block.
    pub fn classic_read_value(
        tag: freefare_sys::FreefareTag,
        block: freefare_sys::MifareClassicBlockNumber,
    ) -> Result<(i32, freefare_sys::MifareClassicBlockNumber), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot read value from Mifare Classic block.".to_string());
        }

        let mut value: i32 = 0;
        let mut adr: freefare_sys::MifareClassicBlockNumber = 0;
        let result = unsafe {
            freefare_sys::mifare_classic_read_value(tag, block, &mut value as *mut _, &mut adr)
        };

        if result < 0 {
            Err("Failed to read value from Mifare Classic block.".to_string())
        } else {
            Ok((value, adr))
        }
    }

    /// Writes data to a Mifare Classic block.
    pub fn classic_write(
        tag: freefare_sys::FreefareTag,
        block: freefare_sys::MifareClassicBlockNumber,
        data: &freefare_sys::MifareClassicBlock,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot write to Mifare Classic block.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_classic_write(tag, block, data as *const _) };
        if result < 0 {
            Err("Failed to write to Mifare Classic block.".to_string())
        } else {
            Ok(())
        }
    }

    /// Increments a value in a Mifare Classic block.
    pub fn classic_increment(
        tag: freefare_sys::FreefareTag,
        block: freefare_sys::MifareClassicBlockNumber,
        amount: u32,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot increment Mifare Classic block.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_classic_increment(tag, block, amount) };
        if result < 0 {
            Err("Failed to increment Mifare Classic block.".to_string())
        } else {
            Ok(())
        }
    }

    /// Decrements a value in a Mifare Classic block.
    pub fn classic_decrement(
        tag: freefare_sys::FreefareTag,
        block: freefare_sys::MifareClassicBlockNumber,
        amount: u32,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot decrement Mifare Classic block.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_classic_decrement(tag, block, amount) };
        if result < 0 {
            Err("Failed to decrement Mifare Classic block.".to_string())
        } else {
            Ok(())
        }
    }

    /// Restores a value from a Mifare Classic block.
    pub fn classic_restore(
        tag: freefare_sys::FreefareTag,
        block: freefare_sys::MifareClassicBlockNumber,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot restore Mifare Classic block.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_classic_restore(tag, block) };
        if result < 0 {
            Err("Failed to restore Mifare Classic block.".to_string())
        } else {
            Ok(())
        }
    }

    /// Transfers a value to a Mifare Classic block.
    pub fn classic_transfer(
        tag: freefare_sys::FreefareTag,
        block: freefare_sys::MifareClassicBlockNumber,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot transfer to Mifare Classic block.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_classic_transfer(tag, block) };
        if result < 0 {
            Err("Failed to transfer to Mifare Classic block.".to_string())
        } else {
            Ok(())
        }
    }

    /// Gets permissions for a trailer block in a Mifare Classic tag.
    pub fn classic_get_trailer_block_permission(
        tag: freefare_sys::FreefareTag,
        block: freefare_sys::MifareClassicBlockNumber,
        permission: u16,
        key_type: freefare_sys::MifareClassicKeyType,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot get trailer block permissions.".to_string());
        }

        let result = unsafe {
            freefare_sys::mifare_classic_get_trailer_block_permission(tag, block, permission, key_type)
        };
        if result < 0 {
            Err("Failed to get trailer block permissions.".to_string())
        } else {
            Ok(())
        }
    }

    /// Formats a sector of a Mifare Classic tag.
    pub fn classic_format_sector(
        tag: freefare_sys::FreefareTag,
        sector: freefare_sys::MifareClassicSectorNumber,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot format sector.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_classic_format_sector(tag, sector) };
        if result < 0 {
            Err("Failed to format sector.".to_string())
        } else {
            Ok(())
        }
    }

    /// Creates a trailer block for a Mifare Classic tag.
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
    pub fn classic_get_data_block_permission(
        tag: freefare_sys::FreefareTag,
        block: freefare_sys::MifareClassicBlockNumber,
        permission: u8,
        key_type: freefare_sys::MifareClassicKeyType,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot get data block permissions.".to_string());
        }

        let result = unsafe {
            freefare_sys::mifare_classic_get_data_block_permission(tag, block, permission, key_type)
        };
        if result < 0 {
            Err("Failed to get data block permissions.".to_string())
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

    /// Allocates sectors for an application in a Mifare Classic tag.
    pub fn application_alloc(
        mad: freefare_sys::Mad,
        aid: freefare_sys::MadAid,
        size: usize,
    ) -> Result<Vec<freefare_sys::MifareClassicSectorNumber>, String> {
        if mad.is_null() {
            return Err("MAD structure is null. Cannot allocate application.".to_string());
        }

        let sectors = unsafe { freefare_sys::mifare_application_alloc(mad, aid, size) };
        if sectors.is_null() {
            return Err("Failed to allocate application.".to_string());
        }

        // Convert C array into Vec
        let mut result = Vec::new();
        let mut ptr = sectors;
        unsafe {
            while !ptr.is_null() {
                result.push(*ptr);
                ptr = ptr.add(1);
            }
        }

        Ok(result)
    }

    /// Reads application data from a Mifare Classic tag.
    pub fn application_read(
        tag: freefare_sys::FreefareTag,
        mad: freefare_sys::Mad,
        aid: freefare_sys::MadAid,
        buffer: &mut [u8],
        key: &freefare_sys::MifareClassicKey,
        key_type: freefare_sys::MifareClassicKeyType,
    ) -> Result<usize, String> {
        if tag.is_null() || mad.is_null() {
            return Err("Tag or MAD structure is null. Cannot read application.".to_string());
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
            Err("Failed to read application data.".to_string())
        } else {
            Ok(result as usize)
        }
    }

    /// Writes application data to a Mifare Classic tag.
    pub fn application_write(
        tag: freefare_sys::FreefareTag,
        mad: freefare_sys::Mad,
        aid: freefare_sys::MadAid,
        buffer: &[u8],
        key: &freefare_sys::MifareClassicKey,
        key_type: freefare_sys::MifareClassicKeyType,
    ) -> Result<usize, String> {
        if tag.is_null() || mad.is_null() {
            return Err("Tag or MAD structure is null. Cannot write application.".to_string());
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
            Err("Failed to write application data.".to_string())
        } else {
            Ok(result as usize)
        }
    }

    /// Frees an application in a Mifare Classic tag.
    pub fn application_free(
        mad: freefare_sys::Mad,
        aid: freefare_sys::MadAid,
    ) -> Result<(), String> {
        if mad.is_null() {
            return Err("MAD structure is null. Cannot free application.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_application_free(mad, aid) };
        if result < 0 {
            Err("Failed to free application.".to_string())
        } else {
            Ok(())
        }
    }

    /// Finds an application in a Mifare Classic tag.
    pub fn application_find(
        mad: freefare_sys::Mad,
        aid: freefare_sys::MadAid,
    ) -> Result<Vec<freefare_sys::MifareClassicSectorNumber>, String> {
        if mad.is_null() {
            return Err("MAD structure is null. Cannot find application.".to_string());
        }

        let sectors = unsafe { freefare_sys::mifare_application_find(mad, aid) };
        if sectors.is_null() {
            return Err("Application not found.".to_string());
        }

        // Convert C array into Vec
        let mut result = Vec::new();
        let mut ptr = sectors;
        unsafe {
            while !ptr.is_null() {
                result.push(*ptr);
                ptr = ptr.add(1);
            }
        }
        Ok(result)
    }

    /// Checks if the NFC target is a Mifare DESFire tag.
    pub fn desfire_taste(
        device: *mut nfc_sys::nfc_device,
        target: nfc_sys::nfc_target,
    ) -> Result<bool, String> {
        if device.is_null() {
            return Err("Device is null. Cannot check Mifare DESFire target.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_desfire_taste(device, target) };
        Ok(result != 0)
    }

    /// Creates a new Mifare DESFire AID.
    pub fn desfire_aid_new(aid: u32) -> freefare_sys::MifareDESFireAID {
        unsafe { freefare_sys::mifare_desfire_aid_new(aid) }
    }

    /// Creates a new Mifare DESFire AID with MAD AID.
    pub fn desfire_aid_new_with_mad_aid(
        mad_aid: freefare_sys::MadAid,
        n: u8,
    ) -> freefare_sys::MifareDESFireAID {
        unsafe { freefare_sys::mifare_desfire_aid_new_with_mad_aid(mad_aid, n) }
    }

    /// Gets the AID value from a Mifare DESFire AID.
    pub fn desfire_aid_get_aid(aid: freefare_sys::MifareDESFireAID) -> u32 {
        unsafe { freefare_sys::mifare_desfire_aid_get_aid(aid) }
    }

    /// Gets the last PCD error from a Mifare DESFire tag.
    pub fn desfire_last_pcd_error(tag: freefare_sys::FreefareTag) -> Result<u8, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot retrieve last PCD error.".to_string());
        }

        Ok(unsafe { freefare_sys::mifare_desfire_last_pcd_error(tag) })
    }

    /// Gets the last PICC error from a Mifare DESFire tag.
    pub fn desfire_last_picc_error(tag: freefare_sys::FreefareTag) -> Result<u8, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot retrieve last PICC error.".to_string());
        }

        Ok(unsafe { freefare_sys::mifare_desfire_last_picc_error(tag) })
    }

    /// Creates a new Mifare DESFire tag object.
    pub fn desfire_tag_new(
        device: *mut nfc_sys::nfc_device,
        target: nfc_sys::nfc_target,
    ) -> Result<freefare_sys::FreefareTag, String> {
        if device.is_null() {
            return Err("Device is null. Cannot create Mifare DESFire tag.".to_string());
        }

        let tag = unsafe { freefare_sys::mifare_desfire_tag_new(device, target) };
        if tag.is_null() {
            Err("Failed to create Mifare DESFire tag.".to_string())
        } else {
            Ok(tag)
        }
    }

    /// Frees a Mifare DESFire tag object.
    pub fn desfire_tag_free(tag: freefare_sys::FreefareTag) {
        if !tag.is_null() {
            unsafe { freefare_sys::mifare_desfire_tag_free(tag) };
        }
    }

    /// Connects to a Mifare DESFire tag.
    pub fn desfire_connect(tag: freefare_sys::FreefareTag) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot connect to Mifare DESFire tag.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_desfire_connect(tag) };
        if result < 0 {
            Err("Failed to connect to Mifare DESFire tag.".to_string())
        } else {
            Ok(())
        }
    }

    /// Disconnects from a Mifare DESFire tag.
    pub fn desfire_disconnect(tag: freefare_sys::FreefareTag) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot disconnect from Mifare DESFire tag.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_desfire_disconnect(tag) };
        if result < 0 {
            Err("Failed to disconnect from Mifare DESFire tag.".to_string())
        } else {
            Ok(())
        }
    }

    /// Authenticates with a Mifare DESFire tag using a key.
    pub fn desfire_authenticate(
        tag: freefare_sys::FreefareTag,
        key_no: u8,
        key: freefare_sys::MifareDESFireKey,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot authenticate with Mifare DESFire tag.".to_string());
        }

        if key.is_null() {
            return Err("Key is null. Cannot authenticate with Mifare DESFire tag.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_desfire_authenticate(tag, key_no, key) };
        if result < 0 {
            Err("Failed to authenticate with Mifare DESFire tag.".to_string())
        } else {
            Ok(())
        }
    }

    /// Authenticates with a Mifare DESFire tag using ISO authentication.
    pub fn desfire_authenticate_iso(
        tag: freefare_sys::FreefareTag,
        key_no: u8,
        key: freefare_sys::MifareDESFireKey,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot authenticate with Mifare DESFire tag.".to_string());
        }

        if key.is_null() {
            return Err("Key is null. Cannot authenticate with Mifare DESFire tag.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_desfire_authenticate_iso(tag, key_no, key) };
        if result < 0 {
            Err("Failed to authenticate with Mifare DESFire tag using ISO.".to_string())
        } else {
            Ok(())
        }
    }

        /// Authenticates with a Mifare DESFire tag using AES.
    pub fn desfire_authenticate_aes(
        tag: freefare_sys::FreefareTag,
        key_no: u8,
        key: freefare_sys::MifareDESFireKey,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot authenticate with Mifare DESFire tag using AES.".to_string());
        }

        if key.is_null() {
            return Err("Key is null. Cannot authenticate with Mifare DESFire tag using AES.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_desfire_authenticate_aes(tag, key_no, key) };
        if result < 0 {
            Err("Failed to authenticate with Mifare DESFire tag using AES.".to_string())
        } else {
            Ok(())
        }
    }

    /// Changes the key settings of a Mifare DESFire tag.
    pub fn desfire_change_key_settings(
        tag: freefare_sys::FreefareTag,
        settings: u8,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot change key settings.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_desfire_change_key_settings(tag, settings) };
        if result < 0 {
            Err("Failed to change key settings.".to_string())
        } else {
            Ok(())
        }
    }

    /// Retrieves the key settings of a Mifare DESFire tag.
    pub fn desfire_get_key_settings(
        tag: freefare_sys::FreefareTag,
    ) -> Result<(u8, u8), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot retrieve key settings.".to_string());
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
            Err("Failed to retrieve key settings.".to_string())
        } else {
            Ok((settings, max_keys))
        }
    }

    /// Changes a key on a Mifare DESFire tag.
    pub fn desfire_change_key(
        tag: freefare_sys::FreefareTag,
        key_no: u8,
        new_key: freefare_sys::MifareDESFireKey,
        old_key: freefare_sys::MifareDESFireKey,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot change key.".to_string());
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_change_key(tag, key_no, new_key, old_key)
        };

        if result < 0 {
            Err("Failed to change key.".to_string())
        } else {
            Ok(())
        }
    }

    /// Retrieves the version of a key on a Mifare DESFire tag.
    pub fn desfire_get_key_version(
        tag: freefare_sys::FreefareTag,
        key_no: u8,
    ) -> Result<u8, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot retrieve key version.".to_string());
        }

        let mut version: u8 = 0;

        let result = unsafe {
            freefare_sys::mifare_desfire_get_key_version(tag, key_no, &mut version as *mut _)
        };

        if result < 0 {
            Err("Failed to retrieve key version.".to_string())
        } else {
            Ok(version)
        }
    }

    /// Creates an application on a Mifare DESFire tag.
    pub fn desfire_create_application(
        tag: freefare_sys::FreefareTag,
        aid: freefare_sys::MifareDESFireAID,
        settings: u8,
        key_no: u8,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot create application.".to_string());
        }

        let result =
            unsafe { freefare_sys::mifare_desfire_create_application(tag, aid, settings, key_no) };

        if result < 0 {
            Err("Failed to create application.".to_string())
        } else {
            Ok(())
        }
    }

    /// Deletes an application on a Mifare DESFire tag.
    pub fn desfire_delete_application(
        tag: freefare_sys::FreefareTag,
        aid: freefare_sys::MifareDESFireAID,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot delete application.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_desfire_delete_application(tag, aid) };

        if result < 0 {
            Err("Failed to delete application.".to_string())
        } else {
            Ok(())
        }
    }

    /// Retrieves the application IDs on a Mifare DESFire tag.
    pub fn desfire_get_application_ids(
        tag: freefare_sys::FreefareTag,
    ) -> Result<Vec<freefare_sys::MifareDESFireAID>, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot retrieve application IDs.".to_string());
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
            return Err("Failed to retrieve application IDs.".to_string());
        }

        let aid_vec = unsafe {
            std::slice::from_raw_parts(aids, count)
                .iter()
                .cloned()
                .collect::<Vec<_>>()
        };

        Ok(aid_vec)
    }

        /// Creates a Mifare DESFire application with 3K3DES encryption.
    pub fn desfire_create_application_3k3des(
        tag: freefare_sys::FreefareTag,
        aid: freefare_sys::MifareDESFireAID,
        settings: u8,
        key_no: u8,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot create 3K3DES application.".to_string());
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_create_application_3k3des(tag, aid, settings, key_no)
        };

        if result < 0 {
            Err("Failed to create 3K3DES application.".to_string())
        } else {
            Ok(())
        }
    }

    /// Creates a Mifare DESFire application with AES encryption.
    pub fn desfire_create_application_aes(
        tag: freefare_sys::FreefareTag,
        aid: freefare_sys::MifareDESFireAID,
        settings: u8,
        key_no: u8,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot create AES application.".to_string());
        }

        let result =
            unsafe { freefare_sys::mifare_desfire_create_application_aes(tag, aid, settings, key_no) };

        if result < 0 {
            Err("Failed to create AES application.".to_string())
        } else {
            Ok(())
        }
    }

    /// Creates a Mifare DESFire application with ISO parameters.
    pub fn desfire_create_application_iso(
        tag: freefare_sys::FreefareTag,
        aid: freefare_sys::MifareDESFireAID,
        settings: u8,
        key_no: u8,
        want_iso_file_identifiers: bool,
        iso_file_id: u16,
        iso_file_name: &mut [u8],
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot create ISO application.".to_string());
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
            Err("Failed to create ISO application.".to_string())
        } else {
            Ok(())
        }
    }

    /// Gets the data file names of a Mifare DESFire tag.
    pub fn desfire_get_df_names(
        tag: freefare_sys::FreefareTag,
    ) -> Result<Vec<freefare_sys::MifareDESFireDF>, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot retrieve data file names.".to_string());
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
            return Err("Failed to retrieve data file names.".to_string());
        }

        let dfs = unsafe {
            std::slice::from_raw_parts(dfs_ptr, count)
                .iter()
                .cloned()
                .collect::<Vec<_>>()
        };

        Ok(dfs)
    }

    /// Selects an application on a Mifare DESFire tag.
    pub fn desfire_select_application(
        tag: freefare_sys::FreefareTag,
        aid: freefare_sys::MifareDESFireAID,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot select application.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_desfire_select_application(tag, aid) };

        if result < 0 {
            Err("Failed to select application.".to_string())
        } else {
            Ok(())
        }
    }

    /// Formats the PICC of a Mifare DESFire tag.
    pub fn desfire_format_picc(tag: freefare_sys::FreefareTag) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot format PICC.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_desfire_format_picc(tag) };

        if result < 0 {
            Err("Failed to format PICC.".to_string())
        } else {
            Ok(())
        }
    }

    /// Gets the version information of a Mifare DESFire tag
    pub fn desfire_get_version(
        tag: freefare_sys::FreefareTag,
    ) -> Result<freefare_sys::Struct_mifare_desfire_version_info, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot retrieve version information.".to_string());
        }

        let mut version_info: freefare_sys::Struct_mifare_desfire_version_info =
            unsafe { std::mem::zeroed() };

        let result = unsafe {
            freefare_sys::mifare_desfire_get_version(tag, &mut version_info as *mut _)
        };

        if result < 0 {
            Err("Failed to retrieve version information.".to_string())
        } else {
            Ok(version_info)
        }
    }

    /// Creates a new 3DES key for Mifare DESFire
    pub fn desfire_3des_key_new(value: &mut [u8]) -> Result<freefare_sys::MifareDESFireKey, String> {
        if value.len() != 16 {
            return Err("3DES key must be 16 bytes.".to_string());
        }

        let key = unsafe { freefare_sys::mifare_desfire_3des_key_new(value.as_mut_ptr()) };

        if key.is_null() {
            Err("Failed to create 3DES key.".to_string())
        } else {
            Ok(key)
        }
    }

    /// Creates a new DES key with a version for Mifare DESFire
    pub fn desfire_des_key_new_with_version(
        value: &mut [u8],
    ) -> Result<freefare_sys::MifareDESFireKey, String> {
        if value.len() != 8 {
            return Err("DES key must be 8 bytes.".to_string());
        }

        let key =
            unsafe { freefare_sys::mifare_desfire_des_key_new_with_version(value.as_mut_ptr()) };

        if key.is_null() {
            Err("Failed to create DES key with version.".to_string())
        } else {
            Ok(key)
        }
    }

    /// Creates a new 3DES key with a version for Mifare DESFire
    pub fn desfire_3des_key_new_with_version(
        value: &mut [u8],
    ) -> Result<freefare_sys::MifareDESFireKey, String> {
        if value.len() != 16 {
            return Err("3DES key must be 16 bytes.".to_string());
        }

        let key =
            unsafe { freefare_sys::mifare_desfire_3des_key_new_with_version(value.as_mut_ptr()) };

        if key.is_null() {
            Err("Failed to create 3DES key with version.".to_string())
        } else {
            Ok(key)
        }
    }

    /// Creates a new 3K3DES key for Mifare DESFire
    pub fn desfire_3k3des_key_new(
        value: &mut [u8],
    ) -> Result<freefare_sys::MifareDESFireKey, String> {
        if value.len() != 24 {
            return Err("3K3DES key must be 24 bytes.".to_string());
        }

        let key = unsafe { freefare_sys::mifare_desfire_3k3des_key_new(value.as_mut_ptr()) };

        if key.is_null() {
            Err("Failed to create 3K3DES key.".to_string())
        } else {
            Ok(key)
        }
    }

    /// Creates a new 3K3DES key with a version for Mifare DESFire
    pub fn desfire_3k3des_key_new_with_version(
        value: &mut [u8],
    ) -> Result<freefare_sys::MifareDESFireKey, String> {
        if value.len() != 24 {
            return Err("3K3DES key must be 24 bytes.".to_string());
        }

        let key =
            unsafe { freefare_sys::mifare_desfire_3k3des_key_new_with_version(value.as_mut_ptr()) };

        if key.is_null() {
            Err("Failed to create 3K3DES key with version.".to_string())
        } else {
            Ok(key)
        }
    }

    /// Creates a new AES key for Mifare DESFire
    pub fn desfire_aes_key_new(value: &mut [u8]) -> Result<freefare_sys::MifareDESFireKey, String> {
        if value.len() != 16 {
            return Err("AES key must be 16 bytes.".to_string());
        }

        let key = unsafe { freefare_sys::mifare_desfire_aes_key_new(value.as_mut_ptr()) };

        if key.is_null() {
            Err("Failed to create AES key.".to_string())
        } else {
            Ok(key)
        }
    }

    /// Creates a new AES key with a version for Mifare DESFire
    pub fn desfire_aes_key_new_with_version(
        value: &mut [u8],
        version: u8,
    ) -> Result<freefare_sys::MifareDESFireKey, String> {
        if value.len() != 16 {
            return Err("AES key must be 16 bytes.".to_string());
        }

        let key =
            unsafe { freefare_sys::mifare_desfire_aes_key_new_with_version(value.as_mut_ptr(), version) };

        if key.is_null() {
            Err("Failed to create AES key with version.".to_string())
        } else {
            Ok(key)
        }
    }

    /// Gets the version of a Mifare DESFire key
    pub fn desfire_key_get_version(key: freefare_sys::MifareDESFireKey) -> Result<u8, String> {
        if key.is_null() {
            return Err("Key is null. Cannot get version.".to_string());
        }

        Ok(unsafe { freefare_sys::mifare_desfire_key_get_version(key) })
    }

    /// Sets the version of a Mifare DESFire key
    pub fn desfire_key_set_version(key: freefare_sys::MifareDESFireKey, version: u8) -> Result<(), String> {
        if key.is_null() {
            return Err("Key is null. Cannot set version.".to_string());
        }

        unsafe { freefare_sys::mifare_desfire_key_set_version(key, version) };

        Ok(())
    }

    /// Frees a Mifare DESFire key.
    pub fn desfire_key_free(key: freefare_sys::MifareDESFireKey) -> Result<(), String> {
        if key.is_null() {
            return Err("Key is null. Cannot free key.".to_string());
        }

        unsafe { freefare_sys::mifare_desfire_key_free(key) };

        Ok(())
    }

    /// Debits an amount from a file with communication settings
    pub fn desfire_debit_ex(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        amount: i32,
        cs: ::std::os::raw::c_int,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot perform debit.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_desfire_debit_ex(tag, file_no, amount, cs) };

        if result < 0 {
            Err("Failed to debit the file.".to_string())
        } else {
            Ok(())
        }
    }

    /// Performs a limited credit operation on a file
    pub fn desfire_limited_credit(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        amount: i32,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot perform limited credit.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_desfire_limited_credit(tag, file_no, amount) };

        if result < 0 {
            Err("Failed to perform limited credit.".to_string())
        } else {
            Ok(())
        }
    }

    /// Performs a limited credit operation on a file with communication settings
    pub fn desfire_limited_credit_ex(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        amount: i32,
        cs: ::std::os::raw::c_int,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot perform limited credit.".to_string());
        }

        let result =
            unsafe { freefare_sys::mifare_desfire_limited_credit_ex(tag, file_no, amount, cs) };

        if result < 0 {
            Err("Failed to perform limited credit with communication settings.".to_string())
        } else {
            Ok(())
        }
    }

    /// Writes a record to a file
    pub fn desfire_write_record(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        offset: off_t,
        length: usize,
        data: &[u8],
    ) -> Result<isize, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot write record.".to_string());
        }

        if data.len() < length {
            return Err("Provided data length is smaller than specified.".to_string());
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
            Err("Failed to write record.".to_string())
        } else {
            Ok(result)
        }
    }

    /// Writes a record to a file with communication settings
    pub fn desfire_write_record_ex(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        offset: off_t,
        length: usize,
        data: &[u8],
        cs: ::std::os::raw::c_int,
    ) -> Result<isize, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot write record.".to_string());
        }

        if data.len() < length {
            return Err("Provided data length is smaller than specified.".to_string());
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
            Err("Failed to write record with communication settings.".to_string())
        } else {
            Ok(result)
        }
    }

    /// Reads records from a file
    pub fn desfire_read_records(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        offset: off_t,
        length: usize,
        buffer: &mut [u8],
    ) -> Result<isize, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot read records.".to_string());
        }

        if buffer.len() < length {
            return Err("Buffer is too small for the specified length.".to_string());
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
            Err("Failed to read records.".to_string())
        } else {
            Ok(result)
        }
    }

    /// Reads records from a file with communication settings
    pub fn desfire_read_records_ex(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        offset: off_t,
        length: usize,
        buffer: &mut [u8],
        cs: ::std::os::raw::c_int,
    ) -> Result<isize, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot read records.".to_string());
        }

        if buffer.len() < length {
            return Err("Buffer is too small for the specified length.".to_string());
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
            Err("Failed to read records with communication settings.".to_string())
        } else {
            Ok(result)
        }
    }

    /// Clears a record file
    pub fn desfire_clear_record_file(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot clear record file.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_desfire_clear_record_file(tag, file_no) };

        if result < 0 {
            Err("Failed to clear record file.".to_string())
        } else {
            Ok(())
        }
    }

    /// Commits a transaction on a Mifare DESFire tag
    pub fn desfire_commit_transaction(tag: freefare_sys::FreefareTag) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot commit transaction.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_desfire_commit_transaction(tag) };

        if result < 0 {
            Err("Failed to commit transaction.".to_string())
        } else {
            Ok(())
        }
    }

    /// Aborts a transaction on a Mifare DESFire tag
    pub fn desfire_abort_transaction(tag: freefare_sys::FreefareTag) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot abort transaction.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_desfire_abort_transaction(tag) };

        if result < 0 {
            Err("Failed to abort transaction.".to_string())
        } else {
            Ok(())
        }
    }

    /// Creates a new DES key for Mifare DESFire
    pub fn desfire_des_key_new(value: &mut [u8]) -> Result<freefare_sys::MifareDESFireKey, String> {
        if value.len() != 8 {
            return Err("DES key must be 8 bytes.".to_string());
        }

        let key = unsafe { freefare_sys::mifare_desfire_des_key_new(value.as_mut_ptr()) };

        if key.is_null() {
            Err("Failed to create DES key.".to_string())
        } else {
            Ok(key)
        }
    }

    /// Creates a linear record file with ISO parameters
    pub fn desfire_create_linear_record_file_iso(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        record_size: u32,
        max_number_of_records: u32,
        iso_file_id: u16,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot create linear record file.".to_string());
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
            Err("Failed to create linear record file with ISO parameters.".to_string())
        } else {
            Ok(())
        }
    }

    /// Creates a cyclic record file
    pub fn desfire_create_cyclic_record_file(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        record_size: u32,
        max_number_of_records: u32,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot create cyclic record file.".to_string());
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
            Err("Failed to create cyclic record file.".to_string())
        } else {
            Ok(())
        }
    }

    /// Deletes a file on a Mifare DESFire tag
    pub fn desfire_delete_file(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot delete file.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_desfire_delete_file(tag, file_no) };

        if result < 0 {
            Err("Failed to delete file.".to_string())
        } else {
            Ok(())
        }
    }

    /// Reads data from a file
    pub fn desfire_read_data(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        offset: off_t,
        length: usize,
        buffer: &mut [u8],
    ) -> Result<isize, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot read data.".to_string());
        }

        if buffer.len() < length {
            return Err("Buffer is too small for the specified length.".to_string());
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
            Err("Failed to read data.".to_string())
        } else {
            Ok(result)
        }
    }

    /// Writes data to a file
    pub fn desfire_write_data(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        offset: off_t,
        length: usize,
        data: &[u8],
    ) -> Result<isize, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot write data.".to_string());
        }

        if data.len() < length {
            return Err("Provided data length is smaller than specified.".to_string());
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
            Err("Failed to write data.".to_string())
        } else {
            Ok(result)
        }
    }

    /// Gets a value from a file
    pub fn desfire_get_value(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
    ) -> Result<i32, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot get value.".to_string());
        }

        let mut value: i32 = 0;

        let result =
            unsafe { freefare_sys::mifare_desfire_get_value(tag, file_no, &mut value as *mut _) };

        if result < 0 {
            Err("Failed to get value.".to_string())
        } else {
            Ok(value)
        }
    }

    /// Credits an amount to a file
    pub fn desfire_credit(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        amount: i32,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot credit amount.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_desfire_credit(tag, file_no, amount) };

        if result < 0 {
            Err("Failed to credit amount.".to_string())
        } else {
            Ok(())
        }
    }

    /// Debits an amount from a file
    pub fn desfire_debit(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        amount: i32,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot debit amount.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_desfire_debit(tag, file_no, amount) };

        if result < 0 {
            Err("Failed to debit amount.".to_string())
        } else {
            Ok(())
        }
    }

    /// Creates a 3K3DES application with ISO parameters
    pub fn desfire_create_application_3k3des_iso(
        tag: freefare_sys::FreefareTag,
        aid: freefare_sys::MifareDESFireAID,
        settings: u8,
        key_no: u8,
        want_iso_file_identifiers: bool,
        iso_file_id: u16,
        iso_file_name: &mut [u8],
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot create 3K3DES application with ISO parameters.".to_string());
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
            Err("Failed to create 3K3DES application with ISO parameters.".to_string())
        } else {
            Ok(())
        }
    }

    /// Creates an AES application with ISO parameters
    pub fn desfire_create_application_aes_iso(
        tag: freefare_sys::FreefareTag,
        aid: freefare_sys::MifareDESFireAID,
        settings: u8,
        key_no: u8,
        want_iso_file_identifiers: bool,
        iso_file_id: u16,
        iso_file_name: &mut [u8],
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot create AES application with ISO parameters.".to_string());
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
            Err("Failed to create AES application with ISO parameters.".to_string())
        } else {
            Ok(())
        }
    }

    /// Frees a list of application IDs
    pub fn desfire_free_application_ids(aids: *mut freefare_sys::MifareDESFireAID) -> Result<(), String> {
        if aids.is_null() {
            return Err("Application IDs are null. Cannot free.".to_string());
        }

        unsafe { freefare_sys::mifare_desfire_free_application_ids(aids) };

        Ok(())
    }

    /// Gets the amount of free memory on the tag
    pub fn desfire_free_mem(tag: freefare_sys::FreefareTag) -> Result<u32, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot retrieve free memory.".to_string());
        }

        let mut size: u32 = 0;

        let result = unsafe { freefare_sys::mifare_desfire_free_mem(tag, &mut size as *mut _) };

        if result < 0 {
            Err("Failed to retrieve free memory.".to_string())
        } else {
            Ok(size)
        }
    }

    /// Sets configuration options on the tag.
    pub fn desfire_set_configuration(
        tag: freefare_sys::FreefareTag,
        disable_format: bool,
        enable_random_uid: bool,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot set configuration.".to_string());
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_set_configuration(
                tag,
                disable_format as u8,
                enable_random_uid as u8,
            )
        };

        if result < 0 {
            Err("Failed to set configuration.".to_string())
        } else {
            Ok(())
        }
    }

    /// Sets the default key for the tag.
    pub fn desfire_set_default_key(
        tag: freefare_sys::FreefareTag,
        key: freefare_sys::MifareDESFireKey,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot set default key.".to_string());
        }

        if key.is_null() {
            return Err("Key is null. Cannot set default key.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_desfire_set_default_key(tag, key) };

        if result < 0 {
            Err("Failed to set default key.".to_string())
        } else {
            Ok(())
        }
    }

    /// Sets the ATS (Answer To Select) string for the tag
    pub fn desfire_set_ats(tag: freefare_sys::FreefareTag, ats: &mut [u8]) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot set ATS.".to_string());
        }

        let result = unsafe { freefare_sys::mifare_desfire_set_ats(tag, ats.as_mut_ptr()) };

        if result < 0 {
            Err("Failed to set ATS.".to_string())
        } else {
            Ok(())
        }
    }

    /// Gets the card UID
    pub fn desfire_get_card_uid(tag: freefare_sys::FreefareTag) -> Result<String, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot retrieve card UID.".to_string());
        }

        let mut uid_ptr: *mut ::std::os::raw::c_char = std::ptr::null_mut();

        let result = unsafe { freefare_sys::mifare_desfire_get_card_uid(tag, &mut uid_ptr as *mut _) };

        if result < 0 {
            return Err("Failed to retrieve card UID.".to_string());
        }

        let uid = unsafe { std::ffi::CStr::from_ptr(uid_ptr) }
            .to_str()
            .map_err(|_| "Failed to convert card UID to UTF-8 string.".to_string())?;

        Ok(uid.to_string())
    }

    /// Gets the file IDs on the tag
    pub fn desfire_get_file_ids(tag: freefare_sys::FreefareTag) -> Result<Vec<u8>, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot retrieve file IDs.".to_string());
        }

        let mut files_ptr: *mut u8 = std::ptr::null_mut();
        let mut count: usize = 0;

        let result = unsafe {
            freefare_sys::mifare_desfire_get_file_ids(tag, &mut files_ptr as *mut _, &mut count as *mut _)
        };

        if result < 0 {
            return Err("Failed to retrieve file IDs.".to_string());
        }

        let files = unsafe { std::slice::from_raw_parts(files_ptr, count).to_vec() };

        Ok(files)
    }

    /// Gets the ISO file IDs on the tag
    pub fn desfire_get_iso_file_ids(tag: freefare_sys::FreefareTag) -> Result<Vec<u16>, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot retrieve ISO file IDs.".to_string());
        }

        let mut files_ptr: *mut u16 = std::ptr::null_mut();
        let mut count: usize = 0;

        let result = unsafe {
            freefare_sys::mifare_desfire_get_iso_file_ids(tag, &mut files_ptr as *mut _, &mut count as *mut _)
        };

        if result < 0 {
            return Err("Failed to retrieve ISO file IDs.".to_string());
        }

        let files = unsafe { std::slice::from_raw_parts(files_ptr, count).to_vec() };

        Ok(files)
    }

    /// Gets the file settings of a file
    pub fn desfire_get_file_settings(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
    ) -> Result<freefare_sys::Struct_mifare_desfire_file_settings, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot retrieve file settings.".to_string());
        }

        let mut settings: freefare_sys::Struct_mifare_desfire_file_settings = unsafe { std::mem::zeroed() };

        let result =
            unsafe { freefare_sys::mifare_desfire_get_file_settings(tag, file_no, &mut settings as *mut _) };

        if result < 0 {
            Err("Failed to retrieve file settings.".to_string())
        } else {
            Ok(settings)
        }
    }

    /// Changes the settings of a file
    pub fn desfire_change_file_settings(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot change file settings.".to_string());
        }

        let result = unsafe {
            freefare_sys::mifare_desfire_change_file_settings(tag, file_no, communication_settings, access_rights)
        };

        if result < 0 {
            Err("Failed to change file settings.".to_string())
        } else {
            Ok(())
        }
    }

    /// Creates a standard data file.
    pub fn desfire_create_std_data_file(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        file_size: u32,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot create standard data file.".to_string());
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
            Err("Failed to create standard data file.".to_string())
        } else {
            Ok(())
        }
    }

    /// Creates a standard data file with ISO parameters
    pub fn desfire_create_std_data_file_iso(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        file_size: u32,
        iso_file_id: u16,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot create standard data file with ISO parameters.".to_string());
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
            Err("Failed to create standard data file with ISO parameters.".to_string())
        } else {
            Ok(())
        }
    }

    /// Creates a backup data file
    pub fn desfire_create_backup_data_file(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        file_size: u32,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot create backup data file.".to_string());
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
            Err("Failed to create backup data file.".to_string())
        } else {
            Ok(())
        }
    }

    /// Creates a backup data file with ISO parameters
    pub fn desfire_create_backup_data_file_iso(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        file_size: u32,
        iso_file_id: u16,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot create backup data file with ISO parameters.".to_string());
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
            Err("Failed to create backup data file with ISO parameters.".to_string())
        } else {
            Ok(())
        }
    }

    /// Creates a value file
    pub fn desfire_create_value_file(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        lower_limit: i32,
        upper_limit: i32,
        value: i32,
        limited_credit_enable: bool,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot create value file.".to_string());
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
            Err("Failed to create value file.".to_string())
        } else {
            Ok(())
        }
    }

    /// Creates a linear record file
    pub fn desfire_create_linear_record_file(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        record_size: u32,
        max_number_of_records: u32,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot create linear record file.".to_string());
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
            Err("Failed to create linear record file.".to_string())
        } else {
            Ok(())
        }
    }

    /// Creates a cyclic record file with ISO parameters
    pub fn desfire_create_cyclic_record_file_iso(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        record_size: u32,
        max_number_of_records: u32,
        iso_file_id: u16,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot create cyclic record file with ISO parameters.".to_string());
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
            Err("Failed to create cyclic record file with ISO parameters.".to_string())
        } else {
            Ok(())
        }
    }

    /// Reads data from a file with communication settings
    pub fn desfire_read_data_ex(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        offset: off_t,
        length: usize,
        buffer: &mut [u8],
        communication_settings: ::std::os::raw::c_int,
    ) -> Result<isize, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot read data.".to_string());
        }

        if buffer.len() < length {
            return Err("Buffer is too small for the specified length.".to_string());
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
            Err("Failed to read data with communication settings.".to_string())
        } else {
            Ok(result)
        }
    }

    /// Writes data to a file with communication settings
    pub fn desfire_write_data_ex(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        offset: off_t,
        length: usize,
        data: &[u8],
        communication_settings: ::std::os::raw::c_int,
    ) -> Result<isize, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot write data.".to_string());
        }

        if data.len() < length {
            return Err("Provided data length is smaller than specified.".to_string());
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
            Err("Failed to write data with communication settings.".to_string())
        } else {
            Ok(result)
        }
    }

    /// Gets a value from a file with communication settings
    pub fn desfire_get_value_ex(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        communication_settings: ::std::os::raw::c_int,
    ) -> Result<i32, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot get value.".to_string());
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
            Err("Failed to get value with communication settings.".to_string())
        } else {
            Ok(value)
        }
    }

    /// Credits an amount to a file with communication settings
    pub fn desfire_credit_ex(
        tag: freefare_sys::FreefareTag,
        file_no: u8,
        amount: i32,
        communication_settings: ::std::os::raw::c_int,
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot credit amount.".to_string());
        }

        let result =
            unsafe { freefare_sys::mifare_desfire_credit_ex(tag, file_no, amount, communication_settings) };

        if result < 0 {
            Err("Failed to credit amount with communication settings.".to_string())
        } else {
            Ok(())
        }
    }
    
}





