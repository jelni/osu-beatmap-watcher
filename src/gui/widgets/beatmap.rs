use eframe::egui::{self, RichText};
use osu_beatmap_watcher::osu::types::RankStatus;

use crate::osu::client::TaskState;
use crate::osu::types::Beatmap;

pub struct DrawBeatmap<'a> {
    beatmap: &'a Beatmap,
    updater_state: &'a TaskState,
}

impl<'a> DrawBeatmap<'a> {
    pub fn new(beatmap: &'a Beatmap, updater_state: &'a TaskState) -> Self {
        Self {
            beatmap,
            updater_state,
        }
    }
}

impl egui::Widget for DrawBeatmap<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                match self.beatmap.cover.as_ref() {
                    Some(beatmap_icon) => ui.image(beatmap_icon, egui::Vec2::splat(64.)),
                    None => ui.add(egui::Spinner::new().size(64.)),
                };
                ui.vertical(|ui| {
                    let artist = self.beatmap.beatmapset.artist.as_str();
                    let title = self.beatmap.beatmapset.title.as_str();
                    ui.label(format!("{artist} — {title}"));
                    ui.label(self.beatmap.beatmapset.creator.as_str());
                    ui.label(RichText::new(format!("{:?}", self.beatmap.ranked)).color(
                        match self.beatmap.ranked {
                            RankStatus::Ranked => egui::Color32::GREEN,
                            _ => egui::Color32::WHITE,
                        },
                    ));
                    if *self.updater_state == TaskState::Running {
                        ui.spinner();
                    }
                })
            })
        })
        .response
    }
}
