use crate::api::put_status;
use colored::Colorize;
use log::{error, info};
use serde_derive::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Deserialize)]
struct Config {
    url: String,
    server_id: u64,
    server_token: String,
    time: u64,
    online: bool,
}

pub async fn config(_path: PathBuf) {
    if _path.exists() {
        ()
    } else {
        panic!(
            "{}\"{}\"",
            "No such file ".red().bold(),
            &_path.to_string_lossy()
        )
    }
    let mut file = match File::open(&_path) {
        Ok(f) => f,
        Err(e) => panic!("No such file {} exception:{}", &_path.to_string_lossy(), e),
    };
    let mut config = String::new();
    file.read_to_string(&mut config).unwrap();
    let _toml: Config = match toml::from_str(&config) {
        Ok(c) => c,
        Err(e) => panic!("Error Reading file: {}", e),
    };
    let url = format!("{}/api/status/put", _toml.url);
    loop {
        let res = put_status(&url, &_toml.server_token, _toml.server_id, _toml.online).await;
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
        std::thread::sleep(Duration::from_secs(_toml.time));
    }
}

pub fn mkconfig(_path: PathBuf) {
    println!(
        "{}",
        "The configuration file resolution function is not yet complete".yellow()
    );
    todo!()
}
