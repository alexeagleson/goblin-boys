use ae_position::Position;
use bevy::prelude::*;
use core_api::{EntityIndex, ServerMessageSingleClient};

use crate::{
    components::{MapPosition, User},
    events::ShouldSendFullMapUpdateToClient,
    resources::{world::MapId, CurrentUserMaps, MessageSenderSingleClient},
};

/// [TODO] Turn this prototype into something more permanent
/// Prototype system that creates a magic warp point to transition between maps
/// Used during development of adding support for more than one game map
pub fn change_map_system(
    sender_single_client: Res<MessageSenderSingleClient>,
    // Change detection
    // https://bevy-cheatbook.github.io/programming/change-detection.html
    mut query: Query<(Entity, &mut MapPosition, &User), Changed<MapPosition>>,
    mut ev_update_client: EventWriter<ShouldSendFullMapUpdateToClient>,
    mut current_user_maps: ResMut<CurrentUserMaps>,
) {
    for (entity, mut map_pos, user) in query.iter_mut() {
        // [TODO]
        let arbitrary_position = Position { x: 1, y: 1 };
        if map_pos.pos == arbitrary_position {
            let previous_map_id = map_pos.map_id;
            let new_map_id = if map_pos.map_id == (MapId(1)) {
                MapId(2)
            } else {
                MapId(1)
            };

            current_user_maps
                .0
                .iter()
                .for_each(|(user_id, user_map_pos)| {
                    // Communicate to any users on the old map that the sprite should be removed
                    if user_map_pos.map_id == previous_map_id {
                        sender_single_client
                            .0
                            .send((
                                *user_id,
                                ServerMessageSingleClient::RemoveSprite(EntityIndex {
                                    idx: entity.index(),
                                }),
                            ))
                            .ok();
                    }
                });

            map_pos.map_id = new_map_id;

            // Track that this user is not viewing a new map
            current_user_maps.0.insert(
                user.0,
                MapPosition {
                    pos: arbitrary_position,
                    map_id: new_map_id,
                },
            );

            ev_update_client.send(ShouldSendFullMapUpdateToClient(new_map_id));
        }
    }
}
