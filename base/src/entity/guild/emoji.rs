use super::{super::user::UserEntity, GuildEntity, RoleEntity};
use crate::{
    repository::{GetEntityFuture, ListEntitiesFuture, Repository},
    Entity,
};
use twilight_model::id::{EmojiId, GuildId, RoleId, UserId};

/// Cachable version of an emoji.
#[allow(clippy::struct_excessive_bools)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmojiEntity {
    pub animated: bool,
    pub available: bool,
    pub guild_id: GuildId,
    pub id: EmojiId,
    pub managed: bool,
    pub name: String,
    pub require_colons: bool,
    pub role_ids: Vec<RoleId>,
    pub user_id: Option<UserId>,
}

impl Entity for EmojiEntity {
    type Id = EmojiId;

    /// Return the emoji's ID.
    fn id(&self) -> Self::Id {
        self.id
    }
}

pub trait EmojiRepository<Error: 'static>: Repository<EmojiEntity, Error> {
    /// Retrieve the guild associated with an emoji.
    fn guild(&self, emoji_id: EmojiId) -> GetEntityFuture<'_, GuildEntity, Error>;

    /// Retrieve a stream of roles associated with an emoji.
    fn roles(&self, emoji_id: EmojiId) -> ListEntitiesFuture<'_, RoleEntity, Error>;

    /// Retrieve the user associated with an emoji.
    fn user(&self, emoji_id: EmojiId) -> GetEntityFuture<'_, UserEntity, Error>;
}