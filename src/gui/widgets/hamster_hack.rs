use eframe::egui::{Color32, Response, RichText, Ui, Widget};

use crate::gui::HamsterHackData;

#[allow(clippy::module_name_repetitions)]
pub struct HamsterHackWidget<'a> {
    pub hamster_hack: &'a HamsterHackData,
}

impl Widget for HamsterHackWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new("hack!!")
                    .size(96.)
                    .strong()
                    .color(Color32::RED)
                    .background_color(Color32::YELLOW),
            );
            ui.label(
                RichText::new("twój kompóter został zhaxowany przez chomik box !")
                    .size(48.)
                    .underline()
                    .color(Color32::DARK_BLUE)
                    .background_color(Color32::RED),
            );
            ui.label(
                RichText::new(&self.hamster_hack.ip)
                    .size(64.)
                    .color(Color32::GREEN)
                    .background_color(Color32::BLUE),
            );
            ui.label(
                RichText::new(format!(
                    "musisz przesłać 500 żappsów na adres 0x{}",
                    self.hamster_hack.address
                ))
                .size(32.)
                .color(Color32::WHITE)
                .background_color(Color32::DARK_GREEN),
            );
            ui.button(
                RichText::new("WYŚLIJ TERAZ!!!")
                    .size(48.)
                    .color(Color32::WHITE),
            )
        })
        .inner
    }
}
