// use chrono::{Local, NaiveDate};
use clap::{crate_authors, crate_version, App, Arg, SubCommand};
use dialoguer::{theme::ColorfulTheme, Select};
// use healthcare::{get_available_times, load_config, reserve};
use env_logger;
use log::{debug, info, trace};
use std::env;

use std::path::{Path, PathBuf};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use templatehoshii::add::add;
use templatehoshii::config::{Config, EnvConfig, StaticConfig};
use templatehoshii::dump::dump;
use templatehoshii::repository::{get_template, list_templates};
use templatehoshii::rm::rm;

fn cli(config: &impl Config, args: Vec<String>) -> i32 {
    let matches = App::new("templatehoshii")
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(
            SubCommand::with_name("dump")
                .about("dump template to stdout or file")
                .arg(
                    Arg::with_name("template")
                        .help("[optional] name of template to dump"),
                )
                .arg(
                    Arg::with_name("to_file")
                        .help("dump to file using default filename")
                        .short("f")
                        .long("to-file"),
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .alias("ls")
                .about("list all templates. * means template has only 1 file, and it dumps to stdout as default."),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("add new template")
                .arg(
                    Arg::with_name("name")
                        .required(true)
                        .help("new template name"),
                )
                .arg(
                    Arg::with_name("source")
                        .required(true)
                        .help("template content. file or directory"),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm").about("remove template").arg(
                Arg::with_name("name")
                    .required(true)
                    .help("template name to remove"),
            ),
        )
        .get_matches_from(args);

    let template_dir = config.get_templates_dir();
    std::fs::create_dir_all(template_dir).unwrap();

    if let Some(matches) = matches.subcommand_matches("add") {
        let template_name = matches.value_of("name").unwrap().to_string();
        let content_path = matches.value_of("source").unwrap();
        let content_path = PathBuf::from(content_path);

        add(config, template_name, content_path);

        return 0;
    }
    if let Some(matches) = matches.subcommand_matches("rm") {
        let template_name = matches.value_of("name").unwrap().to_string();

        rm(config, template_name);

        return 0;
    }

    if let Some(matches) = matches.subcommand_matches("dump") {
        let to_file = matches.is_present("to_file");
        let template_name = match matches.value_of("template").map(|s| s.to_string()) {
            None => {
                info!("[mode] interactive");
                let templates = list_templates(config);
                let availables: Vec<_> = templates
                    .iter()
                    .map(|a| {
                        if a.is_single_file {
                            format!("*{}", a.name.to_string())
                        } else {
                            a.name.to_string()
                        }
                    })
                    .collect();
                debug!("availables: {:?}", availables);
                if availables.is_empty() {
                    println!("No template is available ");
                    return 0;
                }
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select template to dump?")
                    .default(0)
                    .items(&availables)
                    .interact()
                    .unwrap();
                let selected = &availables[selection];
                selected.to_string()
            }
            Some(template_name) => template_name,
        };
        let current_dir = env::current_dir().unwrap();
        let template = get_template(config, template_name);
        if let Some(template) = template {
            if template.is_single_file {
                if to_file {
                    dump(&template, current_dir);
                } else {
                    // dump to stdio
                    let contents =
                        std::fs::read_to_string(template.content_file_path_if_sft().unwrap())
                            .unwrap();
                    println!("{}", contents);
                }
                return 0;
            }
            dump(&template, current_dir);
            return 0;
        } else {
            println!("template not found!");
            return 1;
        }
    }

    if let Some(_) = matches.subcommand_matches("list") {
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
    return 1;
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let config = EnvConfig {};
    let exit_code = cli(&config, args);
    std::process::exit(exit_code);
}

#[cfg(test)]
mod tests {
    use crate::cli;
    use std::env;
    use templatehoshii::config::{Config, EnvConfig, StaticConfig};

    #[test]
    fn test_cli_list() {
        let test_dir = env::current_dir().unwrap().join("test");
        let config = StaticConfig {
            templates_dir: test_dir.to_str().unwrap().to_string(),
        };
        let args = vec!["templatehoshii", "list"];
        let args = args.iter().map(|a| a.to_string()).collect();
        let exit_code = cli(&config, args);
        assert_eq!(exit_code, 0);
    }
    #[test]
    fn test_cli_dump() {
        let test_dir = env::current_dir().unwrap().join("test");
        let config = StaticConfig {
            templates_dir: test_dir.to_str().unwrap().to_string(),
        };
        let args = vec!["templatehoshii", "dump", "template1"];
        let args = args.iter().map(|a| a.to_string()).collect();
        let exit_code = cli(&config, args);
        assert_eq!(exit_code, 0);
    }
    #[test]
    fn test_cli_dump2() {
        let test_dir = env::current_dir().unwrap().join("test");
        let config = StaticConfig {
            templates_dir: test_dir.to_str().unwrap().to_string(),
        };
        let args = vec!["templatehoshii", "dump", "template2"];
        let args = args.iter().map(|a| a.to_string()).collect();
        let exit_code = cli(&config, args);
        assert_eq!(exit_code, 1);
    }
}
