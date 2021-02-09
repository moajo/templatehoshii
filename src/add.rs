use crate::config::{Config, EnvConfig, StaticConfig};
use crate::data::Template;
use std::fs;
use std::path::{Path, PathBuf};

fn write_file(template_path: &PathBuf, dist_dir: &PathBuf) {
    println!("{:?} => {:?}", template_path, dist_dir);
    let fname = template_path.file_name().unwrap();
    fs::create_dir_all(dist_dir).unwrap();
    fs::copy(template_path, dist_dir.join(fname)).unwrap();
}

fn _copy(template_path: &PathBuf, dist_dir: &PathBuf) {
    for entry in fs::read_dir(&template_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let entity_name = entry.file_name();
        if path.is_dir() {
            _copy(&path, &dist_dir.join(entity_name))
        } else {
            write_file(&path, dist_dir);
        }
    }
}

pub fn add(config: &impl Config, template_name: String, content_path: PathBuf) {
    let templates_dir = config.get_templates_dir();
    let template_dist_dir = PathBuf::from(templates_dir).join(&template_name);
    if template_dist_dir.exists() {
        println!("ERROR: template {} already exists", template_name);
        return;
    }

    if !content_path.exists() {
        println!("ERROR: {} not exists", content_path.to_str().unwrap());
        return;
    }

    if content_path.is_file() {
        write_file(&content_path, &template_dist_dir);
        return;
    }

    if content_path.is_dir() {
        let fname = content_path.file_name().unwrap();
        _copy(&content_path, &template_dist_dir.join(fname));
        return;
    }
    panic!();
}
