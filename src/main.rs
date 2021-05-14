use clap::{crate_authors, crate_version, App, Arg};
use dialoguer::{theme::ColorfulTheme, Select};
use env_logger;
use log::{debug, info};
use std::env;

use std::option::Option;
use std::path::PathBuf;

use templatehoshii::add::add;
use templatehoshii::config::{Config, EnvConfig};
use templatehoshii::dump::dump;
use templatehoshii::repository::{get_template, list_templates};
use templatehoshii::rm::rm;

fn select_template_interactively(config: &impl Config) -> Option<String> {
    info!("[mode] interactive");
    let templates = list_templates(config);
    let available_template_names: Vec<_> = templates.iter().map(|a| a.name.to_string()).collect();
    let select_items: Vec<_> = templates
        .iter()
        .map(|a| {
            if a.is_single_file {
                format!("*{}", a.name.to_string())
            } else {
                a.name.to_string()
            }
        })
        .collect();
    debug!("availables: {:?}", select_items);
    if select_items.is_empty() {
        println!("No template is available ");
        return None;
    }
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select template to dump?")
        .default(0)
        .items(&select_items)
        .interact()
        .unwrap();
    let selected = &available_template_names[selection];
    Some(selected.to_string())
}

pub fn build_argparser<'a, 'b>() -> clap::App<'a, 'b> {
    App::new("templatehoshii")
    .version(crate_version!())
    .author(crate_authors!())
    .arg(
        Arg::with_name("template")
            .help("target template name"),
    )
    .arg(
        Arg::with_name("list")
        .long("list")
        .short("ls")
        .conflicts_with_all(&[ "template"])
            .help("list all templates. * means template has only 1 file, and it dumps to stdout as default."),
    )
    .arg(
        Arg::with_name("add")
        .long("add")
        .short("a")
        .requires("template")
        .conflicts_with_all(&[ "list"])
        .takes_value(true)
            .help("new template name to add"),
    )
    .arg(
        Arg::with_name("remove")
        .long("rm")
        .requires("template")
        .conflicts_with_all(&["add", "list"])
            .help("template name to remove"),
    )
}

fn cli(config: &impl Config, args: Vec<String>) -> i32 {
    let argparser = build_argparser();
    let matches = argparser.get_matches_from(args);

    let template_dir = config.get_templates_dir();
    std::fs::create_dir_all(template_dir).unwrap();

    // add template
    if let Some(template_name) = matches.value_of("add") {
        let content_path = PathBuf::from(matches.value_of("template").unwrap());
        add(config, template_name.to_string(), content_path);
        return 0;
    }

    // remove template
    if matches.is_present("remove") {
        let template_name = matches.value_of("template").unwrap();
        rm(config, template_name.to_string());
        return 0;
    }

    // list template
    if matches.is_present("list") {
        let templates = list_templates(config);
        for a in templates.iter() {
            if a.is_single_file {
                println!("*{}", a.name);
            } else {
                println!("{}", a.name);
            }
        }
        return 0;
    }

    // dump template
    if let Some(template_name) = matches.value_of("template") {
        let current_dir = env::current_dir().unwrap();
        let template = get_template(config, template_name.to_string());
        if let Some(template) = template {
            if template.is_single_file {
                dump(&template, current_dir);
                return 0;
            }
            dump(&template, current_dir);
            return 0;
        } else {
            println!("template not found!");
            return 1;
        }
    }

    // select and dump template
    let template_name = {
        let selected = select_template_interactively(config);
        if selected == None {
            return 0;
        }
        selected.unwrap()
    };
    let current_dir = env::current_dir().unwrap();
    let template = get_template(config, template_name);
    if let Some(template) = template {
        dump(&template, current_dir);
        return 0;
    } else {
        println!("template not found!");
        return 1;
    }
}

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let config = EnvConfig {};
    let exit_code = cli(&config, args);
    std::process::exit(exit_code);
}

#[cfg(test)]
mod tests {
    use crate::{build_argparser, cli};
    use std::env;
    use tempdir::TempDir;
    use templatehoshii::config::StaticConfig;
    use test_case::test_case;

    #[test]
    fn test_cli_list() {
        let test_dir = env::current_dir().unwrap().join("test");
        let config = StaticConfig {
            templates_dir: test_dir.to_str().unwrap().to_string(),
        };
        let args = vec!["templatehoshii", "--list"];
        let args = args.iter().map(|a| a.to_string()).collect();
        let exit_code = cli(&config, args);
        assert_eq!(exit_code, 0);
    }
    #[test]
    fn test_cli_dump_single_file() {
        let old_current_dir = env::current_dir().unwrap();
        let test_dir = env::current_dir().unwrap().join("test");
        let config = StaticConfig {
            templates_dir: test_dir.to_str().unwrap().to_string(),
        };

        let tmp_dir = TempDir::new("templatehosii-test1").unwrap();
        assert!(env::set_current_dir(&tmp_dir).is_ok());

        let args = vec!["templatehoshii", "template1"];
        let args = args.iter().map(|a| a.to_string()).collect();
        let exit_code = cli(&config, args);
        assert_eq!(exit_code, 0);
        assert!(env::set_current_dir(&old_current_dir).is_ok());
    }
    #[test]
    fn test_cli_dump_multi_file() {
        let old_current_dir = env::current_dir().unwrap();
        let test_dir = old_current_dir.join("test");
        let config = StaticConfig {
            templates_dir: test_dir.to_str().unwrap().to_string(),
        };
        let tmp_dir = TempDir::new("templatehosii-test").unwrap();
        assert!(env::set_current_dir(&tmp_dir).is_ok());

        let args = vec!["templatehoshii", "template2"];
        let args = args.iter().map(|a| a.to_string()).collect();
        let exit_code = cli(&config, args);
        assert_eq!(exit_code, 0);
        assert!(env::set_current_dir(&old_current_dir).is_ok());
    }

    #[test]
    fn test_cli_dump_include_subdir() {
        let old_current_dir = env::current_dir().unwrap();
        let test_dir = old_current_dir.join("test");
        let config = StaticConfig {
            templates_dir: test_dir.to_str().unwrap().to_string(),
        };
        let tmp_dir = TempDir::new("templatehosii-test").unwrap();
        assert!(env::set_current_dir(&tmp_dir).is_ok());

        let args = vec!["templatehoshii", "template3"];
        let args = args.iter().map(|a| a.to_string()).collect();
        let exit_code = cli(&config, args);
        assert_eq!(exit_code, 0);
        assert!(env::set_current_dir(&old_current_dir).is_ok());
    }

    #[test_case("templatehoshii --list --add --rm" ; "1")]
    #[test_case("templatehoshii -l -a" ; "2")]
    #[test_case("templatehoshii --rm -l" ; "3")]
    fn test_argparser_failed_conflict_args(args: &str) {
        let result = build_argparser().get_matches_from_safe(args.split_whitespace());
        assert_eq!(result.is_ok(), false);
    }

    #[test_case("templatehoshii -a" ; "add")]
    #[test_case("templatehoshii --rm" ; "rm")]
    #[test_case("templatehoshii -l hoge" ; "list")]
    fn test_argparser_failed_mising_required_args(args: &str) {
        let result = build_argparser().get_matches_from_safe(args.split_whitespace());
        assert_eq!(result.is_ok(), false);
    }
    #[test_case("templatehoshii -a templatename sourcedir" ; "add")]
    #[test_case("templatehoshii --rm hoge" ; "rm")]
    #[test_case("templatehoshii -l" ; "list")]
    #[test_case("templatehoshii hoge" ; "dump")]
    #[test_case("templatehoshii" ; "default")]
    fn test_argparser_ok(args: &str) {
        let result = build_argparser().get_matches_from_safe(args.split_whitespace());
        assert_eq!(result.is_ok(), true);
    }
}
