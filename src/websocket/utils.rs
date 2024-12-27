use log::{debug, error};
use num_traits::FromPrimitive as _;

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
    pub async fn handle(self) {
        use OpCodes::*;
        let Some(opcode) = OpCodes::from_i32(self.op) else {
            error!("Unknown OpCode received, wtf ? : {}", self.op);
            return;
        };
        debug!("op {} translates to {:?}", self.op, opcode);
        match opcode {
            Hello => super::heartbeat::heartbeat_loop(self).await,
            _ => todo!("You have yet to implement this"),
        }
    }
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
