# rippy

Rippy rips a cd to Flac, adding track info from Muscibrainz. That's it. Simple and efficient CD Ripping.

This script depends on the following:

- python3
- cdparanoia
- flac

To run just clone the repo and run `./rippy`.
The script will setup the python env and start the ripping process.

## Missing features

This needs a good refactoring into a lib, better dependency handling.

Even more than then it needs a config file of sort, to set default output dir and default file naming structure.