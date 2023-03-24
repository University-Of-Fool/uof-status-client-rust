use std::env;
mod api;
use api::get;
use api::new_server;

#[tokio::main]
async fn main() {
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
    let res = new_server("http://127.0.0.1", "aaabbbaaa", "Test1", "A test server").await;

    println!("{:#?}", res);
}
