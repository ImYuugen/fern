use std::sync::Arc;

use log::{debug, error, trace};
use tokio::sync::Mutex;

use super::utils::*;

pub async fn heartbeat_loop(fswm: FernWebsocketMessage, write: Arc<Mutex<WsSplitSink>>) {
    let hearbeat = fswm
        .d
        .get("heartbeat_interval")
        .expect("No heartbeat_interval !?")
        .as_u64()
        .expect("heartbeat_interval was not a number !?");
    debug!("Starting heartbeat with interval of {}ms", hearbeat);
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(hearbeat));
    loop {
        interval.tick().await;
        let success = send_message(
            write.clone(),
            serde_json::json!({
                "op": OpCodes::Heartbeat as i32,
                "d": "null",
            }),
        )
        .await;
        // TODO: Check if HeartbeatACK is receieved
        if success {
            trace!("Succesfully sent heartbeat");
        } else {
            error!("Failed to send heartbeat");
        }
    }
}
