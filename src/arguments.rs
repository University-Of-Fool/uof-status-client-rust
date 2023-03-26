use clap::{arg, builder::Str, command, Args, Parser, Subcommand};
use std::path::{Path, PathBuf};

///设置参数解析
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
    put(Put),
    drop(Drop),
    status(Status),
    list(List),
    inquire(Inquire),
}
#[derive(Args)]
struct Put {
    #[arg(short = 'u', long = "url", help = "Input a API URL of main server")]
    url: String,
    #[arg(short = 't', long = "token", help = "Input a global_token")]
    token: String,
    #[arg(short = 'n', long = "name", help = "Input a server name")]
    name: String,
    #[arg(short = 'd', long = "description", help = "Input a description")]
    description: String,
}
#[derive(Args)]
struct Drop {
    #[arg(short = 'u', long = "url", help = "Input a API URL of main server")]
    url: String,
    #[arg(short = 't', long = "token", help = "Input a global_token")]
    token: String,
    #[arg(short = 'i', long = "id", help = "Input a server ID")]
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
    #[arg(short = 't', long = "token", help = "Input a user token")]
    token: String,
    #[arg(short = 'i', long = "id", help = "Input a server ID")]
    id: u64,
    #[arg(
        short = 'o',
        long = "online",
        help = "Input a status(\"ture\" of \"false\")"
    )]
    online: bool,
}
#[derive(Args)]
struct List {
    #[arg(short = 'u', long = "url", help = "Input a API URL of main server")]
    url: String,
}
#[derive(Args)]
struct Inquire {
    #[arg(short = 'u', long = "url", help = "Input a API URL of main server")]
    url: String,
    #[arg(short = 'i', long = "id", help = "Input a server ID")]
    id: u64,
}

pub async fn argument() {
    let cli = Cli::parse();
}
