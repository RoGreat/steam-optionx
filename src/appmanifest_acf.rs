use log::debug;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Clone)]
struct AppState {
    name: String,
}

pub fn read_app_names(
    apps: (Vec<PathBuf>, Vec<Vec<String>>),
) -> Result<BTreeMap<u32, String>, Box<dyn Error>> {
    let mut result = BTreeMap::new();
    for (i, path) in apps.0.iter().enumerate() {
        let mut path = PathBuf::from(path.clone());
        path.push("steamapps");
        for appid in apps.1[i].iter() {
            let filename = "appmanifest_".to_owned() + appid.as_str() + ".acf";
            let mut path = PathBuf::from(path.clone());
            path.push(filename.clone());
            let contents = fs::read_to_string(&path);
            let contents = match contents {
                Ok(contents) => contents,
                Err(_) => {
                    debug!("error reading: {}", &path.display());
                    continue;
                }
            };
            let appstate: AppState = keyvalues_serde::from_str(contents.as_str())?;
            let name = appstate.name;
            result.insert(appid.parse::<u32>()?, name.clone());
        }
    }
    Ok(result)
}
