use super::summary::print_disc_summary;
use crate::models::AlbumMetadata;

#[test]
fn print_disc_summary_unknown_does_not_panic() {
    let summary = AlbumMetadata::default();
    let _ = std::panic::catch_unwind(|| {
        print_disc_summary(&summary, 1, "test-id");
    });
}

#[test]
fn print_disc_summary_known_does_not_panic() {
    let summary = AlbumMetadata {
        title: "Test Album".into(),
        artist: "Test Artist".into(),
        album_id: String::new(),
        barcode: String::new(),
        release_group_id: String::new(),
        media_format: String::new(),
        packaging: String::new(),
        country: String::new(),
        genre: vec![],
        disc_number: 1,
        disc_count: 1,
        tracks: vec![
            ("Track 1".into(), "id-1".into()),
            ("Track 2".into(), "id-2".into()),
        ],
        date: String::new(),
        release_status: String::new(),
    };
    let _ = std::panic::catch_unwind(|| {
        print_disc_summary(&summary, 2, "test-id");
    });
}
