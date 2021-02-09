// use chrono::{Local, NaiveDate};
use clap::{crate_authors, crate_version, App, Arg, SubCommand};
// use dialoguer::{theme::ColorfulTheme, Select};
// use healthcare::{get_available_times, load_config, reserve};
use env_logger;
use log::{debug, info, trace};
use std::env;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use templatehoshii::dump::dump;
use templatehoshii::repository::{get_template, list_templates};

#[tokio::main]
async fn main() {
    env_logger::init();

    let matches = App::new("templatehoshii")
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(
            SubCommand::with_name("dump")
                .about("sample subcommand") // このサブコマンドについて
                .arg(
                    Arg::with_name("template") // フラグを定義
                        .help("sample flag by sub"), // ヘルプメッセージ
                )
                .arg(
                    Arg::with_name("to_file")
                        .help("dump to file using default filename.")
                        .short("f")
                        .long("to-file"),
                ),
        )
        .subcommand(
            SubCommand::with_name("list"), // .about("sample subcommand") // このサブコマンドについて
        )
        .subcommand(
            SubCommand::with_name("add"), // .about("sample subcommand") // このサブコマンドについて
        )
        .get_matches();

    // subサブコマンドの解析結果を取得
    if let Some(ref matches) = matches.subcommand_matches("add") {
        println!("add!!!"); // subが指定されていればメッセージを表示
                            // subflgのON/OFFで表示するメッセージを切り替え
                            // println!("subflg is {}", if matches.is_present("subflg") {"ON"} else {"OFF"});
    }

    if let Some(ref matches) = matches.subcommand_matches("dump") {
        let to_file = matches.value_of("to_file") != None;
        if let Some(template_name) = matches.value_of("template") {
            let current_dir = env::current_dir().unwrap();
            let template = get_template(template_name.to_string());
            if let Some(template) = template {
                if !to_file {
                    if !template.is_single_file {
                        panic!("not --to-file but this template is not is_single_file")
                    }
                    // dump to stdio
                    let contents =
                        std::fs::read_to_string(template.content_file_path_if_sft().unwrap())
                            .unwrap();
                    println!("{}", contents)
                }

                dump(&template, current_dir);
            } else {
                println!("template not found!")
            }
        } else {
            println!("TODO: select templates interactively and dump it!");
        }
    }

    if let Some(_) = matches.subcommand_matches("list") {
        let templates = list_templates();
        for a in templates.iter() {
            println!("{:?}", a);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
