//! Entities related to and within guilds.

pub mod emoji;
pub mod member;
pub mod role;

pub use self::{
    emoji::{EmojiEntity, EmojiRepository},
    member::{MemberEntity, MemberRepository},
    role::{RoleEntity, RoleRepository},
};

use super::{
    channel::{GuildChannelEntity, TextChannelEntity, VoiceChannelEntity},
    gateway::PresenceEntity,
    user::UserEntity,
    voice::VoiceStateEntity,
};
use crate::{
    repository::{GetEntityFuture, ListEntitiesFuture, ListEntityIdsFuture, Repository},
    utils, Backend, Entity,
};
use twilight_model::{
    guild::{
        DefaultMessageNotificationLevel, ExplicitContentFilter, Guild, MfaLevel, PartialGuild,
        Permissions, PremiumTier, SystemChannelFlags, VerificationLevel,
    },
    id::{ApplicationId, ChannelId, EmojiId, GuildId, RoleId, UserId},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GuildEntity {
    pub afk_channel_id: Option<ChannelId>,
    pub afk_timeout: u64,
    pub application_id: Option<ApplicationId>,
    pub approximate_member_count: Option<u64>,
    pub approximate_presence_count: Option<u64>,
    pub banner: Option<String>,
    pub default_message_notifications: DefaultMessageNotificationLevel,
    pub description: Option<String>,
    pub discovery_splash: Option<String>,
    pub explicit_content_filter: ExplicitContentFilter,
    pub features: Vec<String>,
    pub icon: Option<String>,
    pub id: GuildId,
    pub joined_at: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub large: bool,
    // Not documented so I marked it as optional.
    pub lazy: Option<bool>,
    pub max_members: Option<u64>,
    pub max_presences: Option<u64>,
    pub max_video_channel_users: Option<u64>,
    pub member_count: Option<u64>,
    pub mfa_level: MfaLevel,
    pub name: String,
    pub owner_id: UserId,
    pub owner: Option<bool>,
    pub permissions: Option<Permissions>,
    pub preferred_locale: String,
    pub premium_subscription_count: Option<u64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub premium_tier: PremiumTier,
    pub region: String,
    pub rules_channel_id: Option<ChannelId>,
    pub splash: Option<String>,
    pub system_channel_flags: SystemChannelFlags,
    pub system_channel_id: Option<ChannelId>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub unavailable: bool,
    pub vanity_url_code: Option<String>,
    pub verification_level: VerificationLevel,
    pub widget_channel_id: Option<ChannelId>,
    pub widget_enabled: Option<bool>,
}

impl From<Guild> for GuildEntity {
    fn from(guild: Guild) -> Self {
        Self {
            afk_channel_id: guild.afk_channel_id,
            afk_timeout: guild.afk_timeout,
            application_id: guild.application_id,
            approximate_member_count: guild.approximate_member_count,
            approximate_presence_count: guild.approximate_presence_count,
            banner: guild.banner,
            default_message_notifications: guild.default_message_notifications,
            description: guild.description,
            discovery_splash: guild.discovery_splash,
            explicit_content_filter: guild.explicit_content_filter,
            features: guild.features,
            icon: guild.icon,
            id: guild.id,
            joined_at: guild.joined_at,
            large: guild.large,
            lazy: guild.lazy,
            max_members: guild.max_members,
            max_presences: guild.max_presences,
            max_video_channel_users: guild.max_video_channel_users,
            member_count: guild.member_count,
            mfa_level: guild.mfa_level,
            name: guild.name,
            owner_id: guild.owner_id,
            owner: guild.owner,
            permissions: guild.permissions,
            preferred_locale: guild.preferred_locale,
            premium_subscription_count: guild.premium_subscription_count,
            premium_tier: guild.premium_tier,
            region: guild.region,
            rules_channel_id: guild.rules_channel_id,
            splash: guild.splash,
            system_channel_flags: guild.system_channel_flags,
            system_channel_id: guild.system_channel_id,
            unavailable: guild.unavailable,
            vanity_url_code: guild.vanity_url_code,
            verification_level: guild.verification_level,
            widget_channel_id: guild.widget_channel_id,
            widget_enabled: guild.widget_enabled,
        }
    }
}

impl GuildEntity {
    pub fn update(self, update: PartialGuild) -> Self {
        Self {
            afk_channel_id: update.afk_channel_id.or(self.afk_channel_id),
            afk_timeout: update.afk_timeout,
            application_id: update.application_id.or(self.application_id),
            banner: update.banner.or(self.banner),
            default_message_notifications: update.default_message_notifications,
            description: update.description.or(self.description),
            discovery_splash: update.discovery_splash.or(self.discovery_splash),
            explicit_content_filter: update.explicit_content_filter,
            features: update.features,
            icon: update.icon.or(self.icon),
            id: update.id,
            max_members: update.max_members.or(self.max_members),
            max_presences: update.max_presences.or(self.max_presences),
            member_count: update.member_count.or(self.member_count),
            mfa_level: update.mfa_level,
            name: update.name,
            owner_id: update.owner_id,
            owner: update.owner.or(self.owner),
            permissions: update.permissions.or(self.permissions),
            preferred_locale: update.preferred_locale,
            premium_subscription_count: update
                .premium_subscription_count
                .or(self.premium_subscription_count),
            premium_tier: update.premium_tier,
            region: update.region,
            rules_channel_id: update.rules_channel_id.or(self.rules_channel_id),
            splash: update.splash.or(self.splash),
            system_channel_flags: update.system_channel_flags,
            system_channel_id: update.system_channel_id.or(self.system_channel_id),
            vanity_url_code: update.vanity_url_code.or(self.vanity_url_code),
            verification_level: update.verification_level,
            widget_channel_id: update.widget_channel_id.or(self.widget_channel_id),
            widget_enabled: update.widget_enabled.or(self.widget_enabled),
            ..self
        }
    }
}

impl Entity for GuildEntity {
    type Id = GuildId;

    /// Return the guild's ID.
    fn id(&self) -> Self::Id {
        self.id
    }
}

/// Repository to work with guilds and their associated entities.
pub trait GuildRepository<B: Backend>: Repository<GuildEntity, B> {
    /// Retrieve the AFK voice channel associated with a guild.
    ///
    /// Backend implementations should return `None` if the AFK channel isn't
    /// configured (meaning [`GuildEntity::afk_channel_id`] is `None`) or is not
    /// present in the cache.
    ///
    /// [`GuildEntity::afk_channel_id`]: struct.GuildEntity.html#structfield.afk_channel_id
    fn afk_channel(&self, guild_id: GuildId) -> GetEntityFuture<'_, VoiceChannelEntity, B::Error> {
        utils::relation_and_then(
            self.backend().guilds(),
            self.backend().voice_channels(),
            guild_id,
            |guild| guild.afk_channel_id,
        )
    }

    /// Retrieve a stream of channel IDs within a guild.
    fn channel_ids(&self, guild_id: GuildId) -> ListEntityIdsFuture<'_, ChannelId, B::Error>;

    /// Retrieve a stream of channels within a guild.
    fn channels(&self, guild_id: GuildId) -> ListEntitiesFuture<'_, GuildChannelEntity, B::Error>;

    /// Retrieve a stream of emoji IDs within a guild.
    fn emoji_ids(&self, guild_id: GuildId) -> ListEntityIdsFuture<'_, EmojiId, B::Error>;

    /// Retrieve a stream of emojis within a guild.
    fn emojis(&self, guild_id: GuildId) -> ListEntitiesFuture<'_, EmojiEntity, B::Error> {
        utils::stream_ids(self.emoji_ids(guild_id), self.backend().emojis())
    }

    /// Retrieve a stream of member IDs within a guild.
    fn member_ids(&self, guild_id: GuildId) -> ListEntityIdsFuture<'_, UserId, B::Error>;

    /// Retrieve a stream of members within a guild.
    fn members(&self, guild_id: GuildId) -> ListEntitiesFuture<'_, MemberEntity, B::Error>;

    /// Retrieve the owner associated with a guild.
    ///
    /// Backend implementations should return `None` if the user is not in the
    /// cache.
    fn owner(&self, guild_id: GuildId) -> GetEntityFuture<'_, UserEntity, B::Error> {
        utils::relation_map(
            self.backend().guilds(),
            self.backend().users(),
            guild_id,
            |guild| guild.owner_id,
        )
    }

    /// Retrieve a stream of user IDs of presences within a guild.
    fn presence_ids(&self, guild_id: GuildId) -> ListEntityIdsFuture<'_, UserId, B::Error>;

    /// Retrieve a stream of presences within a guild.
    fn presences(&self, guild_id: GuildId) -> ListEntitiesFuture<'_, PresenceEntity, B::Error>;

    /// Retrieve a stream of role IDs within a guild.
    fn role_ids(&self, guild_id: GuildId) -> ListEntityIdsFuture<'_, RoleId, B::Error>;

    /// Retrieve a stream of roles within a guild.
    fn roles(&self, guild_id: GuildId) -> ListEntitiesFuture<'_, RoleEntity, B::Error> {
        utils::stream_ids(self.role_ids(guild_id), self.backend().roles())
    }

    /// Retrieve the rules channel associated with a guild.
    ///
    /// Backend implementations should return `None` if the rules channel isn't
    /// configured (meaning [`GuildEntity::rules_channel_id`] is `None`) or is
    /// not present in the cache.
    ///
    /// [`GuildEntity::rules_channel_id`]: struct.GuildEntity.html#structfield.rules_channel_id
    fn rules_channel(&self, guild_id: GuildId) -> GetEntityFuture<'_, TextChannelEntity, B::Error> {
        utils::relation_and_then(
            self.backend().guilds(),
            self.backend().text_channels(),
            guild_id,
            |guild| guild.rules_channel_id,
        )
    }

    /// Retrieve the system channel associated with a guild.
    ///
    /// Backend implementations should return `None` if the system channel isn't
    /// configured (meaning [`GuildEntity::system_channel_id`] is `None`) or is
    /// not present in the cache.
    ///
    /// [`GuildEntity::system_channel_id`]: struct.GuildEntity.html#structfield.system_channel_id
    fn system_channel(
        &self,
        guild_id: GuildId,
    ) -> GetEntityFuture<'_, TextChannelEntity, B::Error> {
        utils::relation_and_then(
            self.backend().guilds(),
            self.backend().text_channels(),
            guild_id,
            |guild| guild.system_channel_id,
        )
    }

    /// Retrieve a stream of voice states' user IDs within a guild.
    fn voice_state_ids(&self, guild_id: GuildId) -> ListEntityIdsFuture<'_, UserId, B::Error>;

    /// Retrieve a stream of voice states within a guild.
    fn voice_states(&self, guild_id: GuildId)
        -> ListEntitiesFuture<'_, VoiceStateEntity, B::Error>;

    /// Retrieve the widget channel associated with a guild.
    ///
    /// Backend implementations should return `None` if the widget channel isn't
    /// configured (meaning [`GuildEntity::widget_channel_id`] is `None`) or is
    /// not present in the cache.
    ///
    /// [`GuildEntity::widget_channel_id`]: struct.GuildEntity.html#structfield.widget_channel_id
    fn widget_channel(
        &self,
        guild_id: GuildId,
    ) -> GetEntityFuture<'_, GuildChannelEntity, B::Error> {
        let backend = self.backend();

        Box::pin(async move {
            let guilds = backend.guilds();

            let channel_id = match guilds
                .get(guild_id)
                .await?
                .and_then(|g| g.widget_channel_id)
            {
                Some(channel_id) => channel_id,
                None => return Ok(None),
            };

            let text_channels = backend.text_channels();

            if let Some(channel) = text_channels.get(channel_id).await? {
                return Ok(Some(GuildChannelEntity::Text(channel)));
            }

            let voice_channels = backend.voice_channels();

            if let Some(channel) = voice_channels.get(channel_id).await? {
                return Ok(Some(GuildChannelEntity::Voice(channel)));
            }

            let category_channels = backend.category_channels();

            if let Some(channel) = category_channels.get(channel_id).await? {
                return Ok(Some(GuildChannelEntity::Category(channel)));
            }

            Ok(None)
        })
    }
}
