mod cdio;
mod disc_id;
mod models;
mod musicbrainz;
mod path;
mod ripper;
mod signal;
mod tags;
mod ui;

use crate::models::{AlbumMetadata, TrackMetadata};
use crate::ui::colors::{BOLD, CYAN, DIM, GREEN, RESET};
use crate::ui::input;
use anyhow::Result;

const RIPPY_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!();
    print!(
        "  {}{} >> Rippy! << {}CDDA to FLAC ripper (v{}) {}\n\n",
        BOLD, CYAN, DIM, RIPPY_VERSION, RESET
    );

    let drive = match cdio::CdDrive::new() {
        Some(d) => d,
        None => print_error_and_exit("No CD drive found. Is the drive connected?"),
    };
    println!("  {}Reading from {}{}", DIM, drive.get_path(), RESET);

    let mut device = cdio::CdDevice::from_drive(drive);
    let total_tracks = match device.track_count() {
        Ok(n) => n,
        Err(e) => print_error_and_exit(&e.to_string()),
    };
    let first_track: u32 = 1;

    status!(
        "✓",
        GREEN,
        "Drive ready — {}{}{} audio track{} detected",
        BOLD,
        total_tracks,
        RESET,
        if total_tracks == 1 { "" } else { "s" }
    );

    let disc_id = match disc_id::compute(&mut device, first_track, total_tracks) {
        Ok(id) => id,
        Err(e) => print_error_and_exit(&e.to_string()),
    };
    println!("  {}{}ID:{} {}{}{}", DIM, BOLD, RESET, DIM, disc_id, RESET);

    let albums = match musicbrainz::fetch_album_metadata(&disc_id) {
        Ok(albums) => albums,
        Err(e) => print_error_and_exit(&e.to_string()),
    };

    let metadata = match select_album(albums) {
        Ok(meta) => meta,
        Err(e) => print_error_and_exit(&e.to_string()),
    };

    let summary = ui::summary::DiscSummary {
        album_title: metadata.as_ref().map(|m| m.title.clone()),
        artist: metadata.as_ref().map(|m| m.artist.clone()),
        tracks: metadata
            .as_ref()
            .map(|m| m.tracks.clone())
            .unwrap_or_default(),
        total_tracks,
        unknown_disc: metadata.is_none(),
    };
    ui::summary::print_disc_summary(&summary, &disc_id);
    println!();

    if !input::confirm("Proceed with ripping").unwrap_or(false) {
        println!("\n  {}Aborted.", DIM);
        return;
    }
    println!();

    // Initialize Ctrl+C handler before starting the long-running rip process.
    signal::init_handler();

    match rip_tracks(&mut device, first_track, total_tracks, &metadata) {
        Ok(()) => status!(
            "✓",
            GREEN,
            "All {} track{} ripped successfully.",
            total_tracks,
            if total_tracks == 1 { "" } else { "s" }
        ),
        Err(e) if e.to_string().contains("User interrupted") => {
            println!("\n  {}Aborted.", DIM);
        }
        Err(e) => print_error_and_exit(&e.to_string()),
    }
}

/// Prompt the user to select an album (or auto-select if only one match).
fn select_album(albums: Vec<AlbumMetadata>) -> Result<Option<AlbumMetadata>> {
    match albums.len() {
        0 => Ok(None),
        1 => Ok(Some(albums.into_iter().next().unwrap())),
        _ => {
            println!();
            println!(
                "  {}Multiple releases found. Please select one:{}",
                BOLD, RESET
            );
            for (i, album) in albums.iter().enumerate() {
                let barcode = if album.barcode.is_empty() {
                    "N/A".to_string()
                } else {
                    album.barcode.clone()
                };
                println!(
                    "  {} [{}] {}{}{} (Barcode: {}{}){}",
                    BOLD,
                    i + 1,
                    GREEN,
                    album.title,
                    RESET,
                    DIM,
                    barcode,
                    RESET
                );
            }
            println!();

            let metadata = input::choose("Which release would you like to use?", albums)?;
            Ok(Some(metadata))
        }
    }
}

/// Rip all tracks on the disc.
fn rip_tracks(
    device: &mut cdio::CdDevice,
    first_track: u32,
    total_tracks: u32,
    metadata: &Option<AlbumMetadata>,
) -> Result<()> {
    for track_idx in first_track..first_track + total_tracks {
        let track_meta = resolve_track_metadata(track_idx, first_track, metadata);
        ripper::rip_and_encode_track(device, track_idx, total_tracks, &track_meta)?;
    }
    Ok(())
}

/// Resolve per-track metadata from the album info, falling back to defaults.
fn resolve_track_metadata(
    track_idx: u32,
    first_track: u32,
    album_meta: &Option<AlbumMetadata>,
) -> TrackMetadata {
    let mut meta = TrackMetadata::default();
    meta.number = track_idx;
    meta.title = format!("Track {}", track_idx);

    if let Some(album) = album_meta {
        meta.artist.clone_from(&album.artist);
        meta.album.clone_from(&album.title);
        meta.album_id.clone_from(&album.album_id);
        meta.barcode.clone_from(&album.barcode);
        meta.release_group_id.clone_from(&album.release_group_id);
        meta.media_format.clone_from(&album.media_format);
        meta.packaging.clone_from(&album.packaging);
        meta.country.clone_from(&album.country);
        meta.disc_number = Some(track_idx);
        meta.date.clone_from(&album.date);
        meta.release_status.clone_from(&album.release_status);

        if let Some((title, track_id)) = album
            .tracks
            .get((track_idx - first_track) as usize)
            .cloned()
        {
            meta.title = title;
            meta.track_id = track_id;
        }
    }

    meta
}

/// Print a bold red error message and exit with a non-zero status.
fn print_error_and_exit(message: &str) -> ! {
    error!("{}", message);
    std::process::exit(1);
}
