use super::{AlbumMetadata, TrackMetadata};

#[test]
fn track_metadata_default_values() {
    let meta = TrackMetadata {
        number: 1,
        title: "Test Track".to_string(),
        artist: "Test Artist".to_string(),
        album: "Test Album".to_string(),
        album_id: String::new(),
        barcode: String::new(),
        track_id: String::new(),
        release_group_id: String::new(),
        media_format: String::new(),
        packaging: String::new(),
        country: String::new(),
        disc_number: Some(1),
        date: String::new(),
        release_status: String::new(),
    };

    assert_eq!(meta.number, 1);
    assert_eq!(meta.title, "Test Track");
    assert_eq!(meta.artist, "Test Artist");
    assert_eq!(meta.album, "Test Album");
    assert_eq!(meta.disc_number, Some(1));
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
        disc_number: None,
        date: "2024-01-01".to_string(),
        release_status: "official".to_string(),
    };

    assert_eq!(meta.number, 3);
    assert_eq!(meta.disc_number, None);
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
}

#[test]
fn album_metadata_empty_tracks() {
    let meta = AlbumMetadata {
        title: "Instrumental".to_string(),
        artist: "Orchestra".to_string(),
        album_id: String::new(),
        barcode: String::new(),
        release_group_id: String::new(),
        media_format: String::new(),
        packaging: String::new(),
        country: String::new(),
        tracks: vec![],
        date: String::new(),
        release_status: String::new(),
    };

    assert!(meta.tracks.is_empty());
}
