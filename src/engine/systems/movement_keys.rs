use ae_direction::{BodyRelative, Cardinal};
use ae_position::{Delta, Position};
use bevy::prelude::*;

use crate::{
    api::{EntityIndex, EntityPosition, ServerMessageAllClients},
    engine::{
        components::{BlocksLight, BlocksMovement, User},
        events::ShouldUpdateMap,
        resources::{map::Map, KeypressBuffer, MessageSenderAllClients},
    },
};

/// Moves an entity based on a user keypress
pub fn movement_keys_system(
    map: Res<Map>,
    sender: Res<MessageSenderAllClients>,
    mut ev_update_map: EventWriter<ShouldUpdateMap>,
    mut keypress_buffer: ResMut<KeypressBuffer>,
    mut query: Query<(
        Entity,
        &User,
        &mut Position,
        &Name,
        Option<&BlocksLight>,
        Option<&BlocksMovement>,
    )>,
) {
    let key = keypress_buffer.0.pop_front();

    if let Some((id, key)) = key {
        for (entity, user, mut pos, name, blocks_light, blocks_movement) in query.iter_mut() {
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

                if !map.movement_blocked(&new_pos) {
                    *pos = new_pos;
                    info!("{} moved to {:?}", name, pos);

                    // If an entity that blocks light or movement moves the map should update
                    if blocks_light.is_some() || blocks_movement.is_some() {
                        ev_update_map.send(ShouldUpdateMap);
                    }

                    sender
                        .0
                        .send(ServerMessageAllClients::EntityPositionChange(
                            EntityPosition {
                                entity_index: EntityIndex {
                                    index: entity.index(),
                                },
                                pos: pos.clone(),
                            },
                        ))
                        .ok();
                } else {
                    info!("{} attempted to move but failed", name);
                }
            }
        }
    }
}
