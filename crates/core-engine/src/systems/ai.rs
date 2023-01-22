use crate::{
    components::{
        ai::{Ai, AiAction},
        eyes::Eyes,
        hp::Hp,
        intend_melee_attack::IntendMeleeAttack,
        intend_move::IntendMove,
        MapPosition, User,
    },
    resources::world::GameWorld,
};
use ae_position::Position;
use bevy::prelude::*;
use rand::seq::SliceRandom;
use tv_utility_ai::{choose::choose_action_fuzzy, curve, WeightedAction};

fn distance_between_positions(pos: &Position, other: &Position) -> u32 {
    ((pos.x.max(other.x) - pos.x.min(other.x)) + (pos.y.max(other.y) - pos.y.min(other.y))) as u32
}

fn is_adjacent(pos: &Position, other: &Position) -> bool {
    distance_between_positions(pos, other) <= 1
}

fn get_visible_floor_positions(
    visibility_grid: &Vec<u8>,
    floor_idxs: &Vec<usize>,
    grid_width: usize,
) -> Vec<Position> {
    floor_idxs.iter().fold(vec![], |mut acc, idx| {
        if let Some(1) = visibility_grid.get(*idx) {
            acc.push(Position::from_idx(*idx, grid_width))
        }
        acc
    })
}

fn make_attack_action(entity: Entity, enemy_hp: &Hp, offset: f32) -> WeightedAction<AiAction> {
    WeightedAction {
        action: AiAction::Attack(entity),
        weight: (curve::linear(enemy_hp.current as f32, enemy_hp.max as f32) + offset)
            .clamp(0.0, 1.0),
    }
}

fn make_chase_action(
    entity: Entity,
    enemy_hp: &Hp,
    distance: u32,
    visibility_range: u32,
    offset: f32,
) -> WeightedAction<AiAction> {
    let hp_weight = curve::linear(enemy_hp.current as f32, enemy_hp.max as f32);
    let distance_weight =
        curve::invert(curve::linear(distance as f32, visibility_range as f32)).clamp(0.0, 1.0);
    let total_weight = (((hp_weight + distance_weight) / 2.0) + offset).clamp(0.0, 1.0);
    WeightedAction {
        action: AiAction::Chase(entity),
        weight: curve::quadratic(total_weight, 1.0, 0.33),
    }
}

fn make_wander_action(position: &Position) -> WeightedAction<AiAction> {
    WeightedAction {
        action: AiAction::Wander(position.clone()),
        weight: 0.0001,
    }
}

pub fn ai_system(
    mut query: Query<(Entity, &mut Ai, &MapPosition, &Eyes)>,
    visible_user_query: Query<(Entity, &User, &Hp, &MapPosition)>,
    chase_target_query: Query<(&MapPosition)>,
    mut commands: Commands,
    game_world: Res<GameWorld>,
    time: Res<Time>,
) {
    for (ent, mut ai, map_pos, eyes) in query.iter_mut() {
        if ai.cooldown > 0.0 {
            ai.cooldown -= time.delta().as_secs_f32();
            continue;
        }
        let mut weighted_actions: Vec<WeightedAction<AiAction>> = vec![];
        if let Some(map) = game_world.game_maps.get(&map_pos.map_id) {
            let visibility_grid =
                map.visibility_grid_from_position(&map_pos.pos, eyes.visible_distance);
            for (user_ent, _user, user_hp, user_pos) in visible_user_query.iter() {
                if user_pos.map_id == map_pos.map_id
                    && visibility_grid.position_visible(&user_pos.pos)
                {
                    if is_adjacent(&user_pos.pos, &map_pos.pos) {
                        let offset = match ai.action {
                            Some(AiAction::Attack(ent)) if ent == user_ent => 0.1,
                            _ => 0.0,
                        };
                        weighted_actions.push(make_attack_action(user_ent, user_hp, offset))
                    } else {
                        let offset = match ai.action {
                            Some(AiAction::Chase(ent)) if ent == user_ent => 0.1,
                            _ => 0.0,
                        };
                        let chase_action = make_chase_action(
                            user_ent,
                            user_hp,
                            distance_between_positions(&user_pos.pos, &map_pos.pos),
                            eyes.visible_distance,
                            offset,
                        );
                        weighted_actions.push(chase_action);
                    }
                }
            }

            // Keep wandering in the same direction if it is already wandering
            let position = match &ai.action {
                Some(AiAction::Wander(position)) if position != &map_pos.pos => {
                    Some(position.clone())
                }
                _ => get_visible_floor_positions(
                    &visibility_grid.grid,
                    &map.get_unblocked_idxs(),
                    visibility_grid.width,
                )
                .choose(&mut rand::thread_rng())
                .cloned(),
            };
            if let Some(position) = position {
                println!("got position {:?}", position);
                weighted_actions.push(make_wander_action(&position));
            }
            // we hold the ai action to give the next chosen action some additional weight
            // choice offset should use random number here
            ai.action = choose_action_fuzzy(weighted_actions, 0.1, 0.6);
            ai.cooldown = 1.0;
            println!("{:?}", &ai.action);
            match &ai.action {
                Some(AiAction::Attack(target_ent)) => {
                    commands.entity(ent).insert(IntendMeleeAttack {
                        target: target_ent.clone(),
                    });
                }
                Some(AiAction::Chase(target_ent)) => {
                    if let Ok(target_pos) = chase_target_query.get(target_ent.clone()) {
                        let path = map.generate_astar(&map_pos.pos, &target_pos.pos);
                        if let Some(idx) = path.get(0) {
                            commands.entity(ent).insert(IntendMove {
                                position: Position::from_idx(*idx as usize, map.width() as usize),
                            });
                        }
                    }
                }
                Some(AiAction::Wander(pos)) => {
                    let path = map.generate_astar(&map_pos.pos, &pos);
                    if let Some(idx) = path.get(0) {
                        commands.entity(ent).insert(IntendMove {
                            position: Position::from_idx(*idx as usize, map.width() as usize),
                        });
                    }
                }
                None => {}
            };
        }
    }
}
