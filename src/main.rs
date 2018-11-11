#[macro_use]
extern crate serde_derive;

extern crate clap;
extern crate curl;
extern crate itertools;
extern crate preferences;
extern crate scopeguard;
extern crate serde_json;

mod command;
mod settings;
mod webtask;
use command::Command;
use preferences::{AppInfo, Preferences};
use settings::Settings;

use clap::{App, Arg, SubCommand};

const APP_INFO: AppInfo = AppInfo {
    name: "clitask",
    author: "Francisco López",
};

fn main() {
    let matches = App::new("clitask")
        .version("0.1.0")
        .author("Francisco López")
        .about("Call webtask from your cli")
        .subcommand(
            SubCommand::with_name("add")
                .about("Add a new webtask to call")
                .arg(
                    Arg::with_name("webtask")
                        .required(true)
                        .takes_value(true)
                        .index(1)
                        .help("The name of the webtask you want to add"),
                )
                .arg(
                    Arg::with_name("url")
                        .required(false)
                        .takes_value(true)
                        .index(2)
                        .help("The url to call"),
                ),
        )
        .subcommand(
            SubCommand::with_name("call")
                .about("Call a webtask for you")
                .arg(
                    Arg::with_name("webtask")
                        .required(true)
                        .takes_value(true)
                        .index(1)
                        .help("The name of the webtask you want to call"),
                )
                .arg(
                    Arg::with_name("data")
                        .required(false)
                        .takes_value(true)
                        .index(2)
                        .help("The input for your data, you can pipe in here :D"),
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("List webtask from the store")
                .arg(
                    Arg::with_name("query")
                        .required(false)
                        .multiple(true)
                        .takes_value(true)
                        .index(1)
                        .help("Query to look for in the store"),
                ),
        )
        .subcommand(
            SubCommand::with_name("publish")
                .about("Publish a new webtask in the webtask store.")
                .arg(
                    Arg::with_name("webtask")
                        .required(true)
                        .takes_value(true)
                        .index(1)
                        .help("The name of the webtask you want to publish"),
                )
                .arg(
                    Arg::with_name("url")
                        .required(true)
                        .takes_value(true)
                        .index(2)
                        .help("The url of the webtask"),
                )
                .arg(
                    Arg::with_name("description")
                        .required(true)
                        .takes_value(true)
                        .short("d")
                        .long("description")
                        .help("Description of the webtask"),
                ),
        )
        .subcommand(
            SubCommand::with_name("config")
                .about("Add your user name to be able to publish and see your private funtions")
                .arg(
                    Arg::with_name("username")
                        .required(true)
                        .takes_value(true)
                        .index(1)
                        .help("Your username"),
                )
                .arg(
                    Arg::with_name("password")
                        .required(true)
                        .takes_value(true)
                        .index(2)
                        .help("Your password"),
                ),
        )
        .get_matches();

    let path = ".config";

    let s = Settings::load(&APP_INFO, path);
    let settings = match s {
        Ok(s) => s,
        Err(e) => Settings {
            webtasks: None,
            user: None,
        },
    };

    if !settings.is_configured() {
        println!(
            "You need to configure your user before running -> {}",
            matches.subcommand_name().unwrap_or_default()
        );
    } else {
        match matches.subcommand() {
            ("add", Some(sub_matched)) => {
                let cmd = Command::new("add".to_owned(), settings, sub_matched.clone());

                cmd.run();
            }
            ("call", Some(sub_matched)) => {
                let cmd = Command::new("call".to_owned(), settings, sub_matched.clone());

                cmd.run();
            }
            ("list", Some(sub_matched)) => {
                let cmd = Command::new("list".to_owned(), settings, sub_matched.clone());

                cmd.run();
            }
            ("publish", Some(sub_matched)) => {
                let cmd = Command::new("publish".to_owned(), settings, sub_matched.clone());

                cmd.run();
            }
            ("config", Some(sub_matched)) => {
                let cmd = Command::new("config".to_owned(), settings, sub_matched.clone());

                cmd.run();
            }
            ("", None) => {
                println!("{}", matches.usage());
            }
            (_, _) => {
                unreachable!();
            }
        }
    }
}
