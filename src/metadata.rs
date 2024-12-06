use camera_info::CameraInfo;
use chrono::NaiveDateTime;
use enumset::EnumSet;
use image::image_dimensions;
use little_exif::rational::iR64;
use little_exif::{
    exif_tag::ExifTag, metadata::Metadata as LittleMetadata, rational::uR64,
    u8conversion::U8conversion,
};
use std::ffi::OsStr;
use std::fs::rename;
use std::{
    io::Error,
    path::{Path, PathBuf},
};
use tag::Tag;

pub mod camera_info;
pub mod tag;

trait ExifConversion {
    fn to_exif_string(&self) -> String;
    fn from_exif_string(input: String) -> Result<Self, Error>
    where
        Self: Sized;
}
impl ExifConversion for NaiveDateTime {
    fn to_exif_string(&self) -> String {
        self.format("%Y:%m:%d %H:%M:%S").to_string()
    }
    fn from_exif_string(input: String) -> Result<Self, Error> {
        match NaiveDateTime::parse_from_str(&input, "%Y:%m:%d %H:%M:%S") {
            Ok(dt) => Ok(dt),
            Err(error) => Err(Error::other(error.to_string())),
        }
    }
}

pub struct Metadata {
    path: PathBuf,
    litte_metadata: LittleMetadata,
    dimentions: (u32, u32),
    date: Option<NaiveDateTime>,
    description: Option<String>,
    camera_info: CameraInfo,
    modified_tags: EnumSet<Tag>,
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

        // Load and parse date
        let date =
            Self::get_tag_string(&litte_metadata, &ExifTag::DateTimeOriginal(String::new())).or(
                Self::get_tag_string(&litte_metadata, &ExifTag::CreateDate(String::new())),
            );
        let date = match date {
            None => None,
            Some(str_date) => NaiveDateTime::from_exif_string(str_date).ok(),
        };

        // Load description
        let description =
            Self::get_tag_string(&litte_metadata, &ExifTag::ImageDescription(String::new()));

        // Load and format CameraInfo
        let make = Self::get_tag_string(&litte_metadata, &ExifTag::Make(String::new()));
        let model = Self::get_tag_string(&litte_metadata, &ExifTag::Model(String::new()));
        let software = Self::get_tag_string(&litte_metadata, &ExifTag::Software(String::new()));
        let camera = if make.is_some() && model.is_some() {
            let mut camera = make.unwrap().clone();
            camera.push(' ');
            camera.push_str(&model.unwrap());
            Some(camera)
        } else if make.is_some() {
            make
        } else if model.is_some() {
            model
        } else {
            None
        };
        let camera = if camera.is_some() && software.is_some() {
            let mut camera = camera.unwrap().clone();
            camera.push_str(" (");
            camera.push_str(&software.unwrap());
            camera.push(')');
            Some(camera)
        } else {
            camera
        };

        let exposure = Self::get_tag_ur64(&litte_metadata, &ExifTag::ExposureTime(Vec::new()))
            .map(|v| format!("{}/{}", v.nominator, v.denominator))
            .or(
                Self::get_tag_ir64(&litte_metadata, &ExifTag::ShutterSpeedValue(Vec::new())).map(
                    |rational| {
                        let value: f64 = rational.into();
                        // Convert APEX format to seconds
                        let value = 2f64.powf(-value);
                        // Convert second to rational if possible
                        if value < 0.25001 && value > 0f64 {
                            format!("1/{}", (0.5f64 + 1f64 / value).trunc())
                        } else {
                            value.to_string()
                        }
                    },
                ),
            );

        let exposure_bias =
            Self::get_tag_ir64(&litte_metadata, &ExifTag::ExposureCompensation(Vec::new())).map(
                |v| {
                    if v.nominator == 0 {
                        "0".to_string()
                    } else {
                        format!("{}/{}", v.nominator, v.denominator)
                    }
                },
            );

        let aperture = Self::get_tag_ur64(&litte_metadata, &ExifTag::FNumber(Vec::new()))
            .map(std::convert::Into::<f64>::into)
            .or(
                Self::get_tag_ur64(&litte_metadata, &ExifTag::ApertureValue(Vec::new())).map(
                    |rational| {
                        let value: f64 = rational.into();
                        // Convert APEX format to f-number
                        2f64.powf(value / 2f64)
                    },
                ),
            )
            .map(|value| format!("{:.1}", value));

        let iso = Self::get_tag_u16(&litte_metadata, &ExifTag::ISO(Vec::new()));

        let focal = Self::get_tag_ur64(&litte_metadata, &ExifTag::FocalLength(Vec::new()))
            .map(std::convert::Into::<f64>::into);

        let flash = Self::get_tag_u16(&litte_metadata, &ExifTag::Flash(Vec::new()))
            .map(Self::flash_code_to_string);

        let camera_info = CameraInfo {
            camera,
            exposure,
            exposure_bias,
            aperture,
            iso,
            focal,
            flash,
        };

        Ok(Metadata {
            path: PathBuf::from(path),
            litte_metadata,
            dimentions,
            date,
            description,
            camera_info,
            modified_tags: EnumSet::empty(),
        })
    }

    // Accessors
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
        self.date().map(|d| d.to_exif_string())
    }
    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }
    pub fn camera_info(&self) -> &CameraInfo {
        &self.camera_info
    }

    /// Set description.
    /// Note: file will not modified unless you call save().
    pub fn set_description(&mut self, description: &str) {
        if !self.description.eq(&Some(description.to_string())) {
            self.description = Some(description.to_string());
            self.modified_tags.insert(Tag::Description);
            self.litte_metadata
                .set_tag(ExifTag::ImageDescription(description.to_string()));
        }
    }

    /// Set date.
    /// Note: file will not modified unless you call save().
    pub fn set_date(&mut self, date: NaiveDateTime) {
        if !self.date.eq(&Some(date)) {
            self.date = Some(date);
            self.modified_tags.insert(Tag::Date);
            self.litte_metadata
                .set_tag(ExifTag::DateTimeOriginal(date.to_exif_string()));
            self.litte_metadata
                .set_tag(ExifTag::CreateDate(date.to_exif_string()));
        }
    }

    /// Set date from an exif date string.
    /// Note: file will not modified unless you call save().
    /// Will return an error if str_date cannot be parsed
    pub fn set_date_from_exif(&mut self, str_date: String) -> Result<(), Error> {
        let date = NaiveDateTime::from_exif_string(str_date)?;
        self.set_date(date);
        Ok(())
    }

    /// Check if ExifImageWidth/Height have the good values or fix them.
    /// Note: file will not modified unless you call save().
    /// Return true if dimensions has been fixed
    pub fn fix_dimentions(&mut self) -> bool {
        let exif_width =
            Self::get_tag_u32(&self.litte_metadata, &ExifTag::ExifImageWidth(Vec::new()));
        let exif_height =
            Self::get_tag_u32(&self.litte_metadata, &ExifTag::ExifImageHeight(Vec::new()));

        if !exif_width.eq(&Some(self.width())) || !exif_height.eq(&Some(self.height())) {
            self.modified_tags.insert(Tag::Dimensions);
            self.litte_metadata
                .set_tag(ExifTag::ExifImageWidth(vec![self.width()]));
            self.litte_metadata
                .set_tag(ExifTag::ExifImageHeight(vec![self.height()]));

            true
        } else {
            false
        }
    }

    /// Mark file to be renamed to %Y_%m_%d-%H_%M_%S[ - %description]
    /// Note: file will not modified unless you call save().
    pub fn fix_file_name(&mut self) {
        // The file name will be computed on save
        // to take in account potential other set_xxx calls.
        self.modified_tags.insert(Tag::FileName);
    }

    /// Save modified tags
    /// Return the list of modified tags
    pub fn save(&mut self) -> Result<EnumSet<Tag>, Error> {
        if !self.modified_tags.is_empty() {
            // Update filename if needed
            if self.modified_tags.contains(Tag::FileName) {
                match self.date {
                    None => {
                        self.modified_tags.remove(Tag::FileName);
                    }
                    Some(date) => {
                        let mut target_file_name = date.format("%Y_%m_%d-%H_%M_%S").to_string();

                        if self.description.is_some() {
                            target_file_name.push_str(" - ");
                            target_file_name.push_str(self.description.as_ref().unwrap());
                        }

                        if let Some(os_ext) = self.path.extension() {
                            target_file_name.push('.');
                            target_file_name.push_str(&os_ext.to_string_lossy());
                        }

                        target_file_name = sanitise_file_name::sanitise(&target_file_name);

                        if Some(OsStr::new(&target_file_name)) != self.path.file_name() {
                            let new_path = self.path.with_file_name(target_file_name);
                            rename(&self.path, &new_path)?;
                            self.path = new_path;
                        } else {
                            self.modified_tags.remove(Tag::FileName);
                        }
                    }
                }
            }

            // Save tags
            self.litte_metadata.write_to_file(&self.path)?;
            let modified_tags = self.modified_tags;
            self.modified_tags = EnumSet::empty();
            Ok(modified_tags)
        } else {
            Ok(EnumSet::empty())
        }
    }

    // Read a string tag
    fn get_tag_string(litte_metadata: &LittleMetadata, tag: &ExifTag) -> Option<String> {
        let tag = litte_metadata.get_tag(tag).next()?;
        let endian = litte_metadata.get_endian();
        Some(String::from_u8_vec(&tag.value_as_u8_vec(&endian), &endian))
    }

    // Read an u16 tag
    fn get_tag_u16(litte_metadata: &LittleMetadata, tag: &ExifTag) -> Option<u16> {
        let tag = litte_metadata.get_tag(tag).next()?;
        let endian = litte_metadata.get_endian();
        Some(u16::from_u8_vec(&tag.value_as_u8_vec(&endian), &endian))
    }

    // Read an u32 tag
    fn get_tag_u32(litte_metadata: &LittleMetadata, tag: &ExifTag) -> Option<u32> {
        let tag = litte_metadata.get_tag(tag).next()?;
        let endian = litte_metadata.get_endian();
        Some(u32::from_u8_vec(&tag.value_as_u8_vec(&endian), &endian))
    }

    // Read an uR64 tag
    fn get_tag_ur64(litte_metadata: &LittleMetadata, tag: &ExifTag) -> Option<uR64> {
        let endian = litte_metadata.get_endian();
        litte_metadata
            .get_tag(tag)
            .next()
            .map(|tag| uR64::from_u8_vec(&tag.value_as_u8_vec(&endian), &endian))
    }

    //  Read an iR64 tag
    fn get_tag_ir64(litte_metadata: &LittleMetadata, tag: &ExifTag) -> Option<iR64> {
        let endian = litte_metadata.get_endian();
        litte_metadata
            .get_tag(tag)
            .next()
            .map(|tag| iR64::from_u8_vec(&tag.value_as_u8_vec(&endian), &endian))
    }

    fn flash_code_to_string(flash_code: u16) -> String {
        match flash_code {
            0x00 => "No Flash",
            0x01 => "Fired",
            0x05 => "Fired, Return not detected",
            0x07 => "Fired, Return detected",
            0x08 => "On, Did not fire",
            0x09 => "On, Fired",
            0x0d => "On, Return not detected",
            0x0f => "On, Return detected",
            0x10 => "Off, Did not fire",
            0x14 => "Off, Did not fire, Return not detected",
            0x18 => "Auto, Did not fire",
            0x19 => "Auto, Fired",
            0x1d => "Auto, Fired, Return not detected",
            0x1f => "Auto, Fired, Return detected",
            0x20 => "No flash function",
            0x30 => "Off, No flash function",
            0x41 => "Fired, Red-eye reduction",
            0x45 => "Fired, Red-eye reduction, Return not detected",
            0x47 => "Fired, Red-eye reduction, Return detected",
            0x49 => "On, Red-eye reduction",
            0x4d => "On, Red-eye reduction, Return not detected",
            0x4f => "On, Red-eye reduction, Return detected",
            0x50 => "Off, Red-eye reduction",
            0x58 => "Auto, Did not fire, Red-eye reduction",
            0x59 => "Auto, Fired, Red-eye reduction",
            0x5d => "Auto, Fired, Red-eye reduction, Return not detected",
            0x5f => "Auto, Fired, Red-eye reduction, Return detected",
            _ => "Unknown flash code",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use chrono::NaiveDate;
    use enumset::enum_set;

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
        assert_eq!(metadata.path, Path::new("tests/all_tags.jpg"));
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
        assert_eq!(
            metadata.camera_info().camera,
            Some("Pablo Picasso (1.4)".to_string())
        );
        assert_eq!(metadata.camera_info().exposure, Some("1/32".to_string()));
        assert_eq!(metadata.camera_info().exposure_bias, Some("0".to_string()));
        assert_eq!(metadata.camera_info().aperture, Some("5.6".to_string()));
        assert_eq!(metadata.camera_info().iso, Some(100));
        assert_eq!(metadata.camera_info().focal, Some(7.9));
        assert_eq!(
            metadata.camera_info().flash,
            Some(Metadata::flash_code_to_string(0x18))
        );
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

        let result = Metadata::new(Path::new("tests/no_camera.jpg"));
        assert!(result.is_ok());
        let metadata = result.unwrap();
        assert_eq!(metadata.camera_info().camera, None);

        let result = Metadata::new(Path::new("tests/no_exposure.jpg"));
        assert!(result.is_ok());
        let metadata = result.unwrap();
        assert_eq!(metadata.camera_info().exposure, None);

        let result = Metadata::new(Path::new("tests/no_bias.jpg"));
        assert!(result.is_ok());
        let metadata = result.unwrap();
        assert_eq!(metadata.camera_info().exposure_bias, None);

        let result = Metadata::new(Path::new("tests/no_iso.jpg"));
        assert!(result.is_ok());
        let metadata = result.unwrap();
        assert_eq!(metadata.camera_info().iso, None);

        let result = Metadata::new(Path::new("tests/no_focal.jpg"));
        assert!(result.is_ok());
        let metadata = result.unwrap();
        assert_eq!(metadata.camera_info().focal, None);

        let result = Metadata::new(Path::new("tests/no_flash.jpg"));
        assert!(result.is_ok());
        let metadata = result.unwrap();
        assert_eq!(metadata.camera_info().flash, None);
    }

    #[test]
    fn update_tags() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmp_file_path = tmpdir.path().join("photo_norm_test.jpg");
        assert!(fs::copy(Path::new("tests/all_tags.jpg"), &tmp_file_path,).is_ok());

        // Description tag check
        let result = Metadata::new(&tmp_file_path);
        assert!(result.is_ok());
        let mut metadata = result.unwrap();

        metadata.set_description("Description 1");
        assert_eq!(metadata.description(), Some("Description 1".to_string()));

        assert_eq!(metadata.save().ok(), Some(enum_set!(Tag::Description)));

        let result = Metadata::new(&tmp_file_path);
        assert!(result.is_ok());
        let mut metadata = result.unwrap();
        assert_eq!(metadata.description(), Some("Description 1".to_string()));
        assert_eq!(
            metadata.date(),
            NaiveDate::from_ymd_opt(2006, 10, 29)
                .unwrap()
                .and_hms_opt(16, 27, 21)
        );

        // invalid Date tag check
        assert!(metadata
            .set_date_from_exif("2001:01:01".to_string())
            .is_err());
        assert_eq!(
            metadata.date(),
            NaiveDate::from_ymd_opt(2006, 10, 29)
                .unwrap()
                .and_hms_opt(16, 27, 21)
        );

        assert_eq!(metadata.save().ok(), Some(enum_set!()));

        let result = Metadata::new(&tmp_file_path);
        assert!(result.is_ok());
        let mut metadata = result.unwrap();
        assert_eq!(metadata.description(), Some("Description 1".to_string()));
        assert_eq!(
            metadata.date(),
            NaiveDate::from_ymd_opt(2006, 10, 29)
                .unwrap()
                .and_hms_opt(16, 27, 21)
        );

        // Date tag check
        assert!(metadata
            .set_date_from_exif("2001:01:01 01:01:01".to_string())
            .is_ok());
        assert_eq!(
            metadata.date(),
            NaiveDate::from_ymd_opt(2001, 1, 1)
                .unwrap()
                .and_hms_opt(1, 1, 1)
        );

        assert_eq!(metadata.save().ok(), Some(enum_set!(Tag::Date)));

        let result = Metadata::new(&tmp_file_path);
        assert!(result.is_ok());
        let mut metadata = result.unwrap();
        assert_eq!(metadata.description(), Some("Description 1".to_string()));
        assert_eq!(
            metadata.date(),
            NaiveDate::from_ymd_opt(2001, 1, 1)
                .unwrap()
                .and_hms_opt(1, 1, 1)
        );

        // All tags check
        assert!(metadata
            .set_date_from_exif("2002:02:02 02:02:02".to_string())
            .is_ok());
        metadata.set_description("Description 2");
        assert_eq!(metadata.description(), Some("Description 2".to_string()));
        assert_eq!(
            metadata.date(),
            NaiveDate::from_ymd_opt(2002, 2, 2)
                .unwrap()
                .and_hms_opt(2, 2, 2)
        );

        assert_eq!(
            metadata.save().ok(),
            Some(enum_set!(Tag::Date | Tag::Description))
        );

        let result = Metadata::new(&tmp_file_path);
        assert!(result.is_ok());
        let metadata = result.unwrap();
        assert_eq!(metadata.description(), Some("Description 2".to_string()));
        assert_eq!(
            metadata.date(),
            NaiveDate::from_ymd_opt(2002, 2, 2)
                .unwrap()
                .and_hms_opt(2, 2, 2)
        );
    }

    #[test]
    fn fix_dimensions() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmp_file_path = tmpdir.path().join("photo_norm_test.jpg");

        // Check a valid file
        assert!(fs::copy(Path::new("tests/all_tags.jpg"), &tmp_file_path,).is_ok());
        let result = Metadata::new(&tmp_file_path);
        assert!(result.is_ok());
        let mut metadata = result.unwrap();
        assert!(!metadata.fix_dimentions());
        assert_eq!(metadata.save().ok(), Some(enum_set!()));

        // Check an invalid file
        assert!(fs::copy(Path::new("tests/invalid_dim.jpg"), &tmp_file_path).is_ok());
        let result = Metadata::new(&tmp_file_path);
        assert!(result.is_ok());
        let mut metadata = result.unwrap();
        assert!(metadata.fix_dimentions());
        assert_eq!(metadata.save().ok(), Some(enum_set!(Tag::Dimensions)));

        // Reload file and check dimensions
        let litte_metadata = LittleMetadata::new_from_path(&tmp_file_path);
        assert!(litte_metadata.is_ok());
        let width = Metadata::get_tag_u32(
            litte_metadata.as_ref().unwrap(),
            &ExifTag::ExifImageWidth(Vec::new()),
        );
        assert_eq!(width, Some(2048));
        let height = Metadata::get_tag_u32(
            litte_metadata.as_ref().unwrap(),
            &ExifTag::ExifImageHeight(Vec::new()),
        );
        assert_eq!(height, Some(1536));
    }

    #[test]
    fn fix_file_name() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmp_file_path = tmpdir.path().join("photo_norm_test.jpg");
        let target_file_path = tmpdir
            .path()
            .join("2006_10_29-16_27_21 - A fun picture!.jpg");
        assert!(fs::copy(Path::new("tests/all_tags.jpg"), &tmp_file_path,).is_ok());
        assert!(tmp_file_path.exists());
        assert!(!target_file_path.exists());

        // Check file rename
        let result = Metadata::new(&tmp_file_path);
        assert!(result.is_ok());
        let mut metadata = result.unwrap();
        metadata.fix_file_name();
        assert_eq!(metadata.save().ok(), Some(enum_set!(Tag::FileName)));
        assert!(!tmp_file_path.exists());
        assert!(target_file_path.exists());

        // No change on valid filename
        let result = Metadata::new(&target_file_path);
        assert!(result.is_ok());
        let mut metadata = result.unwrap();
        metadata.fix_file_name();
        assert_eq!(metadata.save().ok(), Some(enum_set!()));
        assert!(!tmp_file_path.exists());
        assert!(target_file_path.exists());
    }
}
