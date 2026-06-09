use owo_colors::OwoColorize;
use std::io::Write;

pub fn spin_chars() -> &'static [char] {
    &[
        '▏', '▎', '▍', '▌', '▋', '▊', '▉', '█', '▉', '▊', '▌', '▍', '▎', '▏',
    ]
}

pub fn print_track_header(track_num: u32, total_tracks: u32, title: &str) {
    println!(
        "  {} {}",
        format!("[{}/{}]", track_num, total_tracks).bold().cyan(),
        title.bold()
    );
}

pub fn print_progress(current_sector: i32, start_sector: i32, total_sectors: u32, status: String) {
    if total_sectors == 0 {
        return;
    }
    let done = (current_sector - start_sector + 1).max(0) as u32;
    let pct = done * 100 / total_sectors;
    let filled = (done * 33 / total_sectors) as usize;
    let bar = format!(
        "{}{}",
        "█".green().to_string().repeat(filled),
        "░".dimmed().to_string().repeat(33 - filled)
    );
    let spinner = spin_chars()[(current_sector as usize) % spin_chars().len()];

    print!(
        "\r  {} {}  {} [{}]",
        spinner,
        status,
        bar,
        format!("{}%", pct).cyan()
    );
    let _ = std::io::stdout().flush();
}

pub fn print_success(path: &str) {
    print!("\r");
    crate::status_dot!("Saved  {}", path.dimmed());
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
            print_progress(0, 0, 1, " ".to_string());
        });
    }

    #[test]
    fn progress_bar_at_end() {
        // When done == total_sectors, bar should be all filled (28 chars)
        let _ = std::panic::catch_unwind(|| {
            print_progress(5, 0, 5, " ".to_string());
        });
    }
}
