use chrono::NaiveDateTime;
use image::image_dimensions;
use little_exif::{
    exif_tag::ExifTag, metadata::Metadata as LittleMetadata, u8conversion::U8conversion,
};
use std::{
    io::Error,
    path::{Path, PathBuf},
};

pub struct Metadata {
    path: PathBuf,
    litte_metadata: LittleMetadata,
    dimentions: (u32, u32),
    date: Option<NaiveDateTime>,
    description: Option<String>,
}

impl Metadata {
    pub fn new(path: &Path) -> Result<Metadata, Error> {
        // Check file type because little_exif will panic on these errors
        let Some(kind) = infer::get_from_path(path)? else {
            return Err(Error::other("Unknown file type."));
        };
        if !kind.mime_type().starts_with("image") {
            return Err(Error::other("Unsuported file type."));
        }

        // Load dimention from image data (not from exif data)
        let Ok(dimentions) = image_dimensions(path) else {
            return Err(Error::other("Cannot read image dimentions."));
        };

        // Load little_exif metadata
        let litte_metadata = LittleMetadata::new_from_path(path)?;
        if litte_metadata.into_iter().count() == 0 {
            return Err(Error::other("No EXIF info in this file."));
        }

        let date =
            Self::get_string_tag(&litte_metadata, &ExifTag::DateTimeOriginal(String::new())).or(
                Self::get_string_tag(&litte_metadata, &ExifTag::CreateDate(String::new())),
            );
        let date = match date {
            None => None,
            Some(str_date) => NaiveDateTime::parse_from_str(&str_date, "%Y:%m:%d %H:%M:%S").ok(),
        };

        let description =
            Self::get_string_tag(&litte_metadata, &ExifTag::ImageDescription(String::new()));

        Ok(Metadata {
            path: PathBuf::from(path),
            litte_metadata,
            dimentions,
            date,
            description,
        })
    }

    // Accessors
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
    pub fn width(&self) -> u32 {
        self.dimentions.0
    }
    pub fn height(&self) -> u32 {
        self.dimentions.1
    }
    pub fn date(&self) -> Option<NaiveDateTime> {
        self.date
    }
    pub fn exif_date(&self) -> Option<String> {
        self.date.map(|d| d.format("%Y:%m:%d %H:%M:%S").to_string())
    }
    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    // Read a tag as a string
    fn get_string_tag(litte_metadata: &LittleMetadata, tag: &ExifTag) -> Option<String> {
        let tag = litte_metadata.get_tag(tag).next()?;
        let endian = litte_metadata.get_endian();
        Some(String::from_u8_vec(&tag.value_as_u8_vec(&endian), &endian))
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

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
    fn file_all_tags() {
        let result = Metadata::new(Path::new("tests/all_tags.jpg"));
        assert!(result.is_ok());
        let metadata = result.unwrap();
        assert_eq!(metadata.path(), Path::new("tests/all_tags.jpg"));
        assert_eq!(metadata.width(), 2048);
        assert_eq!(metadata.height(), 1536);
        assert_eq!(
            metadata.date(),
            NaiveDate::from_ymd_opt(2006, 10, 29)
                .unwrap()
                .and_hms_opt(16, 27, 21)
        );
        assert_eq!(
            metadata.exif_date(),
            Some("2006:10:29 16:27:21".to_string())
        );
        assert_eq!(metadata.description(), Some("A fun picture!".to_string()));
    }

    #[test]
    fn file_missing_tags() {
        let result = Metadata::new(Path::new("tests/no_date.jpg"));
        assert!(result.is_ok());
        let metadata = result.unwrap();
        assert_eq!(metadata.date(), None);

        let result = Metadata::new(Path::new("tests/no_description.jpg"));
        assert!(result.is_ok());
        let metadata = result.unwrap();
        assert_eq!(metadata.description(), None);
    }
}
