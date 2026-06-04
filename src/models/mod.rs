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
            disc_number: None,
            date: String::new(),
            release_status: String::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AlbumMetadata {
    pub title: String,
    pub artist: String,
    pub album_id: String,
    pub barcode: String,
    pub release_group_id: String,
    pub media_format: String,
    pub packaging: String,
    pub country: String,
    pub tracks: Vec<(String, String)>, // (Track Title, Track MBID)
    pub date: String,
    pub release_status: String,
}

#[cfg(test)]
mod tests;
