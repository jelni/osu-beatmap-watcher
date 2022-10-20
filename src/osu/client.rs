use std::sync::mpsc;
use std::time;

use eframe::epaint::ColorImage;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

use super::http::Http;
use super::types::RankStatus;
use crate::osu::types;

pub enum LoginState {
    LoggedOut,
    LoggedIn { access_token: String },
    LoggingIn,
    LoginError(String),
}

pub enum Update {
    LoginState(LoginState),
    Beatmap(Option<types::Beatmap>),
    BeatmapCover(Option<ColorImage>),
}

pub struct Client {
    http: Http,
    tx: mpsc::Sender<Update>,
    rx: mpsc::Receiver<Update>,
    rt: Runtime,
}

impl Default for Client {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            http: Http::new(),
            tx,
            rx,
            rt: Runtime::new().unwrap(),
        }
    }
}

impl Client {
    pub fn log_in(&self, client_id: String, client_secret: String) {
        self.tx
            .send(Update::LoginState(LoginState::LoggingIn))
            .unwrap();

        let http = self.http.clone();
        let tx = self.tx.clone();

        self.rt.spawn(async move {
            let logged_in = match http.get_access_token(client_id, client_secret).await {
                Ok(access_token) => LoginState::LoggedIn { access_token },
                Err(err) => LoginState::LoginError(
                    err.status()
                        .map_or_else(|| "Network error".to_string(), |e| e.to_string()),
                ),
            };
            tx.send(Update::LoginState(logged_in)).unwrap();
        });
    }

    pub fn poll_beatmap(&self, access_token: String, beatmap_id: u32) -> JoinHandle<()> {
        self.tx.send(Update::Beatmap(None)).unwrap();

        let http = self.http.clone();
        let tx = self.tx.clone();

        self.rt.spawn(async move {
            loop {
                match http.get_beatmap(beatmap_id, &access_token).await {
                    Ok(beatmap) => {
                        if let Some(beatmap) = beatmap {
                            let ranked = beatmap.ranked;
                            tx.send(Update::Beatmap(Some(beatmap))).unwrap();
                            if matches!(
                                ranked,
                                RankStatus::Graveyard
                                    | RankStatus::Wip
                                    | RankStatus::Ranked
                                    | RankStatus::Loved
                            ) {
                                break;
                            }
                        } else {
                            tx.send(Update::Beatmap(None)).unwrap();
                            break;
                        }
                    }
                    Err(err) => {
                        tx.send(Update::Beatmap(None)).unwrap();
                        eprintln!("{err:?}");
                        break;
                    }
                }
                tokio::time::sleep(time::Duration::from_millis(1000)).await;
            }
        })
    }

    pub fn get_beatmap_cover(&self, beatmap_id: u32) {
        self.tx.send(Update::BeatmapCover(None)).unwrap();

        let http = self.http.clone();
        let tx = self.tx.clone();

        self.rt.spawn(async move {
            match http.get_beatmap_cover(beatmap_id).await {
                Ok(cover) => tx.send(Update::BeatmapCover(cover)).unwrap(),
                Err(err) => {
                    tx.send(Update::BeatmapCover(None)).unwrap();
                    eprintln!("{err:?}");
                }
            }
        });
    }

    pub fn poll_updates(&self) -> mpsc::TryIter<Update> {
        self.rx.try_iter()
    }
}
