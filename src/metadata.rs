use little_exif::metadata::Metadata as LittleMetadata;
use std::{
    io::Error,
    path::{Path, PathBuf},
};

pub struct Metadata {
    path: PathBuf,
    litte_metadata: LittleMetadata,
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

        Ok(Metadata {
            path: PathBuf::from(path),
            litte_metadata,
        })
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
}
