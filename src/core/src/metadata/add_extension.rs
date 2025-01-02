use std::ffi::{OsStr, OsString};

pub trait AddExtension {
    fn add_ext(&mut self, extension: &OsStr);
}

impl AddExtension for OsString {
    fn add_ext(&mut self, extension: &OsStr) {
        if !extension.is_empty() {
            let mut os_string = self.to_os_string();
            os_string.push(".");
            os_string.push(extension);
            self.clone_from(&os_string);
        }
    }
}
