use serde::{Deserialize, Serialize};
use serde_value::Value;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;

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

pub fn read_launch_options(filename: &String) -> Result<BTreeMap<u32, String>, Box<dyn Error>> {
    let mut result = BTreeMap::new();
    let contents = fs::read_to_string(filename)?;
    let config: UserLocalConfigStore = keyvalues_serde::from_str(contents.as_str())?;
    let apps = config.software.valve.steam.apps.values;
    for (appid, values) in apps.iter() {
        let properties = values.clone().deserialize_into::<BTreeMap<String, Value>>();
        let appid = appid.clone();
        if let Some(launch_options) = properties?.get(OPTION) {
            let launch_options = launch_options.clone().deserialize_into::<String>()?;
            result.insert(appid.parse::<u32>()?, launch_options);
        } else {
            result.insert(appid.parse::<u32>()?, String::new());
        }
    }
    Ok(result)
}

pub fn write_launch_options(
    filename: &String,
    all_launch_options: &BTreeMap<u32, String>,
) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let mut config: UserLocalConfigStore = keyvalues_serde::from_str(contents.as_str())?;
    for (appid, launch_options) in all_launch_options.iter() {
        let mut map = BTreeMap::new();
        // Get other values from app
        let values = config
            .software
            .valve
            .steam
            .apps
            .values
            .get(appid.to_string().as_str());
        // Set other values from app
        if let Some(values) = values {
            map = values
                .clone()
                .deserialize_into::<BTreeMap<String, Value>>()?;
        }
        // If new launch options are not empty override them else delete it
        if !launch_options.trim().is_empty() {
            map.insert(OPTION.to_string(), serde_value::to_value(launch_options)?);
        } else {
            map.remove(&OPTION.to_string());
        }
        // Merge new values together
        let value = serde_value::to_value(map)?;
        config
            .software
            .valve
            .steam
            .apps
            .values
            .insert(appid.to_string(), value);
    }
    let serialized = keyvalues_serde::to_string_with_key(&config, KEY)?;
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(filename)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}
