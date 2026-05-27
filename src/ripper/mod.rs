use anyhow::{Result, anyhow};
use flac_codec::encode::{FlacSampleWriter, Options};
use flac_codec::metadata::VorbisComment;
use libcdio_sys::{
    cdio_paranoia_read, cdio_paranoia_seek, cdrom_drive_t, cdrom_paranoia_t, track_t,
};
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::models::TrackMetadata;
use crate::ui::progress;

const SAMPLES_PER_SECTOR: usize = 1176;

pub fn rip_and_encode_track(
    drive_ptr: *mut cdrom_drive_t,
    paranoia_ptr: *mut cdrom_paranoia_t,
    track_num: u32,
    total_tracks: u32,
    meta: &TrackMetadata,
) -> Result<()> {
    let dir = Path::new(&sanitize(&meta.artist)).join(sanitize(&meta.album));
    fs::create_dir_all(&dir)?;

    let filename = format!("{:02} - {}.flac", meta.number, sanitize(&meta.title));
    let output_path = dir.join(&filename);

    // Track header
    progress::print_track_header(track_num, total_tracks, &meta.title);

    // ── Build Vorbis Comment Metadata Blocks ───────────────────────────────
    let mut comments = VorbisComment::default();
    let _ = comments.insert("TITLE", &meta.title);
    let _ = comments.insert("ARTIST", &meta.artist);
    let _ = comments.insert("ALBUM", &meta.album);
    let _ = comments.insert("TRACKNUMBER", &meta.number.to_string());
    let _ = comments.insert("TRACKTOTAL", &total_tracks.to_string());
    if let Some(disc) = meta.disc_number {
        let _ = comments.insert("DISCNUMBER", &disc.to_string());
    }
    if !meta.date.is_empty() {
        let _ = comments.insert("DATE", &meta.date);
    }
    if !meta.release_status.is_empty() {
        let _ = comments.insert("RELEASESTATUS", &meta.release_status);
    }

    if !meta.album_id.is_empty() {
        let _ = comments.insert("MUSICBRAINZ_ALBUMID", &meta.album_id);
    }
    if !meta.barcode.is_empty() {
        let _ = comments.insert("BARCODE", &meta.barcode);
    }
    if !meta.track_id.is_empty() {
        let _ = comments.insert("MUSICBRAINZ_TRACKID", &meta.track_id);
    }
    if !meta.release_group_id.is_empty() {
        let _ = comments.insert("MUSICBRAINZ_RELEASEGROUPID", &meta.release_group_id);
    }
    if !meta.media_format.is_empty() {
        let _ = comments.insert("MEDIA", &meta.media_format);
    }
    if !meta.packaging.is_empty() {
        let _ = comments.insert("RELEASEPACKAGING", &meta.packaging);
    }
    if !meta.country.is_empty() {
        let _ = comments.insert("RELEASECOUNTRY", &meta.country);
    }
    if !meta.date.is_empty() {
        let _ = comments.insert("DATE", &meta.date);
    }

    let file = File::create(&output_path)?;
    let writer = BufWriter::new(file);
    let options = Options::best().comment(comments);

    // Construct the writer
    let mut encoder = FlacSampleWriter::new_cdda(writer, options, None)
        .map_err(|e| anyhow!("Failed to build FLAC encoder: {:?}", e))?;

    let start_sector =
        unsafe { libcdio_sys::cdio_cddap_track_firstsector(drive_ptr, track_num as track_t) };
    let end_sector =
        unsafe { libcdio_sys::cdio_cddap_track_lastsector(drive_ptr, track_num as track_t) };

    if start_sector < 0 || end_sector < 0 {
        return Err(anyhow!(
            "Failed to acquire track boundary sectors from libcdio."
        ));
    }

    unsafe { cdio_paranoia_seek(paranoia_ptr, start_sector, 0) };

    let total_sectors = (end_sector - start_sector + 1) as u32;
    let mut current_sector = start_sector;

    while current_sector <= end_sector {
        let sector_raw_ptr = unsafe { cdio_paranoia_read(paranoia_ptr, Some(dummy_callback)) };
        if sector_raw_ptr.is_null() {
            return Err(anyhow!("Read error at sector {}.", current_sector));
        }

        let sector_samples: &[i16] =
            unsafe { std::slice::from_raw_parts(sector_raw_ptr as *const i16, SAMPLES_PER_SECTOR) };

        progress::print_progress(current_sector, start_sector, total_sectors);

        let i32_samples: Vec<i32> = sector_samples.iter().map(|&s| s as i32).collect();
        encoder
            .write(&i32_samples)
            .map_err(|e| anyhow!("FLAC encoding failed: {:?}", e))?;

        current_sector += 1;
    }

    encoder
        .finalize()
        .map_err(|e| anyhow!("Failed to finalize FLAC: {:?}", e))?;

    // Overwrite progress line with completion
    progress::print_success(&output_path.display().to_string());

    Ok(())
}

pub fn sanitize(s: &str) -> String {
    s.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_")
}

#[cfg(test)]
mod tests {
    use super::sanitize;

    #[test]
    fn sanitize_no_special_chars() {
        assert_eq!(sanitize("normal string"), "normal string");
    }

    #[test]
    fn sanitize_slashes() {
        assert_eq!(sanitize("path/to/file"), "path_to_file");
        assert_eq!(sanitize("path\\to\\file"), "path_to_file");
    }

    #[test]
    fn sanitize_colon() {
        assert_eq!(sanitize("title: subtitle"), "title_ subtitle");
    }

    #[test]
    fn sanitize_all_special_chars() {
        let input = "/:*?\"<>|";
        let expected = "________";
        assert_eq!(sanitize(input), expected);
    }

    #[test]
    fn sanitize_mixed_content() {
        assert_eq!(sanitize("Artist / Band: Vol. 1"), "Artist _ Band_ Vol. 1");
    }

    #[test]
    fn sanitize_empty_string() {
        assert_eq!(sanitize(""), "");
    }

    #[test]
    fn sanitize_unicode() {
        assert_eq!(sanitize("Artisté – Album™"), "Artisté – Album™");
    }
}

unsafe extern "C" fn dummy_callback(_sector: i64, _status: u32) {}
