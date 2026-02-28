import os
import subprocess
import sys
from posix import device_encoding

import discid
import musicbrainzngs

# -----------------------------------------------------------------------------
# SETUP
# -----------------------------------------------------------------------------
# MusicBrainz requires a User-Agent string to identify your application.
APP_NAME = "Ripper"
VERSION = "1.0"
CONTACT = "info@elgraz.net"
DEVICE = "/dev/sr0"


def setup_musicbrainz():
    """Initializes the MusicBrainz NGS library with a proper User-Agent."""
    musicbrainzngs.set_useragent(APP_NAME, VERSION, CONTACT)


def get_id_from_drive(device=DEVICE):
    """Reads the physical CD in the specified drive and returns the Disc ID."""
    try:
        print(f"Reading CD from {device}...")
        disc = discid.read(device)
        return disc.id
    except discid.DiscError as e:
        print(f"Error reading disc: {e}")
        return None
    except Exception as e:
        print(f"Unexpected error accessing drive: {e}")
        return None


def lookup_disc_id(disc_id):
    """Queries MusicBrainz and returns the release list."""
    try:
        print(f"Searching MusicBrainz for ID: {disc_id}...")
        result = musicbrainzngs.get_releases_by_discid(
            id=disc_id, includes=["artists", "recordings"]
        )
        if "disc" in result and "release-list" in result["disc"]:
            return result["disc"]["release-list"]
    except Exception as e:
        print(f"MusicBrainz Error: {e}")
    return []


def rip_and_encode(release):
    """Iterates through tracks, rips with cdparanoia, and encodes with flac."""
    album = release.get("title", "Unknown Album")
    artist = release.get("artist-credit-phrase", "Unknown Artist")
    year = release.get("date", "")[:4]  # Get YYYY from YYYY-MM-DD

    print(f"\nProcessing Album: {album} by {artist}")

    # Create directory for the album
    dir_name = f"{artist} - {album}".replace("/", "-")
    if not os.path.exists(dir_name):
        os.makedirs(dir_name)

    # Get track list from the first medium
    try:
        tracks = release["medium-list"][0]["track-list"]
    except (KeyError, IndexError):
        print("Could not find tracklist information.")
        return

    for track in tracks:
        num = track.get("number", "0")
        title = track.get("recording", {}).get("title", "Unknown Track")

        wav_file = f"track{num}.wav"
        flac_file = os.path.join(
            dir_name, f"{num.zfill(2)} - {title}.flac".replace("/", "-")
        )

        print(f"\n>>> Ripping Track {num}: {title}")

        # 1. Rip with cdparanoia
        # [num] specifies the track number to rip
        rip_cmd = ["cdparanoia", str(num), wav_file]
        subprocess.run(rip_cmd, check=True)

        # 2. Encode and Tag with flac
        print(">>> Encoding to FLAC...")
        flac_cmd = [
            "flac",
            "--best",
            "-T",
            f"TITLE={title}",
            "-T",
            f"ARTIST={artist}",
            "-T",
            f"ALBUALBUM={album}",
            "-T",
            f"TRACKNUMBER={num}",
            "-T",
            f"DATE={year}",
            "-o",
            flac_file,
            wav_file,
        ]
        subprocess.run(flac_cmd, check=True)

        # 3. Clean up temp WAV
        if os.path.exists(wav_file):
            os.remove(wav_file)

    print(f"\nFinished! Files saved in: {dir_name}")


def main():
    """Main execution flow."""
    setup_musicbrainz()

    # Get Disc ID
    if len(sys.argv) > 1:
        target_id = sys.argv[1]
    else:
        target_id = get_id_from_drive("/dev/sr0")

    if not target_id:
        print("Error: Could not obtain Disc ID.")
        return

    results = lookup_disc_id(target_id)

    if not results:
        print("No matches found on MusicBrainz.")
        return

    # Automatically accept the first match
    selected_release = results[0]
    rip_and_encode(selected_release)


if __name__ == "__main__":
    main()
