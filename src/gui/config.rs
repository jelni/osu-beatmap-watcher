use eframe::emath::Align2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub beatmap_id: String,
    pub dark_mode: bool,
    pub hamster_position: Align2,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            client_id: String::new(),
            client_secret: String::new(),
            beatmap_id: String::new(),
            dark_mode: true,
            hamster_position: Align2::RIGHT_BOTTOM,
        }
    }
}
