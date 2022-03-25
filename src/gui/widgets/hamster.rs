use eframe::egui;
use eframe::epaint::TextureHandle;
use egui::Widget;

pub struct Hamster {
    hamster: TextureHandle,
}

impl Hamster {
    pub fn new(hamster: TextureHandle) -> Self {
        Self { hamster }
    }
}

impl Widget for Hamster {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add(
            egui::Image::new(&self.hamster, self.hamster.size_vec2()).sense(egui::Sense::click()),
        )
    }
}
