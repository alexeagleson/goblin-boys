use ae_position::Position;
use bevy::prelude::*;

use crate::{engine::{resources::{MessageSender, ConnectBuffer}, components::User}, api::{PlayerPosition, ServerMessage}};

/// Adds an entity to the game when the user connects
pub fn join_game_system(
    sender: Res<MessageSender>,
    mut commands: Commands,
    mut connect_buffer: ResMut<ConnectBuffer>,
    query: Query<(&User, &Position)>,
) {
    if let Some(connected_user_id) = connect_buffer.0.pop_front() {
        let initial_position: Position = Position { x: 0, y: 0 };
        let player_name = format!("Player {}", connected_user_id);

        trace!("Adding new player named {}", &player_name);

        commands
            .spawn(User(connected_user_id))
            .insert(Name::new(player_name))
            .insert(initial_position.clone());

        // [TODO] The below communication could be handled in its own system using position change detection
        // https://bevy-cheatbook.github.io/programming/change-detection.html

        let mut all_player_positions = query
            .iter()
            .map(|(user, pos)| PlayerPosition {
                id: user.0,
                pos: pos.clone(),
            })
            .collect::<Vec<_>>();

        // Need to include the position of the player we just added which will not be in the current query
        all_player_positions.push(PlayerPosition {
            id: connected_user_id,
            pos: initial_position,
        });

        // Communicate to the connecting client what their player user ID is
        sender
            .0
            .send((
                connected_user_id,
                ServerMessage::Initialize(connected_user_id),
            ))
            .ok();

        // Communicate to all clients the positions of all players including the new one
        sender
            .0
            .send((
                connected_user_id,
                ServerMessage::AllPlayerPositions(all_player_positions),
            ))
            .ok();
    }
}
