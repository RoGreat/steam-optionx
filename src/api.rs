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
    appid: u32,
    name: String,
}

pub fn app_names() -> Result<BTreeMap<u32, String>, Box<dyn Error>> {
    let mut result = BTreeMap::new();
    let request: AppList =
        reqwest::blocking::get("https://api.steampowered.com/ISteamApps/GetAppList/v2/")?.json()?;
    let apps = request.applist.apps;
    for app in apps {
        result.insert(app.appid, app.name);
    }
    Ok(result)
}
