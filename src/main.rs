#![windows_subsystem = "windows"]

mod api;
mod vdf;

use directories::BaseDirs;
use eframe::egui;
use egui_extras::{Column, TableBuilder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::path::PathBuf;

fn main() -> eframe::Result {
    let config: Config = confy::load("steam-optionx", None).unwrap();
    let picked_path = Some(config.steam_config);
    let appids = Some(vdf::appids(picked_path.clone().unwrap()).unwrap_or(vec![]));
    let game_names = Some(api::game_names().expect("Error getting steam games"));
    let user_games = Some(user_games(appids.clone(), game_names.clone()).unwrap());
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
                appids: appids,
                game_names: game_names,
                user_games: user_games,
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

fn user_games(
    appids: Option<Vec<String>>,
    game_names: Option<HashMap<String, String>>,
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut result = HashMap::new();
    if let Some(appids) = appids {
        if let Some(game_names) = game_names.clone() {
            for appid in appids.iter() {
                if let Some(game_name) = game_names.get(appid) {
                    println!("{} {}", appid, game_name);
                    result.insert(appid.to_string(), game_name.to_string());
                }
            }
        }
    }
    Ok(result)
}

#[derive(Default)]
struct EguiApp {
    picked_path: Option<String>,
    appids: Option<Vec<String>>,
    game_names: Option<HashMap<String, String>>,
    user_games: Option<HashMap<String, String>>,
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
                    self.appids = Some(vdf::appids(self.picked_path.clone().unwrap()).unwrap());
                    println!("{:?}", self.appids);
                    self.user_games =
                        Some(user_games(self.appids.clone(), self.game_names.clone()).unwrap());
                }
            }

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }

            TableBuilder::new(ui)
                .striped(true)
                .column(Column::auto().resizable(true).at_least(200.0))
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
                    body.row(30.0, |mut row| {
                        row.col(|ui| {
                            if let Some(user_games) = &self.user_games {
                                for (appid, game_name) in user_games.keys().zip(user_games.values())
                                {
                                    if !game_name.is_empty() {
                                        ui.hyperlink_to(
                                            game_name,
                                            "https://store.steampowered.com/app/".to_owned()
                                                + appid,
                                        );
                                    }
                                }
                            }
                        });
                        row.col(|ui| {});
                    });
                });
        });
    }
}
