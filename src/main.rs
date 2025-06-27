mod api;
mod vdf;

use directories::BaseDirs;
use eframe::egui;
use egui_extras::{Column, TableBuilder};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const CONFIG_NAME: &str = "steam-optionx";

#[derive(Debug, Default, Serialize, Deserialize)]
struct Config {
    steam_config: Option<String>,
    default_launch_options: String,
    app_sort: Option<String>,
}

struct App {
    name: String,
    launch_options: String,
}

#[derive(Default, PartialEq, Clone)]
enum AppSort {
    #[default]
    IdAscending,
    IdDescending,
    NameAscending,
    NameDescending,
}

impl AppSort {
    fn as_str(&self) -> &'static str {
        match self {
            AppSort::IdAscending => "App ID Ascending",
            AppSort::IdDescending => "App ID Descending",
            AppSort::NameAscending => "App Name Ascending",
            AppSort::NameDescending => "App Name Descending",
        }
    }

    fn from(value: &str) -> Self {
        match value {
            "App ID Ascending" => AppSort::IdAscending,
            "App ID Descending" => AppSort::IdDescending,
            "App Name Ascending" => AppSort::NameAscending,
            "App Name Descending" => AppSort::NameDescending,
            _ => AppSort::IdAscending,
        }
    }
}

#[derive(Default)]
struct EguiApp {
    steam_config: Option<String>,
    app_names: BTreeMap<u32, String>,
    apps: Option<BTreeMap<u32, App>>,
    all_launch_options: BTreeMap<u32, String>,
    default_launch_options: String,
    filter_apps: String,
    app_sort: AppSort,
}

fn main() -> eframe::Result {
    let app_names = api::app_names().expect("Error getting Steam apps from Steam API");

    let config: Config = confy::load(CONFIG_NAME, None).unwrap_or_default();

    let steam_config = config.steam_config;
    let apps = if let Some(path) = &steam_config {
        backup_file(path, ".orig");
        let properties = vdf::read(path).unwrap_or(BTreeMap::default());
        Some(get_apps(&properties, &app_names).unwrap_or_default())
    } else {
        None
    };

    let default_launch_options = config.default_launch_options;

    let app_sort = config.app_sort;
    let app_sort = if let Some(sort) = &app_sort {
        AppSort::from(sort.as_str())
    } else {
        AppSort::default()
    };

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Steam OptionX",
        native_options,
        Box::new(|_cc| {
            Ok(Box::new(EguiApp {
                steam_config: steam_config,
                app_names: app_names,
                apps: apps,
                default_launch_options: default_launch_options,
                app_sort: app_sort,
                ..Default::default()
            }))
        }),
    )
}

fn backup_file(picked_path: &String, ext: &str) {
    let backup_path = PathBuf::from(picked_path.clone() + ext);
    match ext {
        ".orig" => {
            if !backup_path.is_file() {
                _ = fs::copy(PathBuf::from(picked_path), backup_path)
            }
        }
        _ => _ = fs::copy(PathBuf::from(picked_path), backup_path),
    }
}

fn userdata_dir() -> PathBuf {
    if cfg!(windows) {
        PathBuf::from(r"C:\Program Files (x86)\Steam\userdata")
    } else if let Some(home_dir) = BaseDirs::new() {
        home_dir.data_dir().to_path_buf().join("Steam/userdata")
    } else {
        PathBuf::new()
    }
}

fn get_apps(
    properties: &BTreeMap<u32, String>,
    app_names: &BTreeMap<u32, String>,
) -> Result<BTreeMap<u32, App>, Box<dyn Error>> {
    let mut apps = BTreeMap::new();
    let appids: Vec<u32> = properties.clone().into_keys().collect();
    for appid in appids {
        if let Some(app_name) = app_names.get(&appid) {
            let launch_options = properties.get(&appid).unwrap_or(&String::new()).clone();
            let game = App {
                name: app_name.clone(),
                launch_options: launch_options,
            };
            apps.insert(appid, game);
        }
    }
    Ok(apps)
}

fn is_filtered(filter: &String, app_name: &String) -> bool {
    filter.is_empty()
        || app_name
            .to_lowercase()
            .contains(&filter.trim().to_lowercase())
}

fn sort_apps(sort: AppSort, apps: &BTreeMap<u32, App>) -> Vec<(&u32, &App)> {
    match sort {
        AppSort::IdAscending => apps.into_iter().collect(),
        AppSort::IdDescending => apps.into_iter().rev().collect(),
        AppSort::NameAscending => {
            let mut v = apps.into_iter().collect::<Vec<(&u32, &App)>>();
            v.sort_by(|a, b| a.1.name.to_lowercase().cmp(&b.1.name.to_lowercase()));
            v
        }
        AppSort::NameDescending => {
            let mut v = apps.into_iter().collect::<Vec<(&u32, &App)>>();
            v.sort_by(|a, b| b.1.name.to_lowercase().cmp(&a.1.name.to_lowercase()));
            v
        }
    }
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
                        .set_directory(userdata_dir())
                        .pick_file()
                    {
                        self.steam_config = Some(path.to_str().unwrap_or_default().to_owned());
                        if let Some(picked_path) = &self.steam_config {
                            let config: Config = confy::load(CONFIG_NAME, None).unwrap_or_default();
                            let config = Config {
                                steam_config: Some(picked_path.clone()),
                                default_launch_options: config.default_launch_options,
                                app_sort: config.app_sort,
                            };
                            confy::store(CONFIG_NAME, None, config).unwrap_or_default();
                            let properties = vdf::read(picked_path).unwrap_or_default();
                            self.apps =
                                Some(get_apps(&properties, &self.app_names).unwrap_or_default());
                            backup_file(picked_path, ".orig");
                        }
                    }
                }

                if let Some(picked_path) = &self.steam_config {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                }
            });

            if let Some(picked_path) = &self.steam_config {
                ui.separator();

                ui.horizontal_wrapped(|ui| {
                    let response = ui.button("Save");
                    let popup_id = ui.make_persistent_id("Save");
                    if response.clicked() {
                        let config: Config = confy::load(CONFIG_NAME, None).unwrap_or_default();
                        let config = Config {
                            steam_config: config.steam_config,
                            default_launch_options: self.default_launch_options.clone(),
                            app_sort: Some(self.app_sort.as_str().to_string()),
                        };
                        confy::store(CONFIG_NAME, None, config).unwrap_or_default();
                        if !self.default_launch_options.trim().is_empty() {
                            for launch_options in self.all_launch_options.values_mut() {
                                if launch_options.is_empty() {
                                    *launch_options = self.default_launch_options.clone();
                                }
                            }
                        }
                        backup_file(picked_path, ".bak");
                        _ = vdf::write(picked_path, &self.all_launch_options);
                        ui.memory_mut(|mem| mem.open_popup(popup_id));
                    };
                    egui::popup_above_or_below_widget(
                        ui,
                        popup_id,
                        &response,
                        egui::AboveOrBelow::Above,
                        egui::popup::PopupCloseBehavior::CloseOnClick,
                        |ui| {
                            ui.set_min_width(100.0);
                            ui.label("Configs saved");
                        },
                    );

                    if ui.button("Clear").clicked() {
                        for launch_options in self.all_launch_options.values_mut() {
                            launch_options.clear();
                        }
                    }

                    if ui.button("Restore").clicked() {
                        if let Some(apps) = &self.apps {
                            for (appid, properties) in apps.iter() {
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

                ui.separator();

                ui.horizontal_wrapped(|ui| {
                    egui::ComboBox::from_id_salt("AppSort")
                        .selected_text(format!("{}", &self.app_sort.as_str()))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.app_sort,
                                AppSort::IdAscending,
                                AppSort::IdAscending.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.app_sort,
                                AppSort::IdDescending,
                                AppSort::IdDescending.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.app_sort,
                                AppSort::NameAscending,
                                AppSort::NameAscending.as_str(),
                            );
                            ui.selectable_value(
                                &mut self.app_sort,
                                AppSort::NameDescending,
                                AppSort::NameDescending.as_str(),
                            );
                        });

                    ui.label("Filter apps:");
                    ui.add_sized(
                        ui.available_size_before_wrap(),
                        egui::TextEdit::singleline(&mut self.filter_apps),
                    );
                });

                ui.separator();

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
                                    let sorted_apps = sort_apps(self.app_sort.clone(), apps);
                                    for (appid, properties) in sorted_apps.into_iter() {
                                        if is_filtered(&self.filter_apps, &properties.name) {
                                            ui.style_mut().wrap_mode =
                                                Some(egui::TextWrapMode::Truncate);
                                            ui.add_sized(
                                                [ui.available_width(), 20.0],
                                                egui::Hyperlink::from_label_and_url(
                                                    &properties.name,
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
                                    let sorted_apps = sort_apps(self.app_sort.clone(), apps);
                                    for (appid, properties) in sorted_apps.into_iter() {
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

                                        if is_filtered(&self.filter_apps, &properties.name) {
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
