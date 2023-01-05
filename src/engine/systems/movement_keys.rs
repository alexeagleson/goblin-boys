use ae_direction::Cardinal;
use ae_position::{Delta, Position};
use bevy::prelude::*;

use crate::{
    api::{Key, ServerMessage, PlayerPosition},
    engine::{resources::{map::Map, KeypressBuffer, MessageSender}, components::User},
};

/// Moves an entity based on a user keypress
pub fn movement_keys_system(
    map: Res<Map>,
    sender: Res<MessageSender>,
    mut keypress_buffer: ResMut<KeypressBuffer>,
    mut query: Query<(&User, &mut Position, &Name)>,
) {
    let key = keypress_buffer.0.pop_front();

    if let Some((id, key)) = key {
        for (user, mut pos, name) in query.iter_mut() {
            // This user ID matches the component of the one trying to make the move
            if user.0 == id {
                let new_pos = match key {
                    Key::Up => pos.add_delta(&Delta::from(ae_direction::Direction::Cardinal(
                        Cardinal::North,
                    ))),
                    Key::Down => pos.add_delta(&Delta::from(ae_direction::Direction::Cardinal(
                        Cardinal::South,
                    ))),
                    Key::Left => pos.add_delta(&Delta::from(ae_direction::Direction::Cardinal(
                        Cardinal::West,
                    ))),
                    Key::Right => pos.add_delta(&Delta::from(ae_direction::Direction::Cardinal(
                        Cardinal::East,
                    ))),
                };

                if map.valid_position(&new_pos) {
                    *pos = new_pos;
                    trace!("{} moved to {:?}", name, pos);

                    sender
                        .0
                        .send((
                            id,
                            ServerMessage::PlayerPosition(PlayerPosition {
                                id,
                                pos: pos.clone(),
                            }),
                        ))
                        .ok();
                } else {
                    trace!("{} attempted to move but failed", name);
                }
            }
        }
    }
}
