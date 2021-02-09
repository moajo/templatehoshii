use std::env;

pub struct EnvConfig {}
pub struct StaticConfig {
    pub templates_dir: String,
}

impl Config for StaticConfig {
    fn get_templates_dir(&self) -> String {
        self.templates_dir.clone()
    }
}

impl Config for EnvConfig {
    fn get_templates_dir(&self) -> String {
        let home = env::home_dir().unwrap();
        let templates_dir = home.join(".templatehoshii/templates");
        templates_dir.into_os_string().into_string().unwrap()
    }
}

pub trait Config {
    fn get_templates_dir(&self) -> String;
}
