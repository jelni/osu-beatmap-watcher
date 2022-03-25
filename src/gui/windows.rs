use eframe::egui;
use rand::Rng;

use self::gui::HamsterHack;
use crate::gui;
use crate::gui::widgets;
use crate::osu::client::{LoginState, TaskState};

const HAMSTER_OFFSET: f32 = 48.;

impl gui::App {
    const SETTINGS_TITLE: &'static str = "\u{26ED} Settings";

    pub fn draw(&mut self, ctx: &egui::Context) {
        match self.ui_state.hamster_hack.is_some() {
            false => {
                self.draw_top_panel(ctx);
                self.draw_main_panel(ctx);
                self.draw_settings(ctx);
                self.draw_hamster(ctx);
            }
            true => self.draw_hamster_hack(ctx, self.ui_state.hamster_hack.as_ref().unwrap()),
        }
    }

    fn draw_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.set_enabled(!self.should_show_settings());
            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                if ui.button(Self::SETTINGS_TITLE).clicked() {
                    self.ui_state.config_open = true;
                }
            });
        });
    }

    fn draw_main_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_enabled(!self.should_show_settings());
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    ui.add_enabled(
                        self.ui_state.updater_state == TaskState::Stopped,
                        egui::TextEdit::singleline(&mut self.config.as_mut().unwrap().beatmap_id),
                    );

                    match self.ui_state.updater_state {
                        TaskState::Running => {
                            if ui.button("Stop").clicked() {
                                self.client.stop_updating_beatmap();
                            }
                        }
                        TaskState::Stopping => {
                            ui.add(egui::Spinner::new());
                            ui.label("Stopping…");
                        }
                        TaskState::Stopped => {
                            let beatmap_id =
                                self.config.as_ref().unwrap().beatmap_id.parse::<u32>();
                            if ui
                                .add_enabled(beatmap_id.is_ok(), egui::Button::new("Start"))
                                .clicked()
                            {
                                self.client.start_updating_beatmap(beatmap_id.unwrap());
                            }
                        }
                    }
                });

                egui::Area::new("beatmap_area")
                    .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                    .show(ctx, |ui| match self.ui_state.beatmap.as_ref() {
                        Some(beatmap) => {
                            ui.add(widgets::DrawBeatmap::new(
                                beatmap,
                                &self.ui_state.updater_state,
                            ));
                        }
                        None => {
                            if self.ui_state.updater_state == TaskState::Running {
                                ui.add(egui::Spinner::new());
                            }
                        }
                    });
            });
        });
    }

    fn draw_settings(&mut self, ctx: &egui::Context) {
        let mut window = egui::Window::new(Self::SETTINGS_TITLE);
        if self.ui_state.login_state == LoginState::LoggedIn {
            window = window.open(&mut self.ui_state.config_open);
        }
        window
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .collapsible(false)
            .auto_sized()
            .default_width(256.)
            .show(ctx, |ui| {
                let login_inputs_interactive = matches!(
                    self.ui_state.login_state,
                    LoginState::LoggedOut | LoginState::LoginError(_)
                );
                ui.label("Client ID");
                ui.add(
                    egui::TextEdit::singleline(&mut self.config.as_mut().unwrap().client_id)
                        .interactive(login_inputs_interactive)
                        .hint_text("client_id"),
                );

                ui.label("Client Secret");
                ui.add(
                    egui::TextEdit::singleline(&mut self.config.as_mut().unwrap().client_secret)
                        .password(true)
                        .interactive(login_inputs_interactive)
                        .hint_text("client_secret"),
                );

                ui.horizontal(|ui| match &self.ui_state.login_state {
                    LoginState::LoggedOut => {
                        if ui.button("\u{27A1} Log In").clicked() {
                            let config = self.config.as_ref().unwrap();
                            self.client
                                .log_in(config.client_id.clone(), config.client_secret.clone());
                        }
                    }
                    LoginState::LoggingIn => {
                        ui.add(egui::Spinner::new());
                        ui.label("Logging In…");
                    }
                    LoginState::LoggedIn => {
                        if ui.button("\u{2B05} Log Out").clicked() {
                            self.client.log_out();
                        }
                    }
                    LoginState::LoginError(err) => {
                        ui.colored_label(egui::Color32::LIGHT_RED, format!("Error: {err}"));
                    }
                });

                ui.separator();

                ui.label("Theme");
                ui.horizontal(|ui| {
                    let dark_mode = ui.visuals().dark_mode;
                    if ui.selectable_label(dark_mode, "\u{1F319} Dark").clicked() {
                        self.config.as_mut().unwrap().dark_mode = true;
                        ctx.set_visuals(egui::Visuals::dark());
                    }
                    if ui.selectable_label(!dark_mode, "\u{2600} Light").clicked() {
                        self.config.as_mut().unwrap().dark_mode = false;
                        ctx.set_visuals(egui::Visuals::light());
                    }
                });

                ui.label("Handedness");
                ui.horizontal(|ui| {
                    if ui
                        .selectable_label(
                            self.config.as_ref().unwrap().hamster_position
                                == egui::Align2::LEFT_BOTTOM,
                            "\u{25C0} Left-Handed",
                        )
                        .clicked()
                    {
                        self.config.as_mut().unwrap().hamster_position = egui::Align2::LEFT_BOTTOM;
                    }
                    if ui
                        .selectable_label(
                            self.config.as_ref().unwrap().hamster_position
                                == egui::Align2::RIGHT_BOTTOM,
                            "\u{25B6} Right-Handed",
                        )
                        .clicked()
                    {
                        self.config.as_mut().unwrap().hamster_position = egui::Align2::RIGHT_BOTTOM;
                    }
                });

                ui.separator();

                ui.hyperlink_to("Help!", "https://youtu.be/9oyC4ArBf1Y");
            });
    }

    pub fn draw_hamster(&mut self, ctx: &egui::Context) {
        egui::Area::new("hamster_area")
            .order(egui::Order::Background)
            .anchor(
                self.config.as_ref().unwrap().hamster_position,
                egui::Vec2::new(0., HAMSTER_OFFSET),
            )
            .show(ctx, |ui| {
                if ui
                    .add(widgets::Hamster::new(
                        self.hamster.as_ref().unwrap().clone(),
                    ))
                    .clicked()
                {
                    self.ui_state.hamster_hack = Some(HamsterHack {
                        ip: None,
                        address: {
                            let address: u32 = rand::thread_rng().gen();
                            let digest = md5::compute(address.to_be_bytes());
                            format!("{:x}", digest)
                        },
                    });
                    self.client.get_ip();
                };
            });
    }

    fn draw_hamster_hack(&self, ctx: &egui::Context, hamster_hack: &HamsterHack) {
        egui::Area::new("hamster_hack_area")
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("hack!!")
                            .size(96.)
                            .strong()
                            .color(egui::Color32::RED)
                            .background_color(egui::Color32::YELLOW),
                    );
                    ui.label(
                        egui::RichText::new("twój kompóter został zhaxowany przez chomixi box !")
                            .size(48.)
                            .underline()
                            .color(egui::Color32::DARK_BLUE)
                            .background_color(egui::Color32::RED),
                    );
                    match hamster_hack.ip.as_ref() {
                        Some(ip) => {
                            ui.label(
                                egui::RichText::new(ip)
                                    .size(64.)
                                    .color(egui::Color32::GREEN)
                                    .background_color(egui::Color32::BLUE),
                            );
                        }
                        None => {
                            ui.add_space(64.);
                            ctx.request_repaint();
                        }
                    }
                    ui.label(
                        egui::RichText::new(format!(
                            "musisz przesłać 500 żappsów na adres 0x{}",
                            hamster_hack.address
                        ))
                        .size(32.)
                        .underline()
                        .color(egui::Color32::WHITE)
                        .background_color(egui::Color32::DARK_GREEN),
                    );
                });
            });
    }
}
