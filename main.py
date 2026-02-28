#!/usr/bin/env python3
"""
CD MusicBrainz Lookup + Rip + FLAC Encoding
"""

import itertools
import re
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path

try:
    import discid
except ImportError:
    print("❌ Needs 'discid': pip install discid")
    print("   and the system library: sudo apt install libdiscid0")
    sys.exit(1)

try:
    import musicbrainzngs
except ImportError:
    print("❌ Needs 'musicbrainzngs': pip install musicbrainzngs")
    sys.exit(1)


# --- Configurazione MusicBrainz ---
musicbrainzngs.set_useragent(
    app="CD-Ripper-FLAC", version="2.0", contact="https://github.com/elgraz"
)

# --- Directory di output base ---
OUTPUT_DIR = Path("./ripped")
CDPARANOIA = shutil.which("cdparanoia")
SPINNER_FRAMES = ["◐◓◑◒", "⣾⣽⣻⢿⡿⣟⣯⣷", "▏▎▍▌▋▊▉█▉▊▌▍▎▏", ".oO0Oo.", "·∘○◉○∘·"]

# Color codes
RED = "\033[91m"
GREEN = "\033[92m"
YELLOW = "\033[93m"
CYAN = "\033[96m"
RESET = "\033[0m"
BOLD = "\033[1m"
DIM = "\033[2m"

# ---------------------------------------------------------------------------
# Utilità
# ---------------------------------------------------------------------------


def print_error(message: str):
    """Print an error message."""
    print(f"\n{RED}❌{RESET} {message}")


def print_success(message: str):
    """Print a success message."""
    print(f"\n{GREEN}✔{RESET} {message}")


def print_warning(message: str):
    """Print a warning message."""
    print(f"\n{YELLOW}⚠{RESET} {message}")


def sanitize_filename(name: str) -> str:
    """Remove special characters from a filename."""
    name = re.sub(r'[\\/*?:"<>|] ', "_", name)
    return name.strip()


def dependency_check():
    """Verify that required tools are installed."""
    missing = [t for t in ("cdparanoia", "flac") if not shutil.which(t)]
    if missing:
        print_error(f"Tools not found: {', '.join(missing)}")
        sys.exit(1)


# ---------------------------------------------------------------------------
# Lettura disco e MusicBrainz
# ---------------------------------------------------------------------------


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
            print_error(f"Errore nella risposta MusicBrainz: {e}")
        return None
    except musicbrainzngs.NetworkError as e:
        print_error(f"Errore di rete: {e}")
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
    anno = release.get("date", "")[:4]

    tracks = []
    for medium in release.get("medium-list", []):
        track_list = medium.get("track-list", [])
        totale = len(track_list)
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

            tracks.append(
                {
                    "num": int(num) if str(num).isdigit() else len(tracks) + 1,
                    "title": title,
                    "artist": track_artist,
                    "album_artist": album_artist,
                    "album": album,
                    "year": anno,
                    "total": totale,
                    "mbid": recording.get("id", ""),
                }
            )

    return tracks


# ---------------------------------------------------------------------------
# Selezione release interattiva
# ---------------------------------------------------------------------------


def select_release(releases: list[dict]) -> dict:
    """If more than one release is found, prompt the user to select the correct one."""
    if len(releases) == 1:
        return releases[0]

    print(f"\n  Multiple ({len(releases)}) releases found. \n Select the one to use:\n")
    for i, r in enumerate(releases, 1):
        nomi = [
            a["artist"]["name"]
            for a in r.get("artist-credit", [])
            if isinstance(a, dict) and "artist" in a
        ]
        artist = ", ".join(nomi) or "?"
        title = r.get("title", "?")
        data = r.get("date", "?")[:4]
        cc = r.get("country", "?")
        print(f"  [{i}] {artist} — {title} ({data}, {cc})")

    while True:
        try:
            selected = int(input(f"\n  Select [1-{len(releases)}]: "))
            if 1 <= selected <= len(releases):
                return releases[selected - 1]
        except (ValueError, KeyboardInterrupt):
            pass
        print_warning(f"Please select a number between 1 and {len(releases)}")


# ---------------------------------------------------------------------------
# Ripping con cdparanoia
# ---------------------------------------------------------------------------


def rip_track(num_traccia: int, wav_path: Path, device: str = "/dev/sr0") -> bool:
    """
    Extracts an audio track using cdparanoia → WAV file.
    Returns True if successful.
    """
    print(f"    📀 cdparanoia track {num_traccia:02d}...", end=" ", flush=True)
    cmd = [
        CDPARANOIA,
        "-e",  # Progresso su stderr, niente stdout
        "-d",
        device,
        str(num_traccia),  # Numero traccia da estrarre
        str(wav_path),  # Destinazione WAV
    ]

    spinner = itertools.cycle(SPINNER_FRAMES[2])

    process = subprocess.Popen(
        cmd, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True, bufsize=1
    )
    for line in process.stderr:
        frame = next(spinner)
        cols = shutil.get_terminal_size().columns - 6  # +6 for spinner and padding
        text = line.rstrip("\n").rstrip("\r")
        # Truncate text to fit terminal
        if len(text) > cols:
            text = text[: cols - 1] + "…"

        output = f"\r {CYAN}{BOLD}{frame}{RESET} {DIM}{GREEN}{text}{RESET}"
        sys.stdout.write(output.ljust(cols))
        sys.stdout.flush()

    process.wait()

    if process.returncode != 0:
        print_error("FALLITA")
        return False
    print_success("✅ OK")
    return True


# ---------------------------------------------------------------------------
# Encoding FLAC
# ---------------------------------------------------------------------------


def encode_flac(wav_path: Path, flac_path: Path, meta: dict) -> bool:
    """
    Codifica WAV → FLAC con tag Vorbis completi.
    Ritorna True se riuscito.
    """
    print(f"\n 🎵 FLAC encoding → {flac_path.name}...", flush=True)

    tags = [
        f"--tag=TITLE={meta.get('title', '')}",
        f"--tag=ARTIST={meta.get('artist', '')}",
        f"--tag=ALBUMARTIST={meta.get('album_artist', '')}",
        f"--tag=ALBUM={meta.get('album', '')}",
        f"--tag=DATE={meta.get('year', '')}",
        f"--tag=TRACKNUMBER={meta.get('num', '')}",
        f"--tag=TRACKTOTAL={meta.get('total', '')}",
    ]
    if meta.get("mbid"):
        tags.append(f"--tag=MUSICBRAINZ_TRACKID={meta['mbid']}")

    cmd = [
        "flac",
        "--best",  # c8
        "--silent",
        *tags,
        "-o",
        str(flac_path),
        str(wav_path),
    ]
    result = subprocess.run(
        cmd, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True
    )
    if result.returncode != 0:
        print_error("FAILED")
        for line in result.stderr.strip().splitlines()[-5:]:
            print(f"       {line}")
        return False
    size_mb = flac_path.stat().st_size / 1024 / 1024
    print_success(f"OK ({size_mb:.1f} MB)")
    return True


# ---------------------------------------------------------------------------
# Pipeline principale di ripping
# ---------------------------------------------------------------------------


def rip_cd(tracce: list[dict], device: str = "/dev/sr0") -> None:
    """For each track:
    1. Extract WAV with cdparanoia in a temporary folder
    2. Encode in FLAC with tags
    3. Delete the WAV
    """
    if not tracce:
        print_warning("No tracks found .")
        return

    # Directory organizzata: ./ripped/Artista/Album/
    meta0 = tracce[0]
    out_dir = (
        OUTPUT_DIR
        / sanitize_filename(meta0["artista_album"])
        / sanitize_filename(meta0["album"])
    )
    out_dir.mkdir(parents=True, exist_ok=True)

    print(f"\n📁 Output in: {out_dir.resolve()}")
    print("=" * 60)
    print(f"🚀 Ripping {len(tracce)} tracks from {device}\n")

    success, errors = 0, 0

    with tempfile.TemporaryDirectory(prefix="cd_rip_") as tmpdir:
        for meta in tracce:
            num = meta["num"]
            nome = f"{num:02d}-{sanitize_filename(meta['title'])}"
            wav_p = Path(tmpdir) / f"{nome}.wav"
            flac_p = out_dir / f"{nome}.flac"

            print(f"\n──> [{num:02d}/{len(tracce):02d}] {meta['title']}")

            # Salta se FLAC già presente
            if flac_p.exists():
                print(
                    f"    ⏭️  {GREEN}File {RESET}{CYAN}{flac_p.name}{RESET} {GREEN}already exists, skip!{RESET}"
                )
                success += 1
                continue

            # Rip → WAV
            if not rip_track(num, wav_p, device):
                errors += 1
                continue

            # WAV → FLAC
            if encode_flac(wav_p, flac_p, meta):
                success += 1
            else:
                errors += 1
            # wav_p eliminato automaticamente all'uscita dal context manager

    # Riepilogo finale
    print("\n" + "=" * 60)
    print("📊  Summary")
    print("=" * 60)
    print_success(f"Completed : {success}/{len(tracce)} tracce")
    if errors:
        print_error(f"Errors     : {errors}/{len(tracce)} tracce")
    print(f"  📁 File FLAC  : {out_dir.resolve()}")
    print("=" * 60)


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------


def main():
    device = sys.argv[1] if len(sys.argv) > 1 else "/dev/sr0"

    dependency_check()

    disc = read_discid(device)
    print_disc_info(disc)

    result = musicbrainz_search(disc)

    tracce = []

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
            tracce = extract_tracks(release)

            names = [
                a["artist"]["name"]
                for a in release.get("artist-credit", [])
                if isinstance(a, dict) and "artist" in a
            ]
            print(f"\n  Artist : {', '.join(names)}")
            print(f"  Album   : {release.get('title', '?')}")
            print(f"  Year    : {release.get('date', '?')[:4]}")
            print(f"  Tracks  : {len(tracce)}")
            for t in tracce:
                print(f"    {t['num']:>2}. {t['title']}")
        else:
            print_warning(" No releases found on MusicBrainz.")

    if not tracce:
        # Fallback: crea tracce numeriche senza metadati
        print_warning("Proceeding without MusicBrainz metadata.")
        for i in range(1, disc.last_track_num + 1):
            tracce.append(
                {
                    "num": i,
                    "title": f"Track {i:02d}",
                    "artist": "Unknown Artist",
                    "album_artist": "Unknown Artist",
                    "album": "Unknown Album",
                    "year": "",
                    "total": disc.last_track_num,
                    "mbid": "",
                }
            )

    # 3. Conferma prima di avviare il ripping
    print()
    try:
        resp = input("▶️  Ready to rip and encode to FLAC? [y/N] ").strip().lower()
    except KeyboardInterrupt:
        print("\nAborted.")
        return

    if resp not in ("y", "s", "si", "yes"):
        print("Operation aborted.")
        return

    rip_cd(tracce, device)


def test_spinner(idx):
    spinner = itertools.cycle(SPINNER_FRAMES[idx])

    line = "# Test"
    while True:
        frame = next(spinner)
        cols = shutil.get_terminal_size().columns - 6  # +6 for spinner and padding
        text = line.rstrip("\n").rstrip("\r")
        # Truncate text to fit terminal
        if len(text) > cols:
            text = text[: cols - 1] + "…"

        output = f"\r {CYAN}{BOLD}{frame}{RESET} {DIM}{GREEN}{text}{RESET}"
        sys.stdout.write(output.ljust(cols))
        sys.stdout.flush()
        time.sleep(0.1)


if __name__ == "__main__":
    # test_spinner(2)
    main()
