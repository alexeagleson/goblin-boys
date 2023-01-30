use ae_direction::Cardinal;
use ae_position::Delta;
use bevy::prelude::*;
use core_api::{EntityIndex, ServerMessageSingleClient, SpriteUpdate};

use crate::{
    components::{paths::Paths, BlocksLight, BlocksMovement, MapPosition, Renderable, User},
    events::ShouldUpdateMap,
    resources::{world::GameWorld, CurrentUserMaps, MessageSenderSingleClient, DebugStopwatch},
};

/// THIS SYSTEM IS NO LONGER USED CAN PROBABLY DELETE
/// THIS SYSTEM IS NO LONGER USED CAN PROBABLY DELETE
/// 
/// Move randomly
pub fn pathing_system(
    mut move_stopwatch: ResMut<DebugStopwatch>,
    // sender: Res<MessageSenderAllClients>,
    game_world: Res<GameWorld>,
    time: Res<Time>,
    mut ev_update_map: EventWriter<ShouldUpdateMap>,
    mut query: Query<(
        Entity,
        // &User,
        &mut MapPosition,
        &Name,
        Option<&mut Paths>,
        Option<&BlocksMovement>,
        Option<&BlocksLight>,
        Option<&Renderable>,
    )>,
    mut current_user_maps: ResMut<CurrentUserMaps>,
    sender_single_client: Res<MessageSenderSingleClient>,
) {
    if move_stopwatch.0.elapsed_secs() < 0.5 {
        move_stopwatch.0.tick(time.delta());
    } else {
        move_stopwatch.0.reset();
        for (
            entity,
            //  user,
            mut map_pos,
            name,
            paths,
            blocks_movement,
            blocks_light,
            renderable,
        ) in query.iter_mut()
        {
            let map = game_world
                .game_maps
                .get(&map_pos.map_id)
                .expect("Map doesn't exist somehow");

            let new_pos = if let Some(mut paths) = paths {
                // Make sure it's a valid move

                if let Some(next_pos) = paths.get_next() {
                    Some(next_pos)
                } else {
                    // [TODO] This is still pretty janky, right now the entity will still pop from
                    // their path even if they try to move to a blocked tile and probably teleport to the
                    // next one on their turn after
                    let unlocked_position = map.random_movement_unblocked_tile();

                    // let new_path = Paths::generate_direct_to_position(&pos, &unlocked_position);
                    let new_path = Paths::generate_astar(&map_pos.pos, &unlocked_position, &map);

                    paths.set(new_path);
                    continue;
                }
            } else {
                // Move randomly

                // [TODO] Use random movement somewhere
                // let random_direction: Cardinal = rand::random();
                // let delta: Delta = random_direction.into();
                // map_pos.pos.add_delta(&delta)
                None
            };

            if let Some(new_pos) = new_pos {
                if !map.movement_blocked(&new_pos) {
                    map_pos.pos = new_pos;

                    // [TODO] Most of this is the same as in [`movement_keys_system`] and could be improved
                    // to reduce a lot of duplication

                    info!("{} moved to {:?}", name, map_pos.pos);

                    // Update the user's position in the user resource tracker
                    // current_user_maps.0.insert(user.0, map_pos.clone());

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

                        // // Tell the user that moved to update their camera
                        // sender_single_client
                        //     .0
                        //     .send((
                        //         user.0,
                        //         ServerMessageSingleClient::CentreCamera(map_pos.pos.clone()),
                        //     ))
                        //     .ok();
                    }
                } else {
                    info!("{} attempted to move but failed", name);
                }
            }
        }
    }
}
