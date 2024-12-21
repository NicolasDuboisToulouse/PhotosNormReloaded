# PhotosNorm

This document contains the help content for the `photosnorm` command-line program.

**Command Overview:**

* [`photosnorm`↴](#photosnorm)
* [`photosnorm info`↴](#photosnorm-info)
* [`photosnorm set`↴](#photosnorm-set)
* [`photosnorm fix`↴](#photosnorm-fix)

## `photosnorm`

PhotosNorm: A simple tool to lossless manipulate images properties.

info: display some EXIF info.
set:  Update some EXIF tags. More info below or with set --help.
fix:  Fix properties like orientation, file name, ... More info below or with fix --help.

To each command, you can provide one or more files and/or folders.
Each known files (aka images) will be processed, other ones will be ignored.
For each folder, all files within will be analysed like described just before. Sub-folders will be ignored (this is non-recursive).

**Usage:** `photosnorm info <IMAGES/FOLDERS>...
       photosnorm set [OPTIONS] <--description <DESCRIPTION>|--date <DATE>> <IMAGES/FOLDERS>...
       photosnorm fix [OPTIONS] <IMAGES/FOLDERS>...
       photosnorm help [COMMAND]...`

###### **Subcommands:**

* `info` — info: display some EXIF info
* `set` — set: Update tags
* `fix` — fix: Fix file properties



## `photosnorm info`

info: display some EXIF info

**Usage:** `photosnorm info <IMAGES/FOLDERS>...`

###### **Arguments:**

* `<IMAGES/FOLDERS>` — images to load



## `photosnorm set`

set: Update tags

**Usage:** `photosnorm set [OPTIONS] <--description <DESCRIPTION>|--date <DATE>> <IMAGES/FOLDERS>...`

###### **Arguments:**

* `<IMAGES/FOLDERS>` — images to update

###### **Options:**

* `-t`, `--description <DESCRIPTION>` — Update ImageDescription tag (-t: title)
* `-d`, `--date <DATE>` — Update DateTimeOriginal and CreateDate tags
* `-f`, `--force` — Allows to set same tag values to several images



## `photosnorm fix`

fix: Fix file properties

**Usage:** `photosnorm fix [OPTIONS] <IMAGES/FOLDERS>...`

###### **Arguments:**

* `<IMAGES/FOLDERS>` — images to fix

###### **Options:**

* `-a`, `--all` — Apply all fixes (default)

  Default value: `true`
* `-d`, `--dimensions` — Fix ExifImageWidth/Height according to real image width/height
* `-n`, `--name` — Fix file name to %Y_%m_%d-%H_%M_%S[ - %description]. File names may be numbered to prevent erasing file with same name
* `-o`, `--orientation` — Fix image orientation (lossless rotate the image). Only JPEG files are supported



