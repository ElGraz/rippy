# Rippy!

I like my physical media, but it's still more convenient to have a digital copy at hand.
Current available software for Linux is usually "too much" or "broken"/"too old".
This should be easy — all you need is to glue together some simple CLIs, the Unix way. That sometimes is tedious, especially having to fill in metadata manually.

So I made it simple: `Rippy`!

Rippy dumps (rips) audio CDs to FLAC, adding track info from MusicBrainz. That's it. Simple and efficient CD ripping.

This tool depends on the following system libraries:

- libcdio (with paranoia support)
- libdiscid

## Prerequisites

Install the required system dependencies:

```bash
# Debian/Ubuntu
sudo apt-get install libcdio-dev libcdio-paranoia-dev libdiscid-dev

# Fedora
sudo dnf install cdio-devel cdio-paranoia-devel libdiscid-devel

# Arch
sudo pacman -S libcdio paranoia libdiscid
```

## Running

Clone the repo and run:

```bash
cargo run --release
```

Or build once and run directly:

```bash
cargo build --release
./target/release/rippy
```

## How it works

Simple! 
- First clone `rippy` code 😅
- Insert a CD in your reader and run `cargo run`.

Rippy will lookup your disc in the MusicBrainz database and, if needed, ask you to pick between similar discs:

```
🔍 Reading Disc ID from /dev/sr0 ...

============================================================
💿  Disc Information
============================================================
  Disc ID      : xxxxxxxxxxxxxxxxxx-
  FreeDB ID    : xxxxxxxx
  Tracks       : 5
  Duration     : 34:32 min
  MusicBrainz  : https://musicbrainz.org/cdtoc/attach?id=xxxxxxxxxxxxxxxxxxxxxxxxxxxxx
🌐 Searching on MusicBrainz for Disc ID: xxxxxxxxxxxxxxxxxxxxxxxxxxx

============================================================
🎵  MusicBrainz results:
============================================================

  Multiple (2) releases found. 
 Select the one to use:

  [1] Band — CD! (1988, US, 0777777777777)
  [2] Band — CD! (1988, XE, 077778888888)

  Select [1-4]: 3   

  Artist : Band
  Album   : CD
  Year    : 1988
  Tracks  : 8

     1. First
     2. Second
     3. And
     4. So
     5. On

Ready to rip and encode to FLAC? [y/N] 
  ```
  
Now, confirm and chill. Rippy will do the rest! 😀
Tracks will be extracted and converted to FLAC, automatically filling in all the available details into metadata.

```
Extracting [01/05] "First"
 💿 CD track dump completed
 🎵 FLAC encoding → 01-First.flac...
Encoding complete (size: 24.0 MB)

Extracting [02/05] "Second"
 💿 CD track dump completed
 🎵 FLAC encoding → 02-Second.flac...
Encoding complete (size: 39.8 MB)
```
[...]

```
Extracting [05/05] "On"
 💿 CD track dump completed
 🎵 FLAC encoding → 08-Hook_in_Mouth.flac...
Encoding complete (size: 34.4 MB)

============================================================
 Summary
============================================================
Completed : 5/5 tracks
  📁 : ~/src/rippy/Band/CD/
============================================================
```

Done!

## Running tests

```bash
cargo test
```

## Missing features

This needs a good refactoring into a lib, better dependency handling.

Even more than that it needs a config file of sort, to set default output dir and default file naming structure.
