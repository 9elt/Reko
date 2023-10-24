mod anime;
mod list;
mod timer;

use reqwest::{header::USER_AGENT, Client, StatusCode};
use serde::Deserialize;
use std::env;
use std::fs;
use timer::{Timer, WrappedTimer};

const MAL_API: &str = "https://api.myanimelist.net/v2";

#[derive(Clone)]
pub struct MALClient {
    client_id: String,
    client: Client,
    config: ClientConfig,
    timer: WrappedTimer,
}

impl MALClient {
    pub fn new() -> Self {
        let client_id = env::var("MAL_CLIENT_ID")
            .expect("Missing MAL client id")
            .to_string();

        Self {
            client_id,
            client: Client::new(),
            config: ClientConfig::new(),
            timer: Timer::new(),
        }
    }
    async fn get<R: for<'a> Deserialize<'a>>(&self, url: String) -> Result<R, u16> {
        if self.config.enable_fake_api {
            println!("FAKE {}", &url);
            return self.fake_api(url);
        }

        // enforces max mal requests per second across threads
        // the lock is released when `timer` goes out of scope
        let mut timer = self.timer.lock().await;
        timer.sleep();

        let res = self
            .client
            .get(format!("{MAL_API}{url}"))
            .header(USER_AGENT, "reqwest")
            .header("x-mal-client-id", self.client_id.as_str())
            .send()
            .await;

        match res {
            Ok(res) => match res.status() {
                StatusCode::OK => {
                    println!("GET {} OK", &url);
                    match res.json::<R>().await {
                        Ok(res) => Ok(res),
                        _ => Err(422),
                    }
                }
                code => {
                    let code = code.as_u16();
                    println!("GET {} ERR {}", &url, code);
                    Err(code)
                }
            },
            _ => Err(500),
        }
    }
    /// for development use
    /// requires ENABLE_FAKE_API="true" and FAKE_API_PATH envs
    ///
    /// parses files at:
    /// FAKE_API_PATH/anime/{id}.json
    /// or
    /// FAKE_API_PATH/users/{username}.json
    fn fake_api<R: for<'a> Deserialize<'a>>(&self, url: String) -> Result<R, u16> {
        let mut p = url.split("/");
        p.next();

        let rt = p.next().unwrap_or("");
        let id = p.next().unwrap_or("").split("?").next().unwrap_or("");

        match fs::read_to_string(format!("{}/{}/{}.json", self.config.fake_api_path, rt, id)) {
            Ok(f) => match serde_json::from_str::<R>(&f) {
                Ok(anime) => Ok(anime),
                Err(_) => Err(422),
            },
            Err(_) => Err(404),
        }
    }
}

#[derive(Clone)]
struct ClientConfig {
    enable_fake_api: bool,
    fake_api_path: String,
}

impl ClientConfig {
    fn new() -> Self {
        ClientConfig {
            enable_fake_api: env::var("ENABLE_FAKE_API").unwrap_or(String::new()) == "true",
            fake_api_path: env::var("FAKE_API_PATH").unwrap_or(String::new()),
        }
    }
}
