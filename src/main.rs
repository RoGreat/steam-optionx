use serde::{Deserialize, Serialize};
use serde_value::Value;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;

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

// Need to list all App IDs and put it into a table
// Later will use an API to figure out app names
//
// Inputs:
// Filepath of vdf
// Selected App ID
// Global - envs, cmds, args
// PerGame - envs, cmds, args
fn main() -> keyvalues_serde::Result<()> {
    let mut results: HashMap<String, String> = HashMap::new();

    let contents = fs::read_to_string("localconfig.vdf")?;
    let config: UserLocalConfigStore = keyvalues_serde::from_str(contents.as_str())?;
    let mut vdf = config.clone();

    let apps = config.software.valve.steam.apps.values;
    for (appid, values) in apps.keys().zip(apps.values()) {
        let values = values.clone().deserialize_into::<HashMap<String, Value>>();
        for (key, value) in &values.unwrap() {
            let value = value.clone().deserialize_into::<String>();
            match value {
                Ok(_) => {}
                Err(_) => continue,
            }
            if key == OPTION {
                let value = value.unwrap();
                let appid = appid.to_string();
                results.insert(appid, value);
            }
        }
    }

    println!("App IDs: {:?}", results.keys());

    let appid = "3205720";
    let old_value = results.get(appid).map_or("", |v| v);
    let new_value = "BEEPBEEP".to_string();

    if *old_value != new_value {
        println!("App ID: {}", appid);
        println!("Check: {} != {}", old_value, new_value);

        let mut map = HashMap::new();
        map.insert(OPTION.to_string(), new_value);

        let value = serde_value::to_value(map).unwrap();

        vdf.software
            .valve
            .steam
            .apps
            .values
            .insert(appid.to_string(), value);

        let serialized = keyvalues_serde::to_string_with_key(&vdf, KEY)?;

        let mut file = File::create("test.vdf")?;
        file.write_all(serialized.as_bytes())?;
    }

    Ok(())
}
