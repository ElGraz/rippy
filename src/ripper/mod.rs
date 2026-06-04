use anyhow::{Result, anyhow};
use flac_codec::encode::{FlacSampleWriter, Options};
use libcdio_sys::{cdio_paranoia_read, cdio_paranoia_seek};
use std::io::Write;

use crate::cdio::CdDevice;
use crate::models::TrackMetadata;
use crate::path;
use crate::signal;
use crate::tags;
use crate::ui::colors::RESET;
use crate::ui::progress;

const SAMPLES_PER_SECTOR: usize = 1176;

pub fn rip_and_encode_track(
    device: &mut CdDevice,
    track_num: u32,
    total_tracks: u32,
    meta: &TrackMetadata,
) -> Result<()> {
    use std::path::Path;

    let artist_dir = path::sanitize(&meta.artist);
    let album_dir = path::sanitize(&meta.album);
    let dir = Path::new(&artist_dir).join(&album_dir);
    std::fs::create_dir_all(&dir)?;

    let filename = format!("{:02} - {}.flac", meta.number, path::sanitize(&meta.title));
    let output_path = dir.join(filename);

    progress::print_track_header(track_num, total_tracks, &meta.title);

    // Build Vorbis comment metadata blocks.
    let comments = tags::build_comments(meta, total_tracks);

    let file = std::fs::File::create(&output_path)?;
    let writer = std::io::BufWriter::new(file);
    let options = Options::best().comment(comments);

    let mut encoder = FlacSampleWriter::new_cdda(writer, options, None)
        .map_err(|e| anyhow!("Failed to build FLAC encoder: {:?}", e))?;

    let start_sector = device.track_first_sector(track_num as u8)?;
    let end_sector = device.track_last_sector(track_num as u8)?;

    unsafe { cdio_paranoia_seek(device.paranoia_ptr(), start_sector, 0) };

    let total_sectors = (end_sector - start_sector + 1) as u32;
    let mut current_sector = start_sector;

    while current_sector <= end_sector {
        // Check for Ctrl+C interrupt before reading each sector.
        if signal::is_interrupted() {
            print!(
                "\r  {}Do you want to stop the rip process and quit?{} [y/N] ",
                crate::ui::colors::DIM,
                RESET,
            );
            std::io::stdout().flush()?;

            let mut answer = String::new();
            std::io::stdin().read_line(&mut answer)?;
            if matches!(answer.trim().to_lowercase().as_str(), "y" | "yes") {
                return Err(anyhow!("User interrupted the rip process."));
            }
        }

        let sector_raw_ptr =
            unsafe { cdio_paranoia_read(device.paranoia_ptr(), Some(dummy_callback)) };
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

    progress::print_success(&output_path.display().to_string());

    Ok(())
}

unsafe extern "C" fn dummy_callback(_sector: i64, _status: u32) {}
