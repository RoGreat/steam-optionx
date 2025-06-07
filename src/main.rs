#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use directories::BaseDirs;
use eframe::egui;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
mod vdf;

fn main() -> eframe::Result {
    let config: Config = confy::load("steam-optionx", None).unwrap();
    let picked_path = Some(config.steam_config);
    let app_ids = Some(vdf::appids(picked_path.clone().unwrap()).unwrap());
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 240.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "Steam OptionX",
        options,
        Box::new(|_cc| {
            Ok(Box::new(EguiApp {
                picked_path: picked_path,
                app_ids: app_ids,
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

#[derive(Default)]
struct EguiApp {
    picked_path: Option<String>,
    app_ids: Option<Vec<String>>,
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
                    self.app_ids = Some(vdf::appids(self.picked_path.clone().unwrap()).unwrap());
                    println!("{:?}", self.app_ids);
                }
            }

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }

            egui::Grid::new("game_list").show(ui, |ui| {
                if let Some(ids) = &self.app_ids {
                    for id in ids.iter() {
                        ui.label(id);
                        ui.end_row();
                    }
                }
                //ui.label("First row, first column");
                //ui.label("First row, second column");
                //ui.end_row();

                //ui.label("Second row, first column");
                //ui.label("Second row, second column");
                //ui.label("Second row, third column");
                //ui.end_row();

                //ui.horizontal(|ui| {
                //    ui.label("Same");
                //    ui.label("cell");
                //});
                //ui.label("Third row, second column");
                //ui.end_row();
            });
        });
    }
}
