use crate::osu::http;
use crate::osu::types;
use core::time;
use std::sync::Arc;
use std::{ops::Deref, sync::mpsc};
use tokio::sync::Mutex;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
pub enum Message {
    NewProgramState(TaskState),
    NewLoginState(LoginState),
    NewBeatmap(Option<types::Beatmap>),
    NewIp(String),
}

pub struct Client {
    http: Arc<Mutex<http::Http>>,
    updater_state: Arc<Mutex<TaskState>>,
    tx: mpsc::Sender<Message>,
    rx: mpsc::Receiver<Message>,
    rt: tokio::runtime::Runtime,
}

impl Default for Client {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            http: Arc::new(Mutex::new(http::Http::default())),
            updater_state: Arc::new(Mutex::new(TaskState::default())),
            tx,
            rx,
            rt: tokio::runtime::Runtime::new().unwrap(),
        }
    }
}

impl Client {
    pub fn log_in(&mut self, client_id: String, client_secret: String) {
        let http = self.http.clone();
        let tx = self.tx.clone();
        tx.send(Message::NewLoginState(LoginState::LoggingIn))
            .unwrap();
        self.rt.spawn(async move {
            let mut http = http.lock().await;
            let logged_in = match http
                .get_access_token(client_id.as_str(), client_secret.as_str())
                .await
            {
                Ok(access_token) => {
                    http.access_token = Some(access_token);
                    LoginState::LoggedIn
                }
                Err(err) => {
                    http.access_token = None;
                    LoginState::LoginError(err.to_string())
                }
            };
            tx.send(Message::NewLoginState(logged_in)).unwrap();
        });
    }

    pub fn log_out(&mut self) {
        self.tx
            .send(Message::NewLoginState(LoginState::LoggingIn))
            .unwrap();

        let http = self.http.clone();
        let tx = self.tx.clone();

        self.rt.spawn(async move {
            http.lock().await.access_token = None;
            tx.send(Message::NewLoginState(LoginState::LoggedOut))
                .unwrap();
        });
    }

    pub fn start_updating_beatmap(&self, beatmap_id: u32) {
        self.tx
            .send(Message::NewProgramState(TaskState::Running))
            .unwrap();

        let http = self.http.clone();
        let updater_state = self.updater_state.clone();
        let tx = self.tx.clone();

        self.rt.spawn(async move {
            *updater_state.lock().await = TaskState::Running;

            while let TaskState::Running = updater_state.lock().await.deref() {
                if let Ok(beatmap) = http.lock().await.get_beatmap(beatmap_id).await {
                    tx.send(Message::NewBeatmap(Some(beatmap))).unwrap();
                } else {
                    tx.send(Message::NewBeatmap(None)).unwrap();
                    break;
                };
                tokio::time::sleep(time::Duration::SECOND).await;
            }

            tx.send(Message::NewProgramState(TaskState::Stopped))
                .unwrap();
        });
    }

    pub fn stop_updating_beatmap(&self) {
        self.tx
            .send(Message::NewProgramState(TaskState::Stopping))
            .unwrap();

        let updater_state = self.updater_state.clone();

        self.rt.spawn(async move {
            *updater_state.lock().await = TaskState::Stopping;
        });
    }

    pub fn get_ip(&self) {
        let http = self.http.clone();
        let tx = self.tx.clone();
        self.rt.spawn(async move {
            if let Ok(ip) = http.lock().await.get_ip().await {
                tx.send(Message::NewIp(ip)).unwrap();
            }
        });
    }

    pub fn poll_updates(&mut self) -> mpsc::TryIter<Message> {
        self.rx.try_iter()
    }
}
