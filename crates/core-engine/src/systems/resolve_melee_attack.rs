use crate::{
    components::{
        combat_stats::CombatStats,
        cooldown::{self, Cooldown},
        hp::Hp,
        intend_melee_attack::IntendMeleeAttack,
        MapPosition, User,
    },
    resources::{CurrentUserMaps, MessageSenderAllClients, MessageSenderSingleClient},
};
use bevy::prelude::*;
use core_api::{
    AnimationTexture, EntityIndex, LogMessage, ServerMessageAllClients, ServerMessageSingleClient,
    Sound,
};
use rand::Rng;

pub fn resolve_melee_attack_system(
    attacker_query: Query<(
        Entity,
        &CombatStats,
        &IntendMeleeAttack,
        &Name,
        Option<&User>,
        &Cooldown,
    )>,
    mut target_query: Query<(
        Entity,
        &CombatStats,
        &mut Hp,
        &Name,
        &MapPosition,
        Option<&User>,
    )>,
    mut commands: Commands,
    sender_all_clients: Res<MessageSenderAllClients>,
    sender_single_client: Res<MessageSenderSingleClient>,
    current_user_maps: Res<CurrentUserMaps>,
) {
    for (ent, attacker_combat_stats, intend_melee_attack, name, attacker_user, cooldown) in
        attacker_query.iter()
    {
        if let Ok((
            target_entity,
            target_combat_stats,
            mut target_hp,
            target_name,
            target_map_pos,
            target_user,
        )) = target_query.get_mut(intend_melee_attack.target)
        {
            let mut rng = rand::thread_rng();
            let rando = rng.gen_range(0..attacker_combat_stats.attack);

            let damage =
                (attacker_combat_stats.attack - target_combat_stats.defense + rando).max(1);
            target_hp.current -= damage;
            let log_message = LogMessage(format!(
                "{} attacked {} for {} damage {}/{}",
                String::from(name),
                String::from(target_name),
                damage,
                target_hp.current,
                target_hp.max
            ));

            sender_all_clients
                .0
                .send(ServerMessageAllClients::Damage(log_message))
                .ok();

            current_user_maps
                .0
                .iter()
                .for_each(|(user_id, user_map_pos)| {
                    if user_map_pos.map_id == target_map_pos.map_id {
                        let is_matching_user = match target_user {
                            Some(target_user) => target_user.0 == *user_id,
                            None => false,
                        };

                        sender_single_client
                            .0
                            .send((
                                *user_id,
                                ServerMessageSingleClient::ShowDamage {
                                    entity: EntityIndex {
                                        idx: target_entity.index(),
                                    },
                                    damage,
                                    target_is_user: target_user.is_some(),
                                    target_is_me: is_matching_user,
                                    current_hp: target_hp.current,
                                    max_hp: target_hp.max,
                                },
                            ))
                            .ok();
                    }
                });

            if let Some(user) = attacker_user {
                sender_single_client
                    .0
                    .send((
                        user.0,
                        ServerMessageSingleClient::ShowAnimation {
                            position: target_map_pos.pos.clone(),
                            animation: AnimationTexture::AttackBatFrames4,
                            time: cooldown.attack_time,
                        },
                    ))
                    .ok();

                sender_single_client
                    .0
                    .send((user.0, ServerMessageSingleClient::PlaySound(Sound::Punch)))
                    .ok();
            }
        }
        commands.entity(ent).remove::<IntendMeleeAttack>();
    }
}
