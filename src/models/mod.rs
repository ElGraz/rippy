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
