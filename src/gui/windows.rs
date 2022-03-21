use crate::gui;
use crate::osu::client::{LoginState, TaskState};
use eframe::egui;

const HAMSTER_OFFSET: f32 = 48.;

impl gui::App {
    const SETTINGS_TITLE: &'static str = "\u{26ED} Settings";

    pub fn draw(&mut self, ctx: &egui::Context) {
        self.draw_top_panel(ctx);
        self.draw_main_panel(ctx);
        self.draw_settings(ctx);
        self.draw_hamster(ctx);
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

                match self.ui_state.updater_state {
                    TaskState::Running | TaskState::Stopping => {
                        ui.add(egui::Spinner::new());
                    }
                    _ => (),
                }
                ui.label(if let Some(beatmap) = self.ui_state.beatmap.as_ref() {
                    beatmap.beatmapset.title.as_str()
                } else {
                    "None"
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
                let login_inputs_interactive = match self.ui_state.login_state {
                    LoginState::LoggedOut | LoginState::LoginError(_) => true,
                    _ => false,
                };
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

    fn draw_hamster(&self, ctx: &egui::Context) {
        egui::Area::new("hamster_area")
            .anchor(
                self.config.as_ref().unwrap().hamster_position,
                egui::Vec2::new(0., HAMSTER_OFFSET),
            )
            .show(ctx, |ui| {
                ui.add(egui::widgets::Image::new(
                    self.hamster.texture_id(ctx),
                    self.hamster.size_vec2(),
                ))
            });
    }
}
