import pytest

from rippy.utils import sanitize_filename


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
