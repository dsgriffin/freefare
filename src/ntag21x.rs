use nfc_sys;
use freefare_sys;

pub struct NTAG21x;

impl NTAG21x {
    /// Checks if the NFC target is an NTAG21x card
    pub fn taste(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> Result<bool, String> {
        if device.is_null() {
            return Err("Device is null. Cannot check NTAG21x target.".to_string());
        }

        let result = unsafe { freefare_sys::ntag21x_taste(device, target) };
        Ok(result)
    }

    /// Creates a new NTAG21x tag object
    pub fn tag_new(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> Result<freefare_sys::FreefareTag, String> {
        if device.is_null() {
            return Err("Device is null. Cannot create NTAG21x tag.".to_string());
        }

        let tag = unsafe { freefare_sys::ntag21x_tag_new(device, target) };
        if tag.is_null() {
            Err("Failed to create NTAG21x tag.".to_string())
        } else {
            Ok(tag)
        }
    }

    /// Reuses an existing NTAG21x tag object
    pub fn tag_reuse(tag: freefare_sys::FreefareTag) -> Result<freefare_sys::FreefareTag, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot reuse NTAG21x tag.".to_string());
        }

        let reused_tag = unsafe { freefare_sys::ntag21x_tag_reuse(tag) };
        if reused_tag.is_null() {
            Err("Failed to reuse NTAG21x tag.".to_string())
        } else {
            Ok(reused_tag)
        }
    }

    /// Frees an NTAG21x tag object
    pub fn tag_free(tag: freefare_sys::FreefareTag) {
        if !tag.is_null() {
            unsafe { freefare_sys::ntag21x_tag_free(tag) };
        }
    }

    /// Connects to an NTAG21x tag
    pub fn connect(tag: freefare_sys::FreefareTag) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot connect to NTAG21x tag.".to_string());
        }

        let result = unsafe { freefare_sys::ntag21x_connect(tag) };
        if result < 0 {
            Err("Failed to connect to NTAG21x tag.".to_string())
        } else {
            Ok(())
        }
    }

    /// Disconnects from an NTAG21x tag
    pub fn disconnect(tag: freefare_sys::FreefareTag) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot disconnect from NTAG21x tag.".to_string());
        }

        let result = unsafe { freefare_sys::ntag21x_disconnect(tag) };
        if result < 0 {
            Err("Failed to disconnect from NTAG21x tag.".to_string())
        } else {
            Ok(())
        }
    }

    /// Gets information about the NTAG21x tag
    pub fn get_info(tag: freefare_sys::FreefareTag) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot get info of NTAG21x tag.".to_string());
        }

        let result = unsafe { freefare_sys::ntag21x_get_info(tag) };
        if result < 0 {
            Err("Failed to get info for NTAG21x tag.".to_string())
        } else {
            Ok(())
        }
    }

    /// Reads a block from the NTAG21x card
    pub fn read(tag: freefare_sys::FreefareTag, page: u8, buffer: &mut [u8]) -> Result<usize, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot read NTAG21x block.".to_string());
        }

        let result = unsafe { freefare_sys::ntag21x_read(tag, page, buffer.as_mut_ptr()) };
        if result < 0 {
            Err("Failed to read NTAG21x block.".to_string())
        } else {
            Ok(result as usize)
        }
    }

    /// Writes data to a block on the NTAG21x card
    pub fn write(tag: freefare_sys::FreefareTag, page: u8, buffer: &[u8]) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot write NTAG21x block.".to_string());
        }

        if buffer.len() != 4 {
            return Err("NTAG21x block must be exactly 4 bytes.".to_string());
        }

        let result = unsafe { freefare_sys::ntag21x_write(tag, page, buffer.as_ptr() as *mut u8) };
        if result < 0 {
            Err("Failed to write NTAG21x block.".to_string())
        } else {
            Ok(())
        }
    }

    /// Reads multiple pages from the NTAG21x card
    pub fn fast_read(tag: freefare_sys::FreefareTag, start_page: u8, end_page: u8, buffer: &mut [u8]) -> Result<usize, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot perform fast read on NTAG21x tag.".to_string());
        }

        let result = unsafe {
            freefare_sys::ntag21x_fast_read(tag, start_page, end_page, buffer.as_mut_ptr())
        };

        if result < 0 {
            Err("Failed to perform fast read on NTAG21x tag.".to_string())
        } else {
            Ok(result as usize)
        }
    }

    /// Authenticates the NTAG21x tag with a key
    pub fn authenticate(tag: freefare_sys::FreefareTag, key: freefare_sys::NTAG21xKey) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot authenticate NTAG21x tag.".to_string());
        }

        let result = unsafe { freefare_sys::ntag21x_authenticate(tag, key) };
        if result < 0 {
            Err("Failed to authenticate NTAG21x tag.".to_string())
        } else {
            Ok(())
        }
    }

    /// Checks if the provided tag is an NTAG21x
    pub fn is_ntag21x(tag: freefare_sys::FreefareTag) -> bool {
        if tag.is_null() {
            return false;
        }

        unsafe { freefare_sys::is_ntag21x(tag) }
    }

    /// Creates a new NTAG21x key object
    pub fn key_new(data: &[u8; 16], pack: &[u8; 2]) -> Result<freefare_sys::NTAG21xKey, String> {
        let key = unsafe { freefare_sys::ntag21x_key_new(data.as_ptr(), pack.as_ptr()) };
        if key.is_null() {
            Err("Failed to create NTAG21x key.".to_string())
        } else {
            Ok(key)
        }
    }

    /// Frees an NTAG21x key object
    pub fn key_free(key: freefare_sys::NTAG21xKey) {
        if !key.is_null() {
            unsafe { freefare_sys::ntag21x_key_free(key) };
        }
    }

    /// Gets the subtype of an NTAG21x tag
    pub fn get_subtype(tag: freefare_sys::FreefareTag) -> Result<freefare_sys::ntag_tag_subtype, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot get subtype.".to_string());
        }

        let subtype = unsafe { freefare_sys::ntag21x_get_subtype(tag) };
        Ok(subtype)
    }

    /// Gets the last page of an NTAG21x tag
    pub fn get_last_page(tag: freefare_sys::FreefareTag) -> Result<u8, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot get last page.".to_string());
        }

        let last_page = unsafe { freefare_sys::ntag21x_get_last_page(tag) };
        Ok(last_page)
    }

    /// Reads the signature of an NTAG21x tag
    pub fn read_signature(tag: freefare_sys::FreefareTag, buffer: &mut [u8; 32]) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot read signature.".to_string());
        }

        let result = unsafe { freefare_sys::ntag21x_read_signature(tag, buffer.as_mut_ptr()) };
        if result < 0 {
            Err("Failed to read signature.".to_string())
        } else {
            Ok(())
        }
    }

    /// Sets the password of an NTAG21x tag
    pub fn set_pwd(tag: freefare_sys::FreefareTag, password: &[u8; 4]) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot set password.".to_string());
        }

        let result = unsafe { freefare_sys::ntag21x_set_pwd(tag, password.as_ptr() as *mut u8) };
        if result < 0 {
            Err("Failed to set password.".to_string())
        } else {
            Ok(())
        }
    }
    
    /// Sets the pack of an NTAG21x tag
    pub fn set_pack(tag: freefare_sys::FreefareTag, pack: &[u8; 2]) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot set pack.".to_string());
        }

        let result = unsafe { freefare_sys::ntag21x_set_pack(tag, pack.as_ptr() as *mut u8) };
        if result < 0 {
            Err("Failed to set pack.".to_string())
        } else {
            Ok(())
        }
    }

    /// Sets the NTAG21x key
    pub fn set_key(tag: freefare_sys::FreefareTag, key: freefare_sys::NTAG21xKey) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot set key.".to_string());
        }

        let result = unsafe { freefare_sys::ntag21x_set_key(tag, key) };
        if result < 0 {
            Err("Failed to set NTAG21x key.".to_string())
        } else {
            Ok(())
        }
    }

    /// Sets the authentication byte for NTAG21x
    pub fn set_auth(tag: freefare_sys::FreefareTag, byte: u8) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot set authentication byte.".to_string());
        }

        let result = unsafe { freefare_sys::ntag21x_set_auth(tag, byte) };
        if result < 0 {
            Err("Failed to set NTAG21x authentication byte.".to_string())
        } else {
            Ok(())
        }
    }

    /// Gets the authentication byte for NTAG21x
    pub fn get_auth(tag: freefare_sys::FreefareTag) -> Result<u8, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot get authentication byte.".to_string());
        }

        let mut byte: u8 = 0;
        let result = unsafe { freefare_sys::ntag21x_get_auth(tag, &mut byte) };
        if result < 0 {
            Err("Failed to get NTAG21x authentication byte.".to_string())
        } else {
            Ok(byte)
        }
    }

    /// Enables access for NTAG21x
    pub fn access_enable(tag: freefare_sys::FreefareTag, byte: u8) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot enable access.".to_string());
        }

        let result = unsafe { freefare_sys::ntag21x_access_enable(tag, byte) };
        if result < 0 {
            Err("Failed to enable NTAG21x access.".to_string())
        } else {
            Ok(())
        }
    }

    /// Disables access for NTAG21x
    pub fn access_disable(tag: freefare_sys::FreefareTag, byte: u8) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot disable access.".to_string());
        }

        let result = unsafe { freefare_sys::ntag21x_access_disable(tag, byte) };
        if result < 0 {
            Err("Failed to disable NTAG21x access.".to_string())
        } else {
            Ok(())
        }
    }

    /// Gets the access byte for NTAG21x
    pub fn get_access(tag: freefare_sys::FreefareTag) -> Result<u8, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot get access byte.".to_string());
        }

        let mut byte: u8 = 0;
        let result = unsafe { freefare_sys::ntag21x_get_access(tag, &mut byte) };
        if result < 0 {
            Err("Failed to get NTAG21x access byte.".to_string())
        } else {
            Ok(byte)
        }
    }

    /// Checks access for NTAG21x
    pub fn check_access(tag: freefare_sys::FreefareTag, byte: u8) -> Result<bool, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot check access.".to_string());
        }

        let mut result: bool = false;
        let status = unsafe { freefare_sys::ntag21x_check_access(tag, byte, &mut result) };
        if status < 0 {
            Err("Failed to check NTAG21x access.".to_string())
        } else {
            Ok(result)
        }
    }

    /// Gets the authentication limit for NTAG21x
    pub fn get_authentication_limit(tag: freefare_sys::FreefareTag) -> Result<u8, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot get authentication limit.".to_string());
        }

        let mut byte: u8 = 0;
        let result = unsafe { freefare_sys::ntag21x_get_authentication_limit(tag, &mut byte) };
        if result < 0 {
            Err("Failed to get NTAG21x authentication limit.".to_string())
        } else {
            Ok(byte)
        }
    }

    /// Sets the authentication limit for NTAG21x
    pub fn set_authentication_limit(tag: freefare_sys::FreefareTag, byte: u8) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot set authentication limit.".to_string());
        }

        let result = unsafe { freefare_sys::ntag21x_set_authentication_limit(tag, byte) };
        if result < 0 {
            Err("Failed to set NTAG21x authentication limit.".to_string())
        } else {
            Ok(())
        }
    }

    /// Reads 4 blocks from the NTAG21x card
    pub fn read4(tag: freefare_sys::FreefareTag, page: u8, buffer: &mut [u8; 16]) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot read 4 blocks.".to_string());
        }

        let result = unsafe { freefare_sys::ntag21x_read4(tag, page, buffer.as_mut_ptr()) };
        if result < 0 {
            Err("Failed to read 4 blocks.".to_string())
        } else {
            Ok(())
        }
    }

    /// Performs a fast read of 4 blocks
    pub fn fast_read4(tag: freefare_sys::FreefareTag, page: u8, buffer: &mut [u8; 16]) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot perform fast read of 4 blocks.".to_string());
        }

        let result = unsafe { freefare_sys::ntag21x_fast_read4(tag, page, buffer.as_mut_ptr()) };
        if result < 0 {
            Err("Failed to perform fast read of 4 blocks.".to_string())
        } else {
            Ok(())
        }
    }

    /// Reads the counter value
    pub fn read_cnt(tag: freefare_sys::FreefareTag) -> Result<u32, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot read counter.".to_string());
        }

        let mut data: [u8; 4] = [0; 4];
        let result = unsafe { freefare_sys::ntag21x_read_cnt(tag, data.as_mut_ptr()) };
        if result < 0 {
            Err("Failed to read NTAG21x counter.".to_string())
        } else {
            let counter = u32::from_be_bytes(data);
            Ok(counter)
        }
    }

    /// Performs a compatibility write to a page
    pub fn compatibility_write(tag: freefare_sys::FreefareTag, page: u8, buffer: &[u8; 4]) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot perform compatibility write.".to_string());
        }

        let result = unsafe { freefare_sys::ntag21x_compatibility_write(tag, page, buffer.as_ptr() as *mut u8) };
        if result < 0 {
            Err("Failed to perform compatibility write.".to_string())
        } else {
            Ok(())
        }
    }

    /// Checks if authentication is supported on the device
    pub fn is_auth_supported(device: *mut nfc_sys::nfc_device, nai: nfc_sys::nfc_iso14443a_info) -> bool {
        if device.is_null() {
            return false;
        }

        unsafe { freefare_sys::ntag21x_is_auth_supported(device, nai) }
    }

}

