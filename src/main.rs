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

    loop {
        let res = put_status("http://127.0.0.1", "an1l7e3yk84p9bsc", 5, true).await;
        info!("Uploading status.");
        argument();
        match res {
            Ok(v) => info!("Succeed: {v:?}"),
            Err(e) => error!("Fail: {e:?}"),
        }
        task::sleep(Duration::from_secs(5)).await;
    }
}
