use reqwest::blocking::get;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

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

pub fn game_names() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut result = HashMap::new();
    let request: AppList = get("https://api.steampowered.com/ISteamApps/GetAppList/v2/")?.json()?;
    let apps = request.applist.apps;
    for app in apps {
        result.insert(app.appid.to_string(), app.name);
    }
    Ok(result)
}
