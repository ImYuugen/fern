use std::sync::Arc;

use super::utils::*;

use futures_util::StreamExt;
use tokio::sync::Mutex;
use tokio_tungstenite::connect_async;

const WEBSOCKET_ADDRESS: &str = "wss://gateway.discord.gg";

/// Creates connection with gateway and starts listening
/// returns a reference to the write stream, to send messages
/// Here we need an Arc because both `handle_incoming` and the
/// client need it
pub async fn initiate_websocket_con() -> Arc<Mutex<WsSplitSink>> {
    // TODO: Handle refused connection (GET on /gateway and reconnect)
    let (ws_stream, _) = connect_async(WEBSOCKET_ADDRESS)
        .await
        .expect("Couldn't handshake with server");
    let (write, read) = ws_stream.split();
    let write: Arc<Mutex<WsSplitSink>> = Arc::new(Mutex::new(write));
    let read_handle = tokio::spawn(handle_incoming(read, write.clone()));
    // TODO: Remove once app loop is present
    let _ = tokio::try_join!(read_handle);
    write
}
