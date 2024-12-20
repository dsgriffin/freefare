use ::freefare_sys;

pub struct Mad {
    inner: freefare_sys::Mad,
}

impl Mad {
    /// Creates a new Mad instance from a version
    pub fn new(version: u8) -> Result<Self, String> {
        let raw_mad = unsafe { freefare_sys::mad_new(version) };

        if raw_mad.is_null() {
            Err(format!("Failed to create Mad instance for version: {}", version))
        } else {
            Ok(Self { inner: raw_mad })
        }
    }

    /// Reads a MAD from the given tag
    pub fn read(tag: freefare_sys::FreefareTag) -> Result<Self, String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot read MAD.".to_string());
        }

        let raw_mad = unsafe { freefare_sys::mad_read(tag) };

        if raw_mad.is_null() {
            Err("Failed to read MAD from the tag.".to_string())
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
    ) -> Result<(), String> {
        if tag.is_null() {
            return Err("Tag is null. Cannot write MAD.".to_string());
        }

        let result = unsafe { freefare_sys::mad_write(tag, self.inner, key_b_sector_00, key_b_sector_10) };

        if result < 0 {
            Err(format!("Failed to write MAD to the tag. Error code: {}", result))
        } else {
            Ok(())
        }
    }

    /// Gets the MAD version
    pub fn get_version(&self) -> Result<i32, String> {
        let result = unsafe { freefare_sys::mad_get_version(self.inner) };

        if result < 0 {
            Err(format!("Failed to get MAD version. Error code: {}", result))
        } else {
            Ok(result)
        }
    }

    /// Sets the MAD version
    pub fn set_version(&mut self, version: u8) -> Result<(), String> {
        unsafe { freefare_sys::mad_set_version(self.inner, version) };

        Ok(())
    }

    /// Gets the card publisher sector
    pub fn get_card_publisher_sector(&self) -> Result<freefare_sys::MifareClassicSectorNumber, String> {
        let result = unsafe { freefare_sys::mad_get_card_publisher_sector(self.inner) };

        Ok(result)
    }

    /// Sets the card publisher sector
    pub fn set_card_publisher_sector(
        &mut self,
        cps: freefare_sys::MifareClassicSectorNumber,
    ) -> Result<(), String> {
        let result = unsafe { freefare_sys::mad_set_card_publisher_sector(self.inner, cps) };

        if result < 0 {
            Err(format!("Failed to set card publisher sector. Error code: {}", result))
        } else {
            Ok(())
        }
    }

    /// Gets an AID from a sector
    pub fn get_aid(
        &self,
        sector: freefare_sys::MifareClassicSectorNumber,
        aid: &mut freefare_sys::MadAid,
    ) -> Result<(), String> {
        let result = unsafe { freefare_sys::mad_get_aid(self.inner, sector, aid) };

        if result < 0 {
            Err(format!("Failed to get AID. Error code: {}", result))
        } else {
            Ok(())
        }
    }

    /// Sets an AID for a sector
    pub fn set_aid(
        &mut self,
        sector: freefare_sys::MifareClassicSectorNumber,
        aid: freefare_sys::MadAid,
    ) -> Result<(), String> {
        let result = unsafe { freefare_sys::mad_set_aid(self.inner, sector, aid) };

        if result < 0 {
            Err(format!("Failed to set AID. Error code: {}", result))
        } else {
            Ok(())
        }
    }

    /// Checks if a sector is reserved
    pub fn sector_reserved(sector: freefare_sys::MifareClassicSectorNumber) -> bool {
        let result = unsafe { freefare_sys::mad_sector_reserved(sector) };

        result != 0
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
