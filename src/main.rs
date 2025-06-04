use serde::{Deserialize, Serialize};
use serde_value::Value;
use std::collections::HashMap;

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
    }
}
"##;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct UserLocalConfigStore {
    software: Software,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Software {
    valve: Valve,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Valve {
    steam: Steam,
}

#[derive(Serialize, Deserialize, Debug)]
struct Steam {
    apps: Apps,
}

#[derive(Serialize, Deserialize, Debug)]
struct Apps {
    #[serde(flatten)]
    values: HashMap<String, Value>,
}

fn main() -> keyvalues_serde::Result<()> {
    const OPTION: &str = "LaunchOptions";

    let user_local_config_store: UserLocalConfigStore = keyvalues_serde::from_str(VDF_TEXT)?;
    let apps = user_local_config_store.software.valve.steam.apps.values;
    println!("{:#?}", apps);

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

    println!("Results: {:#?}", results);

    let app = Apps {
        values: HashMap::from([(
            results[0].clone().0,
            serde_value::to_value((OPTION, "BEEPBEEP")).unwrap(),
        )]),
    };

    println!("App: {:#?}", app);

    Ok(())
}
