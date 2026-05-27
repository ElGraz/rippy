use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use libcdio_sys::cdrom_drive_t;
use libcdio_sys::track_t;
use sha1::{Digest, Sha1};

#[cfg(test)]
mod tests;

pub fn compute_disc_id(
    drive_ptr: *mut cdrom_drive_t,
    first_track: track_t,
    total_tracks: track_t,
) -> Result<String> {
    let last_track = first_track + total_tracks - 1;

    // The MusicBrainz spec stores offsets as physical TOC frame numbers, which include
    // the 150-sector lead-in pregap. libcdio returns LBA values starting at 0, so we
    // must add 150 to every offset to match what the spec expects.
    const PREGAP: i32 = 150;

    let lead_out_lba = unsafe { libcdio_sys::cdio_cddap_track_lastsector(drive_ptr, last_track) };
    if lead_out_lba < 0 {
        return Err(anyhow!("Could not read lead-out sector from CD TOC."));
    }
    let lead_out_offset = lead_out_lba + 1 + PREGAP;

    // offsets[0] = lead-out, offsets[1..=99] = per-track (unused slots stay 0)
    let mut offsets = [0i32; 100];
    offsets[0] = lead_out_offset;
    for t in first_track..=last_track {
        let lba = unsafe { libcdio_sys::cdio_cddap_track_firstsector(drive_ptr, t) };
        if lba < 0 {
            return Err(anyhow!("Could not read sector offset for track {}.", t));
        }
        offsets[t as usize] = lba + PREGAP;
    }

    // Hash input: first_track(2) + last_track(2) + offsets[0..=99](each 8), all uppercase hex
    let mut hash_input = format!("{:02X}{:02X}", first_track, last_track);
    for slot in 0usize..=99 {
        hash_input.push_str(&format!("{:08X}", offsets[slot]));
    }

    // SHA-1 → standard Base64, then MusicBrainz substitutions: + → .  / → _  = → -
    let digest = Sha1::digest(hash_input.as_bytes());
    let b64 = STANDARD.encode(digest);
    Ok(b64.replace('+', ".").replace('/', "_").replace('=', "-"))
}
