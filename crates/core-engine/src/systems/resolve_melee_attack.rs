use crate::components::{
    combat_stats::CombatStats, hp::Hp, intend_melee_attack::IntendMeleeAttack,
};
use bevy::prelude::*;

pub fn resolve_melee_attack_system(
    attacker_query: Query<(Entity, &CombatStats, &IntendMeleeAttack)>,
    mut target_query: Query<(&CombatStats, &mut Hp)>,
    mut commands: Commands,
) {
    for (ent, combat_stats, intend_melee_attack) in attacker_query.iter() {
        if let Ok((target_combat_stats, mut target_hp)) =
            target_query.get_mut(intend_melee_attack.target)
        {
            target_hp.current -= (combat_stats.attack - target_combat_stats.defense).max(0);
            println!("OW");
        }
        commands.entity(ent).remove::<IntendMeleeAttack>();
    }
}
