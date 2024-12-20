use ::nfc_sys;
use ::freefare_sys;

use libc::size_t;

pub struct Felica;

impl Felica {
    /// Checks if the NFC target is a Felica card
    pub fn taste(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> Result<bool, String> {
        if device.is_null() {
            return Err("Device is null. Cannot check Felica target.".to_string());
        }

        let result = unsafe { freefare_sys::felica_taste(device, target) };
        Ok(result != 0)
    }

    /// Creates a new Felica tag object
    pub fn tag_new(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> Result<freefare_sys::FreefareTag, String> {
        if device.is_null() {
            return Err("Device is null. Cannot create Felica tag.".to_string());
        }

        let tag = unsafe { freefare_sys::felica_tag_new(device, target) };
        if tag.is_null() {
            Err("Failed to create Felica tag.".to_string())
        } else {
            Ok(tag)
        }
    }

    /// Frees a Felica tag object
    pub fn tag_free(tag: freefare_sys::FreefareTag) {
        if !tag.is_null() {
            unsafe { freefare_sys::felica_tag_free(tag) };
        }
    }

    /// Reads a block from the Felica card
    pub fn read(
        tag: freefare_sys::FreefareTag,
        service: u16,
        block: u8,
        buffer: &mut [u8],
    ) -> Result<usize, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot read Felica block.".to_string());
        }

        let length = buffer.len() as size_t;
        let result = unsafe {
            freefare_sys::felica_read(tag, service, block, buffer.as_mut_ptr(), length)
        };

        if result < 0 {
            Err("Failed to read Felica block.".to_string())
        } else {
            Ok(result as usize)
        }
    }

    /// Reads multiple blocks from the Felica card
    pub fn read_ex(
        tag: freefare_sys::FreefareTag,
        service: u16,
        block_count: u8,
        blocks: &mut [u8],
        buffer: &mut [u8],
    ) -> Result<usize, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot read Felica blocks.".to_string());
        }

        if blocks.len() < block_count as usize {
            return Err("Block buffer is too small for the specified block count.".to_string());
        }

        let length = buffer.len() as size_t;
        let result = unsafe {
            freefare_sys::felica_read_ex(
                tag,
                service,
                block_count,
                blocks.as_mut_ptr(),
                buffer.as_mut_ptr(),
                length,
            )
        };

        if result < 0 {
            Err("Failed to read Felica blocks.".to_string())
        } else {
            Ok(result as usize)
        }
    }

    /// Writes a block to the Felica card
    pub fn write(
        tag: freefare_sys::FreefareTag,
        service: u16,
        block: u8,
        buffer: &[u8],
    ) -> Result<usize, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot write Felica block.".to_string());
        }

        let length = buffer.len() as size_t;
        let result = unsafe {
            freefare_sys::felica_write(tag, service, block, buffer.as_ptr() as *mut u8, length)
        };

        if result < 0 {
            Err("Failed to write Felica block.".to_string())
        } else {
            Ok(result as usize)
        }
    }

    /// Writes multiple blocks to the Felica card
    pub fn write_ex(
        tag: freefare_sys::FreefareTag,
        service: u16,
        block_count: u8,
        blocks: &mut [u8],
        buffer: &[u8],
    ) -> Result<usize, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot write Felica blocks.".to_string());
        }

        if blocks.len() < block_count as usize {
            return Err("Block buffer is too small for the specified block count.".to_string());
        }

        let length = buffer.len() as size_t;
        let result = unsafe {
            freefare_sys::felica_write_ex(
                tag,
                service,
                block_count,
                blocks.as_mut_ptr(),
                buffer.as_ptr() as *mut u8,
                length,
            )
        };

        if result < 0 {
            Err("Failed to write Felica blocks.".to_string())
        } else {
            Ok(result as usize)
        }
    }
}
