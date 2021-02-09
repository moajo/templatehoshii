// use chrono::{Local, NaiveDate};
use clap::{crate_authors, crate_version, App, Arg, SubCommand};
// use dialoguer::{theme::ColorfulTheme, Select};
// use healthcare::{get_available_times, load_config, reserve};
use env_logger;
use log::{debug, info, trace};
use std::env;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use templatehoshii::repository::list_templates;

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
        println!("dump!!!"); // subが指定されていればメッセージを表示
                             // subflgのON/OFFで表示するメッセージを切り替え
                             // println!("subflg is {}", if matches.is_present("subflg") {"ON"} else {"OFF"});
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
