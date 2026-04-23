use crate::{check_status, copy_c_string, copy_malloc_c_string, Error, Result};
use ::freefare_sys;
use ::nfc_sys;
use std::ffi::CString;

/// Namespace for general libfreefare tag-management helpers.
pub struct Freefare;

impl Freefare {
    /// Retrieves the tags associated with an NFC device
    pub fn get_tags(device: *mut nfc_sys::nfc_device) -> Result<Vec<freefare_sys::FreefareTag>> {
        if device.is_null() {
            return Err(Error::new("Device is null. Cannot get tags."));
        }

        let raw_tags = unsafe { freefare_sys::freefare_get_tags(device) };
        if raw_tags.is_null() {
            return Err(Error::new("No tags found."));
        }

        let mut tags = Vec::new();
        let mut current = raw_tags;

        while !unsafe { *current }.is_null() {
            tags.push(unsafe { *current });
            current = unsafe { current.add(1) };
        }

        // Free the raw tags array after copying
        unsafe { freefare_sys::freefare_free_tags(raw_tags) };

        Ok(tags)
    }

    /// Creates a new FreefareTag associated with the given device and NFC target
    pub fn tag_new(
        device: *mut nfc_sys::nfc_device,
        target: nfc_sys::nfc_iso14443a_info,
    ) -> Result<freefare_sys::FreefareTag> {
        if device.is_null() {
            return Err(Error::new("Device is null. Cannot create new tag."));
        }

        let tag = unsafe { freefare_sys::freefare_tag_new(device, target) };
        if tag.is_null() {
            Err(Error::new("Failed to create new tag."))
        } else {
            Ok(tag)
        }
    }

    /// Gets the type of the tag
    pub fn get_tag_type(tag: freefare_sys::FreefareTag) -> Result<freefare_sys::FreefareTagType> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot get tag type."));
        }

        Ok(unsafe { freefare_sys::freefare_get_tag_type(tag) })
    }

    /// Gets the friendly name of the tag
    pub fn get_tag_friendly_name(tag: freefare_sys::FreefareTag) -> Result<String> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot get friendly name."));
        }

        let name_ptr = unsafe { freefare_sys::freefare_get_tag_friendly_name(tag) };
        if name_ptr.is_null() {
            Err(Error::new("Failed to get tag friendly name."))
        } else {
            Ok(copy_c_string(name_ptr))
        }
    }

    /// Gets the UID of the tag
    pub fn get_tag_uid(tag: freefare_sys::FreefareTag) -> Result<String> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot get UID."));
        }

        let uid_ptr = unsafe { freefare_sys::freefare_get_tag_uid(tag) };
        if uid_ptr.is_null() {
            Err(Error::new("Failed to get tag UID."))
        } else {
            copy_malloc_c_string(uid_ptr)
        }
    }

    /// Frees a single tag
    pub fn free_tag(tag: freefare_sys::FreefareTag) {
        if !tag.is_null() {
            unsafe { freefare_sys::freefare_free_tag(tag) };
        }
    }

    /// Frees the memory allocated for a list of tags
    pub fn free_tags(tags: *mut freefare_sys::FreefareTag) -> Result<()> {
        if tags.is_null() {
            return Err(Error::new("Tags pointer is null. Nothing to free."));
        }

        unsafe {
            freefare_sys::freefare_free_tags(tags);
        }

        Ok(())
    }

    /// Gets the last error message for a tag
    pub fn strerror(tag: freefare_sys::FreefareTag) -> Result<String> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot get error string."));
        }

        let err_ptr = unsafe { freefare_sys::freefare_strerror(tag) };
        if err_ptr.is_null() {
            Err(Error::new("No error message available."))
        } else {
            Ok(copy_c_string(err_ptr))
        }
    }

    /// Writes an error string to a provided buffer
    pub fn strerror_r(tag: freefare_sys::FreefareTag, buffer: &mut [u8]) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot write error string."));
        }

        let result = unsafe {
            freefare_sys::freefare_strerror_r(tag, buffer.as_mut_ptr() as *mut _, buffer.len())
        };

        check_status(tag, result, "Failed to write error string to buffer.")
    }

    /// Prints an error message associated with a tag and a custom string
    pub fn perror(tag: freefare_sys::FreefareTag, message: &str) {
        if tag.is_null() {
            eprintln!("Cannot print error for null tag.");
            return;
        }

        let c_message =
            CString::new(message).unwrap_or_else(|_| CString::new("Invalid message").unwrap());
        unsafe { freefare_sys::freefare_perror(tag, c_message.as_ptr()) };
    }
}
