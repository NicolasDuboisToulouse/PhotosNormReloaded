# PhotosNorm

This document contains the help content for the `PhotosNorm` command-line program.

**Command Overview:**

* [`PhotosNorm`↴](#PhotosNorm)
* [`PhotosNorm info`↴](#PhotosNorm-info)
* [`PhotosNorm set`↴](#PhotosNorm-set)

## `PhotosNorm`

Automatic rotate, rename and exif fix

**Usage:** `PhotosNorm info <FILES>...
       PhotosNorm set [OPTIONS] <--description <DESCRIPTION>|--date <DATE>> <FILES>...
       PhotosNorm help [COMMAND]...`

###### **Subcommands:**

* `info` — info: Print some metadata from provided files
* `set` — set: Update tags



## `PhotosNorm info`

info: Print some metadata from provided files

**Usage:** `PhotosNorm info <FILES>...`

###### **Arguments:**

* `<FILES>` — images to load



## `PhotosNorm set`

set: Update tags

**Usage:** `PhotosNorm set [OPTIONS] <--description <DESCRIPTION>|--date <DATE>> <FILES>...`

###### **Arguments:**

* `<FILES>` — images to update

###### **Options:**

* `-t`, `--description <DESCRIPTION>` — Update ImageDescription tag
* `-d`, `--date <DATE>` — Update DateTimeOriginal and CreateDate tags
* `-f`, `--force` — Allows to set same tag values to several images



