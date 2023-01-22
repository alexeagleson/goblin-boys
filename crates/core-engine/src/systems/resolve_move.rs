use crate::{
    components::{
        intend_move::IntendMove, BlocksLight, BlocksMovement, MapPosition, Renderable, User,
    },
    events::ShouldUpdateMap,
    resources::{world::GameWorld, CurrentUserMaps, MessageSenderSingleClient},
};
use bevy::prelude::*;
use core_api::{EntityIndex, ServerMessageSingleClient, SpriteUpdate};

/// Moves an entity based on a user keypress
pub fn resolve_move_system(
    game_world: Res<GameWorld>,
    sender_single_client: Res<MessageSenderSingleClient>,
    mut current_user_maps: ResMut<CurrentUserMaps>,
    mut ev_update_map: EventWriter<ShouldUpdateMap>,
    mut query: Query<(
        Entity,
        &mut MapPosition,
        &mut IntendMove,
        &Name,
        Option<&BlocksMovement>,
        Option<&BlocksLight>,
        Option<&Renderable>,
        Option<&User>,
    )>,
    mut commands: Commands,
) {
    for (entity, mut map_pos, intend_move, name, blocks_movement, blocks_light, renderable, user) in
        query.iter_mut()
    {
        let map = game_world.game_maps.get(&map_pos.map_id).expect(&format!(
            "Tried to move on a map that does not exist. Map ID: {:?}",
            map_pos.map_id
        ));
        commands.entity(entity).remove::<IntendMove>();
        // Maybe something else moved there before this ent got to, too bad.
        if map.movement_blocked(&intend_move.position) {
            return;
        }
        // Update the position based on the intent
        map_pos.pos = intend_move.position.clone();
        info!("{} moved to {:?}", name, map_pos.pos);

        // If an entity that blocks movement or light moves, the map needs to update
        if blocks_movement.is_some() || blocks_light.is_some() {
            ev_update_map.send(ShouldUpdateMap(map.id()));
        }

        // If this is a user we need to do some additional work
        if let Some(user) = user {
            // Update the user's position in the user resource tracker
            current_user_maps.0.insert(user.0, map_pos.clone());
            // Tell the user that moved to update their camera
            sender_single_client
                .0
                .send((
                    user.0,
                    ServerMessageSingleClient::CentreCamera(map_pos.pos.clone()),
                ))
                .ok();
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
                                ServerMessageSingleClient::EntityPositionChange(SpriteUpdate {
                                    entity: EntityIndex {
                                        idx: entity.index(),
                                    },
                                    pos: map_pos.pos.clone(),
                                    sprite: renderable.texture,
                                }),
                            ))
                            .ok();
                    }
                });
        }
    }
}
