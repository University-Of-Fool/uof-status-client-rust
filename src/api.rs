use reqwest::header::HeaderMap;
use serde_json::value::Value;
use std::collections::HashMap;

pub async fn get(url: &str) -> Result<HashMap<String, String>, reqwest::Error> {
    Ok(reqwest::get(url)
        .await?
        .json::<HashMap<String, String>>()
        .await?)
}

//创建服务器
pub async fn new_server(
    url: &str,
    token: &str,
    name: &str,
    description: &str,
) -> Result<HashMap<String, Value>, reqwest::Error> {
    let client = reqwest::Client::new();
    let urls: String = format!("{}/api/server/put", url);
    let tokens: String = format!("\"{}\"", token);
    let names: String = format!("\"{}\"", name);
    let descriptions: String = format!("\"{}\"", description);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let mut data = HashMap::new();
    data.insert("token", tokens);
    data.insert("name", names);
    data.insert("description", descriptions);

    Ok(client
        .post(urls)
        .headers(headers)
        .json(&data)
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await?)
}
