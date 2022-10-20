use eframe::egui::{
    self, Area, Button, CentralPanel, Color32, Context, Layout, Order, TextEdit, TopBottomPanel,
    Visuals, Window,
};
use eframe::emath::{Align, Align2};
use eframe::epaint::Vec2;
use rand::Rng;

use self::gui::HamsterHackData;
use super::widgets::beatmap::BeatmapWidget;
use super::widgets::hamster::HamsterWidget;
use super::widgets::hamster_hack::HamsterHackWidget;
use crate::gui;
use crate::osu::client::LoginState;

const HAMSTER_OFFSET: f32 = 48.;

impl gui::App {
    const SETTINGS_TITLE: &'static str = "⛭ Settings";

    pub fn draw(&mut self, ctx: &Context) {
        if self.state.hamster_hack.is_some() {
            self.draw_hamster_hack(ctx);
        } else {
            self.draw_top_panel(ctx);
            self.draw_main_panel(ctx);
            self.draw_settings(ctx);
            self.draw_hamster(ctx);
        }
    }

    fn draw_top_panel(&mut self, ctx: &Context) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.set_enabled(!self.state.config_open);
            ui.horizontal(|ui| {
                egui::warn_if_debug_build(ui);
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button(Self::SETTINGS_TITLE).clicked() {
                        self.state.config_open = true;
                    }
                });
            })
        });
    }

    fn draw_main_panel(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.set_enabled(!self.state.config_open);
            ui.vertical_centered(|ui| {
                ui.add(
                    TextEdit::singleline(&mut self.config.beatmap_id)
                        .interactive(self.state.worker.is_none()),
                );

                match &self.state.worker {
                    Some(worker) => {
                        if ui.button("Stop").clicked() {
                            worker.abort();
                        }
                    }
                    None => {
                        if let LoginState::LoggedIn { access_token } = &self.state.login_state {
                            let beatmap_id = self.config.beatmap_id.parse::<u32>();
                            if ui
                                .add_enabled(beatmap_id.is_ok(), Button::new("Start"))
                                .clicked()
                            {
                                if let Ok(beatmap_id) = beatmap_id {
                                    self.state.worker = Some(
                                        self.client.poll_beatmap(access_token.clone(), beatmap_id),
                                    );
                                }
                            }
                        }
                    }
                }

                Area::new("beatmap_area")
                    .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                    .order(Order::Background)
                    .enabled(!self.state.config_open)
                    .show(ctx, |ui| match self.state.beatmap.as_ref() {
                        Some(beatmap) => {
                            ui.add(BeatmapWidget {
                                beatmap,
                                beatmap_cover: self.state.beatmap_cover.clone(),
                                worker_running: self.state.worker.is_some(),
                            });
                        }
                        None => {
                            if self.state.worker.is_some() {
                                ui.spinner();
                            }
                        }
                    });
            });
        });
    }

    fn draw_settings(&mut self, ctx: &Context) {
        let mut window = Window::new(Self::SETTINGS_TITLE);
        if let LoginState::LoggedIn { .. } = self.state.login_state {
            window = window.open(&mut self.state.config_open);
        }
        window
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .collapsible(false)
            .auto_sized()
            .default_width(256.)
            .show(ctx, |ui| {
                let login_inputs_interactive = matches!(
                    self.state.login_state,
                    LoginState::LoggedOut | LoginState::LoginError(_)
                );
                ui.label("Client ID");
                ui.add(
                    TextEdit::singleline(&mut self.config.client_id)
                        .interactive(login_inputs_interactive)
                        .hint_text("client_id"),
                );

                ui.label("Client Secret");
                ui.add(
                    TextEdit::singleline(&mut self.config.client_secret)
                        .password(true)
                        .interactive(login_inputs_interactive)
                        .hint_text("client_secret"),
                );

                ui.horizontal(|ui| {
                    if login_inputs_interactive && ui.button("➡ Log In").clicked() {
                        self.client.log_in(
                            self.config.client_id.clone(),
                            self.config.client_secret.clone(),
                        );
                    }

                    match &self.state.login_state {
                        LoginState::LoggedOut => (),
                        LoginState::LoggedIn { .. } => {
                            if ui.button("⬅ Log Out").clicked() {
                                self.state.login_state = LoginState::LoggedOut;
                            }
                        }
                        LoginState::LoggingIn => {
                            ui.spinner();
                            ui.label("Logging In…");
                        }
                        LoginState::LoginError(err) => {
                            ui.colored_label(Color32::LIGHT_RED, err);
                        }
                    }
                });

                ui.separator();

                ui.label("Theme");
                ui.horizontal(|ui| {
                    let dark_mode = ui.visuals().dark_mode;
                    if ui.selectable_label(dark_mode, "🌙 Dark").clicked() {
                        self.config.dark_mode = true;
                        ctx.set_visuals(Visuals::dark());
                    }
                    if ui.selectable_label(!dark_mode, "☀ Light").clicked() {
                        self.config.dark_mode = false;
                        ctx.set_visuals(Visuals::light());
                    }
                });

                ui.label("Handedness");
                ui.horizontal(|ui| {
                    if ui
                        .selectable_label(
                            self.config.hamster_position == Align2::LEFT_BOTTOM,
                            "◀ Left-Handed",
                        )
                        .clicked()
                    {
                        self.config.hamster_position = Align2::LEFT_BOTTOM;
                    }
                    if ui
                        .selectable_label(
                            self.config.hamster_position == Align2::RIGHT_BOTTOM,
                            "▶ Right-Handed",
                        )
                        .clicked()
                    {
                        self.config.hamster_position = Align2::RIGHT_BOTTOM;
                    }
                });

                ui.separator();

                ui.hyperlink_to("Help!", "https://youtu.be/9oyC4ArBf1Y");
            });
    }

    pub fn draw_hamster(&mut self, ctx: &Context) {
        Area::new("hamster_area")
            .order(Order::Background)
            .anchor(self.config.hamster_position, Vec2::new(0., HAMSTER_OFFSET))
            .show(ctx, |ui| {
                if ui
                    .add(HamsterWidget {
                        hamster: self.hamster.clone(),
                    })
                    .clicked()
                {
                    let mut rng = rand::thread_rng();
                    self.state.hamster_hack = Some(HamsterHackData {
                        ip: rng.gen::<[u8; 4]>().map(|n| n.to_string()).join("."),
                        address: {
                            let random_bytes = rng.gen::<[u8; 16]>();
                            random_bytes
                                .iter()
                                .map(|byte| format!("{byte:02x}"))
                                .collect()
                        },
                    });
                };
            });
    }

    fn draw_hamster_hack(&mut self, ctx: &Context) {
        Area::new("hamster_hack_area")
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                if ui
                    .add(HamsterHackWidget {
                        hamster_hack: self.state.hamster_hack.as_ref().unwrap(),
                    })
                    .clicked()
                {
                    self.state.hamster_hack = None;
                }
            });
    }
}
