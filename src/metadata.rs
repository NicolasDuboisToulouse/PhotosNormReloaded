use image::image_dimensions;
use little_exif::metadata::Metadata as LittleMetadata;
use std::{
    io::Error,
    path::{Path, PathBuf},
};

pub struct Metadata {
    path: PathBuf,
    litte_metadata: LittleMetadata,
    dimentions: (u32, u32),
}

impl Metadata {
    pub fn new(path: &Path) -> Result<Metadata, Error> {
        let Some(kind) = infer::get_from_path(path)? else {
            return Err(Error::other("Unknown file type."));
        };
        if !kind.mime_type().starts_with("image") {
            return Err(Error::other("Unsuported file type."));
        }

        let litte_metadata = LittleMetadata::new_from_path(path)?;
        if litte_metadata.into_iter().count() == 0 {
            return Err(Error::other("No EXIF info in this file."));
        }

        let Ok(dimentions) = image_dimensions(path) else {
            return Err(Error::other("Cannot read image dimentions."));
        };

        Ok(Metadata {
            path: PathBuf::from(path),
            litte_metadata,
            dimentions,
        })
    }

    pub fn get_path(&self) -> &Path {
        self.path.as_path()
    }
    pub fn width(&self) -> u32 {
        self.dimentions.0
    }
    pub fn height(&self) -> u32 {
        self.dimentions.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_files() {
        let result = Metadata::new(Path::new("/do/not/exists"));
        assert!(result.is_err());
        let result = Metadata::new(Path::new("tests/empty"));
        assert!(result.is_err());
        let result = Metadata::new(Path::new("tests/archive.tar.gz"));
        assert!(result.is_err());
        let result = Metadata::new(Path::new("tests/no_exif.png"));
        assert!(result.is_err());
    }

    #[test]
    fn simple_file() {
        let result = Metadata::new(Path::new("tests/valid.jpg"));
        assert!(result.is_ok());
        let metadata = result.unwrap();
        assert_eq!(metadata.get_path(), Path::new("tests/valid.jpg"));
        assert_eq!(metadata.width(), 2048);
        assert_eq!(metadata.height(), 1536);
    }
}
