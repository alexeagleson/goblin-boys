use ae_direction::{BodyRelative, Cardinal};
use ae_position::Delta;
use bevy::prelude::*;
use core_api::{ServerMessageSingleClient, SpriteUpdate, EntityIndex, Sound};

use crate::{
    components::{BlocksLight, BlocksMovement, MapPosition, Renderable, User},
    events::ShouldUpdateMap,
    resources::{world::GameWorld, CurrentUserMaps, KeypressBuffer, MessageSenderSingleClient},
};

/// Moves an entity based on a user keypress
pub fn movement_keys_system(
    game_world: Res<GameWorld>,
    sender_single_client: Res<MessageSenderSingleClient>,
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
    mut current_user_maps: ResMut<CurrentUserMaps>,
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

                let map = game_world.game_maps.get(&map_pos.map_id).expect(&format!(
                    "Tried to move on a map that does not exist. Map ID: {:?}",
                    map_pos.map_id
                ));

                if !map.movement_blocked(&new_pos) {
                    map_pos.pos = new_pos;

                    // Update the user's position in the user resource tracker
                    current_user_maps.0.insert(user_id, map_pos.clone());

                    info!("{} moved to {:?}", name, map_pos.pos);

                    // If an entity that blocks movement or light moves, the map needs to update
                    if blocks_movement.is_some() || blocks_light.is_some() {
                        ev_update_map.send(ShouldUpdateMap(map.id()));
                    }

                    // If the entity has a sprite to render, we need to tell the client to update that
                    if let Some(renderable) = renderable {
                        current_user_maps
                            .0
                            .iter()
                            .for_each(|(user_id, user_map_pos)| {
                                if user_map_pos.map_id == map.id() {
                                    sender_single_client
                                        .0
                                        .send((
                                            *user_id,
                                            ServerMessageSingleClient::EntityPositionChange(
                                                SpriteUpdate {
                                                    entity: EntityIndex {
                                                        idx: entity.index(),
                                                    },
                                                    pos: map_pos.pos.clone(),
                                                    sprite: renderable.texture,
                                                },
                                            ),
                                        ))
                                        .ok();
                                }
                            });

                        // Tell the user that moved to update their camera
                        sender_single_client
                            .0
                            .send((
                                user_id,
                                ServerMessageSingleClient::CentreCamera(map_pos.pos.clone()),
                            ))
                            .ok();
                    }
                } else {
                    // [TODO] Turn this audio trigger prototype into something more permanent
                    sender_single_client
                        .0
                        .send((user_id, ServerMessageSingleClient::PlaySound(Sound::Punch)))
                        .ok();

                    info!("{} attempted to move but failed", name);
                }
            }
        }
    }
}
