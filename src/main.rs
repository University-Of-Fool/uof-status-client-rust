use std::env;
use log::{info, warn, error};
mod api;
use api::get_list;
use api::get_status;
use api::put_server;
use api::drop_server;
use api::put_status;
extern crate log;

#[tokio::main]
fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    match args[1] {
        list => get_list(&args[2]),
        status => get_status(&args[2], id),

    };







    /*     if let Ok(res) = new_server(
        "http://localhost:4044",
        "aaabbbaaa",
        "test",
        "A test server",
    )
    .await
    {
        println!("{:#?}", res);
    }*/
    let res = drop_server("http://117.50.173.86:4044", "aaabbbaaa", 1).await;

    println!("{:#?}", res);
    warn!("test");
}
