use ae_direction::Cardinal;
use ae_position::{Delta, Position};
use bevy::prelude::*;

use crate::{
    api::{EntityIndex, EntityPosition, ServerMessageAllClients},
    engine::{
        components::{paths::Paths, BlocksLight, BlocksMovement, Renderable, User},
        events::ShouldUpdateMap,
        resources::{map::Map, MessageSenderAllClients, MoveStopwatch},
    },
};

/// Move randomly
pub fn move_timer_system(
    mut move_stopwatch: ResMut<MoveStopwatch>,
    sender: Res<MessageSenderAllClients>,
    map: Res<Map>,
    time: Res<Time>,
    mut ev_update_map: EventWriter<ShouldUpdateMap>,
    mut query: Query<(
        Entity,
        &User,
        &mut Position,
        &Name,
        Option<&mut Paths>,
        Option<&BlocksMovement>,
        Option<&BlocksLight>,
        Option<&Renderable>,
    )>,
) {
    if move_stopwatch.0.elapsed_secs() < 0.5 {
        move_stopwatch.0.tick(time.delta());
    } else {
        move_stopwatch.0.reset();
        for (entity, _user, mut pos, name, paths, blocks_movement, blocks_light, renderable) in
            query.iter_mut()
        {
            let new_pos = if let Some(mut paths) = paths {
                // Make sure it's a valid move

                if let Some(next_pos) = paths.get_next() {
                    next_pos
                } else {
                    // [TODO] This is still pretty janky, right now the entity will still pop from
                    // their path even if they try to move to a blocked tile and probably teleport to the
                    // next one on their turn after
                    let unlocked_position = map.random_movement_unblocked_tile();

                    // let new_path = Paths::generate_direct_to_position(&pos, &unlocked_position);
                    let new_path = Paths::generate_astar(&pos, &unlocked_position, &map);

                    paths.set(new_path);
                    continue;
                }
            } else {
                // Move randomly

                let random_direction: Cardinal = rand::random();
                let delta: Delta = random_direction.into();
                pos.add_delta(&delta)
            };

            if !map.movement_blocked(&new_pos) {
                *pos = new_pos;

                // If an entity that blocks movement or light moves, the map needs to update
                if blocks_movement.is_some() || blocks_light.is_some() {
                    ev_update_map.send(ShouldUpdateMap);
                }

                // If the entity has a sprite to render, we need to tell the client to update that
                if renderable.is_some() {
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
                }
            } else {
                info!("{} attempted to move but failed", name);
            }
        }
    }
}
