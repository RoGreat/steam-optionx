use confy;
use directories::BaseDirs;
use rfd;
use serde::{Deserialize, Serialize};
use slint::{self, SharedString};
use std::cell::RefCell;
use std::default::Default;
use std::env;
use std::path::PathBuf;
use std::rc::Rc;
use webbrowser;

slint::include_modules!();

fn main() {
    let sox = SteamOptionX::new().unwrap();

    #[derive(Debug, Default, Serialize, Deserialize)]
    struct Config {
        steam_config: String,
    }
    let config = Rc::new(RefCell::new(Config::default()));
    sox.on_init_file({
        let picked_path = confy::load("steam-optionx", None)
            .unwrap_or(Config::default())
            .steam_config;
        move || picked_path.clone().into()
    });

    sox.global::<LinkHandler>()
        .on_link_clicked(move |url| webbrowser::open(url.as_str()).unwrap_or(()));
    sox.on_reload_file({
        let config = config.clone();
        move |path, file_path| {
            if PathBuf::from(path.as_str()).is_file() {
                config.borrow_mut().steam_config = path.to_string();
                let _ = confy::store("steam-optionx", None, config.take());
                path
            } else {
                config.borrow_mut().steam_config = file_path.to_string();
                let _ = confy::store("steam-optionx", None, config.take());
                file_path
            }
        }
    });
    sox.on_file_dialog({
        let config = config.clone();
        move |file_path| {
            let path: PathBuf = rfd::FileDialog::new()
                .add_filter("text", &["vdf"])
                .set_directory(userdata())
                .pick_file()
                .unwrap_or(PathBuf::from(file_path.as_str()));
            let path: &str = path.to_str().unwrap_or(file_path.as_str());
            config.borrow_mut().steam_config = path.to_string();
            let _ = confy::store("steam-optionx", None, config.take());
            SharedString::from(path)
        }
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
