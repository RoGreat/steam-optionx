use reqwest::blocking::get;
use serde::Deserialize;
use serde_value::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct AppList {
    applist: Apps,
}

#[derive(Debug, Deserialize)]
struct Apps {
    apps: Vec<App>,
}

#[derive(Debug, Deserialize)]
struct App {
    appid: u64,
    name: String,
}

pub fn get_game_names() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let request: AppList = get("https://api.steampowered.com/ISteamApps/GetAppList/v2/")?.json()?;
    let apps = request.applist.apps;
    let mut result = HashMap::new();
    for app in apps {
        result.insert(app.appid.to_string(), app.name);
    }
    println!("{:?}", result);
    Ok(result)
}
