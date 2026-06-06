#[macro_export]
macro_rules! status {
    ($icon:expr, $color:ident, $($arg:tt)*) => {{
        use owo_colors::{OwoColorize};
        println!("{} {}", $icon.$color().bold(), format!($($arg)*).bold());
    }};
}

#[macro_export]
macro_rules! status_dot {
    ($($arg:tt)*) => {{
        $crate::status!("☉", cyan, $($arg)*)
    }};
}

#[macro_export]
macro_rules! status_ok {
    ($($arg:tt)*) => {{
        $crate::status!("✔", green, $($arg)*)
    }};
}

#[macro_export]
macro_rules! status_err {
    ($($arg:tt)*) => {{
        $crate::status!("✖", red, $($arg)*)
    }};
}
