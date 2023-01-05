use bevy::prelude::*;

use crate::engine::{resources::DisconnectBuffer, components::User};

/// Removes an entity from the game when the user disconnects
pub fn leave_game_system(
    mut commands: Commands,
    mut disconnect_buffer: ResMut<DisconnectBuffer>,
    query: Query<(Entity, &User, &Name)>,
) {
    if let Some(disconnected_user_id) = disconnect_buffer.0.pop_front() {
        for (entity, user, name) in query.iter() {
            if user.0 == disconnected_user_id {
                trace!("Removing {}", name);
                commands.entity(entity).despawn();
            }
        }
    }
}
