use std::sync::Arc;

use log::{debug, warn};
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;

use super::utils::WsSplitSink;

pub async fn handle_close(close_frame: Option<CloseFrame>, _write: Arc<Mutex<WsSplitSink>>) {
    match close_frame {
        Some(cf) => debug!(
            "Received close frame with code {} and reason \"{}\"",
            cf.code, cf.reason
        ),
        None => warn!("Connection closed with empty frame"),
    };
}
