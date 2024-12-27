use std::{error::Error, fmt::Display};

use log::{debug, error, info, warn};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

const BASE_URL: &str = "https://discord.com/api/v10";
const LOGIN_TOKEN_PATH: &str = "./login_token";

#[derive(serde::Deserialize)]
struct FernLoginResponseUserSettings {
    /// The language option chosen by the user
    locale: String,
    /// The client theme chosen by the user
    theme: String,
}

#[derive(serde::Deserialize)]
pub struct FernLoginResponse {
    /// The ID of the user that was logged in
    user_id: String,
    /// The authentication token, if the login was completed
    token: Option<String>,
    /// The user's partial settings, if the login was completed
    user_settings: Option<FernLoginResponseUserSettings>,
    /// The required actions that must be completed before continuing
    required_actions: Option<Vec<String>>,
    /// A ticket to be used in the mfa flow
    ticket: Option<String>,
    /// Wether mfa is required to login
    mfa: Option<bool>,
    /// Wether totp-mfa is enabled
    totp: Option<bool>,
    /// Wether sms-mfa is enabled
    sms: Option<bool>,
    /// Wether backup codes can be used for mfa
    backup: Option<bool>,
    /// The stringified JSON public key credential request options
    /// challenge for WebAuthn
    webauthn: Option<String>,
}

#[derive(Debug)]
pub struct FernLoginError(reqwest::blocking::Response);
impl Display for FernLoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FernLoginError: {:?}", self.0)
    }
}
impl Error for FernLoginError {}

pub fn login(username: String, password: String) -> Result<FernLoginResponse, Box<dyn Error>> {
    let mut response = reqwest::blocking::Client::new()
        .post(format!("{}{}", BASE_URL, "/auth/login"))
        .header(CONTENT_TYPE, "application/json");

    if let Ok(tok) = std::fs::read(LOGIN_TOKEN_PATH) {
        info!("Found login token");
        response = response.header(AUTHORIZATION, tok);
    }
    let response = response
        .body(format!(
            "{{ \"login\": \"{username}\", \"password\": \"{password}\" }}"
        ))
        .send()?;
    if response.status() == 200 {
        debug!(
            "Server responded to login attempt with status {}",
            response.status()
        );

        let flr_string = &response.text()?;
        let flr = serde_json::from_str::<FernLoginResponse>(flr_string)?;
        match std::fs::write(LOGIN_TOKEN_PATH, flr.token.as_ref().unwrap()) {
            Ok(_) => info!("Login token cached in {}", LOGIN_TOKEN_PATH),
            Err(e) => warn!("Could not cache token in {}: {}", LOGIN_TOKEN_PATH, e),
        }
        Ok(flr)
    } else {
        error!(
            "Server responded to login attempt with status {}",
            response.status()
        );
        Err(Box::new(FernLoginError(response)))
    }
}
