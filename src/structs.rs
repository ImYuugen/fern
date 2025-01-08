pub mod user {
    /// Represents both full and partial user
    #[derive(serde::Deserialize)]
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
        /// len < 190, The user's bio - NOT in partial user -
        bio: Option<String>,
        /// The user's banner hash, see CDN Formatting
        banner: Option<String>,
        /// The user's banner color in hex color code
        accent_color: Option<u32>,
        locale: Option<String>,
        /// Whether the user is verified - NOT in partial user -
        verified: Option<bool>,
        email: Option<String>,
        /// The user's phone number, E.164-formatted
        phone: Option<String>,
        /// The type of premium plan: NONE (0), TIER_{1, 2, 3} - NOT in partial user -
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
    #[derive(serde::Deserialize)]
    struct AvatarDecorationData {
        /// Avatar decoration hash
        asset: String,
        /// ID of decoration's SKU
        sku_id: Option<String>,
        /// Unix timestamp of avatar decoration expiration
        expires_at: Option<i32>,
    }
    #[derive(serde::Deserialize)]
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

    #[derive(serde::Deserialize)]
    pub struct Relationship {
        id: String,
        /// ~NONE~, FRIEND, BLOCKED, INCOMING, OUTGOING, IMPLICIT, ~SUGGESTION~
        #[serde(rename = "type")]
        kind: u8,
        /// WARN: Partial user !!
        user: User,
        /// The nickname of the user in this relationship
        nickname: Option<String>,
        /// Is it flagged as spam
        is_spam_request: Option<bool>,
        /// ISO8601 timestamp
        since: Option<String>,
    }

    #[derive(serde::Deserialize)]
    pub struct Connection {
        // TODO: /resources/user#connection-object
    }

    #[derive(serde::Deserialize)]
    pub struct Presence {
        user: User, // Partial
        /// Snowflake
        guild_id: Option<String>,
        /// One of: online, dnd, idle, invisible, offline, unknown
        status: String,
        activities: Vec<Activity>,
    }

    #[derive(serde::Deserialize)]
    pub struct MergedPresences {
        friends: Vec<Presence>,
        guilds: Vec<Vec<Presence>>,
    }

    #[derive(serde::Deserialize)]
    pub struct Activity {
        // TODO: /resources/presence#activity-object
    }

    #[derive(serde::Deserialize)]
    pub struct ClientStatus {
        // TODO: /resources/presence#client-status-object
    }
}

pub mod guild {
    #[derive(serde::Deserialize)]
    pub struct GatewayGuild {
        // TODO: /topics/gateway-events#gateway-guild-object
    }
    #[derive(serde::Deserialize)]
    pub struct GuildJoinRequest {
        // TODO: /resources/guild#guild-join-request-structure
    }
    #[derive(serde::Deserialize)]
    pub struct GuildMember {
        // TODO: /resources/guild#guild-member-object
    }
}

pub mod channel {
    #[derive(serde::Deserialize)]
    pub struct Channel {
        // TODO: /resources/channel#channel-object
    }
}

// TODO: Find a better name or put the structs elsewhere
pub mod misc {
    #[derive(serde::Deserialize)]
    pub struct GatewayApplication {
        /// Snowflake
        id: String,
        /// /resources/application#application-flags
        flags: u32,
    }
}
