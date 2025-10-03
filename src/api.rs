use std::collections::HashMap;

use futures::stream::StreamExt;
use reqwest::{multipart, Method};
use reqwest_eventsource::{Event, EventSource};
use serde::{de::DeserializeOwned, Deserialize};

use crate::{iprintln, plugin::Plugin, stat::Stat, vprintln};

#[derive(Deserialize)]
#[allow(dead_code)]
struct LiveLogMessage {
    #[serde(rename = "type")]
    typo: String,
    level: String,
    time: String,
    data: String,
}

#[derive(Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub status: String,
    #[serde(default, deserialize_with = "null_to_empty_string")]
    pub message: String,
    pub data: Option<T>,
}

fn null_to_empty_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

impl<T> ApiResponse<T> {
    fn is_ok(&self) -> bool {
        self.status.eq_ignore_ascii_case("ok")
    }
}

pub struct ApiClient {
    base_url: String,
    client: reqwest::Client,
    token: String,
}

impl ApiClient {
    pub fn new(base_url: String, token: String) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: reqwest::Client::new(),
            token,
        }
    }

    fn endpoint(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path.trim_start_matches('/'))
    }

    fn token_preview(&self) -> String {
        let mut s: String = self.token.chars().take(8).collect();
        s.push_str("…");
        s
    }

    fn request(&self, method: Method, path: &str) -> reqwest::RequestBuilder {
        let url = self.endpoint(path);
        vprintln!("{} {}", method.as_str(), url);
        vprintln!("Token: {}", self.token_preview());

        let builder = self.client.request(method, url);
        builder.header("Authorization", format!("Bearer {}", self.token))
    }

    async fn send_and_parse<T: DeserializeOwned>(
        &self,
        builder: reqwest::RequestBuilder,
    ) -> anyhow::Result<ApiResponse<T>> {
        let response = builder.send().await?;
        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            let snippet = text.chars().take(256).collect::<String>();
            anyhow::bail!("API request failed: HTTP {}. Body: {}", status, snippet);
        }

        let raw_json: ApiResponse<T> = serde_json::from_str(&text)?;
        vprintln!("ApiResponse.status : {}", raw_json.status);
        vprintln!("ApiResponse.message: {}", raw_json.message);

        Ok(raw_json)
    }

    pub async fn get_plugins(&self) -> anyhow::Result<Vec<Plugin>> {
        let resp = self
            .send_and_parse::<Vec<Plugin>>(self.request(Method::GET, "/api/plugin/get"))
            .await?;

        if resp.is_ok() {
            Ok(resp.data.unwrap_or_default())
        } else {
            anyhow::bail!("API error: {}", resp.message);
        }
    }

    pub async fn install_local_plugin(&self, local_plugin: &String) -> anyhow::Result<String> {
        let form = multipart::Form::new().file("file", local_plugin).await?;

        let resp = self
            .send_and_parse::<serde_json::Value>(
                self.request(Method::POST, "/api/plugin/install-upload")
                    .multipart(form),
            )
            .await?;

        if resp.is_ok() {
            Ok(resp.message)
        } else {
            anyhow::bail!("API error: {}", resp.message);
        }
    }

    pub async fn install_remote_plugin(&self, remote_plugin: &String) -> anyhow::Result<String> {
        let mut body: HashMap<&str, &str> = HashMap::with_capacity(2);
        body.insert("proxy", "");
        body.insert("url", remote_plugin);
        let resp = self
            .send_and_parse::<serde_json::Value>(
                self.request(Method::POST, "/api/plugin/install")
                    .json(&body),
            )
            .await?;

        if resp.is_ok() {
            Ok(resp.message)
        } else {
            anyhow::bail!("API error: {}", resp.message);
        }
    }

    pub async fn plugin_common_actions_request(
        &self,
        plugin_name: &str,
        action_name: &str,
    ) -> anyhow::Result<String> {
        let mut body: HashMap<&str, &str> = HashMap::with_capacity(1);
        body.insert("name", plugin_name);
        let path = format!("/api/plugin/{}", action_name);
        let resp = self
            .send_and_parse::<serde_json::Value>(self.request(Method::POST, &path).json(&body))
            .await?;

        if resp.is_ok() {
            Ok(resp.message)
        } else {
            anyhow::bail!("API error: {}", resp.message);
        }
    }

    pub async fn get_stat(&self) -> anyhow::Result<Stat> {
        let resp = self
            .send_and_parse::<Stat>(self.request(Method::GET, "/api/stat/get"))
            .await?;

        if resp.is_ok() {
            match resp.data {
                Some(stat) => Ok(stat),
                None => anyhow::bail!("No data received"),
            }
        } else {
            anyhow::bail!("API error: {}", resp.message);
        }
    }

    pub async fn get_live_log(&self) -> anyhow::Result<()> {
        let request_builder = self.request(Method::GET, "api/live-log");
        let mut es = EventSource::new(request_builder)?;
        while let Some(event) = es.next().await {
            match event {
                Ok(ev) => match ev {
                    Event::Open => {
                        iprintln!("Start to print live log");
                    }
                    Event::Message(message) => {
                        let message: LiveLogMessage = serde_json::from_str(&message.data)?;
                        println!("{}", message.data);
                    }
                },
                Err(err) => {
                    eprintln!("Error: {}", err);
                    es.close();
                }
            }
        }
        Ok(())
    }
}
