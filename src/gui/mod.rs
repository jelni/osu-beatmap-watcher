use crate::osu::client::{self, LoginState, Message, TaskState};
use crate::osu::types;
use eframe::{egui, epi};
use std::env;
mod config;
mod windows;

#[derive(Default)]
struct UiState {
    pub updater_state: TaskState,
    pub config_open: bool,
    pub login_state: LoginState,
    pub beatmap: Option<types::Beatmap>,
    pub ip: Option<String>,
}

pub struct App {
    config: Option<config::Config>,
    ui_state: UiState,
    client: client::Client,
    hamster: egui_extras::RetainedImage,
}

impl Default for App {
    fn default() -> Self {
        Self {
            config: None,
            ui_state: UiState::default(),
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
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        if ctx.input().key_pressed(egui::Key::Escape) {
            frame.quit();
        }
        for message in self.client.poll_updates() {
            println!("{:?}", message);
            match message {
                Message::NewProgramState(state) => self.ui_state.updater_state = state,
                Message::NewLoginState(state) => self.ui_state.login_state = state,
                Message::NewBeatmap(beatmap) => self.ui_state.beatmap = beatmap,
                Message::NewIp(ip) => self.ui_state.ip = Some(ip),
            }
        }
        self.draw(ctx);
    }

    fn setup(&mut self, ctx: &egui::Context, _: &epi::Frame, storage: Option<&dyn epi::Storage>) {
        self.config = epi::get_value::<config::Config>(storage.unwrap(), epi::APP_KEY)
            .or(Some(config::Config::default()));

        let config = self.config.as_ref().unwrap();

        ctx.set_visuals(match config.dark_mode {
            true => egui::Visuals::dark(),
            false => egui::Visuals::light(),
        });

        if !config.client_id.is_empty() && !config.client_secret.is_empty() {
            self.client
                .log_in(config.client_id.clone(), config.client_secret.clone());
        }
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

    fn should_show_settings(&self) -> bool {
        self.ui_state.config_open || self.ui_state.login_state != LoginState::LoggedIn
    }
}
