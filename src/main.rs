use clap::{arg, command, Args, Parser, Subcommand};
use colored::Colorize;
use log::{error, info, warn};
use std::env;
use std::path::{Path, PathBuf};
use std::time::Duration;
mod api;
use api::{drop_server, get_list, get_status, put_server, put_status};
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
        help = "Run with a configuration file(default=./config.toml)"
    )]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a server to the main server
    put(put),
    /// Delete a server from the main server
    drop(drop),
    /// Start uploading status
    status(status),
    /// Get a list of servers
    list(list),
    /// Query the server status
    inquire(inquire),
}
#[derive(Args)]
struct put {
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
}
#[derive(Args)]
struct drop {
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
struct status {
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
}
#[derive(Args)]
struct list {
    #[arg(
        short = 'u',
        long = "url",
        value_name = "URL",
        help = "Input a API URL of main server"
    )]
    url: String,
}
#[derive(Args)]
struct inquire {
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

    match &cli.command {
        Some(Commands::put(put)) => {
            let put {
                url,
                token,
                name,
                description,
            } = put;
            let res = put_server(&url, &token, &name, &description).await;
            match res {
                Ok(v) => {
                    println!("{}", "NOTE: SUCCESSFULLY PUT SERVER:".yellow().bold());
                    println!("\n{v:?}\n");
                    println!("{}", "THE INFORMATION GIVEN ABOVE WILL".yellow().bold());
                    println!("{}", "BE ONLY DISPLAYED ONCE, PLEASE SAVE".yellow().bold());
                    println!(
                        "{}",
                        "THEM - ESPECIALLY THE TOKEN - CAREFULLY".yellow().bold()
                    );
                }
                Err(e) => error!("Adding server to {},Fail: {e:?}", url),
            }
        }
        Some(Commands::drop(drop)) => {
            let drop { url, token, id } = drop;
            let res = drop_server(&url, &token, id.to_owned()).await;
            match res {
                Ok(v) => (),
                Err(e) => (),
            }
        }
        Some(Commands::status(status)) => {
            let status {
                url,
                token,
                id,
                offline,
            } = status;
            let online = !offline;
            loop {
                let res = put_status(&url, &token, id.to_owned(), online).await;
                match res {
                    Ok(v) => info!("Uploading status to {} : {v:?}", url),
                    Err(e) => error!("Uploading status to {},Fail : {e:?}", url),
                }
                std::thread::sleep(Duration::from_secs(5));
            }
        }
        Some(Commands::list(list)) => (),
        Some(Commands::inquire(inquire)) => (),
        None => (),
    }
}
