use std::env;

use eframe::egui::{Color32, ColorImage, Context, Key, TextureFilter, Visuals};
use eframe::epaint::{Rgba, TextureHandle};
use eframe::Frame;
use tokio::task::JoinHandle;

use self::config::Config;
use crate::osu::client::{Client, LoginState, Update};
use crate::osu::types::Beatmap;

mod config;
mod widgets;
mod windows;

struct State {
    login_state: LoginState,
    worker: Option<JoinHandle<()>>,
    config_open: bool,
    beatmap: Option<Beatmap>,
    beatmap_cover: Option<TextureHandle>,
    hamster_hack: Option<HamsterHackData>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            login_state: LoginState::LoggedOut,
            worker: None,
            config_open: false,
            beatmap: None,
            beatmap_cover: None,
            hamster_hack: None,
        }
    }
}

pub struct HamsterHackData {
    ip: String,
    address: String,
}

pub struct App {
    config: Config,
    state: State,
    client: Client,
    hamster: TextureHandle,
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
            config: eframe::get_value::<config::Config>(cc.storage.unwrap(), eframe::APP_KEY)
                .unwrap_or_default(),
            state: State::default(),
            client: Client::default(),
            hamster: cc.egui_ctx.load_texture(
                "hamster",
                ColorImage::from_rgba_unmultiplied(
                    [
                        hamster.width().try_into().unwrap(),
                        hamster.height().try_into().unwrap(),
                    ],
                    hamster.as_bytes(),
                ),
                TextureFilter::Linear,
            ),
        };

        cc.egui_ctx.set_visuals(if app.config.dark_mode {
            Visuals::dark()
        } else {
            Visuals::light()
        });

        if !app.config.client_id.is_empty() && !app.config.client_secret.is_empty() {
            app.state.config_open = false;
            app.client.log_in(
                app.config.client_id.clone(),
                app.config.client_secret.clone(),
            );
        }

        app
    }

    fn process_io(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        if let Some(worker) = &self.state.worker {
            if worker.is_finished() {
                self.state.worker = None;
            }
        }
        if ctx.input().key_pressed(Key::Escape) {
            frame.close();
        }
    }

    fn poll_client_updates(&mut self, ctx: &Context) {
        for message in self.client.poll_updates() {
            match message {
                Update::LoginState(state) => {
                    if let LoginState::LoginError(_) = state {
                        self.state.config_open = true;
                    }
                    self.state.login_state = state;
                }
                Update::Beatmap(beatmap) => {
                    if let Some(new_beatmap) = beatmap.as_ref() {
                        if self.state.beatmap.is_none() {
                            self.state.beatmap_cover = None;
                            self.client.get_beatmap_cover(new_beatmap.id);
                        }
                    }
                    self.state.beatmap = beatmap;
                }
                Update::BeatmapCover(cover) => {
                    self.state.beatmap_cover = Some(ctx.load_texture(
                        "beatmap_cover",
                        cover.unwrap_or_else(|| {
                            ColorImage::new([64, 64], Color32::from_rgb(34, 34, 34))
                        }),
                        TextureFilter::Linear,
                    ));
                }
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.process_io(ctx, frame);
        self.poll_client_updates(ctx);
        self.draw(ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.config);
    }

    fn clear_color(&self, _: &Visuals) -> Rgba {
        Rgba::BLACK
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
