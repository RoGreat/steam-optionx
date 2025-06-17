use serde::{Deserialize, Serialize};
use serde_value::Value;
use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

const OPTION: &str = "LaunchOptions";
const KEY: &str = "UserLocalConfigStore";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
struct UserLocalConfigStore {
    software: Software,
    #[serde(flatten)]
    other: BTreeMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
struct Software {
    valve: Valve,
    #[serde(flatten)]
    other: BTreeMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
struct Valve {
    steam: Steam,
    #[serde(flatten)]
    other: BTreeMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Steam {
    apps: Apps,
    #[serde(flatten)]
    other: BTreeMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Apps {
    #[serde(flatten)]
    values: BTreeMap<String, Value>,
}

pub fn read(filename: String) -> Result<BTreeMap<u64, String>, Box<dyn Error>> {
    let mut results = BTreeMap::new();
    let contents = fs::read_to_string(filename)?;
    let config: UserLocalConfigStore = keyvalues_serde::from_str(contents.as_str())?;
    let apps = config.software.valve.steam.apps.values;
    for (appid, values) in apps.keys().zip(apps.values()) {
        let properties = values.clone().deserialize_into::<BTreeMap<String, Value>>();
        let appid = appid.clone();
        if let Some(launch_options) = properties?.get(OPTION) {
            let launch_options = launch_options.clone().deserialize_into::<String>()?;
            results.insert(appid.parse::<u64>()?, launch_options);
        } else {
            results.insert(appid.parse::<u64>()?, String::new());
        }
    }
    Ok(results)
}

pub fn write(
    filename: String,
    all_launch_options: BTreeMap<u64, String>,
) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let mut config: UserLocalConfigStore = keyvalues_serde::from_str(contents.as_str())?;
    for (appid, launch_options) in all_launch_options.iter() {
        let mut map = BTreeMap::new();
        if !launch_options.trim().is_empty() {
            map.insert(OPTION, launch_options);
        }
        let value = serde_value::to_value(map)?;
        config
            .software
            .valve
            .steam
            .apps
            .values
            .insert(appid.to_string(), value);
        let serialized = keyvalues_serde::to_string_with_key(&config, KEY)?;
        let mut file = File::create("test.vdf")?;
        if env::consts::OS != "windows" {
            let mut permissions = file.metadata()?.permissions();
            permissions.set_mode(permissions.mode() | 0o755);
            file.set_permissions(permissions)?;
        }
        file.write_all(serialized.as_bytes())?;
    }
    Ok(())
}
