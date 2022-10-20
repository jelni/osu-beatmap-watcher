use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;

#[derive(Clone, Copy, Deserialize_repr)]
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

impl Display for RankStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            RankStatus::Graveyard => "Graveyard",
            RankStatus::Wip => "WIP",
            RankStatus::Pending => "Pending",
            RankStatus::Ranked => "Ranked",
            RankStatus::Approved => "Approved",
            RankStatus::Qualified => "Qualified",
            RankStatus::Loved => "Loved",
        })
    }
}

#[derive(Deserialize)]
pub struct Beatmapset {
    pub title: String,
    pub artist: String,
    pub creator: String,
}

#[derive(Deserialize)]
pub struct Beatmap {
    pub id: u32,
    pub ranked: RankStatus,
    pub beatmapset: Beatmapset,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    ClientCredentials,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantScope {
    Public,
}

#[derive(Serialize)]
pub struct TokenGrantRequest {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: GrantType,
    pub scope: GrantScope,
}

impl TokenGrantRequest {
    pub fn with_credentials<S: Into<String>>(client_id: S, client_secret: S) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            grant_type: GrantType::ClientCredentials,
            scope: GrantScope::Public,
        }
    }
}

#[derive(Deserialize)]
pub struct TokenGrantResponse {
    pub access_token: String,
    pub expires_in: i32,
    pub token_type: String,
}
