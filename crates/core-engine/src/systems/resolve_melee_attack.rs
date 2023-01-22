use crate::{
    components::{
        combat_stats::CombatStats, hp::Hp, intend_melee_attack::IntendMeleeAttack, MapPosition,
        User,
    },
    resources::MessageSenderSingleClient,
};
use bevy::prelude::*;
use core_api::{AnimationTexture, ServerMessageSingleClient};

pub fn resolve_melee_attack_system(
    attacker_query: Query<(Entity, &CombatStats, &IntendMeleeAttack, &User)>,
    mut target_query: Query<(&CombatStats, &mut Hp, &MapPosition)>,
    sender_single_client: Res<MessageSenderSingleClient>,

    mut commands: Commands,
) {
    for (ent, combat_stats, intend_melee_attack, attacker_user) in attacker_query.iter() {
        if let Ok((target_combat_stats, mut target_hp, target_map_pos)) =
            target_query.get_mut(intend_melee_attack.target)
        {
            target_hp.current -= (combat_stats.attack - target_combat_stats.defense).max(0);
            println!("OW");

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
