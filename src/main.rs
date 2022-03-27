#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console on Windows

use eframe::{egui, epi};
use osu_beatmap_watcher::osu;

mod gui;

fn main() {
    let icon = image::load_from_memory(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/icon.png"
    )))
    .unwrap();
    let width = icon.width();
    let height = icon.height();

    eframe::run_native(
        Box::new(gui::App::default()),
        eframe::NativeOptions {
            icon_data: Some(epi::IconData {
                rgba: icon.into_bytes(),
                width,
                height,
            }),
            initial_window_size: Some(egui::Vec2::new(600., 400.)),
            min_window_size: Some(egui::Vec2::new(400., 200.)),
            ..Default::default()
        },
    );
}
