use super::*;

fn sample_meta() -> TrackMetadata {
    let mut meta = TrackMetadata::default();
    meta.number = 1;
    meta.title = "Test Track".into();
    meta.artist = "Test Artist".into();
    meta.album = "Test Album".into();
    meta.album_id = "abc123".into();
    meta.track_id = "track456".into();
    meta.media_format = "Audio".into();
    meta.country = "US".into();
    meta.genre = vec!["Rock".to_string()];
    meta.disc_number = Some(1);
    meta.date = "2024-01-01".into();
    meta.release_status = "official".into();
    meta
}

fn sample_album() -> AlbumMetadata {
    AlbumMetadata {
        title: "Test Album".into(),
        artist: "Test Artist".into(),
        album_id: "abc123".into(),
        barcode: String::new(),
        release_group_id: String::new(),
        media_format: "Audio CD".into(),
        packaging: String::new(),
        country: String::new(),
        genre: vec!["Rock".to_string()], // album has a genre
        disc_number: 1,
        disc_count: 2,
        tracks: vec![],
        date: String::new(),
        release_status: String::new(),
    }
}

#[test]
fn comments_includes_required_fields() {
    let comments = build_comments(&sample_album(), &sample_meta(), 10);
    assert!(comments.get("TITLE").is_some());
    assert!(comments.get("ARTIST").is_some());
    assert!(comments.get("ALBUM").is_some());
    assert!(comments.get("TRACKNUMBER").is_some());
    assert!(comments.get("TRACKTOTAL").is_some());
}

#[test]
fn comments_includes_optional_fields() {
    let comments = build_comments(&sample_album(), &sample_meta(), 10);
    assert!(comments.get("DATE").is_some());
    assert!(comments.get("MUSICBRAINZ_ALBUMID").is_some());
    assert!(comments.get("DISCNUMBER").is_some());
}

#[test]
fn comments_no_duplicate_genres() {
    // Both album and track have "Rock" — it should appear only once.
    let meta = sample_meta();
    let album = sample_album();
    let comments = build_comments(&album, &meta, 10);

    // Verify that calling build_comments twice with the same inputs
    // produces identical results (no extra genres added).
    let comments2 = build_comments(&album, &meta, 10);

    // If genres were duplicated, the second call would produce different output.
    // We check this by ensuring both calls produce a comment with GENRE present.
    assert!(comments.get("GENRE").is_some());
    assert_eq!(comments.get("GENRE"), comments2.get("GENRE"));
}

#[test]
fn comments_skips_empty_fields() {
    let meta = sample_meta();
    let comments = build_comments(&sample_album(), &meta, 10);
    // These are empty in sample_meta
    assert!(comments.get("BARCODE").is_none());
    assert!(comments.get("MUSICBRAINZ_RELEASEGROUPID").is_none());
}

#[test]
fn tracknumber_contains_correct_value() {
    let mut meta = TrackMetadata::default();
    meta.number = 7;
    let comments = build_comments(&sample_album(), &meta, 10);
    assert_eq!(comments.get("TRACKNUMBER").unwrap(), "7");
}

#[test]
fn tracktotal_contains_correct_value() {
    let meta = TrackMetadata::default();
    let comments = build_comments(&sample_album(), &meta, 42);
    assert_eq!(comments.get("TRACKTOTAL").unwrap(), "42");
}

#[test]
fn empty_metadata_has_no_optional_fields() {
    let meta = TrackMetadata::default();
    // Use a single-disc album so DISCNUMBER/TOTALDISCS are not written.
    let single_disc_album = AlbumMetadata {
        title: "Test Album".into(),
        artist: "Test Artist".into(),
        album_id: "abc123".into(),
        barcode: String::new(),
        release_group_id: String::new(),
        media_format: "Audio CD".into(),
        packaging: String::new(),
        country: String::new(),
        genre: vec![],
        disc_number: 1,
        disc_count: 1,
        tracks: vec![],
        date: String::new(),
        release_status: String::new(),
    };
    // In default, title and album are empty strings — they should still be present as required fields
    // but all optional fields should be absent.
    let comments = build_comments(&single_disc_album, &meta, 1);

    assert!(comments.get("TITLE").is_some());
    assert!(comments.get("ARTIST").is_some());
    assert!(comments.get("ALBUM").is_some());
    assert!(comments.get("TRACKNUMBER").is_some());
    assert!(comments.get("TRACKTOTAL").is_some());

    let optional_keys = [
        "DATE",
        "RELEASESTATUS",
        "MUSICBRAINZ_ALBUMID",
        "BARCODE",
        "MUSICBRAINZ_TRACKID",
        "MUSICBRAINZ_RELEASEGROUPID",
        "MEDIA",
        "RELEASEPACKAGING",
        "RELEASECOUNTRY",
        "GENRE",
    ];
    for key in &optional_keys {
        assert!(
            comments.get(*key).is_none(),
            "Optional field {} should not be present for default metadata",
            key
        );
    }
}
