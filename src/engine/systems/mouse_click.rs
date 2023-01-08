use ae_position::Position;
use bevy::prelude::*;

use crate::{
    api::{LogMessage, ServerMessageAllClients},
    engine::resources::{MessageSenderAllClients, MouseClickBuffer},
};

/// Looks for an entity at a tile position being clicked
pub fn mouse_click_system(
    sender: Res<MessageSenderAllClients>,
    mut mouse_click_buffer: ResMut<MouseClickBuffer>,
    query: Query<(&Position, &Name)>,
) {
    if let Some((user_id, click_pos)) = mouse_click_buffer.0.pop_front() {
        let log_message = query.iter().find_map(|(ent_pos, name)| {
            (click_pos == *ent_pos)
                .then_some(LogMessage(format!("User {} clicked {}", user_id.id, &name)))
        });

        if let Some(log_message) = log_message {
            // Communicate the log message about the click to all players
            sender
                .0
                .send(ServerMessageAllClients::TileClick(log_message))
                .ok();
        }
    }
}
