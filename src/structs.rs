pub mod user {
    use super::guild::GuildMember;

    #[derive(serde::Deserialize, Debug)]
    pub struct User {
        /// ID of the user, Snowflake
        id: String,
        /// len = 2-32, unique username
        username: String,
        /// The four numbers after #
        /// 0 for users that have been migrated to the new username system
        discriminator: String,
        /// len = 1-32, User display name, app name for bots
        global_name: Option<String>,
        /// Avatar hash, see CDN Formatting
        avatar: Option<String>,
        avatar_decoration_data: Option<AvatarDecorationData>,
        primary_guild: Option<PrimaryGuild>,
        /// Whether the account is a bot account
        bot: Option<bool>,
        /// Whether the account is an official Discord System user
        system: Option<bool>,
        /// - NOT in partial user -
        mfa_enabled: Option<bool>,
        /// Whether the user is allowed to see NSFW
        nsfw_enabled: Option<bool>,
        /// len < 40, The user's pronouns #wokent :pensive:
        pronouns: Option<String>,
        /// len < 190, The user's bio
        bio: Option<String>,
        /// The user's banner hash, see CDN Formatting
        banner: Option<String>,
        /// The user's banner color in hex color code
        accent_color: Option<u32>,
        locale: Option<String>,
        /// Whether the user is verified
        verified: Option<bool>,
        email: Option<String>,
        /// The user's phone number, E.164-formatted
        phone: Option<String>,
        /// The type of premium plan: NONE (0), TIER_{1, 2, 3}
        premium_type: Option<i32>,
        /// The ID of ther user's personal, non-employee user account, Snowflake
        personnal_connection_id: Option<String>,
        /// /resources/user#user-flags
        flags: Option<u64>,
        public_flags: Option<u64>,
        /// NITRO_CLASSIC = 1, NITRO, GUILD_BOOST, NITRO_BASIC
        purchased_flags: Option<u8>,
        /// PREMIUM_DISCRIMINATOR = 1, ANIMATED_AVATAR, PROFILE_BANNER
        premium_usage_flags: Option<u8>,
        /// Whether the user *has used* the desktop client before
        desktop: Option<bool>,
        /// Whether the user *has used* the mobile client before
        mobile: Option<bool>,
        has_bounced_email: Option<bool>,
        /// WEBAUTHN = 1, TOTP, SMS
        authenticator_types: Option<Vec<u8>>,
    }
    #[derive(serde::Deserialize, Debug)]
    pub struct PartialUser {
        /// Snowflake
        id: String,
        pub username: String,
        discriminator: String,
        global_name: Option<String>,
        /// See CDN Formatting
        avatar: Option<String>,
        avatar_decoration_data: Option<AvatarDecorationData>,
        primary_guild: Option<PrimaryGuild>,
        bot: Option<bool>,
        system: Option<bool>,
        banner: Option<bool>,
        accent_color: Option<u32>,
        public_flags: Option<u64>,
    }
    #[derive(serde::Deserialize, Debug)]
    struct AvatarDecorationData {
        /// Avatar decoration hash
        asset: String,
        /// ID of decoration's SKU
        sku_id: Option<String>,
        /// Unix timestamp of avatar decoration expiration
        expires_at: Option<i32>,
    }
    #[derive(serde::Deserialize, Debug)]
    struct PrimaryGuild {
        /// Whether the user is displaying their clan tag
        identity_enabled: bool,
        /// Identity of the guild, Snowflake
        identity_guild_id: Option<String>,
        /// len <= 4, Clan tag, only populated if identity_enabled == true
        tag: Option<String>,
        /// Clan's badge hash, see CDN Formatting, only populated if identity_enabled == true
        badge: Option<String>,
    }

    #[derive(serde::Deserialize, Debug)]
    pub struct Relationship {
        id: String,
        /// ~NONE~, FRIEND, BLOCKED, INCOMING, OUTGOING, IMPLICIT, ~SUGGESTION~
        #[serde(rename = "type")]
        kind: u8,
        user_id: String,
        /// The nickname of the user in this relationship
        nickname: Option<String>,
        /// Is it flagged as spam
        is_spam_request: Option<bool>,
        /// ISO8601 timestamp
        since: Option<String>,
    }

    #[derive(serde::Deserialize, Debug)]
    pub struct Connection {
        id: String,
        /// Title of the connection, eg. bluesky, steam, ...
        #[serde(rename = "type")]
        kind: String,
        /// The username of the connection account
        name: String,
        verified: bool,
        /// Service-specific metadata about the connection
        metadata: Option<serde_json::Value>,
        /// 0: no one, 1: everyone
        metadata_visibility: u8,
        revoked: bool,
        integrations: Vec<ConnectionIntegration>,
    }
    #[derive(serde::Deserialize, Debug)]
    pub struct ConnectionIntegration {
        /// Snowflake, can be "twitch-partners" for TP integration
        id: String,
        /// Type of integration: twitch, youtube, discord, guild_subscription
        #[serde(rename = "type")]
        kind: String,
        /// The integration account's information
        account: ConnectionAccount,
        guild: IntegrationGuild,
    }
    #[derive(serde::Deserialize, Debug)]
    pub struct ConnectionAccount {
        id: String,
        name: String,
    }
    #[derive(serde::Deserialize, Debug)]
    pub struct IntegrationGuild {
        /// Snowflake
        id: String,
        name: String,
        /// The guild's icon hash
        icon: Option<String>,
    }

    #[derive(serde::Deserialize, Debug)]
    pub struct Presence {
        user: PartialUser,
        /// Snowflake
        guild_id: Option<String>,
        /// One of: online, dnd, idle, invisible, offline, unknown
        status: String,
        activities: Vec<Activity>,
    }

    #[derive(serde::Deserialize, Debug)]
    pub struct MergedPresences {
        friends: Vec<Presence>,
        guilds: Vec<Vec<Presence>>,
    }

    #[derive(serde::Deserialize, Debug)]
    pub struct Activity {
        id: String,
        name: String,
        /// PLAYING, STREAMING, LISTENING, WATCHING, CUSTOM, COMPETING, HANG
        #[serde(rename = "type")]
        kind: u8,
        /// The Stream URL
        url: Option<String>,
        /// UNIX timestamp (ms) of when activity was added to the session
        created_at: u64,
        /// ID of the session associated with the activity
        session_id: Option<String>,
        /// desktop, xbox, samsung, ios, android, embedded, ps4, ps5
        platform: Option<String>,
        supported_platforms: Option<Vec<String>>,
        timestamps: Option<ActivityTimestamps>,
        /// Snowflake
        application_id: Option<String>,
        /// What the user is currently doing
        details: Option<String>,
        /// The user's current party status
        state: Option<String>,
        /// The ID of the synced activity (e.g. Spotify song ID)
        sync_id: Option<String>,
        /// /resources/presence#activity-flags
        flags: Option<u16>,
        /// Custom buttons shown in rich presence (max 2)
        buttons: Option<Vec<String>>,
        emoji: Option<ActivityEmoji>,
        party: Option<ActivityParty>,
        assets: Option<ActivityAssets>,
        /// Secrets for rich presence joining/spectating
        secrets: Option<ActivitySecrets>,
        /// Additional metadata
        metadata: Option<ActivityMetadata>,
    }
    #[derive(serde::Deserialize, Debug)]
    pub struct ActivityTimestamps {
        // Why are those strings 😭😭😭
        /// UNIX timestamp (ms) of when the activity starts
        start: Option<String>,
        /// UNIX timestamp (ms) of when the activity ends
        end: Option<String>,
    }
    #[derive(serde::Deserialize, Debug)]
    pub struct ActivityEmoji {
        name: String,
        /// Snowflake
        id: Option<String>,
        animated: Option<bool>,
    }
    #[derive(serde::Deserialize, Debug)]
    pub struct ActivityParty {
        id: Option<String>,
        /// current_size, max_size
        size: Option<[u32; 2]>,
    }
    #[derive(serde::Deserialize, Debug)]
    pub struct ActivityAssets {
        /// /resources/presence#activity-image
        large_image: Option<String>,
        large_text: Option<String>,
        small_image: Option<String>,
        small_text: Option<String>,
    }
    #[derive(serde::Deserialize, Debug)]
    pub struct ActivitySecrets {
        /// The secret for joining a party
        join: Option<String>,
        // spectate and match are deprecated
    }
    /// WARN: This object consists of arbitrary, unsanitized data
    #[derive(serde::Deserialize, Debug)]
    pub struct ActivityMetadata {
        // TODO: If you really want to put all possible fields, you can
        metadata: serde_json::Value,
    }

    #[derive(serde::Deserialize, Debug)]
    pub struct ClientStatus {
        desktop: Option<String>,
        mobile: Option<String>,
        web: Option<String>,
        embedded: Option<String>,
    }

    #[derive(serde::Deserialize, Debug)]
    pub struct UserExperiment {
        /// Murmur3 hash of the experiment's name
        hash: u32,
        /// Current version of the rollout
        revision: u32,
        /// The requesting user or fingerprint's experiment bucket
        bucket: u32,
        /// Experiment override present: -1 == false, 0 == true
        #[serde(rename = "override")]
        override_present: i8,
        /// The internal population group the requesting user is in
        population: u32,
        /// The rollout position to use: mmh3.hash(...) % 10000
        hash_result: u16,
        /// The experiment's A/A testing mode, 0 or 1
        aa_mode: i8,
        /// Is analytics trigger debugging enabled, 0 or 1
        trigger_debugging: i8,
    }

    #[derive(serde::Deserialize, Debug)]
    pub struct VoiceState {
        /// Snowflake
        guild_id: Option<String>,
        /// Snowflake
        channel_id: Option<String>,
        /// Snowflake
        user_id: String,
        member: Option<GuildMember>,
        session_id: String,
        // PERF: Pack those into u8
        deaf: bool,
        mute: bool,
        self_deaf: bool,
        self_mute: bool,
        self_stream: Option<bool>,
        self_video: bool,
        suppress: bool,
        /// ISO8601 timestamp
        request_to_speak_timestamp: Option<String>,
    }
}

pub mod guild {
    use super::{
        channel::Channel,
        user::{Presence, User},
    };

    #[derive(serde::Deserialize, Debug)]
    pub struct Guild {
        /// Snowflake
        id: String,
        name: String,
    }

    #[derive(serde::Deserialize, Debug)]
    pub struct GatewayGuild {
        // TODO: /topics/gateway-events#gateway-guild-object
        /// ISO8601 timestamp of when the guild was joined
        joined_at: String,
        large: bool,
        /// Can be due to an outage
        unavailable: Option<bool>,
        /// Whether it is not accessible in your region
        geo_restricted: Option<bool>,
        member_count: u32,
        voice_states: Option<Vec<VoiceState>>,
        members: Option<Vec<GuildMember>>,
        channels: Vec<Channel>,
        threads: Vec<Channel>,
        /// Presences of guild members, offline members are not included in large guilds
        presences: Option<Vec<Presence>>,
        stage_instances: Option<Vec<StageInstance>>,
        guild_scheduled_events: Option<Vec<GuildScheduledEvent>>,
        /// full: everything sent, partial: some is cached, unavailable
        data_mode: String,
        properties: Guild,
        stickers: Option<Vec<Sticker>>,
        roles: Option<Vec<Role>>,
        emojis: Option<Vec<Emoji>>,
        /// Number of boosts
        premium_subscription_count: u32,
    }
    #[derive(serde::Deserialize, Debug)]
    pub struct GuildJoinRequest {
        // TODO: /resources/guild#guild-join-request-structure
    }
    #[derive(serde::Deserialize, Debug)]
    pub struct GuildMember {
        // TODO: /resources/guild#guild-member-object
    }
    // TODO: Those who implement :skull: ------>
    #[derive(serde::Deserialize, Debug)]
    pub struct VoiceState;
    #[derive(serde::Deserialize, Debug)]
    pub struct StageInstance {
        /// Snowflake
        id: String,
        /// Snowflake
        guild_id: String,
        /// Snowflake
        channel_id: String,
        topic: String,
        /// PUBLIC = 1, GUILD_ONLY
        privacy_level: u8,
        // TODO: Link doc
        /// See invite object
        invite_code: Option<String>,
        /// ID of the Scheduled event for this stage instance
        guild_scheduled_event_id: Option<String>,
    }
    #[derive(serde::Deserialize, Debug)]
    pub struct GuildScheduledEvent {
        /// Snowflake
        id: String,
        /// Snowflake
        guild_id: String,
        /// Snowflake
        channel_id: Option<String>,
        /// Snowflake
        creator_id: Option<String>,
        creator: Option<User>, // Partial user
        name: String,
        description: Option<String>,
        /// ISO8601 timestamp
        scheduled_start_time: String,
        /// ISO8601 timestamp
        scheduled_end_time: Option<String>,
        /// PUBLIC = 1, GUILD_ONLY
        privacy_level: u8,
        /// SCHEDULED = 1, ACTIVE, COMPLETED, CANCELED
        status: u8,
        /// STAGE_INSTANCE = 1, VOICE, EXTERNAL, PRIME_TIME
        entity_type: u8,
        /// Snowflake, ID of the associated entity
        entity_id: Option<String>,
        entity_metadata: Option<EntityMetadata>,
        user_count: Option<u32>,
        /// CDN hash of the event image
        image: Option<String>,
    }
    #[derive(serde::Deserialize, Debug)]
    pub struct EntityMetadata {
        location: Option<String>,
    }
    #[derive(serde::Deserialize, Debug)]
    pub struct Sticker {
        /// Snowflake
        id: String,
        /// Snowflake, for standard stickers
        pack_id: Option<String>,
        name: String,
        description: Option<String>,
        /// Autocomplete/suggestions
        tags: String,
        /// STANDARD, GUILD
        #[serde(rename = "type")]
        kind: u8,
        /// PNG = 1, APNG, LOTTIE, GIF
        format_type: u8,
        available: Option<bool>,
        /// Snowflake, the ID of the guild the sticker is attached to
        guild_id: Option<String>,
        /// NOTE: Partial user
        /// Only included if fetched through endpoints with MANAGE_EMOJIS_AND_STICKERS perm
        user: Option<User>,
        sort_value: Option<u32>,
    }
    #[derive(serde::Deserialize, Debug)]
    pub struct Role {
        /// Snowflake
        id: String,
        name: String,
        description: Option<String>,
        /// Hex color code
        color: u32,
        /// Whether the role is pinned in the user listing
        hoist: bool,
        /// Icon hash
        icon: Option<String>,
        unicode_emoji: Option<String>,
        /// Position of this role in the role list
        position: u32,
        /// The permission bitwise values
        permissions: u32,
        /// Whether this role is managed by an integration
        managed: bool,
        mentionnable: Option<bool>,
        /// IN_PROMPT
        flags: Option<u8>,
        tags: Option<RoleTags>,
    }
    #[derive(serde::Deserialize, Debug)]
    pub struct RoleTags {
        /// Snowflake
        bot_id: Option<String>,
        /// Snowflake
        integration_id: Option<String>,
        /// Snowflake
        subscription_listing_id: Option<String>,
    }
    #[derive(serde::Deserialize, Debug)]
    pub struct Emoji {
        /// Snowflake, CDN
        id: Option<String>,
        /// Name of the emoji, can be null if deleted
        name: Option<String>,
        /// Snowflakes of roles allowed to use the emoji
        roles: Option<Vec<String>>,
        /// Use who uploaded the emoji
        user: Option<Box<User>>,
        require_colons: Option<bool>,
        managed: Option<bool>,
        animated: Option<bool>,
        available: Option<bool>,
    }

    #[derive(serde::Deserialize, Debug)]
    pub struct GuildExperiment {
        /// mm3 hashed experiment name
        hash: u32,
        /// Experiment name formatted as `year-month_name`
        hash_key: Option<String>,
        revision: u32,
        // Too lazy to do all those structs
        // /topics/experiments#experiment-population-object
        populations: Vec<serde_json::Value>,
        // /topics/experiments#experiment-bucket-override-object
        overrides: Vec<serde_json::Value>,
        // /topics/experiments#experiment-population-object
        overrides_formatted: Vec<Vec<serde_json::Value>>,
        /// Experiment name formatted as `year-month_name` the experiment is dependant on
        holdout_name: Option<String>,
        /// The required bucket for the experiment
        holdout_bucket: Option<u32>,
        aa_mode: i8,
        trigger_debugging: i8,
    }
}

pub mod channel {
    use super::user::User;

    #[derive(serde::Deserialize, Debug)]
    pub struct Channel {
        // TODO: /resources/channel#channel-object
    }

    #[derive(serde::Deserialize, Debug)]
    pub struct PartialChannel {
        /// Snowflake
        id: String,
        #[serde(rename = "type")]
        kind: u8,
        name: Option<String>,
        // Partial
        recipients: Option<Vec<User>>,
        /// See CDN Formatting
        icon: Option<String>,
        /// Snowflake
        guild_id: Option<String>,
    }
}

// TODO: Find a better name or put the structs elsewhere
pub mod misc {
    #[derive(serde::Deserialize, Debug)]
    pub struct GatewayApplication {
        /// Snowflake
        id: String,
        /// /resources/application#application-flags
        flags: u32,
    }
}
