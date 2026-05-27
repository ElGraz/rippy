use crate::ui::progress;

#[test]
fn spin_chars_has_seven_elements() {
    assert_eq!(progress::spin_chars().len(), 7);
}

#[test]
fn spin_chars_contains_expected_symbols() {
    let chars = progress::spin_chars();
    assert_eq!(chars[0], '·');
    assert_eq!(chars[3], '◉');
}
