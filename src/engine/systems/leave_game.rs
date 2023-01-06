use bevy::prelude::*;

use crate::{
    api::{ServerMessage, EntityIndex},
    engine::{
        components::User,
        resources::{DisconnectBuffer, MessageSender},
    },
};

/// Removes an entity from the game when the user disconnects
pub fn leave_game_system(
    sender: Res<MessageSender>,
    mut commands: Commands,
    mut disconnect_buffer: ResMut<DisconnectBuffer>,
    query: Query<(Entity, &User, &Name)>,
) {
    if let Some(disconnected_user_id) = disconnect_buffer.0.pop_front() {
        for (entity, user, name) in query.iter() {
            if user.0 == disconnected_user_id {
                info!("Removing {}", name);
                commands.entity(entity).despawn();

                // Communicate to all clients the player entity has left the game
                sender
                    .0
                    .send((
                        user.0,
                        ServerMessage::RemoveEntity(EntityIndex {
                            index: entity.index(),
                        }),
                    ))
                    .ok();
            }
        }
    }
}
