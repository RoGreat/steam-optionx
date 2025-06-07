use reqwest::blocking::get;
use serde::Deserialize;
use serde_value::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct AppList {
    apps: Vec<Apps>,
    //#[serde(flatten)]
    //values: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
struct Apps {
    appid: u64,
    name: String,
}

pub fn get_game_names() -> Result<(), Box<dyn std::error::Error>> {
    let request: AppList = get("https://api.steampowered.com/ISteamApps/GetAppList/v2/")?.json()?;
    println!("{:?}", request);
    //for appid in request.values.values() {
    //    println!("{:?}", appid);
    //}
    Ok(())
}
