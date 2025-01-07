use std::sync::Arc;

use log::{error, trace};
use tokio::sync::Mutex;

use super::utils::{send_message, WsSplitSink};

pub async fn identify(write: Arc<Mutex<WsSplitSink>>, token: &String) {
    let os_name = std::env::consts::OS[0..1].to_uppercase() + &std::env::consts::OS[1..];
    // https://docs.discord.sex/topics/gateway#list-of-intents
    // enable every event
    let intents = (0..=25).fold(0, |acu, n| acu | 1 << n);
    // opt-in everything #gourmand
    let capabilities = (0..=14).fold(0, |acu, n| acu | 1 << n);
    let success = send_message(
        write,
        serde_json::json!(
            {
                "op": super::utils::OpCodes::Identify as i32,
                "d": {
                    "token": token,
                    "properties": {
                        "os": os_name,
                        "browser": "Discord Client",
                        "system_locale": "en-US",
                        "os_arch": std::env::consts::ARCH,
                        "app_arch": std::env::consts::ARCH,
                        "release_channel": "canary",
                        // Not a mod if it's not the official client :nerd:
                        "has_client_mods": false,
                    },
                    "compress": false,
                    "presence": {
                        // TODO: Get cached/config status
                        "status": "online",
                        "since": 0,
                        "afk": false,
                        "activities": [],
                    },
                    "intents": intents,
                    "capabilities": capabilities,
                },
            }
        ),
    )
    .await;
    if success {
        trace!("Sent identify attempt");
    } else {
        error!("Failed to send identify attempt");
    }
}
