import itertools
import re
import shutil
import sys
from typing import IO, AnyStr

# Color codes
RED = "\033[91m"
GREEN = "\033[92m"
YELLOW = "\033[93m"
CYAN = "\033[96m"
RESET = "\033[0m"
BOLD = "\033[1m"
DIM = "\033[2m"

SPINNER_FRAMES = ["◐◓◑◒", "⣾⣽⣻⢿⡿⣟⣯⣷", "▏▎▍▌▋▊▉█▉▊▌▍▎▏", ".oO0Oo.", "·∘○◉○∘·"]

def print_error(message: str):
    """Print an error message."""
    print(f"\n{RED}❌{RESET} {message}")

def print_success(message: str):
    """Print a success message."""
    print(f"{GREEN}{message}{RESET}")

def print_warning(message: str):
    """Print a warning message."""
    print(f"\n{YELLOW}⚠{RESET} {message}")

def sanitize_filename(name: str) -> str:
    """
    Remove special characters and collapse surrounding
    whitespace into a single underscore.
    """
    name = name.strip()
    # Replace any forbidden character (and surrounding whitespace) with an underscore
    name = re.sub(r'\s*[\\/*?:"<>|]\s*', "_", name)
    # Replace spaces with underscores
    name = name.replace(" ", "_")
    # Collapse multiple underscores
    name = re.sub(r'_+', "_", name)
    return name.strip("_")

def print_progress(stream: IO[AnyStr] | None, frame_set: int = 2):
    if stream is None:
        return

    spinner = itertools.cycle(SPINNER_FRAMES[frame_set])

    for line in stream:
        frame = next(spinner)
        cols = shutil.get_terminal_size().columns - 6
        text = line.rstrip("\n").rstrip("\r")
        if len(text) > cols:
            text = text[: cols - 1] + "…"

        output = f"\r {CYAN}{BOLD}{frame}{RESET} {DIM}{GREEN}{text}{RESET}"
        sys.stdout.write(output.ljust(cols))
        sys.stdout.flush()
