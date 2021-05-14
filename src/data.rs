use std::option::Option;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub struct Template {
    pub name: String,
    pub path: String,
    pub is_single_file: bool,
}

impl Template {
    pub fn content_file_path_if_sft(&self) -> Option<PathBuf> {
        if !self.is_single_file {
            return None;
        }
        for entry in std::fs::read_dir(&self.path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            return Some(path);
        }
        None
    }
}
