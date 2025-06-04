use serde::{Deserialize, Serialize};
use serde_value::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

const OPTION: &str = "LaunchOptions";
const KEY: &str = "UserLocalConfigStore";

const VDF_TEXT: &str = r##"
// this file defines the contents of the platform menu
"UserLocalConfigStore"
{
    "Software"
    {
        "Valve"
        {
            "Steam"
            {
                "apps"
                {
                    "1234567890"
                    {
                        "LaunchOptions"   "\"PLACEHOLDER IN QUOTES\""
                    }
                    "0987654321"
                    {
                        "BadgeData"		"000000000000"
                        "LaunchOptions"   "PLACEHOLDER NOT IN QUOTES"
                        "cloud"
						{
							"last_sync_state"		"synchronized"
						}
                    }
                    "2222222222"
                    {
                        "LaunchOptions"   "PLACEHOLDER ALSO NOT IN QUOTES"
                        "cloud"
						{
							"last_sync_state"		"synchronized"
						}
						"BadgeData"		"000000000000"
                    }
                    "1111111111"
                    {
                        "cloud"
						{
							"last_sync_state"		"synchronized"
						}
						"BadgeData"		"000000000000"
                    }
                }
            }
        }
        "Broadcast"
        {
            "Permissions"		"1"
		}
    }
    "Broadcast"
	{
		"Permissions"		"1"
	}
}
"##;

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

fn main() -> keyvalues_serde::Result<()> {
    let config: UserLocalConfigStore = keyvalues_serde::from_str(VDF_TEXT)?;
    let mut vdf = config.clone();

    let apps = config.software.valve.steam.apps.values;

    let mut results: Vec<(String, String)> = Vec::new();

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
                println!();
                println!("App ID: {:#?}", appid);
                println!("Key: {:#?}", key);
                println!("Value: {:#?}", value);
                results.push((appid, value));
            }
        }
    }

    println!();
    println!("Results: {:#?}", results);

    let mut map = HashMap::new();
    let new_value = "BEEPBEEP".to_string();
    map.insert(OPTION.to_string(), &new_value);

    let results = results[0].clone();
    let appid = results.0;
    let old_value = results.1;

    if old_value != new_value {
        println!("{} != {}", old_value, new_value);
    }

    let value = serde_value::to_value(map).unwrap();

    vdf.software.valve.steam.apps.values.insert(appid, value);

    let serialized = keyvalues_serde::to_string_with_key(&vdf, KEY)?;

    let mut file = File::create("test.vdf")?;
    file.write_all(serialized.as_bytes())?;

    Ok(())
}
