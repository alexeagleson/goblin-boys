use ae_direction::{BodyRelative, Cardinal};
use ae_position::Delta;
use bevy::prelude::*;

use crate::{
    components::{
        combat_stats::CombatStats, cooldown::Cooldown, intend_melee_attack::IntendMeleeAttack,
        intend_move::IntendMove, intend_speak::IntendSpeak, speaks::Speaks, MapPosition, User,
    },
    resources::{world::GameWorld, KeypressBuffer},
};

/// Moves an entity based on a user keypress
pub fn movement_keys_system(
    game_world: Res<GameWorld>,
    mut keypress_buffer: ResMut<KeypressBuffer>,
    mut mover_query: Query<(Entity, &User, &MapPosition, &mut Cooldown)>,
    blocker_query: Query<(Entity, &MapPosition, Option<&CombatStats>, Option<&Speaks>)>,
    mut commands: Commands,
) {
    let key = keypress_buffer.0.pop_front();

    if let Some((user_id, key)) = key {
        for (entity, user, map_pos, mut cooldown) in mover_query.iter_mut() {
            // This user ID matches the component of the one trying to make the move
            if user.0 == user_id && cooldown.time_remaining <= 0.0 {
                let new_pos = match key {
                    BodyRelative::Up => {
                        map_pos
                            .pos
                            .add_delta(&Delta::from(ae_direction::Direction::Cardinal(
                                Cardinal::North,
                            )))
                    }
                    BodyRelative::Down => {
                        map_pos
                            .pos
                            .add_delta(&Delta::from(ae_direction::Direction::Cardinal(
                                Cardinal::South,
                            )))
                    }
                    BodyRelative::Left => {
                        map_pos
                            .pos
                            .add_delta(&Delta::from(ae_direction::Direction::Cardinal(
                                Cardinal::West,
                            )))
                    }
                    BodyRelative::Right => {
                        map_pos
                            .pos
                            .add_delta(&Delta::from(ae_direction::Direction::Cardinal(
                                Cardinal::East,
                            )))
                    }
                };

                let map = game_world.game_maps.get(&map_pos.map_id).expect(&format!(
                    "Tried to move on a map that does not exist. Map ID: {:?}",
                    map_pos.map_id
                ));

                if !map.movement_blocked(&new_pos) {
                    commands.entity(entity).insert(IntendMove {
                        position: new_pos.clone(),
                    });
                    cooldown.time_remaining = cooldown.move_time;
                } else {
                    for (other_ent, other_map_pos, other_combat_stats, other_speaks) in
                        blocker_query.iter()
                    {
                        if other_map_pos.map_id == map_pos.map_id && other_map_pos.pos == new_pos {
                            if other_speaks.is_some() {
                                commands
                                    .entity(entity)
                                    .insert(IntendSpeak { target: other_ent });
                                break;
                            }
                            if other_combat_stats.is_some() {
                                commands
                                    .entity(entity)
                                    .insert(IntendMeleeAttack { target: other_ent });
                                cooldown.time_remaining = cooldown.attack_time;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}
