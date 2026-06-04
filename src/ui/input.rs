use super::colors::{BOLD, RESET};
use anyhow::Result;
use std::io::{self, Write};

/// Core prompt function shared by all input helpers.
/// Writes the formatted prompt to stdout and reads one line from stdin.
pub fn read_line(prompt: &str) -> Result<String> {
    print!("  {}{}{}", BOLD, prompt, RESET);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_lowercase())
}

/// Prompt the user for a yes/no confirmation.
///
/// Accepts "y" or "yes" for true; everything else (including empty) is false.
pub fn confirm(prompt: &str) -> Result<bool> {
    let input = read_line(&format!("{0} [y/N] ", prompt))?;
    Ok(matches!(input.as_str(), "y" | "yes"))
}

/// Prompt the user to select from numbered options, then return the chosen option.
///
/// Keeps asking until the user enters a valid index in `[1..=options.len()]`.
pub fn choose<T>(prompt: &str, options: Vec<T>) -> Result<T> {
    let mut options = options;
    loop {
        let input = read_line(&format!("{0} [1-{1}] ", prompt, options.len()))?;

        if let Ok(idx) = input.parse::<usize>() {
            match options.get(idx - 1) {
                Some(_) => return Ok(options.remove(idx - 1)),
                None => continue, // silently re-prompt for out-of-range
            }
        }
    }
}
