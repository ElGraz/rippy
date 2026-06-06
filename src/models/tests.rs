use super::{AlbumMetadata, TrackMetadata};

#[test]
fn track_metadata_default_values() {
    let meta = TrackMetadata::default();

    assert_eq!(meta.number, 0);
    assert_eq!(meta.artist, "Unknown Artist");
    assert_eq!(meta.album, "Unknown Album");
}

#[test]
fn track_metadata_custom_values() {
    let meta = TrackMetadata {
        number: 1,
        title: "Test Track".to_string(),
        artist: "Test Artist".to_string(),
        album: "Test Album".to_string(),
        ..TrackMetadata::default()
    };

    assert_eq!(meta.number, 1);
    assert_eq!(meta.title, "Test Track");
    assert_eq!(meta.artist, "Test Artist");
    assert_eq!(meta.album, "Test Album");
}

#[test]
fn track_metadata_no_disc_number() {
    let meta = TrackMetadata {
        number: 3,
        title: "Solo".to_string(),
        artist: "Solo Artist".to_string(),
        album: "Solo Album".to_string(),
        album_id: String::new(),
        barcode: String::new(),
        track_id: String::new(),
        release_group_id: String::new(),
        media_format: String::new(),
        packaging: String::new(),
        country: String::new(),
        genre: Vec::new(),
        disc_number: None,
        date: "2024-01-01".to_string(),
        release_status: "official".to_string(),
    };

    assert_eq!(meta.number, 3);
    assert_eq!(meta.date, "2024-01-01");
}

#[test]
fn album_metadata_construction() {
    let meta = AlbumMetadata {
        title: "Album Title".to_string(),
        artist: "Artist Name".to_string(),
        album_id: "album-uuid".to_string(),
        barcode: "1234567890123".to_string(),
        release_group_id: "rg-uuid".to_string(),
        media_format: "Audio CD".to_string(),
        packaging: "Digipak".to_string(),
        country: "US".to_string(),
        genre: Vec::new(),
        disc_number: 1,
        disc_count: 1,
        tracks: vec![
            ("Track One".to_string(), "track-uuid-1".to_string()),
            ("Track Two".to_string(), "track-uuid-2".to_string()),
        ],
        date: "2024-06-15".to_string(),
        release_status: "official".to_string(),
    };

    assert_eq!(meta.title, "Album Title");
    assert_eq!(meta.artist, "Artist Name");
    assert_eq!(meta.tracks.len(), 2);
    assert_eq!(meta.tracks[0].0, "Track One");
    assert_eq!(meta.disc_number, 1);
}

#[test]
fn track_metadata_debug_output() {
    let meta = TrackMetadata::default();
    let debug_str = format!("{:?}", meta);
    assert!(debug_str.contains("TrackMetadata"));
}

#[test]
fn album_metadata_debug_output() {
    let meta = AlbumMetadata {
        title: "Test".into(),
        artist: "Artist".into(),
        album_id: String::new(),
        barcode: String::new(),
        release_group_id: String::new(),
        media_format: String::new(),
        packaging: String::new(),
        country: String::new(),
        genre: Vec::new(),
        disc_number: 2,
        disc_count: 3,
        tracks: vec![],
        date: String::new(),
        release_status: String::new(),
    };
    let debug_str = format!("{:?}", meta);
    assert!(debug_str.contains("AlbumMetadata"));
}

#[test]
fn from_album_inherits_album_fields() {
    let album = AlbumMetadata {
        title: "My Album".into(),
        artist: "My Artist".into(),
        disc_number: 1,
        genre: vec!["Rock".to_string()],
        date: "2024-01-01".into(),
        ..Default::default()
    };

    let track = TrackMetadata::from_album(1, 1, &album);
    assert_eq!(track.number, 1);
    assert_eq!(track.artist, "My Artist");
    assert_eq!(track.album, "My Album");
    assert_eq!(track.date, "2024-01-01");
    assert_eq!(track.genre, vec!["Rock".to_string()]);
}

#[test]
fn from_album_applies_track_overrides() {
    let album = AlbumMetadata {
        title: "My Album".into(),
        artist: "My Artist".into(),
        disc_number: 1,
        tracks: vec![
            ("Song One".to_string(), "mbid-1".to_string()),
            ("Song Two".to_string(), "mbid-2".to_string()),
        ],
        ..Default::default()
    };

    let track1 = TrackMetadata::from_album(1, 1, &album);
    assert_eq!(track1.title, "Song One");
    assert_eq!(track1.track_id, "mbid-1");

    let track2 = TrackMetadata::from_album(2, 1, &album);
    assert_eq!(track2.title, "Song Two");
    assert_eq!(track2.track_id, "mbid-2");
}

#[test]
fn from_album_falls_back_to_defaults() {
    let album = AlbumMetadata {
        title: "My Album".into(),
        artist: "My Artist".into(),
        disc_number: 1,
        tracks: vec![], // no track entries
        ..Default::default()
    };

    let track = TrackMetadata::from_album(3, 1, &album);
    assert_eq!(track.number, 3);
    assert_eq!(track.title, "Track 3"); // fallback title
}
