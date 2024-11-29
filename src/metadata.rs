use chrono::NaiveDateTime;
use image::image_dimensions;
use little_exif::rational::iR64;
use little_exif::{
    exif_tag::ExifTag, metadata::Metadata as LittleMetadata, rational::uR64,
    u8conversion::U8conversion,
};
use std::fmt;
use std::fmt::Formatter;
use std::{
    fmt::Display,
    io::Error,
    path::{Path, PathBuf},
};

pub struct CameraInfo {
    pub camera: Option<String>,
    pub exposure: Option<String>,
    pub exposure_bias: Option<String>,
    pub aperture: Option<String>,
    pub iso: Option<u16>,
    pub focal: Option<f64>,
    pub flash: Option<String>,
}

impl Display for CameraInfo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}, Exposure: {}, Bias: {}, Aperture: {}, ISO: {}, Focal: {}, Flash: {}",
            self.camera
                .as_ref()
                .unwrap_or(&"Unknown camera".to_string()),
            self.exposure.as_ref().unwrap_or(&"Undefined".to_string()),
            self.exposure_bias
                .as_ref()
                .unwrap_or(&"Undefined".to_string()),
            self.aperture.as_ref().unwrap_or(&"Undefined".to_string()),
            match self.iso {
                Some(v) => v.to_string(),
                None => "Undefined".to_string(),
            },
            match self.focal {
                Some(v) => format!("{} mm", v),
                None => "Undefined".to_string(),
            },
            self.flash.as_ref().unwrap_or(&"Undefined".to_string()),
        )
    }
}

pub struct Metadata {
    path: PathBuf,
    litte_metadata: LittleMetadata,
    dimentions: (u32, u32),
    date: Option<NaiveDateTime>,
    description: Option<String>,
    camera_info: CameraInfo,
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
            Some(str_date) => NaiveDateTime::parse_from_str(&str_date, "%Y:%m:%d %H:%M:%S").ok(),
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
    pub fn camera_info(&self) -> &CameraInfo {
        &self.camera_info
    }

    // Read a tag as a string
    fn get_tag_string(litte_metadata: &LittleMetadata, tag: &ExifTag) -> Option<String> {
        let tag = litte_metadata.get_tag(tag).next()?;
        let endian = litte_metadata.get_endian();
        Some(String::from_u8_vec(&tag.value_as_u8_vec(&endian), &endian))
    }

    // Read a tag as a u16
    fn get_tag_u16(litte_metadata: &LittleMetadata, tag: &ExifTag) -> Option<u16> {
        let tag = litte_metadata.get_tag(tag).next()?;
        let endian = litte_metadata.get_endian();
        Some(u16::from_u8_vec(&tag.value_as_u8_vec(&endian), &endian))
    }

    //  Read a tag as a uR64
    fn get_tag_ur64(litte_metadata: &LittleMetadata, tag: &ExifTag) -> Option<uR64> {
        let endian = litte_metadata.get_endian();
        litte_metadata
            .get_tag(tag)
            .next()
            .map(|tag| uR64::from_u8_vec(&tag.value_as_u8_vec(&endian), &endian))
    }

    //  Read a tag as a iR64
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
}
