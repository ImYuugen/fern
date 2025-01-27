use std::sync::Arc;

use log::{debug, error, info, warn};
use num_traits::FromPrimitive as _;

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use super::dispatch::handle_dispatch;

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
    pub s: Option<i32>,
    /// Event name for this payload (null if opcode != DISPATCH)
    pub t: Option<String>,
}

impl FernWebsocketMessage {
    /// Consumes the instance, we don't want to execute multiple times
    pub async fn handle(
        self,
        write: Arc<Mutex<WsSplitSink>>,
        socket_state: Arc<Mutex<SocketState>>,
    ) {
        use OpCodes::*;
        let Some(opcode) = OpCodes::from_i32(self.op) else {
            error!("Unknown OpCode received, wtf ? : {}", self.op);
            return;
        };
        debug!("op {} translates to {:?}", self.op, opcode);
        if let Some(seq) = self.s {
            socket_state.lock().await.heartbeat_sequence = seq
        }

        match opcode {
            Dispatch => handle_dispatch(self).await,
            Hello => super::heartbeat::heartbeat_loop(self, write, socket_state).await,
            Heartbeat => super::heartbeat::send_heartbeat(write, socket_state).await,
            HeartbeatACK => socket_state.lock().await.heartbeat_ack = true,
            _ => todo!("You have yet to implement this"),
        }
    }
}

pub struct SocketState {
    pub heartbeat_ack: bool,
    pub heartbeat_sequence: i32,
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
    let socket_state = Arc::new(Mutex::new(SocketState {
        heartbeat_ack: true, // No ACK for first heartbeat
        heartbeat_sequence: 0,
    }));
    while let Some(message) = read.next().await {
        use tokio_tungstenite::tungstenite::protocol::Message;
        match message {
            Ok(Message::Text(message)) => {
                let fwsm: Option<FernWebsocketMessage> = match serde_json::from_str(&message) {
                    Ok(ok) => Some(ok),
                    Err(_) => None,
                };
                if let Some(fwsm) = fwsm {
                    tokio::spawn(fwsm.handle(write.clone(), socket_state.clone()));
                } else {
                    warn!("Unrecognized message received, connection most likely closed");
                    debug!("Message: {}", message);
                }
            }
            Ok(Message::Close(close_frame)) => {
                // Nothing after close frame, no need to spawn task
                super::disconnect::handle_close(close_frame, write.clone()).await;
            }
            Ok(m) => {
                warn!("Received unhandled message {:?}", m);
            }
            Err(error) => {
                warn!("Error receiving message: {}", error)
            }
        }
    }
    info!("handle_incoming loop stopped");
}
