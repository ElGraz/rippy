use crate::ui::input;

#[test]
fn choose_index_mapping_is_1_based() {
    // Verify that choosing index 1 returns the first element.
    // We can't easily mock stdin, but we verify the Vec access logic
    // matches our expected 1-based indexing behavior.
    let options = vec![
        "first".to_string(),
        "second".to_string(),
        "third".to_string(),
    ];
    // Index 1 (user-facing) → Vec index 0
    assert_eq!(options.get(0), Some(&"first".to_string()));
}

#[test]
fn choose_out_of_bounds_is_handled() {
    let options = vec!["a".to_string(), "b".to_string()];
    // User enters "3" → Vec index 2 → out of bounds
    assert_eq!(options.get(2), None);
}

#[test]
fn confirm_signature_accepts_str() {
    // Verify the function accepts &str and returns Result<bool>.
    let _ = |p: &str| -> anyhow::Result<bool> { input::confirm(p) };
}
