// ── ANSI colour helpers ───────────────────────────────────────────────────────

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const CYAN: &str = "\x1b[36m";
pub const RED: &str = "\x1b[31m";

#[macro_export]
macro_rules! status {
    ($icon:expr, $color:expr, $($arg:tt)*) => {
        println!("{}{}{}{} {}{}", $color, $crate::ui::colors::BOLD, $icon, $crate::ui::colors::RESET, format!($($arg)*), $crate::ui::colors::RESET)
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        eprintln!(
            "  {}{}ERROR:{} {}{}",
            $crate::ui::colors::RED,
            $crate::ui::colors::BOLD,
            $crate::ui::colors::RESET,
            format!($($arg)*),
            $crate::ui::colors::RESET
        )
    };
}
