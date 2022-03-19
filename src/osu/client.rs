use crate::osu::http;
use crate::osu::types;
use std::sync::mpsc;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(PartialEq)]
pub enum LoggedInState {
    LoggedOut,
    LoggingIn,
    LoggedIn,
}

impl Default for LoggedInState {
    fn default() -> Self {
        LoggedInState::LoggedOut
    }
}

#[derive(Default)]
pub struct State {
    pub logged_in: LoggedInState,
    pub beatmap: Option<types::Beatmap>,
    pub ip: Option<String>,
}

enum Message {
    LoggedInState(LoggedInState),
    ShowIp(String),
}

pub struct Client {
    http: Arc<Mutex<http::Http>>,
    tx: mpsc::Sender<Message>,
    rx: mpsc::Receiver<Message>,
    rt: tokio::runtime::Runtime,
    pub state: State,
}

impl Default for Client {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            http: Arc::new(Mutex::new(http::Http::default())),
            tx,
            rx,
            rt: tokio::runtime::Runtime::new().unwrap(),
            state: State::default(),
        }
    }
}

impl Client {
    pub fn log_in(&mut self, client_id: String, client_secret: String) {
        let http = self.http.clone();
        let tx = self.tx.clone();
        tx.send(Message::LoggedInState(LoggedInState::LoggingIn))
            .unwrap();
        self.rt.spawn(async move {
            let mut http = http.lock().await;
            let logged_in = if let Ok(access_token) = http
                .get_access_token(client_id.as_str(), client_secret.as_str())
                .await
            {
                http.access_token = Some(access_token);
                LoggedInState::LoggedIn
            } else {
                http.access_token = None;
                LoggedInState::LoggedOut
            };
            tx.send(Message::LoggedInState(logged_in)).unwrap();
        });
    }

    pub fn log_out(&mut self) {
        let http = self.http.clone();
        let tx = self.tx.clone();
        tx.send(Message::LoggedInState(LoggedInState::LoggingIn))
            .unwrap();
        self.rt.spawn(async move {
            http.lock().await.access_token = None;
            tx.send(Message::LoggedInState(LoggedInState::LoggedOut))
                .unwrap();
        });
    }

    pub async fn get_ip(&self) {
        let http = self.http.clone();
        let tx = self.tx.clone();
        self.rt.spawn(async move {
            if let Ok(ip) = http.lock().await.get_ip().await {
                tx.send(Message::ShowIp(ip)).unwrap();
            }
        });
    }

    pub fn update_state(&mut self) {
        for message in self.rx.try_iter() {
            match message {
                Message::LoggedInState(state) => self.state.logged_in = state,
                Message::ShowIp(ip) => self.state.ip = Some(ip),
            }
        }
    }
}
