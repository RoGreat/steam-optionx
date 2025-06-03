use serde::Deserialize;
use serde_value::Value;
//use std::collections::BTreeMap as Map;
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
                        "LaunchOptions"   "PLACEHOLDER NOT IN QUOTES"
                        "cloud"
						{
							"last_sync_state"		"synchronized"
						}
                    }
                    "1111111111"
                    {
                        "cloud"
						{
							"last_sync_state"		"synchronized"
						}
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
    let appid = user_local_config_store.software.valve.steam.apps.id;

    for x in appid.keys() {
        println!("{:#?}", x);
    }

    for x in appid.values() {
        println!("{:#?}", &x);
        let y = x.clone().deserialize_into::<HashMap<String, String>>();
        println!("{:#?}", &y);
        for z in y.unwrap().iter() {
            if z.0 == "LaunchOptions" {
                println!("{:#?}", z.1);
            }
        }
    }

    Ok(())
}
