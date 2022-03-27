use eframe::egui;
use image::EncodableLayout;

use crate::osu::types::*;

pub struct Http {
    http_client: reqwest::Client,
}

impl Default for Http {
    fn default() -> Self {
        Self {
            http_client: reqwest::Client::new(),
        }
    }
}

impl Http {
    const BASE_URL: &'static str = "https://osu.ppy.sh";

    pub async fn get_access_token(
        &self,
        client_id: &str,
        client_secret: &str,
    ) -> Result<String, reqwest::Error> {
        let response = self
            .http_client
            .post(format!("{}/oauth/token", Self::BASE_URL))
            .json(&TokenGrantRequest::with_credentials(
                client_id.to_string(),
                client_secret.to_string(),
            ))
            .send()
            .await?;

        let data = response
            .error_for_status()?
            .json::<TokenGrantResponse>()
            .await?;

        Ok(data.access_token)
    }

    pub async fn get_beatmap(
        &self,
        beatmap_id: u32,
        access_token: &str,
    ) -> Result<Beatmap, reqwest::Error> {
        let response = self
            .http_client
            .get(format!("{}/api/v2/beatmaps/{beatmap_id}", Self::BASE_URL))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        response.error_for_status()?.json::<Beatmap>().await
    }

    pub async fn get_beatmap_cover(
        &self,
        beatmap_id: u32,
    ) -> Result<egui::ColorImage, reqwest::Error> {
        let response = self
            .http_client
            .get(format!("https://assets.ppy.sh/beatmaps/{beatmap_id}/covers/list@2x.jpg"))
            .send()
            .await?;

        let cover = image::load_from_memory(response.error_for_status()?.bytes().await?.as_bytes())
            .unwrap();

        let cover = egui::ColorImage::from_rgba_unmultiplied(
            [cover.width() as _, cover.height() as _],
            cover.into_rgba8().as_bytes(),
        );

        Ok(cover)
    }

    pub async fn get_ip(&self) -> Result<String, reqwest::Error> {
        self.http_client
            .get("https://ipinfo.io/ip")
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
    }
}
