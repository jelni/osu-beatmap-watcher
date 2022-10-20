#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console on Windows
#![warn(clippy::pedantic)]

use eframe::epaint::Vec2;
use eframe::{IconData, NativeOptions};
use gui::App;

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
        App::NAME,
        NativeOptions {
            icon_data: Some(IconData {
                rgba: icon.into_bytes(),
                width,
                height,
            }),
            initial_window_size: Some(Vec2::new(600., 400.)),
            min_window_size: Some(Vec2::new(400., 200.)),
            ..Default::default()
        },
        Box::new(|cc| Box::new(App::new(cc))),
    );
}
