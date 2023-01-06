use ae_direction::{BodyRelative, Cardinal};
use ae_position::{Delta, Position};
use bevy::prelude::*;

use crate::{
    api::{EntityIndex, EntityPositionChange, ServerMessage},
    engine::{
        components::User,
        resources::{map::Map, KeypressBuffer, MessageSender},
    },
};

/// Moves an entity based on a user keypress
pub fn movement_keys_system(
    map: Res<Map>,
    sender: Res<MessageSender>,
    mut keypress_buffer: ResMut<KeypressBuffer>,
    mut query: Query<(Entity, &User, &mut Position, &Name)>,
) {
    let key = keypress_buffer.0.pop_front();

    if let Some((id, key)) = key {
        for (entity, user, mut pos, name) in query.iter_mut() {
            // This user ID matches the component of the one trying to make the move
            if user.0 == id {
                let new_pos = match key {
                    BodyRelative::Up => pos.add_delta(&Delta::from(
                        ae_direction::Direction::Cardinal(Cardinal::North),
                    )),
                    BodyRelative::Down => pos.add_delta(&Delta::from(
                        ae_direction::Direction::Cardinal(Cardinal::South),
                    )),
                    BodyRelative::Left => pos.add_delta(&Delta::from(
                        ae_direction::Direction::Cardinal(Cardinal::West),
                    )),
                    BodyRelative::Right => pos.add_delta(&Delta::from(
                        ae_direction::Direction::Cardinal(Cardinal::East),
                    )),
                };

                // [TODO] The below communication could be handled in its own system using position change detection
                // https://bevy-cheatbook.github.io/programming/change-detection.html

                if map.valid_position(&new_pos) {
                    *pos = new_pos;
                    info!("{} moved to {:?}", name, pos);

                    sender
                        .0
                        .send((
                            id,
                            ServerMessage::EntityPositionChange(EntityPositionChange {
                                entity_index: EntityIndex {
                                    index: entity.index(),
                                },
                                pos: pos.clone(),
                            }),
                        ))
                        .ok();
                } else {
                    info!("{} attempted to move but failed", name);
                }
            }
        }
    }
}
