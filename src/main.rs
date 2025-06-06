#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use directories::BaseDirs;
use eframe::egui;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 240.0]) // wide enough for the drag-drop overlay text
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "Steam OptionX",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default, Serialize, Deserialize)]
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
struct MyApp {
    picked_path: Option<String>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Find 'Steam/userdata/.../config/localconfig.vdf'");

            let config: Config = confy::load("steam-optionx", None).unwrap();
            if !config.steam_config.is_empty() {
                self.picked_path = Some(config.steam_config);
            }

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
                }
            }

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }

            egui::Grid::new("some_unique_id").show(ui, |ui| {
                ui.label("First row, first column");
                ui.label("First row, second column");
                ui.end_row();

                ui.label("Second row, first column");
                ui.label("Second row, second column");
                ui.label("Second row, third column");
                ui.end_row();

                ui.horizontal(|ui| {
                    ui.label("Same");
                    ui.label("cell");
                });
                ui.label("Third row, second column");
                ui.end_row();
            });
        });
    }
}
