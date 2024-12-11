use enumset::EnumSet;
use enumset::EnumSetType;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

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

pub trait DisplayEnumSet {
    fn to_string_coma(&self) -> String;
}

impl DisplayEnumSet for EnumSet<Tag> {
    fn to_string_coma(&self) -> String {
        if self.is_empty() {
            "None".to_string()
        } else {
            self.iter()
                .map(|t| t.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        }
    }
}
