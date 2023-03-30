use clap::{arg, command, Args, Parser, Subcommand};
use colored::Colorize;
use log::{error, info, warn};
use serde_json::to_string;
use std::env;
use std::path::{Path, PathBuf};
use std::time::Duration;
mod api;
mod config;
use api::{drop_server, get_list, get_status, put_server, put_status};
use config::{config, mkconfig};
extern crate clap;
extern crate log;

#[derive(Parser)]
#[command(name = "uof-status")]
#[command(author = "UrsusFeline <ursusfeline07@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "A client of uof-status written in rust", long_about = None)]
struct Cli {
    #[arg(
        short = 'c',
        long = "config",
        value_name = "FILE",
        help = "Upload status with a configuration file(default=./status.toml)"
    )]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a server to the main server
    Put(Put),
    /// Delete a server from the main server
    Drop(Drop),
    /// Start uploading status
    Status(Status),
    /// Get a list of servers
    List(List),
    /// Query the server status
    Inquire(Inquire),
}
#[derive(Args)]
struct Put {
    #[arg(
        short = 'u',
        long = "url",
        value_name = "URL",
        help = "Input a API URL of main server"
    )]
    url: String,
    #[arg(
        short = 't',
        long = "token",
        value_name = "TOKEN",
        help = "Input a global_token"
    )]
    token: String,
    #[arg(
        short = 'n',
        long = "name",
        value_name = "NAME",
        help = "Input a server name"
    )]
    name: String,
    #[arg(
        short = 'd',
        long = "description",
        value_name = "REMARK",
        help = "Input a description"
    )]
    description: String,
    #[arg(
        short = 'm',
        long = "mkconfig",
        value_name = "FILE",
        help = "Make a configuration file"
    )]
    mkconfig: Option<PathBuf>,
}
#[derive(Args)]
struct Drop {
    #[arg(
        short = 'u',
        long = "url",
        value_name = "URL",
        help = "Input a API URL of main server"
    )]
    url: String,
    #[arg(
        short = 't',
        long = "token",
        value_name = "TOKEN",
        help = "Input a global_token"
    )]
    token: String,
    #[arg(
        short = 'i',
        long = "id",
        value_name = "ID",
        help = "Input a server ID"
    )]
    id: u64,
}
#[derive(Args)]
struct Status {
    #[arg(
        short = 'u',
        long = "url",
        value_name = "URL",
        help = "Input a API URL of main server"
    )]
    url: String,
    #[arg(
        short = 't',
        long = "token",
        value_name = "TOKEN",
        help = "Input a user token"
    )]
    token: String,
    #[arg(
        short = 'i',
        long = "id",
        value_name = "ID",
        help = "Input a server ID"
    )]
    id: u64,
    #[arg(
        short = 'o',
        long = "offline",
        value_name = "STATUS",
        help = "Upload offline status"
    )]
    offline: bool,
    #[arg(
        short = 's',
        long = "seconds",
        value_name = "TIME",
        help = "Set the interval,the unit is seconds(default=60)",
        default_value_t = 60
    )]
    seconds: u64,
}
#[derive(Args)]
struct List {
    #[arg(
        short = 'u',
        long = "url",
        value_name = "URL",
        help = "Input a API URL of main server"
    )]
    url: String,
}
#[derive(Args)]
struct Inquire {
    #[arg(
        short = 'u',
        long = "url",
        value_name = "URL",
        help = "Input a API URL of main server"
    )]
    url: String,
    #[arg(
        short = 'i',
        long = "id",
        value_name = "ID",
        help = "Input a server ID"
    )]
    id: u64,
}

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    let cli = Cli::parse();

    match &cli.config {
        Some(path) => config(path.to_path_buf()),
        None => match &cli.command {
            Some(Commands::Put(put)) => {
                let Put {
                    url,
                    token,
                    name,
                    description,
                    mkconfig,
                } = put;
                let res = put_server(&url, &token, &name, &description).await;
                match res {
                    Ok(v) => {
                        if v.get("success").unwrap() == true {
                            let _id = to_string(&v.get("id")).unwrap();
                            let _token = to_string(&v.get("token")).unwrap();
                            let _token_len = to_string(&v.get("token")).unwrap().len() - 1;
                            println!("{}", "NOTE: SUCCESSFULLY PUT SERVER:".yellow().bold());
                            println!(
                                "\n{} {:?}\n{} {:?}\n",
                                "Server ID:".blue().bold(),
                                &_id,
                                "Server token:".blue().bold(),
                                &_token[1.._token_len]
                            );
                            /*match mkconfig {
                                Some(_path) => mkconfig(_path),
                                None => (),
                            }*/
                            println!("{}", "THE INFORMATION GIVEN ABOVE WILL".red().bold());
                            println!("{}", "BE ONLY DISPLAYED ONCE, PLEASE SAVE".red().bold());
                            println!("{}", "THEM - ESPECIALLY THE TOKEN - CAREFULLY".red().bold())
                        } else {
                            error!("Adding server to {},Fail: {v:?}", url)
                        };
                    }
                    Err(e) => error!("Adding server to {},Fail: {e:?}", url),
                }
            }
            Some(Commands::Drop(drop)) => {
                let Drop { url, token, id } = drop;
                let res = drop_server(&url, &token, id.to_owned()).await;
                match res {
                    Ok(v) => {
                        if v.get("success").unwrap() == true {
                            warn!(
                                "Remove the server {} from {},{}",
                                id,
                                url,
                                "Success".green().bold()
                            )
                        } else {
                            error!(
                                "Remove the server {} from {},{} {v:?}",
                                id,
                                url,
                                "Fail:".red().bold()
                            )
                        };
                    }
                    Err(e) => error!(
                        "Remove the server {} from {},{} {e:?}",
                        id,
                        url,
                        "Fail:".red().bold()
                    ),
                }
            }
            Some(Commands::Status(status)) => {
                let Status {
                    url,
                    token,
                    id,
                    offline,
                    seconds,
                } = status;
                let online = !offline;
                loop {
                    let res = put_status(&url, &token, id.to_owned(), online).await;
                    match res {
                        Ok(v) => {
                            if v.get("success").unwrap() == true {
                                info!("Uploading status to {} : {}", url, "Success".green())
                            } else {
                                error!(
                                    "Uploading status to {},{} : {v:?}",
                                    url,
                                    "Fail".red().bold()
                                )
                            }
                        }
                        Err(e) => error!(
                            "Uploading status to {},{} : {e:?}",
                            url,
                            "Fail".red().bold()
                        ),
                    }
                    std::thread::sleep(Duration::from_secs(seconds.to_owned()));
                }
            }
            Some(Commands::List(list)) => {
                let List { url } = list;
                let res = get_list(url).await;
                match res {
                    Ok(v) => println!("{v:?}"),
                    Err(e) => error!("Fail to get server list: {e:?}"),
                }
            }
            Some(Commands::Inquire(inquire)) => {
                let Inquire { url, id } = inquire;
                let res = get_status(url, id.to_owned()).await;
                match res {
                    Ok(v) => println!("{v:?}"),
                    Err(e) => error!("{e:?}"),
                }
            }
            None => config(Path::new("./status.toml").to_path_buf()),
        },
    }
}
