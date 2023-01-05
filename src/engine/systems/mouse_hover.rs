use ae_position::Position;
use bevy::prelude::*;

use crate::{engine::resources::{MessageSender, MouseHoverBuffer}, api::{PlayerDetails, ServerMessage}};

/// Looks for an entity at a tile position being hovered
pub fn mouse_hover_system(
    sender: Res<MessageSender>,
    mut mouse_hover_buffer: ResMut<MouseHoverBuffer>,
    query: Query<(&Position, &Name)>,
) {
    if let Some((id, hover_pos)) = mouse_hover_buffer.0.pop_front() {
        let player_details_at_tile = query.iter().find_map(|(ent_pos, name)| {
            (hover_pos == *ent_pos).then_some(PlayerDetails {
                name: name.into(),
            })
        });

        // Communicate the entity at the hover position to the client that requested it
        // It's important to specifically communicate `None` if there is no entity to handle
        // the case where the user hovers from a tile with an entity to a tile without one
        sender
            .0
            .send((id, ServerMessage::TileHover(player_details_at_tile)))
            .ok();
    }
}
