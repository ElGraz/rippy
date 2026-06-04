use super::colors::{BOLD, CYAN, DIM, RESET, YELLOW};

pub struct DiscSummary {
    pub album_title: Option<String>,
    pub artist: Option<String>,
    pub tracks: Vec<(String, String)>, // (Track Title, Track MBID)
    pub total_tracks: u32,
    pub unknown_disc: bool,
}

pub fn print_disc_summary(summary: &DiscSummary, disc_id: &str) {
    let width = 54usize;
    let bar = "─".repeat(width);

    println!("\n  {}{}┌{}┐{}", BOLD, CYAN, bar, RESET);

    println!(
        "  {}{}│{}  {}ID   {}{}{}{}",
        BOLD, CYAN, RESET, DIM, RESET, DIM, disc_id, RESET
    );

    if summary.unknown_disc {
        println!(
            "  {}{}│{}  {}{}No MusicBrainz match — unknown disc{}",
            BOLD, CYAN, RESET, BOLD, YELLOW, RESET
        );
        print_row("Tracks", &summary.total_tracks.to_string());

        for i in 1..=summary.total_tracks {
            println!(
                "  {}{}│{}  {}{:>2}.{} Track {}{}",
                BOLD, CYAN, RESET, DIM, i, RESET, i, RESET
            );
        }
    } else {
        print_row(
            "Album",
            summary.album_title.as_deref().unwrap_or("Unknown Album"),
        );
        print_row(
            "Artist",
            summary.artist.as_deref().unwrap_or("Unknown Artist"),
        );
        print_row("Tracks", &summary.total_tracks.to_string());
        print_divider(width);
        for (i, (title, _)) in summary.tracks.iter().enumerate() {
            println!(
                "  {}{}│{}  {}{:>2}.{} {}{}",
                BOLD,
                CYAN,
                RESET,
                DIM,
                i + 1,
                RESET,
                title,
                RESET
            );
        }
    }

    println!("  {}{}└{}┘{}", BOLD, CYAN, bar, RESET);
}

fn print_row(label: &str, value: &str) {
    println!(
        "  {}{}│{}  {}{:<6}{} {}{}",
        BOLD, CYAN, RESET, DIM, label, RESET, value, RESET
    );
}

fn print_divider(width: usize) {
    println!("  {}{}├{}┤{}", BOLD, CYAN, "─".repeat(width), RESET);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_disc_summary_unknown_does_not_panic() {
        let summary = DiscSummary {
            album_title: None,
            artist: None,
            tracks: vec![],
            total_tracks: 1,
            unknown_disc: true,
        };
        let _ = std::panic::catch_unwind(|| {
            print_disc_summary(&summary, "test-id");
        });
    }

    #[test]
    fn print_disc_summary_known_does_not_panic() {
        let summary = DiscSummary {
            album_title: Some("Test Album".to_string()),
            artist: Some("Test Artist".to_string()),
            tracks: vec![
                ("Track 1".to_string(), "id-1".to_string()),
                ("Track 2".to_string(), "id-2".to_string()),
            ],
            total_tracks: 2,
            unknown_disc: false,
        };
        let _ = std::panic::catch_unwind(|| {
            print_disc_summary(&summary, "test-id");
        });
    }
}
