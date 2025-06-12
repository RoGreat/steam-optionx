use confy;
use directories::BaseDirs;
use rfd;
use serde::{Deserialize, Serialize};
use slint;
use std::default::Default;
use std::env;
use std::path::PathBuf;
use webbrowser;

slint::include_modules!();

#[derive(Debug, Default, Serialize, Deserialize)]
struct Config {
    steam_config: String,
}

fn main() {
    let sox = SteamOptionX::new().unwrap();

    sox.global::<Function>().on_init_file(move || {
        let config: Config = confy::load("steam-optionx", None).unwrap();
        let picked_path = config.steam_config.as_str();
        picked_path.to_string().into()
    });

    sox.global::<Function>()
        .on_link_clicked(move |url| webbrowser::open(url.as_str()).unwrap_or(()));
    sox.global::<Function>()
        .on_reload_file(move |path, file_path| {
            if PathBuf::from(path.as_str()).is_file() {
                path
            } else {
                file_path
            }
        });
    sox.global::<Function>().on_file_dialog(|file_path| {
        rfd::FileDialog::new()
            .add_filter("text", &["vdf"])
            .set_directory(userdata())
            .pick_file()
            .unwrap_or(file_path.to_string().into())
            .to_str()
            .unwrap_or(&file_path)
            .into()
    });

    sox.run().unwrap();
}

fn userdata() -> PathBuf {
    match env::consts::OS {
        "windows" => PathBuf::from(r"C:\Program Files (x86)\Steam\userdata"),
        _ => BaseDirs::new()
            .unwrap()
            .data_dir()
            .to_path_buf()
            .join("Steam/userdata"),
    }
}
