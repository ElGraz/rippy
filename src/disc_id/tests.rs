use base64::{Engine as _, engine::general_purpose::STANDARD};
use sha1::{Digest, Sha1};

/// Helper that mirrors the hashing logic in compute_disc_id without requiring libcdio.
fn hash_to_disc_id(hash_input: &str) -> String {
    let digest = Sha1::digest(hash_input.as_bytes());
    let b64 = STANDARD.encode(digest);
    b64.replace('+', ".").replace('/', "_").replace('=', "-")
}

#[test]
fn disc_id_format_contains_no_plus_slash_equals() {
    // MusicBrainz disc IDs must only contain [A-Za-z0-9.-_]
    let input = format!(
        "{:02X}{:02X}{}",
        1u8,
        3u8,
        (0..100)
            .map(|_i| format!("{:08X}", 0i32))
            .collect::<String>()
    );

    let disc_id = hash_to_disc_id(&input);

    // Disc IDs are 28 chars of [A-Za-z0-9._-]
    assert_eq!(disc_id.len(), 28);
    for ch in disc_id.chars() {
        assert!(
            ch.is_ascii_alphanumeric() || ch == '.' || ch == '_' || ch == '-',
            "Disc ID should only contain alphanumeric, '.', '_', or '-' (found '{}')",
            ch
        );
    }
}

#[test]
fn disc_id_length_is_consistent() {
    // SHA-1 produces 20 bytes = 160 bits.
    // Base64 of 20 bytes = ceil(160/6) = 28 chars (no padding needed since 20*4/3 = 26.67 → 28 with padding).
    // After replacing '=', disc ID length should be 28.
    let input = format!(
        "{:02X}{:02X}{}",
        1u8,
        3u8,
        (0..100)
            .map(|_i| format!("{:08X}", 0i32))
            .collect::<String>()
    );

    let disc_id = hash_to_disc_id(&input);
    assert_eq!(disc_id.len(), 28, "Disc ID should be exactly 28 characters");
}

#[test]
fn same_input_produces_same_disc_id() {
    let input = format!(
        "{:02X}{:02X}{}",
        5u8,
        12u8,
        (0..100)
            .map(|i| format!("{:08X}", i as i32))
            .collect::<String>()
    );

    let id1 = hash_to_disc_id(&input);
    let id2 = hash_to_disc_id(&input);
    assert_eq!(id1, id2);
}

#[test]
fn different_input_produces_different_disc_id() {
    let input1 = format!(
        "{:02X}{:02X}{}",
        1u8,
        3u8,
        (0..100)
            .map(|_i| format!("{:08X}", 0i32))
            .collect::<String>()
    );

    let input2 = format!(
        "{:02X}{:02X}{}",
        1u8,
        4u8,
        (0..100)
            .map(|_i| format!("{:08X}", 0i32))
            .collect::<String>()
    );

    let id1 = hash_to_disc_id(&input1);
    let id2 = hash_to_disc_id(&input2);
    assert_ne!(
        id1, id2,
        "Different inputs should produce different disc IDs"
    );
}

#[test]
fn disc_id_with_realistic_offsets() {
    // Simulate a 3-track CD with realistic LBA offsets:
    // Track 1 at offset 0 (LBA), Track 2 at offset 500, Track 3 at offset 1000, lead-out at 1500
    let first_track = 1u8;
    let last_track = 3u8;

    let mut offsets = vec![1651i32]; // lead_out: 1500 + 1 + 150 = 1651
    offsets.push(150); // track 1: 0 + 150 = 150 (lead-in pregap)
    offsets.push(650); // track 2: 500 + 150 = 650
    offsets.push(1150); // track 3: 1000 + 150 = 1150
    offsets.extend((4..=99).map(|_| 0i32));

    let mut hash_input = format!("{:02X}{:02X}", first_track, last_track);
    for offset in &offsets {
        hash_input.push_str(&format!("{:08X}", offset));
    }

    let disc_id = hash_to_disc_id(&hash_input);
    assert_eq!(disc_id.len(), 28);
    assert!(!disc_id.contains('+'));
    assert!(!disc_id.contains('/'));
    assert!(!disc_id.contains('='));
}

#[test]
fn hash_input_format_with_single_track() {
    // first_track=1, last_track=1, all offsets=0
    let hash_input = format!(
        "{:02X}{:02X}{}",
        1u8,
        1u8,
        (0..100)
            .map(|_| format!("{:08X}", 0i32))
            .collect::<String>()
    );

    // The first 4 characters should be "0101" (first_track=01, last_track=01)
    assert_eq!(&hash_input[0..4], "0101");
}
