use crate::models::AlbumMetadata;
use owo_colors::OwoColorize;

pub fn print_disc_summary(summary: &AlbumMetadata, total_tracks: u32, disc_id: &str) {
    let width = 54usize;
    let bar = "─".repeat(width);

    // Top border
    println!("\n  ┌{}┐", bar.cyan().bold());

    // Disc ID Row
    println!("  {}  {}   {}", "│".cyan().bold(), "ID".dimmed(), disc_id);

    // Optional Disc Count Row
    if summary.disc_count > 1 {
        println!(
            "  {}  {} {} of {}",
            "│".cyan().bold(),
            "Disc:".dimmed(),
            summary.disc_number,
            summary.disc_count
        );
    }

    if summary.tracks.is_empty() {
        // Warning Row
        println!(
            "  {}  {}",
            "│".cyan().bold(),
            "No MusicBrainz match — unknown disc".yellow().bold()
        );

        print_row("Tracks", &summary.tracks.len().to_string());

        // Dummy Track List
        for i in 1..=total_tracks {
            println!("  {}  {:>2}. Track {}", "│".cyan().bold(), i.dimmed(), i);
        }
    } else {
        // Metadata Rows
        print_row("Album", &summary.title);
        print_row("Artist", &summary.artist);
        print_row("Tracks", &summary.tracks.len().to_string());

        print_divider(width);

        // Populated Track List
        for (i, (title, _)) in summary.tracks.iter().enumerate() {
            println!(
                "  {}  {:>2}. {}",
                "│".cyan().bold(),
                (i + 1).dimmed(),
                title
            );
        }
    }

    // Bottom border
    println!("  └{}┘", bar.cyan().bold());
}

fn print_row(label: &str, value: &str) {
    println!("  {}  {:<6} {}", "│".cyan().bold(), label.dimmed(), value);
}

fn print_divider(width: usize) {
    let divider = "─".repeat(width);
    println!("  ├{}┤", divider.cyan().bold());
}
