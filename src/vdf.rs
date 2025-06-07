use serde::{Deserialize, Serialize};
use serde_value::Value;
use std::collections::HashMap;
use std::error;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

const OPTION: &str = "LaunchOptions";
const KEY: &str = "UserLocalConfigStore";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
struct UserLocalConfigStore {
    software: Software,
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
struct Software {
    valve: Valve,
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
struct Valve {
    steam: Steam,
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Steam {
    apps: Apps,
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Apps {
    #[serde(flatten)]
    values: HashMap<String, Value>,
}

pub fn appids(filename: String) -> Result<Vec<String>, Box<dyn error::Error>> {
    let mut results = vec![];
    let contents = fs::read_to_string(filename)?;
    let config: UserLocalConfigStore = keyvalues_serde::from_str(contents.as_str())?;
    let apps = config.software.valve.steam.apps.values;
    for appid in apps.keys() {
        results.push(appid.clone());
    }
    Ok(results)
}

// Need to list all App IDs and put it into a table
// Later will use an API to figure out app names
//
// Inputs:
// Filepath of vdf
// Selected App ID
// Global launch options
// - PerGame override
// Don't overcomplicate it!
fn main() -> keyvalues_serde::Result<()> {
    // Inputs
    //let option = "";
    //let global = "gamemoderun %command%";
    //let appid = "3205720";
    //let filename = "localconfig.vdf";

    //let results = deserialize(filename);

    //let old_value = results.get(appid).map_or("", |v| v);
    //let new_value;

    //if option.is_empty() {
    //    if global.is_empty() {
    //        new_value = old_value;
    //    } else {
    //        new_value = global;
    //    }
    //} else {
    //    new_value = option;
    //}

    //if *old_value != *new_value {
    //    println!("App ID: {}", appid);
    //    println!("Check: {} != {}", old_value, new_value);

    //    let mut map = HashMap::new();
    //    map.insert(OPTION.to_string(), new_value);
    //    let value = serde_value::to_value(map).unwrap();
    //    new_config
    //        .software
    //        .valve
    //        .steam
    //        .apps
    //        .values
    //        .insert(appid.to_string(), value);

    //    let serialized = keyvalues_serde::to_string_with_key(&new_config, KEY)?;
    //    let mut file = File::create("test.vdf")?;
    //    file.write_all(serialized.as_bytes())?;
    //}

    Ok(())
}
