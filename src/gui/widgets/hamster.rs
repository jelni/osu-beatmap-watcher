use eframe::egui::{Image, Response, Sense, Ui, Widget};
use eframe::epaint::TextureHandle;

#[allow(clippy::module_name_repetitions)]
pub struct HamsterWidget {
    pub hamster: TextureHandle,
}

impl Widget for HamsterWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.add(Image::new(&self.hamster, self.hamster.size_vec2()).sense(Sense::click()))
    }
}
