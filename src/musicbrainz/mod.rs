use anyhow::{Result, anyhow};
use musicbrainz_rs::entity::discid::Discid;
use musicbrainz_rs::entity::release_group::ReleaseGroup;
use musicbrainz_rs::prelude::*;

use crate::models::AlbumMetadata;

/// Extracts up to the top 3 top genres/tags from a ReleaseGroup, sorted by popularity.
/// Returns an empty vector if no genres or tags are found.
pub fn extract_top_genres(release_group: Option<&ReleaseGroup>) -> Vec<String> {
    let rg = match release_group {
        Some(group) => group,
        None => return Vec::new(),
    };

    let mut weighted_tags: Vec<(String, u32)> = Vec::new();

    if let Some(genres) = &rg.genres {
        for g in genres {
            weighted_tags.push((g.name.clone(), g.count.unwrap_or(100)));
        }
    }

    if let Some(tags) = &rg.tags {
        for t in tags {
            weighted_tags.push((t.name.clone(), t.count.unwrap_or(1) as u32));
        }
    }

    weighted_tags.sort_by(|a, b| b.1.cmp(&a.1));

    let mut seen = std::collections::HashSet::new();
    let mut unique_genres = Vec::new();

    for (name, _) in weighted_tags {
        if !seen.contains(&name) {
            seen.insert(name.clone());
            unique_genres.push(name);
        }
    }

    unique_genres.truncate(3);
    unique_genres
}

pub fn fetch_album_metadata(disc_id: &str) -> Result<Vec<AlbumMetadata>> {
    let discid_data = match Discid::fetch()
        .id(disc_id)
        .with_recordings()
        .with_artist_credits()
        .with_release_groups()
        .with_genres()
        .with_tags()
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

    let mut albums = Vec::new();

    if let Some(releases) = discid_data.releases {
        for release in releases {
            let album_title = release.title.clone();
            let album_id = release.id.clone();
            let barcode = release.barcode.unwrap_or_default();
            let country = release.country.unwrap_or_default();
            let date = release.date.map(|d| d.0.clone()).unwrap_or_default();
            let release_status = release
                .status
                .map(|s| format!("{:?}", s))
                .unwrap_or_default();
            let packaging = release
                .packaging
                .map(|p| format!("{:?}", p))
                .unwrap_or_default();

            let artist_name = release
                .artist_credit
                .as_ref()
                .and_then(|credit| credit.first())
                .map(|c| c.name.clone())
                .unwrap_or_else(|| "Unknown Artist".to_string());

            let release_group_id = release
                .release_group
                .as_ref()
                .map(|rg| rg.id.clone())
                .unwrap_or_default();

            let genres = extract_top_genres(release.release_group.as_ref());

            // Identify media layout and find the exact medium matching our target disc_id
            let media_list = release.media.unwrap_or_default();
            let disc_count = media_list.len() as u32;

            let mut disc_number = 1;
            let mut target_medium = None;

            for medium in &media_list {
                if let Some(ref discs) = medium.discs {
                    if discs.iter().any(|d| d.id == disc_id) {
                        disc_number = medium.position.unwrap_or(1);
                        target_medium = Some(medium.clone());
                        break;
                    }
                }
            }

            // Fallback to the first medium if structural links are loose
            let active_medium = target_medium.or_else(|| media_list.first().cloned());
            let media_format = active_medium
                .as_ref()
                .and_then(|m| m.format.as_ref())
                .map(|f| format!("{:?}", f))
                .unwrap_or_default();

            let mut tracks_vec = Vec::new();
            if let Some(medium) = active_medium {
                if let Some(tracks) = medium.tracks {
                    for track in tracks {
                        let track_title = track.title;
                        let track_mbid = track.recording.map(|r| r.id).unwrap_or_default();
                        tracks_vec.push((track_title, track_mbid));
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
                genre: genres,
                disc_number,
                disc_count,
                tracks: tracks_vec,
                date,
                release_status,
            });
        }
    }

    Ok(albums)
}
