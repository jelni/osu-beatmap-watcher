use eframe::egui::{Color32, Response, RichText, Spinner, Ui, Widget};
use eframe::epaint::{TextureHandle, Vec2};

use crate::osu::types::{Beatmap, RankStatus};

#[allow(clippy::module_name_repetitions)]
pub struct BeatmapWidget<'a> {
    pub beatmap: &'a Beatmap,
    pub beatmap_cover: Option<TextureHandle>,
    pub worker_running: bool,
}

impl Widget for BeatmapWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                match self.beatmap_cover {
                    Some(beatmap_icon) => ui.image(&beatmap_icon, Vec2::splat(64.)),
                    None => ui.add(Spinner::new().size(64.)),
                };
                ui.vertical(|ui| {
                    let beatmapset = &self.beatmap.beatmapset;
                    ui.label(RichText::new(&beatmapset.title).strong());
                    ui.label(&beatmapset.artist);
                    ui.label(&beatmapset.creator);
                    ui.horizontal(|ui| {
                        if self.worker_running {
                            ui.spinner();
                        }
                        ui.label(RichText::new(format!("{}", self.beatmap.ranked)).color(
                            match self.beatmap.ranked {
                                RankStatus::Graveyard | RankStatus::Wip => Color32::GRAY,
                                RankStatus::Ranked => Color32::GREEN,
                                RankStatus::Loved => Color32::LIGHT_RED,
                                _ => Color32::WHITE,
                            },
                        ));
                    });
                })
            })
        })
        .response
    }
}
