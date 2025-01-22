use log::{debug, error};

use crate::{
    gateway::utils::FernWebsocketMessage,
    structs::{guild::Emoji, user::PartialUser},
};

#[derive(serde::Deserialize, Debug)]
struct Message {
    /// Snowflake
    id: String,
    /// Snowflake
    channel_id: String,
    author: PartialUser,
    content: String,
    /// ISO8601 timestamp
    timestamp: String,
    /// ISO8601 timestamp
    edited_timestamp: Option<String>,
    // tts: bool,
    mention_everyone: bool,
    // WHY DO THEY NOT JUST GIVE THE ID FFS
    // mentions: Vec<User>,
    /// Snowflake
    // mention_roles: Vec<String>,
    // mention_channels: Option<Vec<PartialChannel>>,
    attachments: Vec<Attachment>,
    embeds: Vec<Embed>,
    reactions: Option<Vec<Reaction>>,
    /// i32 | String
    nonce: Option<serde_json::Value>,
    pinned: bool,
    /// Snowflake
    // webhook_id: Option<String>,
    #[serde(rename = "type")]
    kind: u8,
    // activity: Option<Activity>,
    // application: Option<IntegrationApplication>,
    // /// Snowflake
    // application_id: String,
    flags: u32,
    // message_reference: Option<MessageReference>,
    // referenced_message: Option<Box<Message>>,
    // message_snapshots: Option<Vec<MessageSnapshot>>
}
#[derive(serde::Deserialize, Debug)]
struct Attachment {}
#[derive(serde::Deserialize, Debug)]
struct Embed {}
#[derive(serde::Deserialize, Debug)]
struct Reaction {
    count: u16,
    count_details: ReactionCountDetails,
    me: bool,
    me_burst: bool,
    emoji: Emoji,
    // A Vec of HEX!!! IN STRINGS!!!!
    burst_colors: Vec<String>,
}
#[derive(serde::Deserialize, Debug)]
struct ReactionCountDetails {
    normal: u16,
    burst: u16,
}
#[derive(serde::Deserialize, Debug)]
struct Activity {
    #[serde(rename = "type")]
    kind: u8,
    session_id: String,
    party_id: Option<String>,
}

pub fn message_create(fwsm: FernWebsocketMessage) {
    let message = serde_json::from_value::<Message>(fwsm.d);
    match message {
        Ok(m) => {
            debug!(
                "{:?} sent a message @ {:?} : {:?}",
                m.author.username, m.timestamp, m.content
            );
        }
        Err(e) => {
            error!("You messed up the structs ! {:?}", e);
        }
    }
}

pub fn message_delete(_fwsm: FernWebsocketMessage) {}

pub fn message_update(_fwsm: FernWebsocketMessage) {}
