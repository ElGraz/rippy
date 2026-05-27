use anyhow::{Result, anyhow};
use libcdio_sys::{
    cdio_cddap_close, cdio_cddap_find_a_cdrom, cdio_cddap_open, cdio_cddap_tracks,
    cdio_paranoia_free, cdio_paranoia_init, cdio_paranoia_modeset, cdrom_paranoia_t, track_t,
};
use std::io::{Write, stdin};

mod disc_id;
mod models;
mod musicbrainz;
mod ripper;
mod ui;

use crate::models::TrackMetadata;
use crate::ui::colors::{BOLD, CYAN, DIM, GREEN, RED, RESET};
use crate::ui::summary::{DiscSummary, print_disc_summary};

const CDIO_INVALID_TRACK: track_t = 0xFF;
const RIPPY_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    println!();
    println!("  {}{}  Rippy! ({})  {}", BOLD, CYAN, RIPPY_VERSION, RESET);
    println!("  {}The FLAC CDDA ripper{}", DIM, RESET);
    println!();

    status!("◉", CYAN, "Opening CD drive…");
    let drive_ptr = unsafe { cdio_cddap_find_a_cdrom(0, std::ptr::null_mut()) };
    if drive_ptr.is_null() {
        status!("✗", RED, "No CD drive found. Is a disc inserted?");
        return Err(anyhow!("Failed to find a CD drive via libcdio."));
    }

    let open_result = unsafe { cdio_cddap_open(drive_ptr) };
    if open_result != 0 {
        unsafe { cdio_cddap_close(drive_ptr) };
        status!(
            "✗",
            RED,
            "Could not open drive for CD-DA (error {}). Is a CD-DA disc inserted?",
            open_result
        );
        return Err(anyhow!("cdio_cddap_open failed (error {}).", open_result));
    }

    let paranoia_ptr: *mut cdrom_paranoia_t = unsafe { cdio_paranoia_init(drive_ptr) };
    if paranoia_ptr.is_null() {
        unsafe { cdio_cddap_close(drive_ptr) };
        return Err(anyhow!("Failed to initialize the cdio paranoia engine."));
    }

    // Full paranoia: overlap + verify + reconstruct
    unsafe { cdio_paranoia_modeset(paranoia_ptr, 0xff) };

    let total_tracks_raw = unsafe { cdio_cddap_tracks(drive_ptr) };
    if total_tracks_raw == CDIO_INVALID_TRACK || total_tracks_raw == 0 {
        unsafe {
            cdio_paranoia_free(paranoia_ptr);
            cdio_cddap_close(drive_ptr);
        }
        return Err(anyhow!(
            "No audio tracks found on disc (cdio_cddap_tracks returned {}).",
            total_tracks_raw
        ));
    }
    let total_tracks = total_tracks_raw as u32;
    let first_track: u32 = 1;

    status!(
        "✓",
        GREEN,
        "Drive ready — {}{}{}  audio track{} detected",
        BOLD,
        total_tracks,
        RESET,
        if total_tracks == 1 { "" } else { "s" }
    );

    status!("◉", CYAN, "Computing MusicBrainz Disc ID…");
    let disc_id =
        disc_id::compute_disc_id(drive_ptr, first_track as track_t, total_tracks as track_t)?;
    println!("  {}{}ID:{} {}{}{}", DIM, BOLD, RESET, DIM, disc_id, RESET);

    status!("◉", CYAN, "Querying MusicBrainz…");
    let metadata = musicbrainz::fetch_album_metadata(&disc_id)?;

    println!();

    let summary = match &metadata {
        Some(album_meta) => DiscSummary {
            album_title: Some(album_meta.title.clone()),
            artist: Some(album_meta.artist.clone()),
            tracks: album_meta.tracks.clone(),
            total_tracks,
            unknown_disc: false,
        },
        None => DiscSummary {
            album_title: None,
            artist: None,
            tracks: vec![],
            total_tracks,
            unknown_disc: true,
        },
    };

    print_disc_summary(&summary, &disc_id);

    print!(
        "\n  {}Proceed with ripping?{} [{}y{}/{}N{}] ",
        BOLD, RESET, GREEN, RESET, DIM, RESET
    );
    std::io::stdout().flush()?;

    let mut answer = String::new();
    stdin().read_line(&mut answer)?;
    if !matches!(answer.trim().to_lowercase().as_str(), "y" | "yes") {
        println!("\n  {}Aborted.{}", DIM, RESET);
        unsafe {
            cdio_paranoia_free(paranoia_ptr);
            cdio_cddap_close(drive_ptr);
        }
        return Ok(());
    }
    println!();

    let track_range = first_track..=(first_track + total_tracks - 1);
    let total = track_range.clone().count() as u32;

    for track_idx in track_range {
        let track_meta = match &metadata {
            Some(album_meta) => {
                let (title, track_id) = album_meta
                    .tracks
                    .get((track_idx - first_track) as usize)
                    .cloned()
                    .unwrap_or_else(|| (format!("Track {}", track_idx), String::new()));
                TrackMetadata {
                    number: track_idx,
                    title,
                    artist: album_meta.artist.clone(),
                    album: album_meta.title.clone(),
                    album_id: album_meta.album_id.clone(),
                    barcode: album_meta.barcode.clone(),
                    track_id,
                    release_group_id: album_meta.release_group_id.clone(),
                    media_format: album_meta.media_format.clone(),
                    packaging: album_meta.packaging.clone(),
                    country: album_meta.country.clone(),
                    disc_number: Some(track_idx),
                    date: album_meta.date.clone(),
                    release_status: album_meta.release_status.clone(),
                }
            }
            None => TrackMetadata {
                number: track_idx,
                title: format!("Track {}", track_idx),
                artist: "Unknown Artist".to_string(),
                album: "Unknown Album".to_string(),
                album_id: String::new(),
                barcode: String::new(),
                track_id: String::new(),
                release_group_id: String::new(),
                media_format: String::new(),
                packaging: String::new(),
                country: String::new(),
                disc_number: None,
                date: String::new(),
                release_status: String::new(),
            },
        };

        ripper::rip_and_encode_track(drive_ptr, paranoia_ptr, track_idx, total, &track_meta)?;
    }

    unsafe {
        cdio_paranoia_free(paranoia_ptr);
        cdio_cddap_close(drive_ptr);
    }

    println!();
    status!(
        "✓",
        GREEN,
        "All {} track{} ripped successfully.",
        total,
        if total == 1 { "" } else { "s" }
    );
    Ok(())
}
