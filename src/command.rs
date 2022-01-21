use clap::ArgMatches;
use serde_json;
use crate::settings::Settings;
use std::io::{self, Read};
use crate::webtask::Webtask;

pub struct Command {
    action: String,
    clitask: Webtask,
    settings: Settings,
    matches: ArgMatches<'static>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct StoreCommand {
    action: String,
    webtask: Webtask,
}

const CLITASK_STORE: &str =
    "https://wt-9453e88d313e9ec8d56d3b94a1504f8c-0.sandbox.auth0-extend.com/clitask";

impl Command {
    pub fn new(action: String, settings: Settings, matches: ArgMatches<'static>) -> Command {
        Command {
            action,
            clitask: Webtask {
                url: String::from(CLITASK_STORE),
                name: String::from("clistore"),
                description: String::from("Store webtask"),
                repository: None,
                private: true,
                author: String::from("Francisco López"),
            },
            settings,
            matches,
        }
    }

    pub fn run(self) {
        match self.action.as_ref() {
            "add" => {
                let wt = self.matches.value_of("webtask").unwrap();
                let mut webtask;

                if self.matches.is_present("url") {
                    webtask = Webtask {
                        name: wt.to_string(),
                        author: self.settings.user.clone().unwrap(),
                        repository: None,
                        description: String::from(""),
                        private: false,
                        url: self.matches.value_of("url").unwrap().to_string(),
                    };

                    self.settings.insert_webtask(&webtask);
                } else {
                    let wt_path = wt.split("/");
                    let vec: Vec<&str> = wt_path.collect();
                    if vec.len() == 2 {
                        let cmd = StoreCommand {
                            action: "get".to_owned(),
                            webtask: Webtask {
                                name: vec[1].to_string(),
                                author: vec[0].to_string(),
                                repository: None,
                                description: String::from(""),
                                private: false,
                                url: "".to_owned(),
                            },
                        };

                        let wt: Webtask = serde_json::from_str(
                            &*self.clitask.call(serde_json::to_string(&cmd).unwrap()),
                        ).unwrap();

                        println!("The wt is: {:?}", wt);

                        self.settings.insert_webtask(&wt);
                    }
                    // TODO: not done yet, look for the webtask in the store and get
                    // the data from there.
                }
            }

            "call" => {
                let wt_name = self.matches.value_of("webtask").unwrap();
                let webtasks = self.settings.get_webtasks();
                let wt = webtasks.get(wt_name);
                let mut data = self
                    .matches
                    .value_of("data")
                    .unwrap_or_default()
                    .to_string();

                if data.len() == 0 {
                    let stdin = io::stdin();
                    let mut stdin = stdin.lock();

                    while let Ok(n_bytes) = stdin.read_to_string(&mut data) {
                        if n_bytes == 0 {
                            break;
                        }
                    }
                };

                let response = match wt {
                    Some(w) => w.call(data),
                    None => {
                        String::from(format!("{} {}", "No webtask found with the name ", wt_name))
                    }
                };

                println!("{}", response.replace("\\n", "\n").replace("\"", ""));
            }

            "list" => {
                let clitask = self.clitask;
                let response = clitask.call(String::from("{\"action\":\"list\"}"));
                println!("{}", response);
            }

            "publish" => {
                let cmd = StoreCommand {
                    action: "publish".to_owned(),
                    webtask: Webtask {
                        name: self
                            .matches
                            .value_of("webtask")
                            .unwrap_or_default()
                            .to_owned(),
                        author: self.settings.user.clone().unwrap(),
                        description: self
                            .matches
                            .value_of("description")
                            .unwrap_or_default()
                            .to_owned(),
                        private: false,
                        url: self.matches.value_of("url").unwrap_or_default().to_owned(),
                        repository: None,
                    },
                };

                self.clitask
                    .call(serde_json::to_string(&cmd).unwrap_or_default());
            }

            "config" => self.settings.configure(
                self.matches
                    .value_of("username")
                    .unwrap_or_default()
                    .to_owned(),
            ),

            _ => (),
        };
    }
}
