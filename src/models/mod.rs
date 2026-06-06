#[derive(Clone, Debug)]
pub struct TrackMetadata {
    pub number: u32,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_id: String,
    pub barcode: String,
    pub track_id: String,
    pub release_group_id: String,
    pub media_format: String,
    pub packaging: String,
    pub country: String,
    pub genre: Vec<String>,
    pub disc_number: Option<u32>,
    pub date: String,
    pub release_status: String,
}

impl Default for TrackMetadata {
    fn default() -> Self {
        Self {
            number: 0,
            title: String::new(),
            artist: "Unknown Artist".into(),
            album: "Unknown Album".into(),
            album_id: String::new(),
            barcode: String::new(),
            track_id: String::new(),
            release_group_id: String::new(),
            media_format: String::new(),
            packaging: String::new(),
            country: String::new(),
            genre: Vec::new(),
            disc_number: None,
            date: String::new(),
            release_status: String::new(),
        }
    }
}

impl TrackMetadata {
    /// Create a track metadata entry from album info, filling in per-track overrides.
    pub fn from_album(track_idx: u32, first_track: u32, album: &AlbumMetadata) -> Self {
        let mut meta = Self::default();
        meta.number = track_idx;
        meta.title = format!("Track {}", track_idx);

        // Inherit album-level fields
        meta.artist.clone_from(&album.artist);
        meta.album.clone_from(&album.title);
        meta.album_id.clone_from(&album.album_id);
        meta.barcode.clone_from(&album.barcode);
        meta.release_group_id.clone_from(&album.release_group_id);
        meta.media_format.clone_from(&album.media_format);
        meta.packaging.clone_from(&album.packaging);
        meta.country.clone_from(&album.country);
        meta.disc_number = Some(album.disc_number);
        meta.date.clone_from(&album.date);
        meta.release_status.clone_from(&album.release_status);

        // Inherit genres from album (used by tags::build_comments).
        meta.genre.extend(album.genre.iter().cloned());

        // Apply per-track overrides if available
        if let Some((title, track_id)) = album
            .tracks
            .get((track_idx - first_track) as usize)
            .cloned()
        {
            meta.title = title;
            meta.track_id = track_id;
        }

        meta
    }
}

#[derive(Clone, Debug, Default)]
pub struct AlbumMetadata {
    pub title: String,
    pub artist: String,
    pub album_id: String,
    pub barcode: String,
    pub release_group_id: String,
    pub media_format: String,
    pub packaging: String,
    pub country: String,
    pub genre: Vec<String>,
    pub disc_number: u32,
    pub disc_count: u32,
    pub tracks: Vec<(String, String)>, // (Track Title, Track MBID)
    pub date: String,
    pub release_status: String,
}

#[cfg(test)]
mod tests;
