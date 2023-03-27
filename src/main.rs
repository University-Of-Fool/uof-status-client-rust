use async_std::task;
use log::{error, info, warn};
use std::time::Duration;
use std::{env, str::FromStr};
mod api;
mod arguments;
mod config;
use api::{drop_server, get_list, get_status, put_server, put_status};
use arguments::argument;
extern crate clap;
extern crate log;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let url = "http://127.0.0.1";
    let token = "an1l7e3yk84p9bsc";
    let id = 5;
    let online = true;
    loop {
        let res = put_status(url, token, id, online).await;
        match res {
            Ok(v) => info!("Uploading status to {},Succeed: {v:?}", url),
            Err(e) => error!("Uploading statusto {},Fail: {e:?}", url),
        }
        task::sleep(Duration::from_secs(5)).await;
    }
}
