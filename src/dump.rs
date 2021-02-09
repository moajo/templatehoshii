use crate::data::Template;
use std::fs;
use std::path::{Path, PathBuf};

fn write_file(template_path: &Path, dist_dir: &PathBuf) {
    println!("{:?} => {:?}", template_path, dist_dir);
    let fname = template_path.file_name().unwrap();
    fs::create_dir_all(dist_dir).unwrap();
    fs::copy(template_path, dist_dir.join(fname)).unwrap();
}

fn _dump(template_path: &Path, dist_dir: &PathBuf) {
    for entry in fs::read_dir(&template_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let entity_name = entry.file_name();
        if path.is_dir() {
            _dump(&path, &dist_dir.join(entity_name))
        } else {
            write_file(&path, dist_dir);
        }
    }
}
fn _check_conflict(template_path: &Path, dist_dir: &PathBuf) -> bool {
    for entry in fs::read_dir(&template_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let entity_name = entry.file_name();
        if path.is_dir() {
            if _check_conflict(&path, &dist_dir.join(entity_name)) {
                return true;
            }
        } else {
            let file = dist_dir.join(entity_name);
            if file.exists() {
                println!("ERROR: file {} already exists", file.to_str().unwrap());
                return true;
            }
        }
    }
    return false;
}

pub fn dump(template: &Template, dist_dir: PathBuf) {
    let template_path = Path::new(&template.path);
    if _check_conflict(template_path, &dist_dir) {
        return;
    }
    _dump(template_path, &dist_dir)
}
