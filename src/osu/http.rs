use crate::osu::types;

pub struct Http {
    http_client: reqwest::Client,
    pub access_token: Option<String>,
}

impl Default for Http {
    fn default() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            access_token: None,
        }
    }
}

impl Http {
    const BASE_URL: &'static str = "https://osu.ppy.sh";

    pub async fn get_access_token(
        &mut self,
        client_id: &str,
        client_secret: &str,
    ) -> Result<String, reqwest::Error> {
        let data = self
            .http_client
            .post(format!("{}/oauth/token", Self::BASE_URL))
            .json(&types::ClientCredentialsGrantRequest::from_client(
                client_id.to_string(),
                client_secret.to_string(),
            ))
            .send()
            .await?
            .json::<types::ClientCredentialsGrantResponse>()
            .await?;

        Ok(data.access_token)
    }

    pub async fn get_beatmap(&self, beatmap_id: i64) -> Result<types::Beatmap, reqwest::Error> {
        let beatmap = self
            .http_client
            .get(format!("{}/api/v2/beatmaps/{beatmap_id}", Self::BASE_URL))
            .header(
                "Authorization",
                format!("Bearer {}", self.access_token.as_ref().unwrap()),
            )
            .send()
            .await?
            .json::<types::Beatmap>()
            .await?;

        Ok(beatmap)
    }

    pub async fn get_ip(&self) -> Result<String, reqwest::Error> {
        let ip = self
            .http_client
            .get("https://ipinfo.io/ip")
            .send()
            .await?
            .text()
            .await?;

        Ok(ip)
    }
}
