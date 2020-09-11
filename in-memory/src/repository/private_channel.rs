use crate::{config::EntityType, InMemoryBackendError, InMemoryBackendRef};
use futures_util::{
    future::{self, FutureExt},
    stream::{self, StreamExt},
};
use rarity_cache::{
    entity::{
        channel::{MessageEntity, PrivateChannelEntity, PrivateChannelRepository},
        user::UserEntity,
        Entity,
    },
    repository::{
        GetEntityFuture, ListEntitiesFuture, RemoveEntityFuture, Repository, UpsertEntityFuture,
    },
};
use std::sync::Arc;
use twilight_model::id::ChannelId;

/// Repository to retrieve and work with private channels and their related
/// entities.
#[derive(Clone, Debug)]
pub struct InMemoryPrivateChannelRepository(pub(crate) Arc<InMemoryBackendRef>);

impl Repository<PrivateChannelEntity, InMemoryBackendError> for InMemoryPrivateChannelRepository {
    fn get(
        &self,
        channel_id: ChannelId,
    ) -> GetEntityFuture<'_, PrivateChannelEntity, InMemoryBackendError> {
        future::ok(
            self.0
                .channels_private
                .get(&channel_id)
                .map(|r| r.value().clone()),
        )
        .boxed()
    }

    fn list(&self) -> ListEntitiesFuture<'_, PrivateChannelEntity, InMemoryBackendError> {
        let stream = stream::iter(
            self.0
                .channels_private
                .iter()
                .map(|r| Ok(r.value().clone())),
        )
        .boxed();

        future::ok(stream).boxed()
    }

    fn remove(&self, channel_id: ChannelId) -> RemoveEntityFuture<'_, InMemoryBackendError> {
        if !self
            .0
            .config
            .entity_types()
            .contains(EntityType::CHANNEL_PRIVATE)
        {
            return future::ok(()).boxed();
        }

        self.0.channels_private.remove(&channel_id);

        future::ok(()).boxed()
    }

    fn upsert(&self, entity: PrivateChannelEntity) -> UpsertEntityFuture<'_, InMemoryBackendError> {
        if !self
            .0
            .config
            .entity_types()
            .contains(EntityType::CHANNEL_PRIVATE)
        {
            return future::ok(()).boxed();
        }

        self.0.channels_private.insert(entity.id(), entity);

        future::ok(()).boxed()
    }
}

impl PrivateChannelRepository<InMemoryBackendError> for InMemoryPrivateChannelRepository {
    fn last_message(
        &self,
        channel_id: ChannelId,
    ) -> GetEntityFuture<'_, MessageEntity, InMemoryBackendError> {
        let message = self
            .0
            .channels_private
            .get(&channel_id)
            .and_then(|channel| channel.last_message_id)
            .and_then(|id| self.0.messages.get(&id))
            .map(|r| r.value().clone());

        future::ok(message).boxed()
    }

    fn recipient(
        &self,
        channel_id: ChannelId,
    ) -> GetEntityFuture<'_, UserEntity, InMemoryBackendError> {
        let user = self
            .0
            .channels_private
            .get(&channel_id)
            .and_then(|channel| channel.recipient_id)
            .and_then(|id| self.0.users.get(&id))
            .map(|r| r.value().clone());

        future::ok(user).boxed()
    }
}

impl InMemoryPrivateChannelRepository {
    /// Retrieve the last message of a private channel.
    pub fn last_message(
        &self,
        channel_id: ChannelId,
    ) -> GetEntityFuture<'_, MessageEntity, InMemoryBackendError> {
        PrivateChannelRepository::last_message(self, channel_id)
    }

    pub fn recipient(
        &self,
        channel_id: ChannelId,
    ) -> GetEntityFuture<'_, UserEntity, InMemoryBackendError> {
        PrivateChannelRepository::recipient(self, channel_id)
    }
}