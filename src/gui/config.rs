use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub dark_mode: bool,
    pub hamster_position: egui::Align2,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            client_id: String::new(),
            client_secret: String::new(),
            dark_mode: true,
            hamster_position: egui::Align2::LEFT_BOTTOM,
        }
    }
}