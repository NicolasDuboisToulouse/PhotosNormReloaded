use core::metadata::Metadata;
use std::{
    fmt::{Debug, Display},
    path::PathBuf,
};

pub struct Data {
    pub filename: String,
    pub uri: String,
    #[allow(dead_code)] // TODO: May be used later
    pub metadata: Metadata,
    // We store a copy of camefa info to fast the render
    pub camera: String,
    // We store a copy of these to know if they have changed or not
    pub description: String,
    pub date: String,
}

#[derive(Debug)]
pub struct Error {
    pub filename: String,
    pub msg: String,
}

impl Error {
    fn new(filename: &str, msg: &str) -> Self {
        Error {
            filename: filename.to_string(),
            msg: msg.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.msg)
    }
}

pub type ImageResult = Result<Data, Error>;

impl Data {
    pub fn new(path: &PathBuf) -> ImageResult {
        let filename = path
            .file_name()
            .unwrap_or(std::ffi::OsStr::new("Unexpected invalid file name"))
            .to_string_lossy()
            .to_string();

        let uri = match std::fs::canonicalize(path) {
            Ok(path) => match path.to_str() {
                Some(path_str) => {
                    let mut uri = String::from("file://");
                    uri.push_str(path_str);
                    Ok(uri)
                }
                None => Err(Error::new(&filename, "Unsupported non-UT8 paths.")),
            },
            Err(e) => Err(Error::new(&filename, &e.to_string())),
        }?;

        let metadata = match Metadata::new(path) {
            Ok(m) => Ok(m),
            Err(e) => Err(Error::new(&filename, &e.to_string())),
        }?;

        let camera = metadata.camera_info().to_string();

        let description = metadata.description().unwrap_or(String::new());

        let date = metadata.exif_date().unwrap_or(String::new());

        Ok(Data {
            filename,
            uri,
            metadata,
            camera,
            description,
            date,
        })
    }
}
