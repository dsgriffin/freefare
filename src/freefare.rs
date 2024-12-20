use ::nfc_sys;
use ::freefare_sys;

use std::ffi::{CStr, CString};

pub struct Freefare;

impl Freefare {
    /// Retrieves the tags associated with an NFC device
    pub fn get_tags(device: *mut nfc_sys::nfc_device) -> Result<Vec<freefare_sys::FreefareTag>, String> {
        if device.is_null() {
            return Err("Device is null. Cannot get tags.".to_string());
        }

        let raw_tags = unsafe { freefare_sys::freefare_get_tags(device) };
        if raw_tags.is_null() {
            return Err("No tags found.".to_string());
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
    pub fn tag_new(device: *mut nfc_sys::nfc_device, target: nfc_sys::nfc_target) -> Result<freefare_sys::FreefareTag, String> {
        if device.is_null() {
            return Err("Device is null. Cannot create new tag.".to_string());
        }

        let tag = unsafe { freefare_sys::freefare_tag_new(device, target) };
        if tag.is_null() {
            Err("Failed to create new tag.".to_string())
        } else {
            Ok(tag)
        }
    }

    /// Gets the type of the tag
    pub fn get_tag_type(tag: freefare_sys::FreefareTag) -> Result<freefare_sys::Enum_freefare_tag_type, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot get tag type.".to_string());
        }

        Ok(unsafe { freefare_sys::freefare_get_tag_type(tag) })
    }

    /// Gets the friendly name of the tag
    pub fn get_tag_friendly_name(tag: freefare_sys::FreefareTag) -> Result<String, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot get friendly name.".to_string());
        }

        let name_ptr = unsafe { freefare_sys::freefare_get_tag_friendly_name(tag) };
        if name_ptr.is_null() {
            Err("Failed to get tag friendly name.".to_string())
        } else {
            Ok(unsafe { CStr::from_ptr(name_ptr) }.to_string_lossy().into_owned())
        }
    }

    /// Gets the UID of the tag
    pub fn get_tag_uid(tag: freefare_sys::FreefareTag) -> Result<String, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot get UID.".to_string());
        }

        let uid_ptr = unsafe { freefare_sys::freefare_get_tag_uid(tag) };
        if uid_ptr.is_null() {
            Err("Failed to get tag UID.".to_string())
        } else {
            let uid = unsafe { CStr::from_ptr(uid_ptr) }.to_string_lossy().into_owned();

            unsafe { freefare_sys::freefare_free_tag(uid_ptr as *mut _) };

            Ok(uid)
        }
    }

    /// Frees a single tag
    pub fn free_tag(tag: freefare_sys::FreefareTag) {
        if !tag.is_null() {
            unsafe { freefare_sys::freefare_free_tag(tag) };
        }
    }

    /// Checks if the selected tag is present
    pub fn selected_tag_is_present(device: *mut nfc_sys::nfc_device) -> bool {
        if device.is_null() {
            false
        } else {
            unsafe { freefare_sys::freefare_selected_tag_is_present(device) != 0 }
        }
    }

    /// Gets the last error message for a tag
    pub fn strerror(tag: freefare_sys::FreefareTag) -> Result<String, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot get error string.".to_string());
        }

        let err_ptr = unsafe { freefare_sys::freefare_strerror(tag) };
        if err_ptr.is_null() {
            Err("No error message available.".to_string())
        } else {
            Ok(unsafe { CStr::from_ptr(err_ptr) }.to_string_lossy().into_owned())
        }
    }

    /// Writes an error string to a provided buffer
    pub fn strerror_r(tag: freefare_sys::FreefareTag, buffer: &mut [u8]) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot write error string.".to_string());
        }

        let result = unsafe {
            freefare_sys::freefare_strerror_r(tag, buffer.as_mut_ptr() as *mut _, buffer.len())
        };

        if result < 0 {
            Err("Failed to write error string to buffer.".to_string())
        } else {
            Ok(())
        }
    }

    /// Prints an error message associated with a tag and a custom string
    pub fn perror(tag: freefare_sys::FreefareTag, message: &str) {
        if tag.is_null() {
            eprintln!("Cannot print error for null tag.");
            return;
        }

        let c_message = CString::new(message).unwrap_or_else(|_| CString::new("Invalid message").unwrap());
        unsafe { freefare_sys::freefare_perror(tag, c_message.as_ptr()) };
    }
}
