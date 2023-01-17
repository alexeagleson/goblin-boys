use std::collections::HashMap;

use ae_position::Position;
use bevy::prelude::*;

const MAP_2_ID: i32 = 2;

use crate::{
    api::{
        EntityIndex, EntityPosition, EntityRenderData, LogMessage, ServerMessageAllClients,
        ServerMessageSingleClient, SpriteTexture, UserId,
    },
    engine::{
        components::{MapPosition, Renderable, User},
        events::ShouldSendFullMapUpdateToClient,
        resources::{
            world::MapId, CurrentUserMaps, MessageSenderAllClients, MessageSenderSingleClient,
            MouseClickBuffer,
        },
    },
};

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
        // let users_on_map = users_by_map.get(&update_map_id);
        if !updated_maps.contains(&update_map_id) {
            updated_maps.push(update_map_id);

            let entities_on_this_map = query
                .iter()
                .filter_map(|(entity, map_pos, _, sprite)| {
                    if map_pos.map_id != update_map_id {
                        None
                    } else {
                        Some(EntityRenderData {
                            entity_position: EntityPosition {
                                entity_index: EntityIndex {
                                    index: entity.index(),
                                },
                                pos: map_pos.pos.clone(),
                            },
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
