#!/usr/bin/env python3
"""
CD MusicBrainz Lookup + Rip + FLAC Encoding
"""

import shutil
import sys

from lib import rippy


def dependency_check() -> tuple[str, str]:
    # Check for dependencies: cdparanoia and flac.
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
    device = sys.argv[1] if len(sys.argv) > 1 else "/dev/sr0"

    cdparanoia_path, flac_path = dependency_check()
    rippy.rippy(device, cdparanoia_path, flac_path)


if __name__ == "__main__":
    # rippy.test_spinner(2)
    main()
