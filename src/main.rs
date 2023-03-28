use async_std::task::{self, block_on};
use clap::{arg, builder::Str, command, Args, Parser, Subcommand};
use log::{error, info, warn};
use std::env::args;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{env, str::FromStr};
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
        help = "Run with a configuration file"
    )]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a server to the primary server
    put(Put),
    /// Delete a server from the primary server
    drop(Drop),
    /// Start uploading status
    status(Status),
    /// Get a list of servers
    list(List),
    /// Query the server status
    inquire(Inquire),
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

async fn async_main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::put(Put)) => {
            let Put {
                url,
                token,
                name,
                description
            } = Put;
            let res = put_server(&url, &token, name, &description).await;
            match res {
                Ok(v) => {
                    warn!("NOTE: SUCCESSFULLY PUT SERVER:");
                    warn!("\n\n{v:?}\n");
                    warn!("THE INFORMATION GIVEN ABOVE WILL");
                    warn!("BE ONLY DISPLAYED ONCE, PLEASE SAVE");
                    warn!("THEM - ESPECIALLY THE TOKEN - CAREFULLY.");
                },
                Err(e) => error!("Adding server to {},Fail: {e:?}", url),
            }
        },
        Some(Commands::drop(Drop)) => (),
        Some(Commands::status(Status)) => {
            let Status {
                url,
                token,
                id,
                offline,
            } = Status;
            let online = !offline;
            loop {
                let res = put_status(&url, &token, id.to_owned(), online).await;
                match res {
                    Ok(v) => info!("Uploading status to {},Succeed: {v:?}", url),
                    Err(e) => error!("Uploading status to {},Fail: {e:?}", url),
                }
                task::sleep(Duration::from_secs(5)).await;
            }
        }
        Some(Commands::list(List)) => (),
        Some(Commands::inquire(Inquire)) => (),
        None => (),
    }
}

fn main() {
    block_on(async_main());
}