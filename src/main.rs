#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console on Windows
#![feature(duration_constants)]

use eframe::{egui, epi};

mod gui;
mod osu;

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
