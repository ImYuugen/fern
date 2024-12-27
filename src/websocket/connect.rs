use std::time::Duration;

use futures_util::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use log::{debug, error, info, warn};
use num_traits::FromPrimitive;
use tokio::{net::TcpStream, time};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

type WsSplitStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
type WsSplitSink = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

const WEBSOCKET_ADDRESS: &str = "wss://gateway.discord.gg";

#[repr(i32)]
#[derive(FromPrimitive, ToPrimitive, Debug)]
enum OpCodes {
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
enum CloseCodes {
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

/// Used for sending and receiving messages
#[derive(serde::Deserialize, Debug)]
struct FernWebsocketMessage {
    /// Opcode
    op: i32,
    /// Arbitrary data
    d: serde_json::Value,
    /// Sequence number, used for resume/heartbeat (null if opcode != DISPATCH)
    s: Option<String>,
    /// Event name for this payload (null if opcode != DISPATCH)
    t: Option<String>,
}

/// Consumes the instance, we don't want to execute multiple times
async fn execute(fwsm: FernWebsocketMessage) {
    let Some(opcode) = OpCodes::from_i32(fwsm.op) else {
        error!("Unknown OpCode received, wtf ? : {}", fwsm.op);
        return;
    };
    debug!("op {} translates to {:?}", fwsm.op, opcode);
    use self::OpCodes::*;
    match opcode {
        Hello => {
            let hearbeat = fwsm
                .d
                .get("heartbeat_interval")
                .expect("No heartbeat_interval")
                .as_u64()
                .expect("heartbeat_interval was not a number !?");
            debug!("Starting heartbeat with interval of {}ms", hearbeat);
            let mut interval = time::interval(Duration::from_millis(hearbeat));
            loop {
                interval.tick().await;
                // TODO: Send hearbeat
                info!("Bop");
            }
        }
        _ => todo!("You have yet to implement this"),
    }
}

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
                    tokio::spawn(execute(fwsm));
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

pub async fn initiate_websocket_con() {
    let (ws_stream, _) = connect_async(WEBSOCKET_ADDRESS)
        .await
        .expect("Couldn't handshake with server");
    let (_write, read) = ws_stream.split();
    let read_handle = tokio::spawn(handle_incoming(read));
    let _ = tokio::try_join!(read_handle);
}
