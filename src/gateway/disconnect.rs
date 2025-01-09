use std::sync::Arc;

use futures_util::SinkExt as _;
use log::{debug, error, info, trace, warn};
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::{
    protocol::{frame::coding::CloseCode, CloseFrame},
    Message,
};

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

/// Sends a close frame to the websocket pointed by `write`
/// Obviously, this closes the connection
pub async fn disconnect(write: Arc<Mutex<WsSplitSink>>) {
    trace!("Sending close frame to gateway");
    if let Err(e) = write
        .lock()
        .await
        .send(Message::Close(Some(CloseFrame {
            code: CloseCode::Away,
            reason: "Skibidi bibidi sigma".into(),
        })))
        .await
    {
        error!("Error sending close frame: {}", e);
    } else {
        info!("Succesfully closed connection with gateway");
    }
}
