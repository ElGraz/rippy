# Rippy! [![Test](https://github.com/ElGraz/rippy/actions/workflows/test.yml/badge.svg)](https://github.com/ElGraz/rippy/actions/workflows/test.yml)

I like my physical media, but sometimes it is convenient to have a digital copy at hand.

Current available software for Linux can be "too much" or "too old".

This should be easy — all you need is to glue together what's already available, the Unix way. 

So I made it simple: `Rippy`!

## Features

Rips am audio CDs to FLAC with automatic MusicBrainz metadata tagging.

Does just what's needed, simple and efficient.

## How it Works

Have an audio CD ready? Simply insert it into your drive and run `rippy`!

Rippy will read the CD, compute the MusicBrainz Disc ID, and query the MusicBrainz database for matching album information. Once confirmed, it will rip the tracks to FLAC and tag them with the album metadata. Done!

Here's an example:

```
◉ Opening CD drive…
✓ Drive ready — 14  audio tracks detected
◉ Computing MusicBrainz Disc ID…
  ID: xxxxxxxxxxxxxxxx.dhWa.0-
◉ Querying MusicBrainz…
✓ Found: Example album

  ┌──────────────────────────────────────────────────────┐
  │  ID   xxxxxxxxxxxxxxxx.dhWa.0-
  │  Album  Example album
  │  Artist Band
  │  Tracks 6
  ├──────────────────────────────────────────────────────┤
  │   1. One
  │   2. Two
  │   3. A third song
  │   4. Followed by another
  │   5. Almost done
  │   6. Finishing up
  └──────────────────────────────────────────────────────┘

  Proceed with ripping? [y/N] 
```

Now, confirm and chill. Rippy will do the rest! 😀 
Tracks will be extracted and converted to FLAC, automatically filling in all the available details into metadata.

## System Dependencies

This tool depends on the following system libraries:

- `libcdio` (with paranoia support)
- `libdiscid`

## Building

Install the required system dependencies:

```bash
# Debian/Ubuntu
sudo apt-get install libcdio-dev libcdio-paranoia-dev libdiscid-dev libiso9660-dev libudf-dev
```

### Running

Clone the repo and run:

```bash
cargo run --release
```

Or build once and run directly:

```bash
cargo build --release
./target/release/rippy
```

## Future Enhancements

- Needs a config file to set default output directory and file naming structure.
