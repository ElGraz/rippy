import subprocess
import tempfile
from pathlib import Path
from .utils import print_error, print_success, print_warning, print_progress, sanitize_filename, GREEN, CYAN, RESET

class Ripper:
    def __init__(self, device: str = "/dev/sr0", cdparanoia_path: str = "cdparanoia", flac_path: str = "flac", output_dir: Path = Path(".")):
        self.device = device
        self.cdparanoia_path = cdparanoia_path
        self.flac_path = flac_path
        self.output_dir = output_dir

    def rip_track(self, track_num: int, wav_path: Path) -> bool:
        """Extracts an audio track using cdparanoia → WAV file."""
        print(f"    📀 cdparanoia track {track_num:02d}...", end=" ", flush=True)
        cmd = [
            self.cdparanoia_path,
            "-e",
            "-d",
            self.device,
            str(track_num),
            str(wav_path),
        ]

        process = subprocess.Popen(
            cmd, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True, bufsize=1
        )
        print_progress(process.stderr)
        process.wait()

        if process.returncode != 0:
            print_error("CD track dump failed")
            return False
        print_success("\r 💿 CD track dump completed")
        return True

    def encode_flac(self, wav_path: Path, flac_path: Path, meta: dict) -> bool:
        """Encodes WAV to FLAC with tags."""
        tag_flags = []
        for tag, value in meta.items():
            if value:
                tag_flags.append(f"--tag={tag}={value}")

        print(f" 🎵 FLAC encoding → {flac_path.name}...", flush=True)
        cmd = [
            self.flac_path,
            "--best",
            "--silent",
            *tag_flags,
            "-o",
            str(flac_path),
            str(wav_path),
        ]
        result = subprocess.Popen(
            cmd, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True, bufsize=1
        )
        print_progress(result.stderr)
        result.wait()

        if result.returncode != 0:
            print_error("FAILED")
            return False
        size_mb = flac_path.stat().st_size / 1024 / 1024
        print_success(f"Encoding complete (size: {size_mb:.1f} MB)")
        return True

    def rip_cd(self, tracks: list[dict]) -> None:
        if not tracks:
            print_warning("No tracks found.")
            return

        meta0 = tracks[0]
        artist_dir = sanitize_filename(meta0.get("ALBUM_ARTIST", meta0.get("ARTIST", "Unknown Artist")))
        album_dir = sanitize_filename(meta0.get("ALBUM", "Unknown Album"))
        
        out_dir = self.output_dir / artist_dir / album_dir
        out_dir.mkdir(parents=True, exist_ok=True)

        print(f"\n📁 Output in: {out_dir.resolve()}")
        print("=" * 60)
        print(f"🚀 Rip and encode of {len(tracks)} tracks from {self.device} starting...\n")

        success, errors = 0, 0

        with tempfile.TemporaryDirectory(prefix="cd_rip_") as tmpdir:
            for meta in tracks:
                num = meta["TRACKNUMBER"]
                title = sanitize_filename(meta['TITLE'])
                nome = f"{num:02d}-{title}"
                wav_p = Path(tmpdir) / f"{nome}.wav"
                flac_p = out_dir / f"{nome}.flac"

                print(f'\nExtracting [{num:02d}/{len(tracks):02d}] "{meta["TITLE"]}"')

                if flac_p.exists():
                    print(f"    ⏭️  {GREEN}File {RESET}{CYAN}{flac_p.name}{RESET} {GREEN}already exists, skip!{RESET}")
                    success += 1
                    continue

                if not self.rip_track(num, wav_p):
                    errors += 1
                    continue

                if self.encode_flac(wav_p, flac_p, meta):
                    success += 1
                else:
                    errors += 1

        print("\n" + "=" * 60)
        print("📊  Summary")
        print("=" * 60)
        print_success(f"Completed : {success}/{len(tracks)} tracks")
        if errors:
            print_error(f"Errors     : {errors}/{len(tracks)} tracks")
        print(f"  📁 File FLAC  : {out_dir.resolve()}")
        print("=" * 60)
