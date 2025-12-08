use log::debug;
use serde::Deserialize;
use serde_value::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Clone)]
struct AppState {
    #[serde(flatten)]
    id: BTreeMap<String, Value>,
}

pub fn read_app_names(appids: Vec<String>, paths: Vec<PathBuf>) {
    // -> Result<BTreeMap<u32, String>, Box<dyn Error>> {
    for path in paths {
        let mut path = PathBuf::from(path.clone());
        path.push("steamapps");
        for appid in &appids {
            let filename = "appmanifest_".to_owned() + appid.as_str() + ".acf";
            let mut path = PathBuf::from(path.clone());
            path.push(filename);
            debug!("{:?}", path);
            fs::read_to_string(path).unwrap();
        }
    }
    // let contents = fs::read_to_string()?;
    // let libraryfolders: LibraryFolders = keyvalues_serde::from_str(contents.as_str())?;
    // let disk = libraryfolders.id;
    // let mut result = vec![];
    // for (_, values) in disk.iter() {
    //     let values = values.clone().deserialize_into::<BTreeMap<String, Value>>();
    //     for value in values.iter() {
    //         let path = value.get("path").expect("No paths found");
    //         let path = path.clone().deserialize_into::<PathBuf>()?;
    //         result.push(path);
    //     }
    // }
    // Ok(result)
}
