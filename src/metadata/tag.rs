use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Clone)]
pub enum Tag {
    Description,
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq)]
pub struct TagList {
    tags: Vec<Tag>,
}

impl TagList {
    pub fn new() -> TagList {
        TagList { tags: Vec::new() }
    }

    #[allow(dead_code)]
    pub fn new_from_slice(vec: &[Tag]) -> TagList {
        TagList { tags: vec.to_vec() }
    }

    pub fn push(&mut self, tag: Tag) {
        self.tags.push(tag);
    }

    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }
}
impl Display for TagList {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "None0")
        } else {
            write!(
                f,
                "{}",
                self.tags
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
    }
}
