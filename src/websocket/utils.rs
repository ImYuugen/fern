use std::sync::Arc;

use log::{debug, error, info, warn};
use num_traits::FromPrimitive as _;

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

type WsSplitStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
pub type WsSplitSink = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

/// Used for sending and receiving messages
#[derive(serde::Deserialize, Debug)]
pub struct FernWebsocketMessage {
    /// Opcode
    pub op: i32,
    /// Arbitrary data
    pub d: serde_json::Value,
    /// Sequence number, used for resume/heartbeat (null if opcode != DISPATCH)
    pub s: Option<String>,
    /// Event name for this payload (null if opcode != DISPATCH)
    pub t: Option<String>,
}

impl FernWebsocketMessage {
    /// Consumes the instance, we don't want to execute multiple times
    pub async fn handle(self, write: Arc<Mutex<WsSplitSink>>, state: Arc<Mutex<SocketState>>) {
        use OpCodes::*;
        let Some(opcode) = OpCodes::from_i32(self.op) else {
            error!("Unknown OpCode received, wtf ? : {}", self.op);
            return;
        };
        debug!("op {} translates to {:?}", self.op, opcode);
        match opcode {
            Hello => super::heartbeat::heartbeat_loop(self, write, state).await,
            Heartbeat => super::heartbeat::send_heartbeat(write, state).await,
            HeartbeatACK => state.lock().await.heartbeat_ack = true,
            _ => todo!("You have yet to implement this"),
        }
    }
}

pub struct SocketState {
    pub heartbeat_ack: bool,
}

#[repr(i32)]
#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum OpCodes {
    Dispatch = 0,
    Heartbeat,
    Identify,
    PresenceUpdate,
    VoiceStateUpdate,
    VoiceServerPing,
    Resume,
    Reconnect,
    RequestGuildMembers,
    InvalidSession,
    Hello,
    HeartbeatACK,
    GuildSync,
    CallConnect,
    GuildSubscription,
    LobbyConnect,
    LobbyDisconnect,
    LobbyVoiceStates,
    StreamCreate,
    StreamDelete,
    StreamWatch,
    StreamPing,
    StreamSetPaused,
    LFGSubscriptions,
    RequestGuildApplicationCommands,
    EmbeddedActivityCreate,
    EmbeddedActivityDelete,
    EmbeddedActivityUpdate,
    RequestForumUnreads,
    RemoteCommand,
    RequestDeletedEntityIDs,
    RequestSoundboardSounds,
    SpeedTestCreate,
    SpeedTestDelete,
    RequestLastMessages,
    SearchRecentMembers,
    RequestChannelStatuses,
    GuildSubscriptionsBulk,
    GuildChannelsResync,
}

#[repr(i32)]
#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum CloseCodes {
    UnknownError = 4000,
    UnknownOpcode,
    DecodeError,
    NotAuthenticated,
    AuthenticationFailed,
    AlreadyAuthenticated,
    SessionNoLongerValid,
    InvalidSeq,
    /// Don't do that !
    RateLimited,
    SessionTimedOut,
    InvalidShard,
    ShardingRequired,
    InvalidAPIVersion,
    InvalidIntents,
    DisallowedIntents,
}

// Sends `message` to the specified `write` stream, returns wether it succeeded or not
pub async fn send_message(write: Arc<Mutex<WsSplitSink>>, message: serde_json::Value) -> bool {
    let message = message.to_string();
    let mut lock = write.lock().await;
    match lock.send(Message::text(message)).await {
        Ok(_) => true,
        Err(e) => {
            error!("Failed to send message: {}", e);
            false
        }
    }
}

pub async fn handle_incoming(mut read: WsSplitStream, write: Arc<Mutex<WsSplitSink>>) {
    let state = Arc::new(Mutex::new(SocketState {
        heartbeat_ack: true, // No ACK for first heartbeat
    }));
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
                    tokio::spawn(fwsm.handle(write.clone(), state.clone()));
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
