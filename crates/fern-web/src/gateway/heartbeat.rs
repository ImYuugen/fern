use std::sync::Arc;

use log::{debug, error};
use tokio::sync::Mutex;

use super::utils::*;

#[inline]
pub async fn send_heartbeat(write: Arc<Mutex<WsSplitSink>>, socket_state: Arc<Mutex<SocketState>>) {
    let sequence = socket_state.lock().await.heartbeat_sequence;
    let success = send_message(
        write.clone(),
        serde_json::json!({
            "op": OpCodes::Heartbeat as i32,
            "d": sequence,
        }),
    )
    .await;
    if success {
        socket_state.lock().await.heartbeat_ack = false;
        debug!("Succesfully sent heartbeat with number {}", sequence);
    } else {
        error!("Failed to send heartbeat");
    }
}

pub async fn heartbeat_loop(
    fwsm: FernWebsocketMessage,
    write: Arc<Mutex<WsSplitSink>>,
    socket_state: Arc<Mutex<SocketState>>,
) {
    let hearbeat = fwsm
        .d
        .get("heartbeat_interval")
        .expect("No heartbeat_interval !?")
        .as_u64()
        .expect("heartbeat_interval was not a number !?");
    debug!("Starting heartbeat with interval of {}ms", hearbeat);
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(hearbeat));
    loop {
        interval.tick().await;
        {
            if !socket_state.lock().await.heartbeat_ack {
                error!("ACK was not received in time, closing connection");
                // TODO: Actually close the connection and retry
                std::process::exit(69);
            }
        }
        send_heartbeat(write.clone(), socket_state.clone()).await;
    }
}
