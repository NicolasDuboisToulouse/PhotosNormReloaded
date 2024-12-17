# PhotosNorm

This document contains the help content for the `PhotosNorm` command-line program.

**Command Overview:**

* [`PhotosNorm`↴](#PhotosNorm)
* [`PhotosNorm info`↴](#PhotosNorm-info)
* [`PhotosNorm set`↴](#PhotosNorm-set)
* [`PhotosNorm fix`↴](#PhotosNorm-fix)

## `PhotosNorm`

PhotosNorm: A simple tool to lossless manipulate images properties.

info: display some EXIF info.
set:  Update some EXIF tags. More info below or with set --help.
fix:  Fix properties like orientation, file name, ... More info below or with fix --help.

To each command, you can provide one or more files and/or folders.
Each known files (aka images) will be processed, other ones will be ignored.
For each folder, all files within will be analysed like described just before. Sub-folders will be ignored (this is non-recursive).

**Usage:** `PhotosNorm info <IMAGES/FOLDERS>...
       PhotosNorm set [OPTIONS] <--description <DESCRIPTION>|--date <DATE>> <IMAGES/FOLDERS>...
       PhotosNorm fix [OPTIONS] <IMAGES/FOLDERS>...
       PhotosNorm help [COMMAND]...`

###### **Subcommands:**

* `info` — info: display some EXIF info
* `set` — set: Update tags
* `fix` — fix: Fix file properties



## `PhotosNorm info`

info: display some EXIF info

**Usage:** `PhotosNorm info <IMAGES/FOLDERS>...`

###### **Arguments:**

* `<IMAGES/FOLDERS>` — images to load



## `PhotosNorm set`

set: Update tags

**Usage:** `PhotosNorm set [OPTIONS] <--description <DESCRIPTION>|--date <DATE>> <IMAGES/FOLDERS>...`

###### **Arguments:**

* `<IMAGES/FOLDERS>` — images to update

###### **Options:**

* `-t`, `--description <DESCRIPTION>` — Update ImageDescription tag (-t: title)
* `-d`, `--date <DATE>` — Update DateTimeOriginal and CreateDate tags
* `-f`, `--force` — Allows to set same tag values to several images



## `PhotosNorm fix`

fix: Fix file properties

**Usage:** `PhotosNorm fix [OPTIONS] <IMAGES/FOLDERS>...`

###### **Arguments:**

* `<IMAGES/FOLDERS>` — images to fix

###### **Options:**

* `-a`, `--all` — Apply all fixes (default)

  Default value: `true`
* `-d`, `--dimensions` — Fix ExifImageWidth/Height according to real image width/height
* `-n`, `--name` — Fix file name to %Y_%m_%d-%H_%M_%S[ - %description]. File names may be numbered to prevent erasing file with same name
* `-o`, `--orientation` — Fix image orientation (lossless rotate the image). Only JPEG files are supported



