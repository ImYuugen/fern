use super::utils::*;

use futures_util::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use log::{info, warn};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

type WsSplitStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
type WsSplitSink = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

const WEBSOCKET_ADDRESS: &str = "wss://gateway.discord.gg";

async fn handle_incoming(mut read: WsSplitStream) {
    while let Some(message) = read.next().await {
        match message {
            Ok(message) => {
                info!("Received message: {}", message);
                let fwsm: Option<FernWebsocketMessage> = match serde_json::from_str(
                    message
                        .to_text()
                        .expect("Message somehow contained non-utf8 bytes"),
                ) {
                    Ok(ok) => Some(ok),
                    Err(_) => None,
                };
                if let Some(fwsm) = fwsm {
                    tokio::spawn(fwsm.handle());
                } else {
                    warn!("Unrecognized message received, connection most likely closed");
                }
            }
            Err(error) => {
                warn!("Error receiving message: {}", error)
            }
        }
    }
}

/// Creates connection with gateway and starts listening
pub async fn initiate_websocket_con() {
    let (ws_stream, _) = connect_async(WEBSOCKET_ADDRESS)
        .await
        .expect("Couldn't handshake with server");
    let (_write, read) = ws_stream.split();
    let read_handle = tokio::spawn(handle_incoming(read));
    // TODO: Remove once app loop is present
    let _ = tokio::try_join!(read_handle);
}
