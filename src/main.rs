#![windows_subsystem = "windows"]

mod api;
mod vdf;

use directories::BaseDirs;
use eframe::egui;
use egui_extras::{Column, TableBuilder};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::path::PathBuf;

fn main() -> eframe::Result {
    let config: Config = confy::load("steam-optionx", None).unwrap();
    let picked_path = Some(config.steam_config);
    let properties =
        Some(vdf::deserialize(picked_path.clone().unwrap()).unwrap_or(BTreeMap::default()));
    let game_names = Some(api::game_names().expect("Error getting steam games"));
    let user_games = Some(user_games(properties.clone(), game_names.clone()).unwrap());
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 360.0])
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
    steam_config: String,
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
    properties: Option<BTreeMap<u64, String>>,
    game_names: Option<BTreeMap<u64, String>>,
) -> Result<BTreeMap<u64, Game>, Box<dyn Error>> {
    let mut result = BTreeMap::new();
    let appids: Vec<u64> = properties.clone().unwrap().into_keys().collect();
    if let (appids, Some(game_names)) = (appids, game_names) {
        for appid in appids {
            if let Some(game_name) = game_names.get(&appid) {
                let properties = properties.clone().unwrap();
                let launch_options = properties.get(&appid).unwrap();
                let game = Game {
                    name: game_name.to_string(),
                    launch_options: launch_options.to_string(),
                };
                result.insert(appid, game);
            }
        }
    }
    Ok(result)
}

#[derive(Default)]
struct EguiApp {
    picked_path: Option<String>,
    properties: Option<BTreeMap<u64, String>>,
    game_names: Option<BTreeMap<u64, String>>,
    user_games: Option<BTreeMap<u64, Game>>,
    all_launch_options: BTreeMap<u64, String>,
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Find 'Steam/userdata/########/config/localconfig.vdf'");

            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("text", &["vdf"])
                    .set_directory(userdata())
                    .pick_file()
                {
                    self.picked_path = Some(path.display().to_string());
                    let config = Config {
                        steam_config: self.picked_path.clone().unwrap(),
                    };
                    confy::store("steam-optionx", None, config).unwrap();
                    self.properties =
                        Some(vdf::deserialize(self.picked_path.clone().unwrap()).unwrap());
                    self.user_games =
                        Some(user_games(self.properties.clone(), self.game_names.clone()).unwrap());
                }
            }

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }

            TableBuilder::new(ui)
                .resizable(true)
                .column(Column::auto().at_least(200.0))
                .column(Column::remainder())
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Game");
                    });
                    header.col(|ui| {
                        ui.heading("Launch Option");
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
                                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
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

                                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
                                    let response = ui.add_sized(
                                        [ui.available_width() - 20.0, 20.0],
                                        egui::TextEdit::singleline(&mut current_launch_options),
                                    );
                                    if response.changed() {
                                        self.all_launch_options
                                            .insert(appid.clone(), current_launch_options.clone());
                                    }
                                    if response.lost_focus()
                                        && ui.input(|i| i.key_pressed(egui::Key::Enter))
                                    {
                                    }
                                }
                            }
                        });
                    });
                });
        });
    }
}
