use bevy::prelude::*;

use crate::{
    api::{EntityIndex, ServerMessageAllClients, ServerMessageSingleClient, UserId},
    engine::{
        components::{MapPosition, User},
        resources::{
            CurrentUserMaps, DisconnectBuffer, MessageSenderAllClients, MessageSenderSingleClient,
        },
    },
};

/// Removes an entity from the game when the user disconnects
pub fn leave_game_system(
    sender_single_client: Res<MessageSenderSingleClient>,
    current_user_maps: Res<CurrentUserMaps>,

    mut commands: Commands,
    mut disconnect_buffer: ResMut<DisconnectBuffer>,
    query: Query<(Entity, &User, &Name, &MapPosition)>,
) {
    if let Some(disconnected_user_id) = disconnect_buffer.0.pop_front() {
        for (entity, user, name, leaving_entity_map_pos) in query.iter() {
            if user.0 == disconnected_user_id {
                current_user_maps.0.iter().for_each(|(user_id, user_map_pos)| {
                    // Communicate to any users on the old map that the sprite should
                    // be removed
                    if user_map_pos.map_id == leaving_entity_map_pos.map_id {
                        sender_single_client
                            .0
                            .send((
                                *user_id,
                                ServerMessageSingleClient::RemoveSprite(EntityIndex {
                                    index: entity.index(),
                                }),
                            ))
                            .ok();
                    }
                });

                info!("Removing {}", name);
                commands.entity(entity).despawn();
            }
        }
    }
}
