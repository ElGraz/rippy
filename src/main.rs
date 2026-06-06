mod cdio;
mod disc_id;
mod models;
mod musicbrainz;
mod ripper;
mod tags;
mod ui;
mod utils;

use crate::models::{AlbumMetadata, TrackMetadata};
use crate::ui::input;
use anyhow::Result;
use owo_colors::OwoColorize;

const RIPPY_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!();
    print!(
        "  {}\n  {}\n\n",
        "─── Rippy! ───".cyan().bold(),
        format!("CDDA to FLAC ripper [v{}]", RIPPY_VERSION).dimmed()
    );

    let drive = match cdio::CdDrive::new() {
        Some(d) => d,
        None => print_error_and_exit("No CD drive found. Is the drive connected?"),
    };
    println!("  {} {}", "Reading from".dimmed(), drive.get_path());

    let mut device = cdio::CdDevice::from_drive(drive);
    let total_tracks = match device.track_count() {
        Ok(n) => n,
        Err(e) => print_error_and_exit(&e.to_string()),
    };
    let first_track: u32 = 1;

    status_ok!(
        "Drive ready — {} audio track{} detected",
        total_tracks.to_string().cyan().bold(),
        if total_tracks == 1 { "" } else { "s" }
    );

    let disc_id = match disc_id::compute(&mut device, first_track, total_tracks) {
        Ok(id) => id,
        Err(e) => print_error_and_exit(&e.to_string()),
    };
    println!("  ID: {}", disc_id.bold().dimmed());

    let albums = match musicbrainz::fetch_album_metadata(&disc_id) {
        Ok(albums) => albums,
        Err(e) => print_error_and_exit(&e.to_string()),
    };

    let metadata = match select_album(albums) {
        Ok(meta) => meta,
        Err(e) => print_error_and_exit(&e.to_string()),
    };

    let summary = metadata.as_ref().cloned().unwrap_or_default();
    ui::summary::print_disc_summary(&summary, total_tracks, &disc_id);
    println!();

    if !input::confirm("Ready to rip?").unwrap_or(false) {
        abort();
    }
    println!();

    // Initialize Ctrl+C handler before starting the long-running rip process.
    utils::signal::init_handler();

    let rip_metadata = metadata.unwrap_or_default();
    match rip_tracks(&mut device, first_track, total_tracks, &rip_metadata) {
        Ok(()) => status_ok!(
            "All {} track{} ripped successfully.",
            total_tracks,
            if total_tracks == 1 { "" } else { "s" }
        ),
        Err(e) if e.to_string().contains("User interrupted") => {
            abort();
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
            println!("{}", "  Multiple releases found. Please select one:".bold());
            for (i, album) in albums.iter().enumerate() {
                println!(
                    "  {num} {title} ({country})\tBarcode: {barcode}\t{packaging}",
                    num = format!("[{}]", i + 1).bold(),
                    title = album.title.green(),
                    country = album.country,
                    barcode = album.barcode.dimmed(),
                    packaging = album.packaging,
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
    metadata: &AlbumMetadata,
) -> Result<()> {
    for track_idx in first_track..first_track + total_tracks {
        let track_meta = TrackMetadata::from_album(track_idx, first_track, metadata);
        ripper::rip_and_encode_track(device, track_idx, total_tracks, &track_meta, metadata)?;
    }
    Ok(())
}

/// Print a bold red error message and exit with a non-zero status.
fn print_error_and_exit(message: &str) -> ! {
    status_err!("{}", message);
    std::process::exit(1);
}

fn abort() -> ! {
    status!("!", yellow, "Aborted.");
    std::process::exit(1);
}
