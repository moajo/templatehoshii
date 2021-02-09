use crate::data::Template;
use std::fs;
use std::path::{Path, PathBuf};

fn write_file(template_path: &Path, dist_dir: PathBuf) {
    println!("{:?} => {:?}", template_path, dist_dir);
}

fn _dump(template_path: &Path, dist_dir: PathBuf) {
    for entry in fs::read_dir(&template_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let entity_name = entry.file_name();
        if path.is_dir() {
            _dump(&path, dist_dir.join(entity_name))
        } else {
            write_file(&path, dist_dir.join(entity_name));
        }
    }
}

pub fn dump(template: &Template, dist_dir: PathBuf) {
    let template_path = Path::new(&template.path);
    _dump(template_path, dist_dir)
}
