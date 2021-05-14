use crate::config::Config;
use rm_rf;
use std::path::PathBuf;

pub fn rm(config: &impl Config, template_name: String) {
    let templates_dir = config.get_templates_dir();
    let template_dist_dir = PathBuf::from(templates_dir).join(&template_name);
    if !template_dist_dir.exists() {
        println!("ERROR: template {} not exists", template_name);
        return;
    }

    rm_rf::ensure_removed(template_dist_dir).unwrap();
    println!("Template '{}' is removed.", template_name);
}
