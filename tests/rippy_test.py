import pytest
from rippy.core import extract_tracks
from rippy.utils import sanitize_filename

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
    minimal_release = {
        "medium-list": [
            {
                "track-list": [{"number": "1"}]
            }
        ]
    }
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

def test_sanitize_standard_replacement():
    """Test that forbidden characters are replaced."""
    assert sanitize_filename("Artist / Album") == "Artist_Album"
    assert sanitize_filename("What? File") == "What_File"

def test_sanitize_no_space_behavior():
    """Test behavior when forbidden characters are NOT followed by a space."""
    assert sanitize_filename("Artist/Album") == "Artist_Album"
    assert sanitize_filename("Data:2024") == "Data_2024"

def test_sanitize_strip_whitespace():
    """Ensure leading and trailing whitespace is removed."""
    assert sanitize_filename("  My Song.mp3  ") == "My_Song.mp3"

def test_sanitize_empty_string():
    """Ensure it handles empty input gracefully."""
    assert sanitize_filename("") == ""

def test_sanitize_multiple_special_chars():
    """Test multiple replacements in one string."""
    # My new regex and multiple underscore collapse results in a single underscore if multiple special chars are adjacent or separated by spaces
    assert sanitize_filename("Menu: <Open> | Save ") == "Menu_Open_Save"
