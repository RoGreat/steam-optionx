use log::warn;
use serde::{Deserialize, Serialize};
use serde_value::Value;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
struct InstallConfigStore {
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
#[serde(rename_all = "PascalCase")]
struct Steam {
    compat_tool_mapping: CompatToolMapping,
    #[serde(flatten)]
    other: BTreeMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CompatToolMapping {
    #[serde(flatten)]
    values: BTreeMap<String, Value>,
}

pub fn read_compat_tools(filename: &String) -> Result<BTreeMap<u32, String>, Box<dyn Error>> {
    let mut result = BTreeMap::new();
    let contents = fs::read_to_string(filename)?;
    let config: InstallConfigStore = keyvalues_serde::from_str(contents.as_str())?;
    let apps = config.software.valve.steam.compat_tool_mapping.values;
    for (appid, values) in apps.iter() {
        let properties = values.clone().deserialize_into::<BTreeMap<String, Value>>();
        let appid_parsed = appid.clone().parse::<u32>();
        if appid_parsed.is_err() {
            warn!(
                "read_compat_tools appid error: {}",
                appid_parsed.unwrap_err()
            );
            continue;
        }
        if let Some(name) = properties?.get("name") {
            let compat = name.clone().deserialize_into::<String>()?;
            result.insert(appid_parsed?, compat);
        } else {
            result.insert(appid_parsed?, String::new());
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_compat_tools() {
        // The test file must contain entries; this is a placeholder.
        let map = read_compat_tools(&"./tests/config.vdf".to_string()).unwrap();
        assert_eq!(map.get(&1111111).unwrap(), "proton_8");
        assert_eq!(map.get(&9999999).unwrap(), "proton_9");
    }
}
