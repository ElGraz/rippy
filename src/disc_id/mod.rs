use anyhow::Result;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use sha1::{Digest, Sha1};
use std::fmt::Write;

use crate::cdio::CdDevice;

/// The MusicBrainz spec stores offsets as physical TOC frame numbers, which include
/// the 150-sector lead-in pregap. libcdio returns LBA values starting at 0, so we
/// must add 150 to every offset to match what the spec expects.
const PREGAP_OFFSET: i32 = 150;

/// Compute a MusicBrainz Disc ID from the CD's TOC data.
pub fn compute(device: &mut CdDevice, first_track: u32, total_tracks: u32) -> Result<String> {
    let last_track = first_track + total_tracks - 1;

    // Build per-track sector offsets (as expected by the MusicBrainz spec).
    // offsets[0] = lead-out position; offsets[1..=99] = per-track (unused slots stay 0).
    let mut offsets = [0i32; 100];

    let lead_out_lba = device.track_last_sector(last_track as u8)?;
    offsets[0] = lead_out_lba + 1 + PREGAP_OFFSET;

    for t in first_track..=last_track {
        let lba = device.track_first_sector(t as u8)?;
        offsets[t as usize] = lba + PREGAP_OFFSET;
    }

    // Hash input: first_track(2) + last_track(2) + 100 offset slots (each 8), all uppercase hex.
    // Pre-allocate capacity: 2 + 2 + (100 * 8) = 804 chars.
    let mut hash_input = String::with_capacity(804);
    write!(hash_input, "{:02X}{:02X}", first_track, last_track).unwrap();
    for &offset in &offsets {
        write!(hash_input, "{:08X}", offset).unwrap();
    }

    // SHA-1 → standard Base64, then MusicBrainz substitutions: + → .  / → _  = → -
    let digest = Sha1::digest(hash_input.as_bytes());
    let b64 = STANDARD.encode(digest);
    Ok(b64.replace('+', ".").replace('/', "_").replace('=', "-"))
}

#[cfg(test)]
mod tests;
