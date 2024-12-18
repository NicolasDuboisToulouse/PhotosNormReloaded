use enumset::EnumSetType;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use super::Metadata;

#[derive(EnumSetType, Debug)]
pub enum Tag {
    Description,
    Date,
    Dimensions,
    FileName,
    Orientation,
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait DisplayWithComment {
    fn to_string_comment(&self, metadata: &Metadata) -> String;
}

impl DisplayWithComment for Tag {
    fn to_string_comment(&self, metadata: &Metadata) -> String {
        match self {
            Tag::FileName => format!(
                "{}({})",
                self,
                metadata.path.file_name().unwrap().to_string_lossy()
            ),
            _ => self.to_string(),
        }
    }
}
