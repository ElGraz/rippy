import sys
import discid
import musicbrainzngs
from .utils import print_error, print_warning

# --- Configurazione MusicBrainz ---
musicbrainzngs.set_useragent(
    app="Ripper", version="1.1", contact="https://codeberg.org/gpz/rippy"
)

def read_discid(device: str = "/dev/sr0") -> discid.Disc:
    """Read the Disc ID from the specified CD device."""
    print(f"🔍 Reading Disc ID from {device} ...")
    try:
        disc = discid.read(device)
        return disc
    except discid.DiscError as e:
        print_error(f"Error reading the disc: {e}")
        print("   Make sure there is a CD audio in the reader.")
        sys.exit(1)

def musicbrainz_search(disc: discid.Disc) -> dict | None:
    """Search for the CD on MusicBrainz using the Disc ID."""
    print(f"🌐 Searching on MusicBrainz for Disc ID: {disc.id}")
    try:
        result = musicbrainzngs.get_releases_by_discid(
            disc.id, includes=["artists", "recordings", "release-groups"]
        )
        return result
    except musicbrainzngs.ResponseError as e:
        if "404" in str(e):
            print_warning("No results found on MusicBrainz for this Disc ID.")
        else:
            print_error(f"MusicBrainz response error: {e}")
        return None
    except musicbrainzngs.NetworkError as e:
        print_error(f"Network error: {e}")
        return None

def print_disc_info(disc: discid.Disc) -> None:
    """Show disc information."""
    print("\n" + "=" * 60)
    print("💿  Disc Information")
    print("=" * 60)
    print(f"  Disc ID      : {disc.id}")
    print(f"  FreeDB ID    : {disc.freedb_id}")
    print(f"  Tracks       : {disc.last_track_num}")
    print(
        f"  Duration     : {disc.sectors // 75 // 60}:{disc.sectors // 75 % 60:02d} min"
    )
    print(
        f"  MusicBrainz  : https://musicbrainz.org/cdtoc/attach?id={disc.id}"
        f"&tracks={disc.last_track_num}&toc={disc.toc_string}"
    )

def extract_tracks(release: dict) -> list[dict]:
    """Extracts track metadata from a MusicBrainz release."""
    artists = [
        a["artist"].get("name", "")
        for a in release.get("artist-credit", [])
        if isinstance(a, dict) and "artist" in a
    ]
    album_artist = ", ".join(artists) or "Unknown"
    album = release.get("title", "Unknown Album")
    date = release.get("date", "")

    tracks = []
    for medium in release.get("medium-list", []):
        track_list = medium.get("track-list", [])
        tracks_len = len(track_list)
        for track in track_list:
            num = track.get("number", str(len(tracks) + 1))
            recording = track.get("recording", {})
            title = recording.get("title", track.get("title", f"Track {num}"))

            # Track artist
            track_artist = [
                a["artist"].get("name", "")
                for a in recording.get("artist-credit", [])
                if isinstance(a, dict) and "artist" in a
            ]
            track_artist = ", ".join(track_artist) or album_artist

            meta = {
                "TRACKNUMBER": int(num) if str(num).isdigit() else len(tracks) + 1,
                "TITLE": title,
                "ARTIST": track_artist,
                "ALBUM_ARTIST": album_artist,
                "ALBUM": album,
                "DATE": date,
                "TRACKTOTAL": tracks_len,
                "BARCODE": release.get("barcode"),
                "ASIN": release.get("asin"),
                "MUSICBRAINZ_ALBUMID": release.get("id"),
                "MUSICBRAINZ_RELEASEGROUPID": release.get("release-group", {}).get(
                    "id"
                ),
                "MEDIA": medium.get("format"),
                "RELEASESTATUS": release.get("status"),
                "RELEASEPACKAGING": release.get("packaging"),
                "RELEASECOUNTRY": release.get("country"),
                "LANGUAGE": release.get("text-representation", {}).get("language"),
                "MUSICBRAINZ_TRACKID": track.get("recording", {}).get("id"),
            }
            # Remove any empty fields
            meta = {k: v for k, v in meta.items() if v}
            tracks.append(meta)

    return tracks

def select_release(releases: list[dict]) -> dict:
    """If more than one release is found, prompt the user to select the correct one."""
    if len(releases) == 1:
        return releases[0]

    print(f"\n  Multiple ({len(releases)}) releases found. \n Select the one to use:\n")
    for i, r in enumerate(releases, 1):
        names = [
            a["artist"]["name"]
            for a in r.get("artist-credit", [])
            if isinstance(a, dict) and "artist" in a
        ]
        artist = ", ".join(names) or "?"
        title = r.get("title", "?")
        date = r.get("date", "?")[:4]
        cc = r.get("country", "?")
        bar = r.get("barcode", "?")
        print(f"  [{i}] {artist} — {title} ({date}, {cc}, {bar})")

    while True:
        try:
            selected = int(input(f"\n  Select [1-{len(releases)}]: "))
            if 1 <= selected <= len(releases):
                return releases[selected - 1]
        except (ValueError, KeyboardInterrupt):
            pass
        print_warning(f"Please select a number between 1 and {len(releases)}")
