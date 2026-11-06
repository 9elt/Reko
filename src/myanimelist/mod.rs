mod anime;
mod list;
mod timer;

use reqwest::{header::USER_AGENT, Client, StatusCode};
use serde::Deserialize;
use std::env;
use timer::Timer;

const MAL_API: &str = "https://api.myanimelist.net/v2";

#[derive(Clone)]
pub struct MALClient {
    client_id: String,
    client: Client,
    timer: Timer,
}

impl Default for MALClient {
    fn default() -> Self {
        let client_id = env::var("MAL_CLIENT_ID")
            .expect("Missing MAL client id")
            .to_string();

        Self {
            client_id,
            client: Client::new(),
            timer: Timer::new(),
        }
    }
}

impl MALClient {
    async fn get<R: for<'a> Deserialize<'a>>(&self, url: String) -> Result<R, u16> {
        self.timer.wait().await;

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
}
