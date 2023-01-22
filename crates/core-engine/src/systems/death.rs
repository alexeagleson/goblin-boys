use bevy::prelude::*;
use core_api::{EntityIndex, ServerMessageSingleClient};

use crate::{
    components::{hp::Hp, MapPosition},
    events::ShouldUpdateMap,
    resources::{CurrentUserMaps, MessageSenderSingleClient},
};

pub fn death_system(
    query: Query<(Entity, &MapPosition, &Hp)>,
    mut commands: Commands,
    current_user_maps: ResMut<CurrentUserMaps>,
    sender_single_client: Res<MessageSenderSingleClient>,
    mut ev_update_map: EventWriter<ShouldUpdateMap>,
) {
    for (ent, map_position, hp) in query.iter() {
        if hp.current <= 0 {
            commands.entity(ent).despawn();
            // Need to update the map if something dies
            ev_update_map.send(ShouldUpdateMap(map_position.map_id));

            current_user_maps
                .0
                .iter()
                .for_each(|(user_id, user_map_pos)| {
                    // Communicate to any users on the old map that the sprite should be removed
                    if user_map_pos.map_id == map_position.map_id {
                        sender_single_client
                            .0
                            .send((
                                *user_id,
                                ServerMessageSingleClient::RemoveSprite(EntityIndex {
                                    idx: ent.index(),
                                }),
                            ))
                            .ok();
                    }
                });
        }
    }
}
