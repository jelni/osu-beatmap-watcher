use std::sync::{mpsc, Arc};
use std::time;

use eframe::egui;
use tokio::sync::{Mutex, RwLock};

use crate::osu::{http, types};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Running,
    Stopping,
    Stopped,
}

impl Default for TaskState {
    fn default() -> Self {
        Self::Stopped
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoginState {
    LoggedOut,
    LoggedIn,
    LoggingIn,
    LoginError(String),
}

impl Default for LoginState {
    fn default() -> Self {
        Self::LoggedOut
    }
}

pub enum Update {
    UpdaterState(TaskState),
    LoginState(LoginState),
    Beatmap(Option<types::Beatmap>),
    BeatmapCover(Option<egui::ColorImage>),
    Ip(String),
}

pub struct Client {
    http: Arc<http::Http>,
    updater_state: Arc<Mutex<TaskState>>,
    access_token: Arc<RwLock<Option<String>>>,
    tx: mpsc::Sender<Update>,
    rx: mpsc::Receiver<Update>,
    rt: tokio::runtime::Runtime,
}

impl Default for Client {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            http: Arc::new(http::Http::default()),
            updater_state: Arc::new(Mutex::new(TaskState::default())),
            access_token: Arc::new(RwLock::new(None)),
            tx,
            rx,
            rt: tokio::runtime::Runtime::new().unwrap(),
        }
    }
}

impl Client {
    pub fn log_in(&mut self, client_id: String, client_secret: String) {
        self.tx
            .send(Update::LoginState(LoginState::LoggingIn))
            .unwrap();

        let http = self.http.clone();
        let tx = self.tx.clone();
        let access_token = self.access_token.clone();

        self.rt.spawn(async move {
            let logged_in = match http
                .get_access_token(client_id.as_str(), client_secret.as_str())
                .await
            {
                Ok(token) => {
                    *access_token.write().await = Some(token);
                    LoginState::LoggedIn
                }
                Err(err) => {
                    *access_token.write().await = None;
                    LoginState::LoginError(err.to_string())
                }
            };
            tx.send(Update::LoginState(logged_in)).unwrap();
        });
    }

    pub fn log_out(&mut self) {
        self.tx
            .send(Update::LoginState(LoginState::LoggingIn))
            .unwrap();

        let tx = self.tx.clone();
        let access_token = self.access_token.clone();

        self.rt.spawn(async move {
            *access_token.write().await = None;
            tx.send(Update::LoginState(LoginState::LoggedOut)).unwrap();
        });
    }

    pub fn start_updating_beatmap(&self, beatmap_id: u32) {
        self.tx
            .send(Update::UpdaterState(TaskState::Running))
            .unwrap();
        self.tx.send(Update::Beatmap(None)).unwrap();
        self.tx.send(Update::BeatmapCover(None)).unwrap();

        let http = self.http.clone();
        let updater_state = self.updater_state.clone();
        let tx = self.tx.clone();
        let access_token = self.access_token.clone();

        self.rt.spawn(async move {
            *updater_state.lock().await = TaskState::Running;

            while *updater_state.lock().await == TaskState::Running {
                let access_token = access_token.read().await;
                // TODO Return error
                let access_token = access_token.as_ref().expect("access token not set");

                match http.get_beatmap(beatmap_id, access_token).await {
                    Ok(beatmap) => {
                        let ranked = beatmap.ranked;
                        tx.send(Update::Beatmap(Some(beatmap))).unwrap();
                        if matches!(
                            ranked,
                            types::RankStatus::Graveyard
                                | types::RankStatus::Wip
                                | types::RankStatus::Ranked
                                | types::RankStatus::Loved
                        ) {
                            break;
                        }
                    }
                    Err(err) => {
                        tx.send(Update::Beatmap(None)).unwrap();
                        eprintln!("{err:?}");
                        break;
                    }
                }
                tokio::time::sleep(time::Duration::SECOND).await;
            }

            tx.send(Update::UpdaterState(TaskState::Stopped)).unwrap();
        });
    }

    pub fn stop_updating_beatmap(&self) {
        self.tx
            .send(Update::UpdaterState(TaskState::Stopping))
            .unwrap();

        let updater_state = self.updater_state.clone();

        self.rt.spawn(async move {
            *updater_state.lock().await = TaskState::Stopping;
        });
    }

    pub fn get_beatmap_cover(&self, beatmap_id: u32) {
        let http = self.http.clone();
        let tx = self.tx.clone();

        self.rt.spawn(async move {
            match http.get_beatmap_cover(beatmap_id).await {
                Ok(cover) => tx.send(Update::BeatmapCover(Some(cover))).unwrap(),
                Err(err) => {
                    // tx.send(Update::BeatmapCover(colo));
                    eprintln!("{err:?}");
                }
            }
        });
    }

    pub fn get_ip(&self) {
        let http = self.http.clone();
        let tx = self.tx.clone();
        self.rt.spawn(async move {
            if let Ok(ip) = http.get_ip().await {
                tx.send(Update::Ip(ip)).unwrap();
            }
        });
    }

    pub fn poll_updates(&mut self) -> mpsc::TryIter<Update> {
        self.rx.try_iter()
    }
}
