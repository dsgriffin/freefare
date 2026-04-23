use crate::{Error, Result, Tag};
use ::freefare_sys;
use ::nfc;

/// Namespace for general libfreefare tag-management helpers.
pub struct Freefare;

impl Freefare {
    /// Retrieves the tags associated with an open NFC device.
    pub fn get_tags<'device>(device: &'device nfc::Device<'_>) -> Result<Vec<Tag<'device>>> {
        let raw_tags = unsafe { freefare_sys::freefare_get_tags(device.as_ptr()) };
        if raw_tags.is_null() {
            return Err(Error::new("No tags found."));
        }

        let mut tags = Vec::new();
        let mut current = raw_tags;

        while !unsafe { *current }.is_null() {
            let tag = unsafe { *current };
            let tag =
                Tag::from_raw(tag).ok_or_else(|| Error::new("libfreefare returned a null tag"))?;
            tags.push(tag);
            current = unsafe { current.add(1) };
        }

        unsafe { freefare_sys::freefare_free_tags(raw_tags) };

        Ok(tags)
    }

    /// Creates a tag wrapper from a raw `nfc_iso14443a_info` target.
    pub fn tag_new<'device>(
        device: &'device nfc::Device<'_>,
        target: nfc::ffi::nfc_iso14443a_info,
    ) -> Result<Tag<'device>> {
        let tag = unsafe { freefare_sys::freefare_tag_new(device.as_ptr(), target) };
        Tag::from_raw(tag).ok_or_else(|| Error::new("Failed to create new tag."))
    }
}
