use log::debug;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug, Clone)]
struct AppState {
    name: String,
}

pub fn read_app_names(
    apps: (Vec<PathBuf>, Vec<Vec<String>>),
) -> Result<BTreeMap<u32, String>, Box<dyn Error>> {
    let mut result = BTreeMap::new();
    for (i, base_path) in apps.0.iter().enumerate() {
        let mut steamapps_path = PathBuf::from(base_path.clone());
        steamapps_path.push("steamapps");
        for appid in apps.1[i].iter() {
            let filename = "appmanifest_".to_owned() + appid.as_str() + ".acf";
            let mut acf_path = steamapps_path.clone();
            acf_path.push(filename.clone());
            let contents = fs::read_to_string(&acf_path);
            let contents = match contents {
                Ok(contents) => contents,
                Err(_) => {
                    debug!("error reading: {}", &acf_path.display());
                    continue;
                }
            };
            let appstate: AppState = keyvalues_serde::from_str(contents.as_str())?;
            let name = appstate.name;

            // Construct the common folder path: base_path/steamapps/common/GameName
            let mut common_path = steamapps_path.clone();
            common_path.push("common");
            common_path.push(&name);

            debug!("Checking for .exe in: {}", common_path.display());

            // Check if the game directory contains a Windows executable
            if has_windows_executable(&common_path) {
                result.insert(appid.parse::<u32>()?, name.clone());
            } else {
                debug!(
                    "Skipping {} (no .exe found in common folder: {})",
                    name,
                    common_path.display()
                );
            }
        }
    }
    Ok(result)
}

fn has_windows_executable(path: &Path) -> bool {
    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => return false,
    };

    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            if has_windows_executable(&p) {
                return true;
            }
        } else if p.extension().map_or(false, |ext| ext == "exe") {
            return true;
        }
    }
    false
}
