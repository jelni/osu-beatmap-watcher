use eframe::egui;
use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr)]
#[repr(i8)]
pub enum RankStatus {
    Graveyard = -2,
    Wip = -1,
    Pending = 0,
    Ranked = 1,
    Approved = 2,
    Qualified = 3,
    Loved = 4,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Beatmapset {
    pub creator: String,
    pub artist: String,
    pub title: String,
}

#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct Beatmap {
    pub id: u32,
    pub ranked: RankStatus,
    pub beatmapset: Beatmapset,
    #[serde(skip)]
    pub cover: Option<egui::TextureHandle>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    ClientCredentials,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantScope {
    Public,
}

#[derive(Debug, Clone, Serialize)]
pub struct TokenGrantRequest {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: GrantType,
    pub scope: GrantScope,
}

impl TokenGrantRequest {
    pub fn with_credentials(client_id: String, client_secret: String) -> Self {
        Self {
            client_id,
            client_secret,
            grant_type: GrantType::ClientCredentials,
            scope: GrantScope::Public,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenGrantResponse {
    pub access_token: String,
    pub expires_in: i32,
    pub token_type: String,
}
