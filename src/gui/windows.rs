use crate::gui;
use crate::osu::client::LoggedInState;
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
            ui.add_enabled_ui(!self.config_open, |ui| {
                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    if ui.button(Self::SETTINGS_TITLE).clicked() {
                        self.config_open = true;
                    }
                });
            });
        });
    }

    fn draw_main_panel(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_enabled_ui(!self.config_open, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label(&self.config.as_ref().unwrap().client_id);
                });
            });
        });
    }

    fn draw_settings(&mut self, ctx: &egui::Context) {
        egui::Window::new(Self::SETTINGS_TITLE)
            .open(&mut self.config_open)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0., 0.))
            .collapsible(false)
            .auto_sized()
            .show(ctx, |ui| {
                egui::Grid::new("settings_grid")
                    .min_col_width(128.)
                    .show(ui, |ui| {
                        let is_logged_out = self.client.state.logged_in == LoggedInState::LoggedOut;
                        ui.label("Client ID: ");
                        ui.add(
                            egui::TextEdit::singleline(
                                &mut self.config.as_mut().unwrap().client_id,
                            )
                            .interactive(is_logged_out)
                            .hint_text("client_id"),
                        );
                        ui.end_row();

                        ui.label("Client Secret: ");
                        ui.add(
                            egui::TextEdit::singleline(
                                &mut self.config.as_mut().unwrap().client_secret,
                            )
                            .password(true)
                            .interactive(is_logged_out)
                            .hint_text("client_secret"),
                        );
                        ui.end_row();

                        ui.horizontal(|ui| match self.client.state.logged_in {
                            LoggedInState::LoggedOut => {
                                if ui.button("\u{27A1} Log In").clicked() {
                                    let config = self.config.as_ref().unwrap();
                                    self.client.log_in(
                                        config.client_id.clone(),
                                        config.client_secret.clone(),
                                    );
                                }
                            }
                            LoggedInState::LoggingIn => {
                                ui.add(egui::Spinner::new());
                                ui.label("Logging In…");
                            }
                            LoggedInState::LoggedIn => {
                                if ui.button("\u{2B05} Log Out").clicked() {
                                    self.client.log_out();
                                }
                            }
                        });
                        ui.end_row();

                        ui.spacing();
                        ui.end_row();

                        ui.label("Theme: ");
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
                        ui.end_row();

                        ui.label("Handedness: ");
                        ui.horizontal(|ui| {
                            if ui
                                .selectable_label(
                                    self.config.as_mut().unwrap().hamster_position
                                        == egui::Align2::LEFT_BOTTOM,
                                    "\u{25C0} Left-Handed",
                                )
                                .clicked()
                            {
                                self.config.as_mut().unwrap().hamster_position =
                                    egui::Align2::LEFT_BOTTOM;
                            }
                            if ui
                                .selectable_label(
                                    self.config.as_mut().unwrap().hamster_position
                                        == egui::Align2::RIGHT_BOTTOM,
                                    "\u{25B6} Right-Handed",
                                )
                                .clicked()
                            {
                                self.config.as_mut().unwrap().hamster_position =
                                    egui::Align2::RIGHT_BOTTOM;
                            }
                        });
                        ui.end_row();

                        ui.hyperlink_to("Help!", "https://youtu.be/9oyC4ArBf1Y");
                        ui.end_row();
                    });
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
