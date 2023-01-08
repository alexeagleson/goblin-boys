use ae_direction::Cardinal;
use ae_position::{Delta, Position};
use bevy::prelude::*;

use crate::{
    api::{EntityIndex, EntityPosition, ServerMessageAllClients},
    engine::{
        components::{BlocksLight, BlocksMovement, Renderable, User},
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
        Option<&BlocksMovement>,
        Option<&BlocksLight>,
        Option<&Renderable>,
    )>,
) {
    if move_stopwatch.0.elapsed_secs() < 1.0 {
        move_stopwatch.0.tick(time.delta());
    } else {
        move_stopwatch.0.reset();
        for (entity, _user, mut pos, name, blocks_light, blocks_movement, renderable) in
            query.iter_mut()
        {
            let random_direction: Cardinal = rand::random();
            let delta: Delta = random_direction.into();
            let new_pos: Position = pos.add_delta(&delta);

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
