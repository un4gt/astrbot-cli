use crate::config::{Config, ConfigManager};
use crate::{iprintln, vprintln};
use reqwest;
use serde::{Deserialize, Serialize};
use std::process;

#[derive(Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    status: String,
    message: Option<String>,
    data: Option<LoginData>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct LoginData {
    token: String,
    username: String,
    change_pwd_hint: bool,
}

pub async fn handle_login(username: String, password: String, url: String) {
    iprintln!("Starting login process...");
    vprintln!("Server: {}", url);
    vprintln!("Username: {}", username);

    // MD5 encrypt the password
    let password_hash = format!("{:x}", md5::compute(password.as_bytes()));

    // Prepare the login request
    let login_request = LoginRequest {
        username: username.clone(),
        password: password_hash,
    };

    // Construct the full API URL
    let api_url = format!("{}/api/auth/login", url.trim_end_matches('/'));
    vprintln!("API URL: {}", api_url);

    // Create HTTP client
    let client = reqwest::Client::new();

    // Send POST request
    vprintln!("Sending login request...");
    match client.post(&api_url).json(&login_request).send().await {
        Ok(response) => {
            let status_code = response.status();
            vprintln!("Response received: HTTP {}", status_code);

            match response.json::<LoginResponse>().await {
                Ok(login_response) => {
                    if login_response.status == "ok" {
                        if let Some(data) = login_response.data {
                            iprintln!("Login successful!");

                            // Create credentials object
                            let credentials = Config {
                                token: data.token.clone(),
                                server_url: url.clone(),
                                username: data.username.clone(),
                            };

                            // Save to persistent config file
                            if let Err(e) = ConfigManager::save_credentials(&credentials) {
                                eprintln!("Warning: Failed to save to config file: {}", e);
                            }

                            iprintln!("Login complete! You can now use other commands.");
                        } else {
                            eprintln!("❌ Error: Login response missing data");
                            process::exit(1);
                        }
                    } else {
                        let error_msg = login_response
                            .message
                            .unwrap_or("Unknown error".to_string());
                        eprintln!("❌ Login failed: {}", error_msg);

                        process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error parsing login response: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error sending login request: {}", e);
            process::exit(1);
        }
    }
}
