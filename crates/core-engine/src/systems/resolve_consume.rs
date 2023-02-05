use crate::{
    components::{hp::Hp, intend_consume::IntendConsume, Bones, User},
    resources::{CurrentUserMaps, MessageSenderAllClients, MessageSenderSingleClient},
};
use bevy::prelude::*;
use core_api::{
    EntityIndex, LogMessage, ServerMessageAllClients, ServerMessageSingleClient, Sound,
};

pub fn resolve_consume_system(
    mut consumer_query: Query<(Entity, Option<&Name>, &IntendConsume, &User, &mut Hp)>,
    target_query: Query<(Entity, &Name), With<Bones>>,
    sender_single_client: Res<MessageSenderSingleClient>,
    mut commands: Commands,
    current_user_maps: Res<CurrentUserMaps>,
    sender_all_clients: Res<MessageSenderAllClients>,
) {
    for (ent, consumer_name, intend_consume, consumer_user, mut hp) in consumer_query.iter_mut() {
        if let Ok((target_entity, target_name)) = target_query.get(intend_consume.target) {
            let healing = (hp.max - hp.current).min(10);
            hp.current += healing;

            if let Some(name) = consumer_name {
                let log_message = LogMessage(format!(
                    "{} eats the {} and recovers {} HP!",
                    String::from(name),
                    String::from(target_name),
                    healing
                ));
                sender_all_clients
                    .0
                    .send(ServerMessageAllClients::Log(log_message))
                    .ok();
            }

            current_user_maps
                .0
                .iter()
                .for_each(|(user_id, _user_map_pos)| {
                    sender_single_client
                        .0
                        .send((
                            *user_id,
                            ServerMessageSingleClient::ShowDamage {
                                entity: EntityIndex {
                                    idx: target_entity.index(),
                                },
                                damage: healing,
                                is_healing: true,
                                target_is_user: true,
                                target_is_me: user_id == &consumer_user.0,
                                current_hp: hp.current,
                                max_hp: hp.max,
                            },
                        ))
                        .ok();

                    // Remove the bones
                    sender_single_client
                        .0
                        .send((
                            *user_id,
                            ServerMessageSingleClient::RemoveSprite(EntityIndex {
                                idx: target_entity.index(),
                            }),
                        ))
                        .ok();

                    sender_single_client
                        .0
                        .send((
                            *user_id,
                            ServerMessageSingleClient::PlaySound(Sound::EatBones),
                        ))
                        .ok();
                });

            commands.entity(target_entity).despawn();
        }
        commands.entity(ent).remove::<IntendConsume>();
    }
}
