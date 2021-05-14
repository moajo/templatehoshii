use crate::data::Template;
use log::info;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn write_file(template_path: &Path, dist_dir: &PathBuf) {
    let current = env::current_dir().unwrap();
    let fname = template_path.file_name().unwrap();
    let dst_path = dist_dir.join(fname);
    let relative_path = dst_path.strip_prefix(current).unwrap();
    info!("{:?} => {:?}", template_path, dst_path);
    println!("👉 {}", relative_path.to_string_lossy());
    fs::create_dir_all(dist_dir).unwrap();
    fs::copy(template_path, dst_path).unwrap();
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
