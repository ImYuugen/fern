use crate::{
    gateway::utils::FernWebsocketMessage,
    structs::{
        channel::Channel,
        guild::{GatewayGuild, GuildExperiment, GuildJoinRequest, GuildMember, VoiceState},
        misc::GatewayApplication,
        user::{Connection, MergedPresences, Presence, Relationship, User, UserExperiment},
    },
};

use log::{error, trace};

use std::collections::HashMap;

/// The body of the message sent on READY Dispatch event
#[derive(serde::Deserialize, Debug)]
struct ReadyEvent {
    // Used for debugging
    // _trace: Vec<String>,
    /// API Version
    v: u8,
    /// The connected user
    user: User,
    /// Preloaded user settings protobuf
    /// https://github.com/dolfies/discord-protos/blob/master/discord_protos/PreloadedUserSettings.proto
    user_settings_proto: Option<String>,
    /// The guilds the user is in
    guilds: Vec<GatewayGuild>,
    // WARN: Mentions of "partial" object in docs
    /// Active guild join requests
    guild_join_requests: Vec<GuildJoinRequest>,
    relationships: Vec<Relationship>,
    friend_suggestion_count: Option<u32>,
    /// DMs and Group DMs
    private_channels: Vec<Channel>,
    /// Third party connections like steam, bluesky, etc..
    connected_accounts: Vec<Connection>,
    /// A mapping of user IDs (Snowflake) and notes made for them
    notes: Option<HashMap<String, String>>,
    /// The presences of the user's non-offline friends
    presences: Option<Vec<Presence>>,
    merged_presences: Option<MergedPresences>,
    /// Same order as the guild array
    merged_members: Vec<Vec<GuildMember>>,
    /// Partial users deduped across all objects (bro why)
    users: Vec<User>,
    application: Option<GatewayApplication>,
    /// Used for resuming connections
    session_id: String,
    /// Only possible value is "normal" ?
    session_type: String,
    /// The hash of the session == auth token used to connect
    auth_session_id_hash: String,
    /// The refreshed auth token for this user;
    /// if present, discard current auth token and replace it with this one
    auth_token: Option<String>,
    /// Token used for analytical tracking requests
    analytics_token: String,
    /// All enabled authenticators
    authenticator_types: Option<Vec<u8>>,
    /// Action needed before continuing to use Fern
    /// values @ /resources/user#required-action-type
    required_action: Option<String>,
    /// Detected ISO 3166-1 a-2 code of the current IP address
    country_code: String,
    /// A list of available regions for voice channels
    geo_ordered_rtc_regions: Vec<String>,
    // tutorial: Option<Tutorial>, // Osef
    /// Shards associated with the session, if sharded (no)
    shard: Option<Vec<(i32, i32)>>,
    /// Websocket URL for resuming connections
    resume_gateway_url: String,
    /// API code version when re-identifying with client state v2
    api_code_version: u8,
    experiments: Vec<UserExperiment>,
    guild_experiments: Vec<GuildExperiment>,
}

#[derive(serde::Deserialize, Debug)]
struct ReadySupplementalEvent {
    guilds: Option<Vec<SupplementalGuild>>,
    merged_members: Option<Vec<Vec<GuildMember>>>,
    merged_presences: Option<MergedPresences>,
    lazy_private_channels: Option<Vec<Channel>>,
    disclose: Option<Vec<String>>,
}
#[derive(serde::Deserialize, Debug)]
struct SupplementalGuild {
    /// Snowflake
    id: String,
    voice_states: Vec<VoiceState>,
    // INFO: Two other fields: activity_instance, embedded_activities
}

pub fn translate_ready(fwsm: FernWebsocketMessage) {
    let ready_event = serde_json::from_value::<ReadyEvent>(fwsm.d);
    match ready_event {
        Ok(_) => {
            trace!("Succesfully translated READY payload");
        }
        Err(e) => {
            error!("You messed up the structs ! {:?}", e);
            std::process::exit(42);
        }
    };
}

pub fn translate_ready_supplemental(fwsm: FernWebsocketMessage) {
    let rs_event = serde_json::from_value::<ReadySupplementalEvent>(fwsm.d);
    match rs_event {
        Ok(_) => {
            trace!("Succesfully translated READY_SUPPLEMENTAL payload");
        }
        Err(e) => {
            error!("You messed up the structs ! {:?}", e);
        }
    };
}
