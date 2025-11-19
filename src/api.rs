use crate::consts;
use directories::ProjectDirs;
use log::debug;
use serde::Deserialize;
use std::collections::BTreeMap;
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

pub fn app_names(refresh: bool) -> BTreeMap<u32, String> {
    let mut cache_dir = cache_dir().unwrap();
    cache_dir.push("applist.json");

    if refresh || !fs::exists(&cache_dir).unwrap() {
        let api = "https://api.steampowered.com/ISteamApps/GetAppList/v2/";
        let resp = reqwest::blocking::get(api);
        if let Ok(mut resp) = resp {
            debug!("GET {}", api);
            let mut buf: Vec<u8> = vec![];
            let json: Result<AppList, serde_json::Error> = serde_json::from_slice(buf.as_slice());
            if let Ok(_) = json {
                resp.copy_to(&mut buf).expect("Unable to copy to buffer");
                fs::write(&cache_dir, buf).expect("Unable to write to file");
                debug!("write cache: {}", &cache_dir.display());
            } else {
                debug!("bad response reading from cache");
            }
        } else {
            debug!("no response reading from cache");
        }
    };

    let cache = fs::read_to_string(&cache_dir).expect("Unable to read from cache");
    let json: AppList = serde_json::from_str(&cache).expect("Invalid JSON file");
    let apps = json.applist.apps;
    debug!("read cache: {}", &cache_dir.display());

    let mut result = BTreeMap::new();
    for app in apps {
        result.insert(app.appid, app.name);
    }
    result
}

fn cache_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    if let Some(proj_dirs) = ProjectDirs::from("", consts::OWNER_NAME, consts::CODE_NAME) {
        let dirs = proj_dirs.cache_dir().to_path_buf();
        fs::create_dir_all(&dirs)?;
        Ok(dirs)
    } else {
        panic!("Error cannot create project cache")
    }
}
