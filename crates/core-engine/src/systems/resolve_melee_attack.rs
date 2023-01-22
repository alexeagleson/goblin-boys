use crate::{
    components::{combat_stats::CombatStats, hp::Hp, intend_melee_attack::IntendMeleeAttack},
    resources::MessageSenderAllClients,
};
use bevy::prelude::*;
use core_api::{LogMessage, ServerMessageAllClients};

pub fn resolve_melee_attack_system(
    attacker_query: Query<(Entity, &CombatStats, &IntendMeleeAttack, &Name)>,
    mut target_query: Query<(&CombatStats, &mut Hp, &Name)>,
    mut commands: Commands,
    sender_all_clients: Res<MessageSenderAllClients>,
) {
    for (ent, combat_stats, intend_melee_attack, name) in attacker_query.iter() {
        if let Ok((target_combat_stats, mut target_hp, target_name)) =
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
        }
        commands.entity(ent).remove::<IntendMeleeAttack>();
    }
}
