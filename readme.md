# Animan
Anime manager, renamer

When working wity anime dumps, there is a need to make it to follow a consistent file structure/name in order for streaming managers to manage it. (Plex)

## The problem
The anime dumps are usually in a format with bunch of files, specials, subtitles. We want to make sure the specials are stored in the proper place and names are changed properly. For example:

### Type 1 (Simple flat)
```
 | [MMWEB][Nobunaga-sensei no Osanazuma][01-12][BIG5][720P]
 |  - [MMWEB][Nobunaga-sensei no Osanazuma][01][BIG5][720P].mp4
 |  - [MMWEB][Nobunaga-sensei no Osanazuma][02][BIG5][720P].mp4
 |  - [MMWEB][Nobunaga-sensei no Osanazuma][03][BIG5][720P].mp4
 |  - [MMWEB][Nobunaga-sensei no Osanazuma][04][BIG5][720P].mp4
 |  - [MMWEB][Nobunaga-sensei no Osanazuma][05][BIG5][720P].mp4
 ...
```
This is a flat structure where only the files in the directory needs to be renamed

### Type 2 (Simple nested)
```
 | [Airota&VCB-Studio] Kiniro Mosaic
 |  - [Airota&VCB-Studio] Hello!! Kiniro Mosaic [Ma10p_1080p]
 |      - [Airota&VCB-Studio] Hello!! Kiniro Mosaic [01][Ma10p_1080p][x265_2flac].mkv
 |      - [Airota&VCB-Studio] Hello!! Kiniro Mosaic [02][Ma10p_1080p][x265_2flac].mkv
 |  - [Airota&VCB-Studio] Kiniro Mosaic [Ma10p_1080p]
 |  - [Airota&VCB-Studio] Kiniro Mosaic Pretty Days [Ma10p_1080p]
```
This structure includes multiple seasons and each file in a season is inside the folder

## Output
The files inside the folder will be correctly named and labeled so that
PMS (Plex Media Server) can detect them correctly

### Type 1 (Solution)
```
 | ノブナガ先生の幼な妻
 |   - Season 1
 |      - ノブナガ先生の幼な妻 - s1e1.mp4
 |      - ノブナガ先生の幼な妻 - s1e2.mp4
 |      - ノブナガ先生の幼な妻 - s1e3.mp4
```

### Type 2 (Solution)
```
 | きんいろモザイク
 |  - Season 1
 |      - きんいろモザイク - s1e1.mkv
 |      - きんいろモザイク - s1e2.mkv
 |      - きんいろモザイク - s1e3.mkv
 |  - Season 2
 |      - きんいろモザイク - s2e1.mkv
 |      - きんいろモザイク - s2e2.mkv
 |      - きんいろモザイク - s2e3.mkv
```
