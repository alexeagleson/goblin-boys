use bevy::prelude::*;

use crate::{
    api::{EntityIndex, ServerMessageSingleClient, SpriteUpdate},
    engine::{
        components::{MapPosition, Renderable, User},
        events::ShouldSendFullMapUpdateToClient,
        resources::{world::MapId, CurrentUserMaps, MessageSenderSingleClient},
    },
};

/// Communicate any relevant sprite change information from the game engine to the client
pub fn update_client_system(
    sender_single_client: Res<MessageSenderSingleClient>,
    mut ev_update_client: EventReader<ShouldSendFullMapUpdateToClient>,
    current_user_maps: Res<CurrentUserMaps>,
    query: Query<(Entity, &MapPosition, Option<&User>, &Renderable)>,
) {
    if ev_update_client.is_empty() {
        return;
    }

    let mut updated_maps: Vec<MapId> = vec![];

    ev_update_client.iter().for_each(|event| {
        let update_map_id = event.0;
        if !updated_maps.contains(&update_map_id) {
            updated_maps.push(update_map_id);

            let entities_on_this_map = query
                .iter()
                .filter_map(|(entity, map_pos, _, sprite)| {
                    if map_pos.map_id != update_map_id {
                        None
                    } else {
                        Some(SpriteUpdate {
                            entity: EntityIndex {
                                idx: entity.index(),
                            },
                            pos: map_pos.pos.clone(),
                            sprite: sprite.texture,
                        })
                    }
                })
                .collect::<Vec<_>>();

            current_user_maps
                .0
                .iter()
                .for_each(|(user_id, user_map_pos)| {
                    if user_map_pos.map_id == update_map_id {
                        sender_single_client
                            .0
                            .send((
                                *user_id,
                                ServerMessageSingleClient::UpdateFullGameMap {
                                    camera: user_map_pos.pos.clone(),
                                    entities: entities_on_this_map.clone(),
                                },
                            ))
                            .ok();
                    }
                });
        }
    });
}
