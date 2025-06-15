use reqwest::blocking::get;
use serde::Deserialize;
use std::collections::BTreeMap;
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

pub fn get_game_names() -> Result<BTreeMap<String, String>, Box<dyn Error>> {
    let mut result = BTreeMap::new();
    let request: AppList = get("https://api.steampowered.com/ISteamApps/GetAppList/v2/")?.json()?;
    let apps = request.applist.apps;
    for app in apps {
        result.insert(app.appid.to_string(), app.name);
    }
    Ok(result)
}
