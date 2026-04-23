use crate::{Error, Result};
use ::freefare_sys;

/// Owned wrapper around a libfreefare MAD handle.
///
/// The underlying native value is released automatically with `mad_free` when
/// this wrapper is dropped.
pub struct Mad {
    inner: freefare_sys::Mad,
}

impl Mad {
    /// Creates a new Mad instance from a version
    pub fn new(version: u8) -> Result<Self> {
        let raw_mad = unsafe { freefare_sys::mad_new(version) };

        if raw_mad.is_null() {
            Err(Error::new(format!(
                "Failed to create Mad instance for version: {}",
                version
            )))
        } else {
            Ok(Self { inner: raw_mad })
        }
    }

    /// Reads a MAD from the given tag
    pub fn read(tag: freefare_sys::FreefareTag) -> Result<Self> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot read MAD."));
        }

        let raw_mad = unsafe { freefare_sys::mad_read(tag) };

        if raw_mad.is_null() {
            Err(Error::new("Failed to read MAD from the tag."))
        } else {
            Ok(Self { inner: raw_mad })
        }
    }

    /// Writes a MAD to the given tag using the specified keys
    pub fn write(
        &self,
        tag: freefare_sys::FreefareTag,
        key_b_sector_00: *const freefare_sys::MifareClassicKey,
        key_b_sector_10: *const freefare_sys::MifareClassicKey,
    ) -> Result<()> {
        if tag.is_null() {
            return Err(Error::new("Tag is null. Cannot write MAD."));
        }

        let result =
            unsafe { freefare_sys::mad_write(tag, self.inner, key_b_sector_00, key_b_sector_10) };

        if result < 0 {
            Err(Error::new(format!(
                "Failed to write MAD to the tag. Error code: {}",
                result
            )))
        } else {
            Ok(())
        }
    }

    /// Gets the MAD version
    pub fn get_version(&self) -> Result<i32> {
        let result = unsafe { freefare_sys::mad_get_version(self.inner) };

        if result < 0 {
            Err(Error::new(format!(
                "Failed to get MAD version. Error code: {}",
                result
            )))
        } else {
            Ok(result)
        }
    }

    /// Sets the MAD version
    pub fn set_version(&mut self, version: u8) -> Result<()> {
        unsafe { freefare_sys::mad_set_version(self.inner, version) };

        Ok(())
    }

    /// Gets the card publisher sector
    pub fn get_card_publisher_sector(&self) -> Result<freefare_sys::MifareClassicSectorNumber> {
        let result = unsafe { freefare_sys::mad_get_card_publisher_sector(self.inner) };

        Ok(result)
    }

    /// Sets the card publisher sector
    pub fn set_card_publisher_sector(
        &mut self,
        cps: freefare_sys::MifareClassicSectorNumber,
    ) -> Result<()> {
        let result = unsafe { freefare_sys::mad_set_card_publisher_sector(self.inner, cps) };

        if result < 0 {
            Err(Error::new(format!(
                "Failed to set card publisher sector. Error code: {}",
                result
            )))
        } else {
            Ok(())
        }
    }

    /// Gets an AID from a sector
    pub fn get_aid(
        &self,
        sector: freefare_sys::MifareClassicSectorNumber,
        aid: &mut freefare_sys::MadAid,
    ) -> Result<()> {
        let result = unsafe { freefare_sys::mad_get_aid(self.inner, sector, aid) };

        if result < 0 {
            Err(Error::new(format!(
                "Failed to get AID. Error code: {}",
                result
            )))
        } else {
            Ok(())
        }
    }

    /// Sets an AID for a sector
    pub fn set_aid(
        &mut self,
        sector: freefare_sys::MifareClassicSectorNumber,
        aid: freefare_sys::MadAid,
    ) -> Result<()> {
        let result = unsafe { freefare_sys::mad_set_aid(self.inner, sector, aid) };

        if result < 0 {
            Err(Error::new(format!(
                "Failed to set AID. Error code: {}",
                result
            )))
        } else {
            Ok(())
        }
    }

    /// Checks if a sector is reserved
    pub fn sector_reserved(sector: freefare_sys::MifareClassicSectorNumber) -> bool {
        unsafe { freefare_sys::mad_sector_reserved(sector) }
    }
}

impl Drop for Mad {
    /// Ensures Mad resources are freed automatically
    fn drop(&mut self) {
        unsafe {
            freefare_sys::mad_free(self.inner);
        }
    }
}
