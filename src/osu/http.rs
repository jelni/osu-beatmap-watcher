use eframe::epaint::ColorImage;
use image::{EncodableLayout, ImageFormat};
use reqwest::StatusCode;

use crate::osu::types::{Beatmap, TokenGrantRequest, TokenGrantResponse};

#[derive(Clone)]
pub struct Http {
    http_client: reqwest::Client,
}

impl Http {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
        }
    }
}

impl Http {
    const BASE_URL: &str = "https://osu.ppy.sh";

    pub async fn get_access_token<S: Into<String>>(
        &self,
        client_id: S,
        client_secret: S,
    ) -> Result<String, reqwest::Error> {
        let response = self
            .http_client
            .post(format!("{}/oauth/token", Self::BASE_URL))
            .json(&TokenGrantRequest::with_credentials(
                client_id,
                client_secret,
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
        access_token: impl AsRef<str>,
    ) -> Result<Option<Beatmap>, reqwest::Error> {
        let response = self
            .http_client
            .get(format!("{}/api/v2/beatmaps/{beatmap_id}", Self::BASE_URL))
            .header("Authorization", format!("Bearer {}", access_token.as_ref()))
            .send()
            .await?;

        if response.status() != StatusCode::OK {
            return Ok(None);
        }

        Ok(Some(response.json::<Beatmap>().await?))
    }

    pub async fn get_beatmap_cover(
        &self,
        beatmap_id: u32,
    ) -> Result<Option<ColorImage>, reqwest::Error> {
        let response = self
            .http_client
            .get(format!(
                "https://assets.ppy.sh/beatmaps/{beatmap_id}/covers/list.jpg"
            ))
            .send()
            .await?;

        if response.status() != StatusCode::OK {
            return Ok(None);
        }

        let cover = image::load_from_memory_with_format(
            response.bytes().await?.as_bytes(),
            ImageFormat::Jpeg,
        )
        .unwrap();

        let cover = ColorImage::from_rgba_unmultiplied(
            [
                cover.width().try_into().unwrap(),
                cover.height().try_into().unwrap(),
            ],
            cover.into_rgba8().as_bytes(),
        );

        Ok(Some(cover))
    }
}
