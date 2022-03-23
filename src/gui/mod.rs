use crate::osu::client::{self, LoginState, TaskState, Update};
use crate::osu::types;
use eframe::{egui, epi};
use std::env;

mod config;
mod windows;

struct HamsterHack {
    ip: Option<String>,
    address: String,
}

#[derive(Default)]
struct UiState {
    login_state: LoginState,
    updater_state: TaskState,
    config_open: bool,
    beatmap: Option<types::Beatmap>,
    beatmap_cover: Option<egui::TextureHandle>,
    hamster_hack: Option<HamsterHack>,
}

#[derive(Default)]
pub struct App {
    config: Option<config::Config>,
    ui_state: UiState,
    client: client::Client,
    hamster: Option<egui::TextureHandle>,
}

impl epi::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        if ctx.input().key_pressed(egui::Key::Escape) {
            frame.quit();
        }
        let mut update_cover = false;
        for message in self.client.poll_updates() {
            match message {
                Update::UpdaterState(state) => self.ui_state.updater_state = state,
                Update::LoginState(state) => self.ui_state.login_state = state,
                Update::Beatmap(beatmap) => {
                    println!(
                        "{:?} - {:?}",
                        beatmap.as_ref(),
                        self.ui_state.beatmap.as_ref()
                    );

                    if let Some(new_beatmap) = beatmap.as_ref() {
                        if let Some(old_beatmap) = self.ui_state.beatmap.as_ref() {
                            if new_beatmap.id != old_beatmap.id {
                                update_cover = true;
                            }
                        } else {
                            update_cover = true;
                        }
                    }

                    self.ui_state.beatmap = beatmap;
                }
                Update::BeatmapCover(cover) => {
                    self.ui_state.beatmap_cover =
                        cover.map(|cover| ctx.load_texture("cover", cover));
                }
                Update::Ip(ip) => {
                    if let Some(hamster_hack) = self.ui_state.hamster_hack.as_mut() {
                        hamster_hack.ip = Some(ip);
                    }
                }
            }
        }
        if update_cover {
            self.client
                .get_beatmap_cover(self.ui_state.beatmap.as_ref().unwrap().id);
        }
        self.draw(ctx);
    }

    fn clear_color(&self) -> egui::Rgba {
        egui::Rgba::BLACK
    }

    fn setup(&mut self, ctx: &egui::Context, _: &epi::Frame, storage: Option<&dyn epi::Storage>) {
        self.config = epi::get_value::<config::Config>(storage.unwrap(), epi::APP_KEY)
            .or_else(|| Some(config::Config::default()));

        let hamster = image::load_from_memory(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/hamster.png"
        )))
        .unwrap();

        self.hamster = Some(ctx.load_texture(
            "hamster",
            egui::ColorImage::from_rgba_unmultiplied(
                [hamster.width() as _, hamster.height() as _],
                hamster.as_bytes(),
            ),
        ));

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
