import argparse
import shutil
import sys
from pathlib import Path

from .core import read_discid, print_disc_info, musicbrainz_search, select_release, extract_tracks
from .ripper import Ripper
from .utils import print_warning

def dependency_check() -> tuple[str, str]:
    """Check for dependencies: cdparanoia and flac."""
    cdparanoia_path = shutil.which("cdparanoia")
    if cdparanoia_path is None:
        print("cdparanoia not found")
        sys.exit(1)

    flac_path = shutil.which("flac")
    if flac_path is None:
        print("flac not found")
        sys.exit(1)

    return cdparanoia_path, flac_path

def main():
    parser = argparse.ArgumentParser(description="CD MusicBrainz Lookup + Rip + FLAC Encoding")
    parser.add_argument("device", nargs="?", default="/dev/sr0", help="CD device path (default: /dev/sr0)")
    parser.add_argument("-o", "--output", default=".", help="Output directory (default: current directory)")
    parser.add_argument("--cdparanoia", help="Path to cdparanoia executable")
    parser.add_argument("--flac", help="Path to flac executable")
    parser.add_argument("-y", "--yes", action="store_true", help="Skip confirmation prompt")

    args = parser.parse_args()

    cdparanoia_sys, flac_sys = dependency_check()
    cdparanoia_path = args.cdparanoia or cdparanoia_sys
    flac_path = args.flac or flac_sys

    disc = read_discid(args.device)
    print_disc_info(disc)

    result = musicbrainz_search(disc)
    tracks = []

    if result:
        print("\n" + "=" * 60)
        print("🎵  MusicBrainz results:")
        print("=" * 60)

        releases = []
        if "disc" in result:
            releases = result["disc"].get("release-list", [])
        elif "cdstub" in result:
            stub = result["cdstub"]
            print(f"  (CD Stub) Title  : {stub.get('title', '?')}")
            print(f"            Artist : {stub.get('artist', '?')}")

        if releases:
            release = select_release(releases)
            tracks = extract_tracks(release)

            names = [
                a["artist"]["name"]
                for a in release.get("artist-credit", [])
                if isinstance(a, dict) and "artist" in a
            ]
            print(f"\n  Artist : {', '.join(names)}")
            print(f"  Album   : {release.get('title', '?')}")
            print(f"  Year    : {release.get('date', '?')[:4]}")
            print(f"  Tracks  : {len(tracks)}\n")
            for t in tracks:
                print(f"    {t['TRACKNUMBER']:>2}. {t['TITLE']}")
        else:
            print_warning(" No releases found on MusicBrainz.")

    if not tracks:
        print_warning("Proceeding without MusicBrainz metadata.")
        for i in range(1, disc.last_track_num + 1):
            tracks.append(
                {
                    "TRACKNUMBER": i,
                    "TITLE": f"Track {i:02d}",
                    "ARTIST": "Unknown Artist",
                    "ALBUM_ARTIST": "Unknown Artist",
                    "ALBUM": "Unknown Album",
                    "TRACKTOTAL": disc.last_track_num,
                }
            )

    print()
    if not args.yes:
        try:
            resp = input("Ready to rip and encode to FLAC? [y/N] ").strip().lower()
            if resp not in ("y", "yes", "si", "s"):
                print("Operation aborted.")
                return
        except KeyboardInterrupt:
            print("\nAborted.")
            return

    ripper = Ripper(
        device=args.device,
        cdparanoia_path=cdparanoia_path,
        flac_path=flac_path,
        output_dir=Path(args.output)
    )
    ripper.rip_cd(tracks)

if __name__ == "__main__":
    main()
