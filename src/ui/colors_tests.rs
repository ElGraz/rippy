use crate::ui::colors;

#[test]
fn ansi_codes_are_valid() {
    // Verify that all ANSI escape codes follow the expected format: ESC[<n>m
    let codes = [
        colors::RESET,
        colors::BOLD,
        colors::DIM,
        colors::GREEN,
        colors::YELLOW,
        colors::CYAN,
        colors::RED,
    ];
    for code in codes {
        assert!(
            code.starts_with("\x1b["),
            "Code should start with escape sequence: {:?}",
            code
        );
        assert!(code.ends_with('m'), "Code should end with 'm': {:?}", code);
    }
}

#[test]
fn reset_is_standard() {
    assert_eq!(colors::RESET, "\x1b[0m");
}

#[test]
fn bold_is_code_1() {
    assert_eq!(colors::BOLD, "\x1b[1m");
}

#[test]
fn dim_is_code_2() {
    assert_eq!(colors::DIM, "\x1b[2m");
}

#[test]
fn green_is_code_32() {
    assert_eq!(colors::GREEN, "\x1b[32m");
}

#[test]
fn yellow_is_code_33() {
    assert_eq!(colors::YELLOW, "\x1b[33m");
}

#[test]
fn cyan_is_code_36() {
    assert_eq!(colors::CYAN, "\x1b[36m");
}

#[test]
fn red_is_code_31() {
    assert_eq!(colors::RED, "\x1b[31m");
}
