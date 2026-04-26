from unittest.mock import patch

import pytest

from rippy.core import _process_track_list, extract_tracks, select_release


@pytest.fixture
def mock_release():
    """Returns a standard MusicBrainz-style release dictionary."""
    return {
        "id": "release-123",
        "title": "Random Access Memories",
        "date": "2013-05-17",
        "artist-credit": [{"artist": {"name": "Daft Punk"}}],
        "release-group": {"id": "rg-456"},
        "medium-list": [
            {
                "format": "CD",
                "track-list": [
                    {
                        "number": "1",
                        "recording": {
                            "id": "rec-001",
                            "title": "Give Life Back to Music",
                            "artist-credit": [{"artist": {"name": "Daft Punk"}}],
                        },
                    }
                ],
            }
        ],
    }


def test_process_track_list_basic():
    """Test basic track list processing with minimal data."""
    release = {
        "artist-credit": [{"artist": {"name": "Test Artist"}}],
        "title": "Test Album",
        "date": "2023-01-01",
    }

    track_list = [
        {
            "number": "1",
            "title": "Test Track 1",
            "recording": {
                "title": "Recording Title 1",
                "artist-credit": [{"artist": {"name": "Recording Artist 1"}}],
            },
        }
    ]

    medium = {"format": "CD"}
    tracks_len = 10

    result = _process_track_list(release, track_list, medium, tracks_len)

    assert len(result) == 1
    track = result[0]
    assert track["TRACKNUMBER"] == 1
    assert track["TITLE"] == "Recording Title 1"
    assert track["ARTIST"] == "Recording Artist 1"
    assert track["ALBUM_ARTIST"] == "Test Artist"
    assert track["ALBUM"] == "Test Album"
    assert track["DATE"] == "2023-01-01"
    assert track["TRACKTOTAL"] == 10
    assert track["MEDIA"] == "CD"


def test_process_track_list_no_track_number():
    """Test processing tracks without explicit track numbers."""
    release = {
        "artist-credit": [{"artist": {"name": "Test Artist"}}],
        "title": "Test Album",
    }

    track_list = [
        {"title": "Track Without Number", "recording": {"title": "Recording Title"}}
    ]

    medium = {}
    tracks_len = 5

    result = _process_track_list(release, track_list, medium, tracks_len)

    assert len(result) == 1
    track = result[0]
    assert track["TRACKNUMBER"] == 1  # Should default to position in list
    assert track["TITLE"] == "Recording Title"


def test_process_track_list_missing_recording_title():
    """Test processing tracks when recording title is missing."""
    release = {
        "artist-credit": [{"artist": {"name": "Test Artist"}}],
        "title": "Test Album",
    }

    track_list = [{"number": "2", "title": "Track Title", "recording": {}}]

    medium = {}
    tracks_len = 3

    result = _process_track_list(release, track_list, medium, tracks_len)

    assert len(result) == 1
    track = result[0]
    assert track["TRACKNUMBER"] == 2
    assert track["TITLE"] == "Track Title"  # Should fall back to track title


def test_process_track_list_fallback_to_album_artist():
    """Test that track artist falls back to album artist when not specified."""
    release = {
        "artist-credit": [{"artist": {"name": "Album Artist"}}],
        "title": "Test Album",
    }

    track_list = [
        {
            "number": "1",
            "title": "Track Title",
            "recording": {
                "title": "Recording Title"
                # No artist-credit in recording
            },
        }
    ]

    medium = {}
    tracks_len = 1

    result = _process_track_list(release, track_list, medium, tracks_len)

    assert len(result) == 1
    track = result[0]
    assert track["ARTIST"] == "Album Artist"  # Should fall back to album artist


def test_process_track_list_multiple_artists():
    """Test processing tracks with multiple artists."""
    release = {
        "artist-credit": [
            {"artist": {"name": "Artist 1"}},
            {"artist": {"name": "Artist 2"}},
        ],
        "title": "Test Album",
    }

    track_list = [
        {
            "number": "1",
            "title": "Track Title",
            "recording": {
                "title": "Recording Title",
                "artist-credit": [
                    {"artist": {"name": "Recording Artist 1"}},
                    {"artist": {"name": "Recording Artist 2"}},
                ],
            },
        }
    ]

    medium = {}
    tracks_len = 1

    result = _process_track_list(release, track_list, medium, tracks_len)

    assert len(result) == 1
    track = result[0]
    assert track["ALBUM_ARTIST"] == "Artist 1, Artist 2"
    assert track["ARTIST"] == "Recording Artist 1, Recording Artist 2"


def test_process_track_list_empty_release_fields():
    """Test processing tracks with empty release fields."""
    release = {"artist-credit": [], "title": "", "date": ""}

    track_list = [
        {
            "number": "1",
            "title": "Track Title",
            "recording": {"title": "Recording Title"},
        }
    ]

    medium = {}
    tracks_len = 1

    result = _process_track_list(release, track_list, medium, tracks_len)

    assert len(result) == 1
    track = result[0]
    assert track["ALBUM_ARTIST"] == "Unknown"
    assert track["ALBUM"] == "Unknown Album"


def test_process_track_list_with_release_metadata():
    """Test processing tracks with various release metadata."""
    release = {
        "artist-credit": [{"artist": {"name": "Test Artist"}}],
        "title": "Test Album",
        "date": "2023-01-01",
        "barcode": "123456789012",
        "asin": "B012345678",
        "id": "release-id-123",
        "release-group": {"id": "group-id-456"},
        "status": "Official",
        "packaging": "Packaged",
        "country": "US",
        "text-representation": {"language": "en"},
    }

    track_list = [
        {
            "number": "1",
            "title": "Track Title",
            "recording": {"title": "Recording Title", "id": "track-id-789"},
        }
    ]

    medium = {"format": "CD"}
    tracks_len = 10

    result = _process_track_list(release, track_list, medium, tracks_len)

    assert len(result) == 1
    track = result[0]
    assert track["BARCODE"] == "123456789012"
    assert track["ASIN"] == "B012345678"
    assert track["MUSICBRAINZ_ALBUMID"] == "release-id-123"
    assert track["MUSICBRAINZ_RELEASEGROUPID"] == "group-id-456"
    assert track["RELEASESTATUS"] == "Official"
    assert track["RELEASEPACKAGING"] == "Packaged"
    assert track["RELEASECOUNTRY"] == "US"
    assert track["LANGUAGE"] == "en"
    assert track["MUSICBRAINZ_TRACKID"] == "track-id-789"
    assert track["MEDIA"] == "CD"


def test_process_track_list_multiple_tracks():
    """Test processing multiple tracks."""
    release = {
        "artist-credit": [{"artist": {"name": "Test Artist"}}],
        "title": "Test Album",
        "date": "2023-01-01",
    }

    track_list = [
        {
            "number": "1",
            "title": "Track 1",
            "recording": {
                "title": "Recording 1",
                "artist-credit": [{"artist": {"name": "Artist 1"}}],
            },
        },
        {
            "number": "2",
            "title": "Track 2",
            "recording": {
                "title": "Recording 2",
                "artist-credit": [{"artist": {"name": "Artist 2"}}],
            },
        },
    ]

    medium = {"format": "CD"}
    tracks_len = 2

    result = _process_track_list(release, track_list, medium, tracks_len)

    assert len(result) == 2
    assert result[0]["TRACKNUMBER"] == 1
    assert result[0]["TITLE"] == "Recording 1"
    assert result[0]["ARTIST"] == "Artist 1"
    assert result[1]["TRACKNUMBER"] == 2
    assert result[1]["TITLE"] == "Recording 2"
    assert result[1]["ARTIST"] == "Artist 2"


def test_extract_tracks_standard(mock_release):
    """Test extraction with a well-formed release dictionary."""
    tracks = extract_tracks(mock_release)

    assert len(tracks) == 1
    track = tracks[0]
    assert track["TITLE"] == "Give Life Back to Music"
    assert track["ALBUM"] == "Random Access Memories"
    assert track["ARTIST"] == "Daft Punk"
    assert track["TRACKNUMBER"] == 1
    assert track["MEDIA"] == "CD"


def test_extract_tracks_missing_fields():
    """Test fallback logic when essential keys are missing."""
    empty_release = {}
    tracks = extract_tracks(empty_release)
    assert isinstance(tracks, list)
    assert len(tracks) == 0


def test_extract_tracks_fallback_names():
    """Test that 'Unknown' defaults work correctly."""
    minimal_release = {"medium-list": [{"track-list": [{"number": "1"}]}]}
    tracks = extract_tracks(minimal_release)

    assert tracks[0]["ALBUM"] == "Unknown Album"
    assert tracks[0]["ARTIST"] == "Unknown"
    assert tracks[0]["TITLE"] == "Track 1"


def test_multiple_artists_join(mock_release):
    """Verify multiple artists are joined by commas."""
    mock_release["artist-credit"].append({"artist": {"name": "Pharrell Williams"}})
    tracks = extract_tracks(mock_release)
    assert tracks[0]["ALBUM_ARTIST"] == "Daft Punk, Pharrell Williams"


def test_track_total_count(mock_release):
    """Ensure TRACKTOTAL reflects the count per medium."""
    mock_release["medium-list"][0]["track-list"].append(
        {"number": "2", "recording": {"title": "The Game of Love"}}
    )
    tracks = extract_tracks(mock_release)
    assert len(tracks) == 2
    assert tracks[0]["TRACKTOTAL"] == 2
    assert tracks[1]["TRACKTOTAL"] == 2


def test_select_release_single_release():
    """Test that select_release returns the only release when there's just one."""
    release = {
        "artist-credit": [{"artist": {"name": "Test Artist"}}],
        "title": "Test Album",
        "date": "2023-01-01",
        "id": "release-id-123",
    }

    result = select_release([release])
    assert result == release


def test_select_release_multiple_releases():
    """Test that select_release properly selects a release from multiple options."""
    release1 = {
        "artist-credit": [{"artist": {"name": "Artist 1"}}],
        "title": "Album 1",
        "date": "2023-01-01",
        "id": "release-id-123",
        "country": "US",
        "barcode": "123456789012",
    }

    release2 = {
        "artist-credit": [{"artist": {"name": "Artist 2"}}],
        "title": "Album 2",
        "date": "2023-01-02",
        "id": "release-id-456",
        "country": "UK",
        "barcode": "987654321098",
    }

    releases = [release1, release2]

    # Test selecting first release
    with patch("builtins.input", return_value="1"):
        result = select_release(releases)
        assert result == release1

    # Test selecting second release
    with patch("builtins.input", return_value="2"):
        result = select_release(releases)
        assert result == release2


def test_select_release_invalid_selection():
    """Test that select_release handles invalid selections properly."""
    release1 = {
        "artist-credit": [{"artist": {"name": "Artist 1"}}],
        "title": "Album 1",
        "date": "2023-01-01",
        "id": "release-id-123",
    }

    release2 = {
        "artist-credit": [{"artist": {"name": "Artist 2"}}],
        "title": "Album 2",
        "date": "2023-01-02",
        "id": "release-id-456",
    }

    releases = [release1, release2]

    # Test with invalid input followed by valid input
    with patch("builtins.input", side_effect=["invalid", "2"]):
        result = select_release(releases)
        assert result == release2
