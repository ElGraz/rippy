use anyhow::{Result, anyhow};
use musicbrainz_rs::entity::discid::Discid;
use musicbrainz_rs::prelude::*;

use crate::models::AlbumMetadata;

/// Helper to extract an `Option<String>` or fall back to an empty string.
fn opt(s: Option<String>) -> String {
    s.unwrap_or_default()
}

pub fn fetch_album_metadata(disc_id: &str) -> Result<Vec<AlbumMetadata>> {
    let discid = match Discid::fetch()
        .id(disc_id)
        .with_recordings()
        .with_artist_credits()
        .with_release_groups()
        .execute()
    {
        Ok(d) => d,
        Err(e) => {
            let msg = format!("{:?}", e);
            if msg.contains("Not Found") {
                return Ok(vec![]);
            }
            return Err(anyhow!("MusicBrainz lookup failed: {:?}", e));
        }
    };

    let releases = match discid.releases {
        Some(r) if !r.is_empty() => r,
        _ => {
            return Ok(vec![]);
        }
    };

    let mut albums = Vec::new();

    for release in releases {
        let album_title = release.title.clone();
        let album_id = release.id.clone();
        let barcode = opt(release.barcode);
        let country = opt(release.country);

        // Safely transform ReleasePackaging enum variant to String if present
        let packaging = release
            .packaging
            .as_ref()
            .map(|p| format!("{:?}", p))
            .unwrap_or_default();

        let release_group_id = opt(release.release_group.as_ref().map(|rg| rg.id.clone()));

        // Extract date (YYYY-MM-DD format)
        let date = opt(release.date.as_ref().map(|d| d.0.to_string()));

        // Extract release status
        let release_status = opt(release.status_id);

        let artist_name = release
            .artist_credit
            .as_ref()
            .and_then(|ac| ac.first())
            .map(|credit| credit.name.clone())
            .unwrap_or_else(|| "Unknown Artist".to_string());

        let mut tracks = Vec::new();
        let mut media_format = String::new();

        if let Some(media) = &release.media {
            if let Some(medium) = media.first() {
                media_format = medium.format.clone().unwrap_or_default();
                if let Some(track_list) = &medium.tracks {
                    for track in track_list {
                        tracks.push((track.title.clone(), track.id.clone()));
                    }
                }
            }
        }

        albums.push(AlbumMetadata {
            title: album_title,
            artist: artist_name,
            album_id,
            barcode,
            release_group_id,
            media_format,
            packaging,
            country,
            tracks,
            date,
            release_status,
        });
    }

    Ok(albums)
}
