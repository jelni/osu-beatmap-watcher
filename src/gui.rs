use std::env;

use eframe::epaint::ColorImage;
use eframe::{egui, epi};

use crate::osu::client::{self, LoginState, TaskState, Update};
use crate::osu::types;

mod config;
mod widgets;
mod windows;

pub struct HamsterHack {
    ip: Option<String>,
    address: String,
}

#[derive(Default)]
struct UiState {
    login_state: LoginState,
    updater_state: TaskState,
    config_open: bool,
    beatmap: Option<types::Beatmap>,
    hamster_hack: Option<HamsterHack>,
}

#[derive(Default)]
pub struct App {
    config: Option<config::Config>,
    ui_state: UiState,
    client: client::Client,
    hamster: Option<egui::TextureHandle>,
}

impl App {
    pub const NAME: &'static str = "osu! Beatmap Watcher";

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let hamster = image::load_from_memory(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/hamster.png"
        )))
        .unwrap();

        let mut app = Self {
            config: epi::get_value::<config::Config>(cc.storage.unwrap(), epi::APP_KEY)
                .or_else(|| Some(config::Config::default())),
            hamster: Some(cc.egui_ctx.load_texture(
                "hamster",
                egui::ColorImage::from_rgba_unmultiplied(
                    [hamster.width() as _, hamster.height() as _],
                    hamster.as_bytes(),
                ),
            )),
            ..Default::default()
        };

        let config = app.config.as_ref().unwrap();

        cc.egui_ctx.set_visuals(match config.dark_mode {
            true => egui::Visuals::dark(),
            false => egui::Visuals::light(),
        });

        if !config.client_id.is_empty() && !config.client_secret.is_empty() {
            app.client
                .log_in(config.client_id.clone(), config.client_secret.clone());
        }

        app
    }

    fn process_inputs(&self, ctx: &egui::Context, frame: &mut epi::Frame) {
        if ctx.input().key_pressed(egui::Key::Escape) {
            frame.quit();
        }
    }

    fn poll_client_updates(&mut self, ctx: &egui::Context) {
        for message in self.client.poll_updates() {
            match message {
                Update::UpdaterState(state) => self.ui_state.updater_state = state,
                Update::LoginState(state) => self.ui_state.login_state = state,
                Update::Beatmap(beatmap) => {
                    if let Some(new_beatmap) = beatmap.as_ref() {
                        if self.ui_state.beatmap.is_none() {
                            self.client.get_beatmap_cover(new_beatmap.id);
                        }
                    }
                    self.ui_state.beatmap = beatmap;
                }
                Update::BeatmapCover(cover) => {
                    if let Some(beatmap) = self.ui_state.beatmap.as_mut() {
                        match cover {
                            Some(cover) => {
                                beatmap.cover = Some(ctx.load_texture("beatmap_cover", cover));
                            }
                            None => {
                                beatmap.cover =
                                    Some(ctx.load_texture("beatmap_cover", ColorImage::example()));
                            }
                        }
                    }
                }
                Update::Ip(ip) => {
                    if let Some(hamster_hack) = self.ui_state.hamster_hack.as_mut() {
                        hamster_hack.ip = Some(ip);
                    }
                }
            }
        }
    }

    fn should_show_settings(&self) -> bool {
        self.ui_state.config_open || self.ui_state.login_state != LoginState::LoggedIn
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut epi::Frame) {
        self.process_inputs(ctx, frame);
        self.poll_client_updates(ctx);
        self.draw(ctx);
    }

    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self.config.as_ref().unwrap());
    }

    fn clear_color(&self) -> egui::Rgba {
        egui::Rgba::BLACK
    }

    fn persist_native_window(&self) -> bool {
        false
    }

    fn persist_egui_memory(&self) -> bool {
        false
    }

    fn warm_up_enabled(&self) -> bool {
        true
    }
}
