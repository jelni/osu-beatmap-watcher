use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize_repr)]
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

#[derive(Debug, serde::Deserialize)]
pub struct Beatmapset {
    pub artist: String,
    pub title: String,
    pub creator: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Beatmap {
    pub ranked: RankStatus,
    pub beatmapset: Beatmapset,
}

#[derive(serde::Serialize)]
pub struct ClientCredentialsGrantRequest {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: String,
    pub scope: String,
}

impl ClientCredentialsGrantRequest {
    pub fn from_client(client_id: String, client_secret: String) -> Self {
        Self {
            client_id,
            client_secret,
            grant_type: String::from("client_credentials"),
            scope: String::from("public"),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct ClientCredentialsGrantResponse {
    pub access_token: String,
    pub expires_in: i32,
    pub token_type: String,
}
