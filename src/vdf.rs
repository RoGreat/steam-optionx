use serde::{Deserialize, Serialize};
use serde_value::Value;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;

// const OPTION: &str = "LaunchOptions";
// const KEY: &str = "UserLocalConfigStore";

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

pub fn deserialize(filename: String) -> Result<BTreeMap<u64, String>, Box<dyn Error>> {
    let mut results = BTreeMap::new();
    let contents = fs::read_to_string(filename).unwrap();
    let config: UserLocalConfigStore = keyvalues_serde::from_str(contents.as_str()).unwrap();
    let apps = config.software.valve.steam.apps.values;
    for (appid, values) in apps.keys().zip(apps.values()) {
        let properties = values.clone().deserialize_into::<BTreeMap<String, Value>>();
        let appid = appid.clone();
        if let Some(launch_options) = properties.unwrap_or_default().get("LaunchOptions") {
            let launch_options = launch_options.clone().deserialize_into::<String>().unwrap();
            results.insert(appid.parse::<u64>().unwrap(), launch_options);
        } else {
            results.insert(appid.parse::<u64>().unwrap(), "".to_owned());
        }
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
//fn main() -> keyvalues_serde::Result<()> {
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

//    let mut map = BTreeMap::new();
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

//Ok(())
//}
