use bevy::prelude::*;
use core_api::{
    EntityIndex, LogMessage, ServerMessageAllClients, ServerMessageSingleClient, SpriteTexture,
    SpriteUpdate,
};

use crate::{
    components::{hp::Hp, Bones, MapPosition, Renderable},
    events::ShouldUpdateMap,
    resources::{CurrentUserMaps, MessageSenderAllClients, MessageSenderSingleClient},
};

pub fn death_system(
    query: Query<(Entity, &MapPosition, &Hp, Option<&Name>, &Renderable)>,
    mut commands: Commands,
    current_user_maps: ResMut<CurrentUserMaps>,
    sender_single_client: Res<MessageSenderSingleClient>,
    mut ev_update_map: EventWriter<ShouldUpdateMap>,
    sender_all_clients: Res<MessageSenderAllClients>,
) {
    for (ent, map_position, hp, name, renderable) in query.iter() {
        if hp.current <= 0 {
            commands.entity(ent).despawn();

            let mut corpse_commands = commands.spawn(Name::new("Bones"));

            let corpse_sprite = SpriteTexture::ObjectBone;
            let corpse_index = corpse_commands
                .insert(map_position.clone())
                .insert(Renderable {
                    texture: corpse_sprite,
                })
                .insert(Bones)
                .id()
                .index();

            // Need to update the map if something dies
            ev_update_map.send(ShouldUpdateMap(map_position.map_id));
            if let Some(name) = name {
                let log_message = LogMessage(format!("{} died!", String::from(name)));
                sender_all_clients
                    .0
                    .send(ServerMessageAllClients::Death(log_message))
                    .ok();
            }
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

                        sender_single_client
                            .0
                            .send((
                                *user_id,
                                ServerMessageSingleClient::AddSprite(SpriteUpdate {
                                    entity: EntityIndex { idx: corpse_index },
                                    pos: map_position.pos.clone(),
                                    sprite: corpse_sprite,
                                }),
                            ))
                            .ok();

                        if renderable.texture == SpriteTexture::NpcKingRatFrames4 {
                            let log_message =
                                LogMessage(format!("🎉 A KING RAT HAS BEEN KILLED! 🎉"));
                            sender_all_clients
                                .0
                                .send(ServerMessageAllClients::Log(log_message))
                                .ok();
                        }
                    }
                });
        }
    }
}
