use crate::ui::summary;

#[test]
fn disc_summary_unknown_disc() {
    let summary = summary::DiscSummary {
        album_title: None,
        artist: None,
        tracks: vec![],
        total_tracks: 3,
        unknown_disc: true,
    };

    assert!(summary.unknown_disc);
    assert!(summary.album_title.is_none());
    assert_eq!(summary.total_tracks, 3);
}

#[test]
fn disc_summary_known_album() {
    let summary = summary::DiscSummary {
        album_title: Some("Test Album".to_string()),
        artist: Some("Test Artist".to_string()),
        tracks: vec![
            ("Track 1".to_string(), "id-1".to_string()),
            ("Track 2".to_string(), "id-2".to_string()),
        ],
        total_tracks: 2,
        unknown_disc: false,
    };

    assert!(!summary.unknown_disc);
    assert_eq!(summary.album_title.as_deref(), Some("Test Album"));
    assert_eq!(summary.tracks.len(), 2);
}

#[test]
fn disc_summary_empty_tracks() {
    let summary = summary::DiscSummary {
        album_title: Some("Empty".to_string()),
        artist: Some("Artist".to_string()),
        tracks: vec![],
        total_tracks: 0,
        unknown_disc: false,
    };

    assert_eq!(summary.total_tracks, 0);
    assert!(summary.tracks.is_empty());
}
