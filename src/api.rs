use reqwest::header::HeaderMap;
use serde_json::{value::Value, json};
use std::{collections::HashMap, fmt::format};

//获取服务器列表
pub async fn get_list(url: &str) -> Result<HashMap<String, String>, reqwest::Error> {
    let urls = format!("{}/api/server/get", url);
    Ok(reqwest::get(urls)
        .await?
        .json::<HashMap<String, String>>()
        .await?)
}

//获取状态
pub async fn get_status(url: &str, id: u64) -> Result<HashMap<String, String>, reqwest::Error> {
    let urls = format!("{}/api/status/get/{}", url, id);
    Ok(reqwest::get(urls)
        .await?
        .json::<HashMap<String, String>>()
        .await?)
}

//创建服务器
pub async fn put_server(
    url: &str,
    token: &str,
    name: &str,
    description: &str,
) -> Result<HashMap<String, Value>, reqwest::Error> {
    let client = reqwest::Client::new();
    let urls: String = format!("{}/api/server/put", url);
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let mut data = HashMap::new();
    data.insert("token", token);
    data.insert("name", name);
    data.insert("description", description);

    Ok(client
        .post(urls)
        .headers(headers)
        .json(&data)
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await?)
}

//删除服务器
pub async fn drop_server(
    url: &str,
    token: &str,
    id: u64
) -> Result<HashMap<String, Value>, reqwest::Error> {
    let client = reqwest::Client::new();
    let urls: String = format!("{}/api/server/drop", url);
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let data = json!({
        "token": token,
        "serverId": id
    });

    Ok(client
        .post(urls)
        .headers(headers)
        .json(&data)
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await?)
}

//上传状态
pub async fn put_status(
    url: &str,
    token: &str,
    id: u64,
    online: bool
) -> Result<HashMap<String, Value>, reqwest::Error> {
    let client = reqwest::Client::new();
    let urls: String = format!("{}/api/status/put", url);
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let data = json!({
        "token": token,
        "serverId": id,
        "online": online
    });

    Ok(client
        .post(urls)
        .headers(headers)
        .json(&data)
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await?)
}
