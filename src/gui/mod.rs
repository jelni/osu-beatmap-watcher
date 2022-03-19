use crate::osu::client;
use eframe::{egui, epi};
use std::env;

mod config;
mod windows;

pub struct App {
    config: Option<config::Config>,
    config_open: bool,
    client: client::Client,
    hamster: egui_extras::RetainedImage,
}

impl Default for App {
    fn default() -> Self {
        Self {
            config: None,
            config_open: false,
            client: client::Client::default(),
            hamster: egui_extras::RetainedImage::from_image_bytes(
                "hamster.png",
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/hamster.png")),
            )
            .unwrap(),
        }
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &epi::Frame) {
        self.client.update_state();
        self.draw(ctx);
    }

    fn setup(
        &mut self,
        ctx: &egui::Context,
        _frame: &epi::Frame,
        storage: Option<&dyn epi::Storage>,
    ) {
        self.config = epi::get_value::<config::Config>(storage.unwrap(), epi::APP_KEY)
            .or(Some(config::Config::default()));

        match self.config.as_ref().unwrap().dark_mode {
            true => ctx.set_visuals(egui::Visuals::dark()),
            false => ctx.set_visuals(egui::Visuals::light()),
        };
    }

    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self.config.as_ref().unwrap());
    }

    fn name(&self) -> &str {
        Self::APP_NAME
    }

    fn persist_native_window(&self) -> bool {
        false
    }

    fn persist_egui_memory(&self) -> bool {
        false
    }
}

impl App {
    const APP_NAME: &'static str = "osu! Beatmap Watcher";
}
