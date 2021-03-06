use std::fs;
use std::option::Option;
use std::path::PathBuf;

use crate::config::Config;
use crate::data::Template;

fn is_single_file_teplate(template_path: &PathBuf) -> bool {
    let children = fs::read_dir(&template_path).unwrap();
    let all_entities: Vec<_> = children.map(|a| a.unwrap()).collect();
    let all_entities_len = all_entities.len();
    let files: Vec<_> = all_entities
        .iter()
        .filter(|a| a.file_type().unwrap().is_file())
        .collect();
    all_entities_len == 1 && files.len() == 1
}

fn load_template(template_path: PathBuf) -> Template {
    let name = template_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let is_single_file = is_single_file_teplate(&template_path);
    let path = template_path.into_os_string().into_string().unwrap();
    Template {
        name,
        path,
        is_single_file,
    }
}

fn _list_templates(templates_dir: &PathBuf) -> Vec<Template> {
    let paths = fs::read_dir(templates_dir).unwrap();
    let mut templates: Vec<Template> = paths.map(|e| load_template(e.unwrap().path())).collect();
    templates.sort_by(|a, b| a.name.cmp(&b.name));
    templates
}

pub fn list_templates(config: &impl Config) -> Vec<Template> {
    let templates_dir = PathBuf::from(&config.get_templates_dir());
    _list_templates(&templates_dir)
}

pub fn _get_template(templates_dir: &PathBuf, template_name: String) -> Option<Template> {
    let templates = _list_templates(templates_dir);
    templates.into_iter().find(|e| e.name == template_name)
}

pub fn get_template(config: &impl Config, template_name: String) -> Option<Template> {
    let templates_dir = PathBuf::from(&config.get_templates_dir());
    _get_template(&templates_dir, template_name)
}

#[cfg(test)]
mod tests {

    use std::env;

    use crate::repository::{Template, _get_template, _list_templates};

    #[test]
    fn test_list_templates() {
        let test_dir = env::current_dir().unwrap().join("test");
        let templates = _list_templates(&test_dir);
        assert_eq!(templates.len(), 3);
        assert_eq!(
            templates[0],
            Template {
                name: "template1".to_string(),
                path: env::current_dir()
                    .unwrap()
                    .join("test/template1")
                    .into_os_string()
                    .into_string()
                    .unwrap(),
                is_single_file: true
            }
        );
        assert_eq!(
            templates[1],
            Template {
                name: "template2".to_string(),
                path: env::current_dir()
                    .unwrap()
                    .join("test/template2")
                    .into_os_string()
                    .into_string()
                    .unwrap(),
                is_single_file: false
            }
        );
        assert_eq!(
            templates[2],
            Template {
                name: "template3".to_string(),
                path: env::current_dir()
                    .unwrap()
                    .join("test/template3")
                    .into_os_string()
                    .into_string()
                    .unwrap(),
                is_single_file: false
            }
        );
    }

    #[test]
    fn test_get_template() {
        let test_dir = env::current_dir().unwrap().join("test");
        assert_eq!(_get_template(&test_dir, "notfound".to_string()), None);
        assert_eq!(
            _get_template(&test_dir, "template1".to_string()),
            Some(Template {
                name: "template1".to_string(),
                path: env::current_dir()
                    .unwrap()
                    .join("test/template1")
                    .into_os_string()
                    .into_string()
                    .unwrap(),
                is_single_file: true
            })
        );
    }
}
