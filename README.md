# Rippy!

I like my phisical media, but is still more conveniet to have a digital copy at hand.
Current available software for Linux is usually "too much" or "broken"/"too old".
This should be easy, all you need is to glue together some simple CLIs, the Unix way. That sometimes is thedious, especially having to fill in metadata manually. 

So I made it simple: `Rippy`!

Rippy dumps (rip) audio CD to Flac, adding track info from Muscibrainz. That's it. Simple and efficient CD Ripping.

This script depends on the following:

- python3
- cdparanoia
- flac

To run just clone the repo and run `./rippy`.
The script will setup the python env and start the ripping process.

## How it works

Simple! 
- First clone `rippy` code 😅
- Insert a CD in your reader (default `/dev/sr0`) and run it with `./run.sh`. 

Rippy will lookup your disk in the Musicbrainz database and, if needed, ask you to pick between similar disks:

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
Tracks will be extracted and converted to Flac, automatically filling in all the evailable details into metatdata.

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

## Missing features

This needs a good refactoring into a lib, better dependency handling.

Even more than then it needs a config file of sort, to set default output dir and default file naming structure.