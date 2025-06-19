#![windows_subsystem = "windows"]

mod api;
mod vdf;

use directories::BaseDirs;
use eframe::egui;
use egui_extras::{Column, TableBuilder};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;
use std::path::PathBuf;
use std::{env, fs};

const APP_NAME: &str = "steam-optionx";

fn main() -> eframe::Result {
    let config: Config = confy::load(APP_NAME, None).unwrap_or_default();
    let picked_path = config.steam_config;
    let app_names = api::app_names().expect("Error getting Steam apps");
    let mut properties = BTreeMap::default();
    let mut apps = None;
    if let Some(path) = &picked_path {
        backup(path, ".orig");
        properties = vdf::read(path).unwrap_or_default();
        apps = Some(get_apps(&properties, &app_names).unwrap_or_default());
    }

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Steam OptionX",
        native_options,
        Box::new(|_cc| {
            Ok(Box::new(EguiApp {
                picked_path: picked_path,
                _properties: properties,
                _app_names: app_names,
                apps: apps,
                ..Default::default()
            }))
        }),
    )
}

fn backup(picked_path: &String, ext: &str) {
    let backup = PathBuf::from(picked_path.clone() + ext);
    match ext {
        ".orig" => {
            if fs::exists(&backup).is_err() {
                _ = fs::copy(PathBuf::from(picked_path), backup)
            }
        }
        ".bak" => _ = fs::copy(PathBuf::from(picked_path), backup),
        _ => panic!(),
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Config {
    steam_config: Option<String>,
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

struct App {
    name: String,
    launch_options: String,
}

fn get_apps(
    properties: &BTreeMap<u64, String>,
    app_names: &BTreeMap<u64, String>,
) -> Result<BTreeMap<u64, App>, Box<dyn Error>> {
    let mut result = BTreeMap::new();
    let appids: Vec<u64> = properties.clone().into_keys().collect();
    for appid in appids {
        if let Some(app_name) = app_names.get(&appid) {
            let properties = properties.clone();
            let launch_options = properties.get(&appid).unwrap_or(&String::new()).to_string();
            let game = App {
                name: app_name.clone(),
                launch_options: launch_options,
            };
            result.insert(appid, game);
        }
    }
    Ok(result)
}

#[derive(Default)]
struct EguiApp {
    picked_path: Option<String>,
    _properties: BTreeMap<u64, String>,
    _app_names: BTreeMap<u64, String>,
    apps: Option<BTreeMap<u64, App>>,
    all_launch_options: BTreeMap<u64, String>,
    default_launch_options: String,
    filter_apps: String,
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("Find file:");
                ui.monospace("Steam/userdata/XXXXXXXX/config/localconfig.vdf");
            });

            ui.horizontal_wrapped(|ui| {
                if ui.button("Open file…").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("text", &["vdf"])
                        .set_directory(userdata())
                        .pick_file()
                    {
                        self.picked_path = Some(path.display().to_string());
                        if let Some(picked_path) = &self.picked_path {
                            let config = Config {
                                steam_config: Some(picked_path.clone()),
                            };
                            confy::store(APP_NAME, None, config).unwrap_or_default();
                            self._properties = vdf::read(picked_path).unwrap_or_default();
                            self.apps = Some(
                                get_apps(&self._properties, &self._app_names).unwrap_or_default(),
                            );
                            backup(picked_path, ".orig");
                        }
                    }
                }
                if let Some(picked_path) = &self.picked_path {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                }
            });

            if let Some(picked_path) = &self.picked_path {
                ui.separator();

                ui.horizontal_wrapped(|ui| {
                    if ui.button("Save").clicked() {
                        println!("Saving `{}`...", picked_path);
                        if !self.default_launch_options.trim().is_empty() {
                            for launch_options in self.all_launch_options.values_mut() {
                                if launch_options.is_empty() {
                                    *launch_options = self.default_launch_options.clone();
                                }
                            }
                            self.default_launch_options.clear();
                        }
                        backup(picked_path, ".bak");
                        _ = vdf::write(picked_path, &self.all_launch_options);
                        println!("Saved `{}`", picked_path);
                    }
                    if ui.button("Clear").clicked() {
                        for launch_options in self.all_launch_options.values_mut() {
                            launch_options.clear();
                        }
                    }
                    if ui.button("Restore").clicked() {
                        if let Some(apps) = &self.apps {
                            for (appid, properties) in apps.keys().zip(apps.values()) {
                                let current_launch_options = &properties.launch_options;
                                _ = self
                                    .all_launch_options
                                    .insert(*appid, current_launch_options.clone());
                            }
                        }
                    }
                    ui.label("Set default launch options:");
                    ui.add_sized(
                        ui.available_size_before_wrap(),
                        egui::TextEdit::singleline(&mut self.default_launch_options),
                    );
                });

                ui.horizontal_wrapped(|ui| {
                    ui.label("Filter apps:");
                    ui.add_sized(
                        ui.available_size_before_wrap(),
                        egui::TextEdit::singleline(&mut self.filter_apps),
                    );
                });

                TableBuilder::new(ui)
                    .resizable(true)
                    .column(Column::auto().at_least(150.0))
                    .column(Column::remainder())
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.heading("Apps");
                        });
                        header.col(|ui| {
                            ui.heading("Launch Options");
                        });
                    })
                    .body(|mut body| {
                        body.row(0.0, |mut row| {
                            row.col(|ui| {
                                if let Some(apps) = &self.apps {
                                    for (appid, properties) in apps.keys().zip(apps.values()) {
                                        let app_name = &properties.name;

                                        if self.filter_apps.is_empty()
                                            || properties
                                                .name
                                                .to_lowercase()
                                                .trim()
                                                .to_lowercase()
                                                .contains(&self.filter_apps)
                                        {
                                            ui.style_mut().wrap_mode =
                                                Some(egui::TextWrapMode::Truncate);
                                            ui.add_sized(
                                                [ui.available_width(), 20.0],
                                                egui::Hyperlink::from_label_and_url(
                                                    app_name,
                                                    "https://store.steampowered.com/app/"
                                                        .to_owned()
                                                        + &appid.to_string(),
                                                ),
                                            );
                                        }
                                    }
                                }
                            });
                            row.col(|ui| {
                                if let Some(apps) = &self.apps {
                                    for (appid, properties) in apps.keys().zip(apps.values()) {
                                        let mut current_launch_options =
                                            properties.launch_options.clone();
                                        match self.all_launch_options.get(&appid) {
                                            Some(launch_options) => {
                                                current_launch_options = launch_options.clone()
                                            }
                                            None => {
                                                _ = self
                                                    .all_launch_options
                                                    .insert(*appid, current_launch_options.clone());
                                            }
                                        }

                                        if self.filter_apps.is_empty()
                                            || properties
                                                .name
                                                .to_lowercase()
                                                .trim()
                                                .to_lowercase()
                                                .contains(&self.filter_apps)
                                        {
                                            ui.style_mut().wrap_mode =
                                                Some(egui::TextWrapMode::Truncate);
                                            let response = ui.add_sized(
                                                [ui.available_width() - 20.0, 20.0],
                                                egui::TextEdit::singleline(
                                                    &mut current_launch_options,
                                                ),
                                            );
                                            if response.changed() {
                                                self.all_launch_options
                                                    .insert(*appid, current_launch_options);
                                            }
                                        }
                                    }
                                }
                            });
                        });
                    });
            }
        });
    }
}
