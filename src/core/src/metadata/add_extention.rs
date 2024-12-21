use std::ffi::{OsStr, OsString};

pub trait AddExtention {
    fn add_ext(&mut self, extention: &OsStr);
}

impl AddExtention for OsString {
    fn add_ext(&mut self, extention: &OsStr) {
        if !extention.is_empty() {
            let mut os_string = self.to_os_string();
            os_string.push(".");
            os_string.push(extention);
            self.clone_from(&os_string);
        }
    }
}
