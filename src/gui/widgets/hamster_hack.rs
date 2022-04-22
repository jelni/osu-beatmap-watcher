use eframe::egui;
use egui::Widget;

use crate::gui::HamsterHack;

pub struct DrawHamsterHack<'a> {
    hamster_hack: &'a HamsterHack,
}

impl<'a> DrawHamsterHack<'a> {
    pub fn new(data: &'a HamsterHack) -> Self {
        Self { hamster_hack: data }
    }
}

impl Widget for DrawHamsterHack<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        if self.hamster_hack.ip.is_none() {
            ui.ctx().request_repaint();
        }

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
            ui.label(
                egui::RichText::new(self.hamster_hack.ip.as_deref().unwrap_or_default())
                    .size(64.)
                    .color(egui::Color32::GREEN)
                    .background_color(egui::Color32::BLUE),
            );
            ui.label(
                egui::RichText::new(format!(
                    "musisz przesłać 500 żappsów na adres 0x{}",
                    self.hamster_hack.address
                ))
                .size(32.)
                .underline()
                .color(egui::Color32::WHITE)
                .background_color(egui::Color32::DARK_GREEN),
            );
            ui.button(
                egui::RichText::new("WYŚLIJ TERAZ!!!")
                    .size(48.)
                    .underline()
                    .color(egui::Color32::BLACK)
                    .background_color(egui::Color32::WHITE),
            )
        })
        .inner
    }
}
