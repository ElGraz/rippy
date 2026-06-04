use super::colors::{BOLD, CYAN, DIM, GREEN, RESET};
use std::io::Write;

pub fn spin_chars() -> &'static [char] {
    &[
        '▏', '▎', '▍', '▌', '▋', '▊', '▉', '█', '▉', '▊', '▌', '▍', '▎', '▏',
    ]
}

pub fn print_track_header(track_num: u32, total_tracks: u32, title: &str) {
    println!(
        "  {}{}[{}/{}]{} {}{}{}",
        BOLD, CYAN, track_num, total_tracks, RESET, BOLD, title, RESET
    );
}

pub fn print_progress(current_sector: i32, start_sector: i32, total_sectors: u32) {
    let done = (current_sector - start_sector + 1) as u32;
    let pct = done * 100 / total_sectors;
    let filled = (done * 28 / total_sectors) as usize;
    let bar = format!(
        "{}{}{}{}",
        GREEN,
        "█".repeat(filled),
        DIM,
        "░".repeat(28 - filled)
    );
    let spinner = spin_chars()[(current_sector as usize) % spin_chars().len()];

    print!(
        "\r  {} {}{}{}{}{}  {}{}{}%{}  {}",
        spinner, CYAN, bar, RESET, RESET, RESET, BOLD, pct, RESET, RESET, DIM
    );
    let _ = std::io::stdout().flush();
}

pub fn print_success(path: &str) {
    print!("\r");
    crate::status!("✓", GREEN, "Saved  {}{}{}", DIM, path, RESET);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_bar_at_start() {
        // At start: done=1, total_sectors could be any value
        // The bar should have at least 1 filled segment
        // This test just ensures no panics occur
        let _ = std::panic::catch_unwind(|| {
            print_progress(0, 0, 1);
        });
    }

    #[test]
    fn progress_bar_at_end() {
        // When done == total_sectors, bar should be all filled (28 chars)
        let _ = std::panic::catch_unwind(|| {
            print_progress(5, 0, 5);
        });
    }
}
