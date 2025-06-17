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
    let properties =
        vdf::deserialize(picked_path.clone().unwrap_or_default()).unwrap_or(BTreeMap::default());
    let game_names = api::game_names().expect("Error getting Steam games");
    let user_games = Some(user_games(properties.clone(), game_names.clone()).unwrap_or_default());
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 450.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "Steam OptionX",
        options,
        Box::new(|_cc| {
            Ok(Box::new(EguiApp {
                picked_path: picked_path,
                properties: properties,
                game_names: game_names,
                user_games: user_games,
                all_launch_options: BTreeMap::new(),
            }))
        }),
    )
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

struct Game {
    name: String,
    launch_options: String,
}

fn user_games(
    properties: BTreeMap<u64, String>,
    game_names: BTreeMap<u64, String>,
) -> Result<BTreeMap<u64, Game>, Box<dyn Error>> {
    let mut result = BTreeMap::new();
    let appids: Vec<u64> = properties.clone().into_keys().collect();
    for appid in appids {
        if let Some(game_name) = game_names.get(&appid) {
            let properties = properties.clone();
            let launch_options = properties.get(&appid).unwrap_or(&String::new()).to_string();
            let game = Game {
                name: game_name.clone(),
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
    properties: BTreeMap<u64, String>,
    game_names: BTreeMap<u64, String>,
    user_games: Option<BTreeMap<u64, Game>>,
    all_launch_options: BTreeMap<u64, String>,
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
                            confy::store("steam-optionx", None, config).unwrap_or_default();
                            self.properties =
                                vdf::deserialize(picked_path.clone()).unwrap_or_default();
                            self.user_games = Some(
                                user_games(self.properties.clone(), self.game_names.clone())
                                    .unwrap_or_default(),
                            );
                        }
                    }
                }
                if ui.button("Reset").clicked() {
                    self.picked_path = None;
                    self.user_games = None;
                    _ = fs::remove_file(
                        confy::get_configuration_file_path(APP_NAME, None).unwrap(),
                    );
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
                        println!("Saving `{}`...", &picked_path);
                        _ = vdf::serialize(picked_path.clone(), self.all_launch_options.clone());
                        println!("Saved `{}`", &picked_path);
                    }
                });

                TableBuilder::new(ui)
                    .resizable(true)
                    .column(Column::auto().at_least(150.0))
                    .column(Column::remainder())
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.heading("Game");
                        });
                        header.col(|ui| {
                            ui.heading("Launch Options");
                        });
                    })
                    .body(|mut body| {
                        body.row(0.0, |mut row| {
                            row.col(|ui| {
                                if let Some(user_games) = &self.user_games {
                                    for (appid, properties) in
                                        user_games.keys().zip(user_games.values())
                                    {
                                        let game_name = properties.name.clone();

                                        ui.style_mut().wrap_mode =
                                            Some(egui::TextWrapMode::Truncate);
                                        ui.add_sized(
                                            [ui.available_width(), 20.0],
                                            egui::Hyperlink::from_label_and_url(
                                                game_name,
                                                "https://store.steampowered.com/app/".to_owned()
                                                    + &appid.to_string(),
                                            ),
                                        );
                                    }
                                }
                            });
                            row.col(|ui| {
                                if let Some(user_games) = &self.user_games {
                                    for (appid, properties) in
                                        user_games.keys().zip(user_games.values())
                                    {
                                        let appid = appid.clone();
                                        let mut current_launch_options =
                                            properties.launch_options.clone();
                                        match self.all_launch_options.get(&appid) {
                                            Some(launch_options) => {
                                                current_launch_options = launch_options.clone();
                                                ()
                                            }
                                            None => {
                                                self.all_launch_options.insert(
                                                    appid.clone(),
                                                    current_launch_options.clone(),
                                                );
                                                ()
                                            }
                                        }

                                        ui.style_mut().wrap_mode =
                                            Some(egui::TextWrapMode::Truncate);
                                        let response = ui.add_sized(
                                            [ui.available_width() - 20.0, 20.0],
                                            egui::TextEdit::singleline(&mut current_launch_options),
                                        );
                                        if response.changed() {
                                            self.all_launch_options
                                                .insert(appid, current_launch_options);
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
