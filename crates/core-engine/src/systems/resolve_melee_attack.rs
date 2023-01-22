use crate::{
    components::{
        combat_stats::CombatStats, hp::Hp, intend_melee_attack::IntendMeleeAttack, MapPosition,
        User,
    },
    resources::{MessageSenderAllClients, MessageSenderSingleClient},
};
use bevy::prelude::*;
use core_api::{AnimationTexture, LogMessage, ServerMessageAllClients, ServerMessageSingleClient};

pub fn resolve_melee_attack_system(
    attacker_query: Query<(Entity, &CombatStats, &IntendMeleeAttack, &Name, &User)>,
    mut target_query: Query<(&CombatStats, &mut Hp, &Name, &MapPosition)>,
    mut commands: Commands,
    sender_all_clients: Res<MessageSenderAllClients>,
    sender_single_client: Res<MessageSenderSingleClient>,
) {
    for (ent, combat_stats, intend_melee_attack, name, attacker_user) in attacker_query.iter() {
        if let Ok((target_combat_stats, mut target_hp, target_name, target_map_pos)) =
            target_query.get_mut(intend_melee_attack.target)
        {
            let damage = (combat_stats.attack - target_combat_stats.defense).max(0);
            target_hp.current -= damage;
            let log_message = LogMessage(format!(
                "{} attacked {} for {} damage",
                String::from(name),
                String::from(target_name),
                damage
            ));

            sender_all_clients
                .0
                .send(ServerMessageAllClients::Damage(log_message))
                .ok();

            sender_single_client
                .0
                .send((
                    attacker_user.0,
                    ServerMessageSingleClient::ShowAnimation {
                        position: target_map_pos.pos.clone(),
                        animation: AnimationTexture::AttackBatFrames4,
                    },
                ))
                .ok();
        }
        commands.entity(ent).remove::<IntendMeleeAttack>();
    }
}
