use flac_codec::metadata::VorbisComment;

use crate::models::TrackMetadata;

/// Builds a Vorbis comment block from track metadata.
pub fn build_comments(meta: &TrackMetadata, total_tracks: u32) -> VorbisComment {
    let mut comments = VorbisComment::default();

    // Required fields
    let _ = comments.insert("TITLE", &meta.title);
    let _ = comments.insert("ARTIST", &meta.artist);
    let _ = comments.insert("ALBUM", &meta.album);
    let _ = comments.insert("TRACKNUMBER", &meta.number.to_string());
    let _ = comments.insert("TRACKTOTAL", &total_tracks.to_string());

    // Optional fields — only insert if non-empty
    macro_rules! try_insert {
        ($key:expr, $value:expr) => {
            if !$value.is_empty() {
                let _ = comments.insert($key, &$value);
            }
        };
    }

    if let Some(disc) = meta.disc_number {
        let _ = comments.insert("DISCNUMBER", &disc.to_string());
    }

    try_insert!("DATE", &meta.date);
    try_insert!("RELEASESTATUS", &meta.release_status);
    try_insert!("MUSICBRAINZ_ALBUMID", &meta.album_id);
    try_insert!("BARCODE", &meta.barcode);
    try_insert!("MUSICBRAINZ_TRACKID", &meta.track_id);
    try_insert!("MUSICBRAINZ_RELEASEGROUPID", &meta.release_group_id);
    try_insert!("MEDIA", &meta.media_format);
    try_insert!("RELEASEPACKAGING", &meta.packaging);
    try_insert!("RELEASECOUNTRY", &meta.country);

    comments
}

#[cfg(test)]
mod tests {
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
        meta.disc_number = Some(1);
        meta.date = "2024-01-01".into();
        meta.release_status = "official".into();
        meta
    }

    #[test]
    fn comments_includes_required_fields() {
        let comments = build_comments(&sample_meta(), 10);
        assert!(comments.get("TITLE").is_some());
        assert!(comments.get("ARTIST").is_some());
        assert!(comments.get("ALBUM").is_some());
        assert!(comments.get("TRACKNUMBER").is_some());
        assert!(comments.get("TRACKTOTAL").is_some());
    }

    #[test]
    fn comments_includes_optional_fields() {
        let comments = build_comments(&sample_meta(), 10);
        assert!(comments.get("DATE").is_some());
        assert!(comments.get("MUSICBRAINZ_ALBUMID").is_some());
        assert!(comments.get("DISCNUMBER").is_some());
    }

    #[test]
    fn comments_skips_empty_fields() {
        let meta = sample_meta();
        let comments = build_comments(&meta, 10);
        // These are empty in sample_meta
        assert!(comments.get("BARCODE").is_none());
        assert!(comments.get("MUSICBRAINZ_RELEASEGROUPID").is_none());
    }

    #[test]
    fn tracknumber_contains_correct_value() {
        let mut meta = TrackMetadata::default();
        meta.number = 7;
        let comments = build_comments(&meta, 10);
        assert_eq!(comments.get("TRACKNUMBER").unwrap(), "7");
    }

    #[test]
    fn tracktotal_contains_correct_value() {
        let meta = TrackMetadata::default();
        let comments = build_comments(&meta, 42);
        assert_eq!(comments.get("TRACKTOTAL").unwrap(), "42");
    }

    #[test]
    fn empty_metadata_has_no_optional_fields() {
        let meta = TrackMetadata::default();
        // In default, title and album are empty strings — they should still be present as required fields
        // but all optional fields should be absent.
        let comments = build_comments(&meta, 1);

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
            "DISCNUMBER",
        ];
        for key in &optional_keys {
            assert!(
                comments.get(*key).is_none(),
                "Optional field {} should not be present for default metadata",
                key
            );
        }
    }
}
