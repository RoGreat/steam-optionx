use crate::consts;
use directories::ProjectDirs;
use serde::Deserialize;
use serde_json;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

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

pub fn app_names(refresh: bool) -> Result<BTreeMap<u32, String>, Box<dyn Error>> {
    let mut cache_dir = cache_dir()?;
    cache_dir.push("applist.json");

    if refresh || !fs::exists(&cache_dir)? {
        let mut resp =
            reqwest::blocking::get("https://api.steampowered.com/ISteamApps/GetAppList/v2/")?;
        let mut buf: Vec<u8> = vec![];
        resp.copy_to(&mut buf)?;
        fs::write(&cache_dir, buf)?;
    };

    let cache = fs::read_to_string(&cache_dir)?;
    let json: AppList = serde_json::from_str(&cache)?;
    let apps = json.applist.apps;

    let mut result = BTreeMap::new();
    for app in apps {
        result.insert(app.appid, app.name);
    }
    Ok(result)
}

fn cache_dir() -> Result<PathBuf, Box<dyn Error>> {
    if let Some(proj_dirs) = ProjectDirs::from("", consts::OWNER_NAME, consts::APP_NAME) {
        let dirs = proj_dirs.cache_dir().to_path_buf();
        fs::create_dir_all(&dirs)?;
        Ok(dirs)
    } else {
        panic!("Error cannot create project cache")
    }
}
