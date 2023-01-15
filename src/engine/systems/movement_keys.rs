use ae_direction::{BodyRelative, Cardinal};
use ae_position::{Delta, Position};
use bevy::prelude::*;

use crate::{
    api::{EntityIndex, EntityPosition, ServerMessageAllClients, ServerMessageSingleClient},
    engine::{
        components::{BlocksLight, BlocksMovement, MapPosition, Renderable, User},
        events::ShouldUpdateMap,
        resources::{
            map::Map, world::GameWorld, KeypressBuffer, MessageSenderAllClients,
            MessageSenderSingleClient,
        },
    },
};

/// Moves an entity based on a user keypress
pub fn movement_keys_system(
    game_world: Res<GameWorld>,
    sender_single_client: Res<MessageSenderSingleClient>,
    sender_all_clients: Res<MessageSenderAllClients>,
    mut ev_update_map: EventWriter<ShouldUpdateMap>,
    mut keypress_buffer: ResMut<KeypressBuffer>,
    mut query: Query<(
        Entity,
        &User,
        &mut MapPosition,
        &Name,
        Option<&BlocksMovement>,
        Option<&BlocksLight>,
        Option<&Renderable>,
    )>,
) {
    let key = keypress_buffer.0.pop_front();

    if let Some((user_id, key)) = key {
        for (entity, user, mut map_pos, name, blocks_movement, blocks_light, renderable) in
            query.iter_mut()
        {
            // This user ID matches the component of the one trying to make the move
            if user.0 == user_id {
                let new_pos = match key {
                    BodyRelative::Up => {
                        map_pos
                            .pos
                            .add_delta(&Delta::from(ae_direction::Direction::Cardinal(
                                Cardinal::North,
                            )))
                    }
                    BodyRelative::Down => {
                        map_pos
                            .pos
                            .add_delta(&Delta::from(ae_direction::Direction::Cardinal(
                                Cardinal::South,
                            )))
                    }
                    BodyRelative::Left => {
                        map_pos
                            .pos
                            .add_delta(&Delta::from(ae_direction::Direction::Cardinal(
                                Cardinal::West,
                            )))
                    }
                    BodyRelative::Right => {
                        map_pos
                            .pos
                            .add_delta(&Delta::from(ae_direction::Direction::Cardinal(
                                Cardinal::East,
                            )))
                    }
                };

                // [TODO] The below communication could be handled in its own system using position change detection
                // https://bevy-cheatbook.github.io/programming/change-detection.html

                let map = game_world.game_maps.get(&map_pos.map_id).expect(&format!(
                    "Tried to move on a map that does not exist. Map ID: {}",
                    map_pos.map_id
                ));

                if !map.movement_blocked(&new_pos) {
                    map_pos.pos = new_pos;
                    info!("{} moved to {:?}", name, map_pos.pos);

                    // If an entity that blocks movement or light moves, the map needs to update
                    if blocks_movement.is_some() || blocks_light.is_some() {
                        ev_update_map.send(ShouldUpdateMap(map.id()));
                    }

                    // If the entity has a sprite to render, we need to tell the client to update that
                    if renderable.is_some() {
                        sender_all_clients
                            .0
                            .send(ServerMessageAllClients::EntityPositionChange(
                                EntityPosition {
                                    entity_index: EntityIndex {
                                        index: entity.index(),
                                    },
                                    pos: map_pos.pos.clone(),
                                },
                            ))
                            .ok();

                        sender_single_client
                            .0
                            .send((
                                user_id,
                                ServerMessageSingleClient::PlayerPositionChange(
                                    map_pos.pos.clone(),
                                ),
                            ))
                            .ok();
                    }
                } else {
                    info!("{} attempted to move but failed", name);
                }
            }
        }
    }
}
