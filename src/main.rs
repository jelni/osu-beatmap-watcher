#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console on Windows
#![feature(duration_constants)]

use eframe::{egui, epi};

mod gui;
mod osu;

fn main() {
    let image = image::load_from_memory(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/icon.png"
    )))
    .unwrap();
    let width = image.width();
    let height = image.height();

    let options = eframe::NativeOptions {
        icon_data: Some(epi::IconData {
            rgba: image.into_bytes(),
            width,
            height,
        }),
        initial_window_size: Some(egui::Vec2::new(600., 400.)),
        min_window_size: Some(egui::Vec2::new(400., 200.)),
        ..Default::default()
    };

    eframe::run_native(Box::new(gui::App::default()), options);
}
