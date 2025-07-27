#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod consts;
mod libraryfolders_vdf;
mod localconfig_vdf;

use directories::BaseDirs;
use eframe::egui;
use egui_extras::{Column, TableBuilder};
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Debug, Default, Serialize, Deserialize)]
struct Config {
    steam_config: Option<String>,
    default_launch_options: Option<String>,
    app_sort: Option<String>,
    protondb: Option<bool>,
}

struct App {
    name: String,
    launch_options: String,
}

#[derive(Default, PartialEq, Clone, Display, EnumString)]
enum AppSort {
    #[default]
    #[strum(serialize = "⬆ App ID")]
    IdAscending,
    #[strum(serialize = "⬇ App ID")]
    IdDescending,
    #[strum(serialize = "⬆ App Name")]
    NameAscending,
    #[strum(serialize = "⬇ App Name")]
    NameDescending,
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
    protondb: bool,
    url: String,
}

fn main() -> eframe::Result {
    env_logger::init();

    let refresh = false;
    let app_names = api::app_names(refresh).expect("Error getting Steam apps from Steam API");

    let config: Config = confy::load(consts::CODE_NAME, None).unwrap_or_default();
    debug!("{} config loaded", consts::CODE_NAME);

    let steam_config = config.steam_config;
    let apps = if let Some(localconfig_vdf_path) = &steam_config {
        debug!("steam_config: {}", localconfig_vdf_path);
        update_apps(localconfig_vdf_path, &app_names)
    } else {
        None
    };

    let default_launch_options = config.default_launch_options.unwrap_or_default();
    debug!("default_launch_options: {}", default_launch_options);

    let app_sort = config.app_sort;
    let app_sort = if let Some(sort) = &app_sort {
        AppSort::from_str(sort.as_str()).unwrap_or_default()
    } else {
        AppSort::default()
    };
    debug!("app_sort: {}", app_sort);

    let protondb = config.protondb.unwrap_or_default();
    debug!("protondb: {}", protondb);
    let url = if protondb {
        "https://www.protondb.com/app/".to_string()
    } else {
        "https://store.steampowered.com/app/".to_string()
    };

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_icon(
            eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon.png")[..])
                .expect("Failed to load icon"),
        ),
        ..Default::default()
    };
    eframe::run_native(
        consts::APP_NAME,
        native_options,
        Box::new(|_cc| {
            Ok(Box::new(EguiApp {
                steam_config: steam_config,
                app_names: app_names,
                apps: apps,
                default_launch_options: default_launch_options,
                app_sort: app_sort,
                protondb: protondb,
                url: url,
                ..Default::default()
            }))
        }),
    )
}

fn update_apps(
    localconfig_vdf_path: &String,
    app_names: &BTreeMap<u32, String>,
) -> Option<BTreeMap<u32, App>> {
    backup_file(localconfig_vdf_path, ".orig").expect("Error backup failed");
    let libraryfolders_vdf_path = config_dir(localconfig_vdf_path);
    let properties =
        localconfig_vdf::read_launch_options(localconfig_vdf_path).unwrap_or(BTreeMap::default());
    let appids = libraryfolders_vdf::read_installed_apps(
        &libraryfolders_vdf_path
            .to_str()
            .unwrap_or_default()
            .to_owned(),
    )
    .unwrap_or(properties.keys().map(|appid| appid.to_string()).collect());
    Some(get_installed_apps(&appids, &properties, &app_names).unwrap_or_default())
}

fn backup_file(picked_path: &String, ext: &str) -> Result<(), Box<dyn Error>> {
    let backup_path = PathBuf::from(picked_path.clone() + ext);
    match ext {
        ".orig" => {
            if !backup_path.is_file() {
                fs::copy(PathBuf::from(picked_path), backup_path)?;
            }
        }
        ".bak" => {
            fs::copy(PathBuf::from(picked_path), backup_path)?;
        }
        _ => {
            warn!("backup extension does not exist");
        }
    }
    Ok(())
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

fn config_dir(localconfig_vdf_path: &String) -> PathBuf {
    let mut path = PathBuf::from(localconfig_vdf_path);
    path.pop();
    path.pop();
    path.pop();
    path.pop();
    path.push("config");
    path.push("libraryfolders.vdf");
    path
}

fn get_installed_apps(
    appids: &Vec<String>,
    properties: &BTreeMap<u32, String>,
    app_names: &BTreeMap<u32, String>,
) -> Result<BTreeMap<u32, App>, Box<dyn Error>> {
    let mut apps = BTreeMap::new();
    for appid in appids.iter() {
        let appid = appid.parse::<u32>().unwrap();
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
                if ui.button("🗁 Open file…").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("text", &["vdf"])
                        .set_directory(userdata_dir())
                        .pick_file()
                    {
                        self.steam_config = Some(path.to_str().unwrap_or_default().to_owned());
                        if let Some(localconfig_vdf_path) = &self.steam_config {
                            let mut config: Config =
                                confy::load(consts::CODE_NAME, None).unwrap_or_default();
                            config.steam_config = Some(localconfig_vdf_path.clone());
                            confy::store(consts::CODE_NAME, None, config).unwrap_or_default();
                            self.apps = update_apps(localconfig_vdf_path, &self.app_names);
                        }
                    }
                }

                let response = ui.button("⟳ Refresh");
                let popup_id = ui.make_persistent_id("refresh");
                if response.clicked() {
                    let refresh = true;
                    self.app_names =
                        api::app_names(refresh).expect("Error getting Steam apps from Steam API");
                    if let Some(localconfig_vdf_path) = &self.steam_config {
                        let mut config: Config =
                            confy::load(consts::CODE_NAME, None).unwrap_or_default();
                        config.steam_config = Some(localconfig_vdf_path.clone());
                        confy::store(consts::CODE_NAME, None, config).unwrap_or_default();
                        self.apps = update_apps(localconfig_vdf_path, &self.app_names);
                        ui.memory_mut(|mem| mem.open_popup(popup_id));
                    }
                }
                egui::popup_above_or_below_widget(
                    ui,
                    popup_id,
                    &response,
                    egui::AboveOrBelow::Below,
                    egui::popup::PopupCloseBehavior::CloseOnClick,
                    |ui| {
                        ui.set_min_width(100.0);
                        ui.label("Apps refreshed");
                    },
                );

                if let Some(picked_path) = &self.steam_config {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                }
            });

            if let Some(picked_path) = &self.steam_config {
                ui.separator();

                ui.horizontal_wrapped(|ui| {
                    let response = ui.button("💾 Save");
                    let popup_id = ui.make_persistent_id("save");
                    if response.clicked() {
                        let mut config: Config =
                            confy::load(consts::CODE_NAME, None).unwrap_or_default();
                        let previous_default_launch_options =
                            config.default_launch_options.unwrap_or_default();
                        config.default_launch_options = Some(self.default_launch_options.clone());
                        confy::store(consts::CODE_NAME, None, config).unwrap_or_default();
                        if !self.default_launch_options.trim().is_empty() {
                            for launch_options in self.all_launch_options.values_mut() {
                                if launch_options.is_empty()
                                    || launch_options == &previous_default_launch_options
                                {
                                    *launch_options = self.default_launch_options.clone();
                                }
                            }
                        }
                        backup_file(picked_path, ".bak").expect("Error backup failed");
                        localconfig_vdf::write_launch_options(
                            picked_path,
                            &self.all_launch_options,
                        )
                        .expect("Error failed to write launch options to config");
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
                            ui.label("File saved");
                        },
                    );

                    if ui.button("🗑 Clear").clicked() {
                        for launch_options in self.all_launch_options.values_mut() {
                            launch_options.clear();
                        }
                    }

                    if ui.button("🔄 Restore").clicked() {
                        if let Some(apps) = &self.apps {
                            for (appid, properties) in apps.iter() {
                                let current_launch_options = &properties.launch_options;
                                self.all_launch_options
                                    .insert(*appid, current_launch_options.clone())
                                    .expect("Error table update failed");
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
                    let mut selected = self.app_sort.clone();
                    let before = selected.clone();
                    egui::ComboBox::from_id_salt("AppSort")
                        .selected_text(format!("{}", selected))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut selected,
                                AppSort::IdAscending,
                                AppSort::IdAscending.to_string(),
                            );
                            ui.selectable_value(
                                &mut selected,
                                AppSort::IdDescending,
                                AppSort::IdDescending.to_string(),
                            );
                            ui.selectable_value(
                                &mut selected,
                                AppSort::NameAscending,
                                AppSort::NameAscending.to_string(),
                            );
                            ui.selectable_value(
                                &mut selected,
                                AppSort::NameDescending,
                                AppSort::NameDescending.to_string(),
                            );
                        });

                    if selected != before {
                        self.app_sort = selected;
                        let mut config: Config =
                            confy::load(consts::CODE_NAME, None).unwrap_or_default();
                        config.app_sort = Some(self.app_sort.to_string());
                        confy::store(consts::CODE_NAME, None, config).unwrap_or_default();
                    }

                    if cfg!(unix) {
                        let mut selected = self.protondb;
                        let before = selected;
                        ui.checkbox(&mut selected, "⚛ ProtonDB");

                        if selected != before {
                            if selected {
                                self.url = "https://www.protondb.com/app/".to_string();
                            } else {
                                self.url = "https://store.steampowered.com/app/".to_string();
                            }
                            self.protondb = selected;
                            let mut config: Config =
                                confy::load(consts::CODE_NAME, None).unwrap_or_default();
                            config.protondb = Some(self.protondb);
                            confy::store(consts::CODE_NAME, None, config).unwrap_or_default();
                        }
                    }

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
                                                    self.url.clone() + &appid.to_string(),
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
                                                self.all_launch_options
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
