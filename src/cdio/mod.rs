use anyhow::{Result, anyhow};
use libcdio_sys::{
    cdio_cddap_close, cdio_cddap_find_a_cdrom, cdio_cddap_open, cdio_cddap_track_firstsector,
    cdio_cddap_track_lastsector, cdio_cddap_tracks, cdio_paranoia_free, cdio_paranoia_init,
    cdio_paranoia_modeset, cdrom_drive_t, cdrom_paranoia_t, track_t,
};

/// Represents a CD drive handle without opening it.
pub struct CdDrive {
    pub(crate) drive: *mut cdrom_drive_t,
}

impl CdDrive {
    /// Find the first available CD-ROM drive. Returns `None` if no drive is found
    /// or if no disc is inserted.
    pub fn new() -> Option<Self> {
        let drive = unsafe { cdio_cddap_find_a_cdrom(0, std::ptr::null_mut()) };
        if drive.is_null() {
            None
        } else {
            Some(Self { drive })
        }
    }

    pub fn get_path(&self) -> String {
        let path = unsafe { std::ffi::CStr::from_ptr((*self.drive).cdda_device_name) }
            .to_string_lossy()
            .into_owned();
        path
    }
}

/// Represents an initialized CD drive with paranoia support.
/// Implements `Drop` to ensure resources are always cleaned up.
pub struct CdDevice {
    drive: *mut cdrom_drive_t,
    paranoia: *mut cdrom_paranoia_t,
    is_open: bool,
}

impl CdDevice {
    /// Create a `CdDevice` from a found drive without opening it yet.
    pub fn from_drive(drive: CdDrive) -> Self {
        let paranoia = std::ptr::null_mut();
        Self {
            drive: drive.drive,
            paranoia,
            is_open: false,
        }
    }

    /// Open the underlying CD-DA device and initialize paranoia support.
    pub fn open(&mut self) -> Result<()> {
        if self.is_open {
            return Ok(());
        }

        let open_result = unsafe { cdio_cddap_open(self.drive) };
        if open_result != 0 {
            unsafe { cdio_cddap_close(self.drive) };
            return Err(anyhow!(
                "Could not open drive for CD-DA (error {}). Is a CD-DA disc inserted?",
                open_result
            ));
        }

        let paranoia = unsafe { cdio_paranoia_init(self.drive) };
        if paranoia.is_null() {
            unsafe { cdio_cddap_close(self.drive) };
            return Err(anyhow!("Failed to initialize the cdio paranoia engine."));
        }

        // Full paranoia mode: overlap + verify + reconstruct
        unsafe { cdio_paranoia_modeset(paranoia, 0xff) };

        self.paranoia = paranoia;
        self.is_open = true;
        Ok(())
    }

    pub fn track_count(&mut self) -> Result<u32> {
        self.ensure_open()?;
        let total = unsafe { cdio_cddap_tracks(self.drive) };
        if total == 0xFF || total == 0 {
            return Err(anyhow!(
                "No audio tracks found on disc (cdio_cddap_tracks returned {})",
                total
            ));
        }
        Ok(total as u32)
    }

    /// Wrapper around a CDIO LBA call that returns an error for negative results.
    fn read_lba(
        &self,
        track: track_t,
        f: unsafe extern "C" fn(*mut cdrom_drive_t, track_t) -> i32,
    ) -> Result<i32> {
        let lba = unsafe { f(self.drive, track) };
        if lba < 0 {
            return Err(anyhow!("Could not read sector offset for track {}.", track));
        }
        Ok(lba)
    }

    pub fn track_first_sector(&mut self, track: track_t) -> Result<i32> {
        self.ensure_open()?;
        Self::read_lba(self, track, cdio_cddap_track_firstsector)
    }

    pub fn track_last_sector(&mut self, track: track_t) -> Result<i32> {
        self.ensure_open()?;
        Self::read_lba(self, track, cdio_cddap_track_lastsector)
    }

    /// Returns a raw pointer to the underlying paranoia engine.
    /// This is needed for FFI calls that the ripper makes directly.
    pub(crate) fn paranoia_ptr(&self) -> *mut cdrom_paranoia_t {
        self.paranoia
    }

    fn ensure_open(&mut self) -> Result<()> {
        if !self.is_open {
            self.open()?;
        }
        Ok(())
    }
}

impl Drop for CdDevice {
    fn drop(&mut self) {
        unsafe {
            cdio_paranoia_free(self.paranoia);
            cdio_cddap_close(self.drive);
        }
    }
}
