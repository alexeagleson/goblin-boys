use ae_position::Position;
use bevy::prelude::*;

const MAP_2_ID: i32 = 2;

use crate::{
    api::{EntityIndex, LogMessage, ServerMessageAllClients, ServerMessageSingleClient, UserId},
    engine::{
        components::{MapPosition, User},
        events::ShouldSendFullMapUpdateToClient,
        resources::{
            CurrentUserMaps, MessageSenderAllClients, MessageSenderSingleClient, MouseClickBuffer,
        },
    },
};

pub fn change_map_system(
    sender_single_client: Res<MessageSenderSingleClient>,
    // mut mouse_click_buffer: ResMut<MouseClickBuffer>,
    mut query: Query<(Entity, &mut MapPosition, &Name, &User), Changed<MapPosition>>,
    mut ev_update_client: EventWriter<ShouldSendFullMapUpdateToClient>,
    mut current_user_maps: ResMut<CurrentUserMaps>,
) {
    for (entity, mut map_pos, name, user) in query.iter_mut() {
        let arbitrary_position = Position { x: 1, y: 1 };
        if map_pos.pos == arbitrary_position {
            let previous_map_id = map_pos.map_id;
            let new_map_id = if map_pos.map_id == 1 { 2 } else { 1 };

            current_user_maps
                .0
                .iter()
                .for_each(|(user_id, user_map_pos)| {
                    // Communicate to any users on the old map that the sprite should
                    // be removed
                    if user_map_pos.map_id == previous_map_id {
                        sender_single_client
                            .0
                            .send((
                                *user_id,
                                ServerMessageSingleClient::RemoveSprite(EntityIndex {
                                    index: entity.index(),
                                }),
                            ))
                            .ok();
                    }
                });

            map_pos.map_id = new_map_id;

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
