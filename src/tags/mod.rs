use flac_codec::metadata::VorbisComment;

use crate::models::{AlbumMetadata, TrackMetadata};

/// Builds a Vorbis comment block from track metadata.
pub fn build_comments(
    album: &AlbumMetadata,
    meta: &TrackMetadata,
    total_tracks: u32,
) -> VorbisComment {
    let mut comments = VorbisComment::default();

    // Required fields
    let _ = comments.insert("TITLE", &meta.title);
    let _ = comments.insert("ARTIST", &meta.artist);
    let _ = comments.insert("ALBUM", &meta.album);
    let _ = comments.insert("TRACKNUMBER", &meta.number.to_string());
    let _ = comments.insert("TRACKTOTAL", &total_tracks.to_string());

    macro_rules! try_insert {
        ($key:expr, $value:expr) => {
            if !$value.is_empty() {
                let _ = comments.insert($key, &$value);
            }
        };
    }

    // Genre: insert only from track metadata (which already includes album genres).
    // Avoid inserting twice by using try_insert on meta.genre.
    for genre in &meta.genre {
        try_insert!("GENRE", genre);
    }

    if album.disc_count > 1 {
        let _ = comments.insert("DISCNUMBER", &album.disc_number.to_string());
        let _ = comments.insert("TOTALDISCS", &album.disc_count.to_string());
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
mod tests;
