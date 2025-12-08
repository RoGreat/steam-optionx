use log::debug;
use serde::Deserialize;
use serde_value::Value;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
struct LibraryFolders {
    #[serde(flatten)]
    id: BTreeMap<String, Value>,
}

pub fn read_installed_apps(filename: PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let libraryfolders: LibraryFolders = keyvalues_serde::from_str(contents.as_str())?;
    let disk = libraryfolders.id;
    let mut result = vec![];
    for (_, values) in disk.iter() {
        let values = values.clone().deserialize_into::<BTreeMap<String, Value>>();
        for value in values.iter() {
            let apps = value.get("apps").expect("No apps found");
            let apps = apps.clone().deserialize_into::<BTreeMap<String, String>>();
            let apps: Vec<String> = apps?.into_keys().collect();
            for app in apps {
                result.push(app);
            }
        }
    }
    Ok(result)
}

pub fn read_paths(filename: PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let libraryfolders: LibraryFolders = keyvalues_serde::from_str(contents.as_str())?;
    let disk = libraryfolders.id;
    let mut result = vec![];
    for (_, values) in disk.iter() {
        let values = values.clone().deserialize_into::<BTreeMap<String, Value>>();
        for value in values.iter() {
            let path = value.get("path").expect("No paths found");
            let path = path.clone().deserialize_into::<PathBuf>()?;
            result.push(path);
        }
    }
    Ok(result)
}
