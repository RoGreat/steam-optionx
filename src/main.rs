use serde::Deserialize;
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct UserLocalConfigStore {
    software: Software,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Software {
    valve: Valve,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Valve {
    steam: Steam,
}

#[derive(Deserialize, Debug)]
struct Steam {
    apps: Apps,
}

#[derive(Deserialize, Debug)]
struct Apps {
    #[serde(flatten)]
    id: HashMap<String, Value>,
}

fn main() -> keyvalues_serde::Result<()> {
    let user_local_config_store: UserLocalConfigStore = keyvalues_serde::from_str(VDF_TEXT)?;
    let apps = user_local_config_store.software.valve.steam.apps.id;

    for (appid, values) in apps.keys().zip(apps.values()) {
        let values = values.clone().deserialize_into::<HashMap<String, Value>>();
        for (option, value) in &values.unwrap() {
            let value = value.clone().deserialize_into::<String>();
            match value {
                Ok(_) => {}
                Err(_) => continue,
            }
            if option == "LaunchOptions" {
                println!();
                println!("App ID: {:#?}", appid);
                println!("Option: {:#?}", option);
                println!("Value: {:#?}", value.unwrap());
            }
        }
    }

    Ok(())
}
