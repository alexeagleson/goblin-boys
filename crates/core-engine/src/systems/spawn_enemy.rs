use bevy::prelude::*;
use core_api::{
    EntityIndex, LogMessage, ServerMessageAllClients, ServerMessageSingleClient, SpawnableEnemy,
    SpriteTexture, SpriteUpdate,
};
use rand::Rng;

use crate::{
    components::{
        ai::Ai, cooldown::Cooldown, eyes::Eyes, BlocksMovement, Bones, Enemy, MapPosition,
        Renderable,
    },
    data::enemy_configs::EnemyConfigs,
    resources::{
        map::{GameMap, BAD_GUY_MAP_ID},
        world::{GameWorld, MapId},
        CurrentUserMaps, MessageSenderAllClients, MessageSenderSingleClient, SpawnStopWatch,
        SpawnableEnemyBuffer,
    },
};

fn spawn_enemy_and_communicate(
    bad_guy_map: &GameMap,
    current_user_maps: &Res<CurrentUserMaps>,
    enemy: &SpawnableEnemy,
    enemy_configs: &Res<EnemyConfigs>,
    commands: &mut Commands,
    sender_single_client: &Res<MessageSenderSingleClient>,
    sender_all_clients: &Res<MessageSenderAllClients>,
) {
    let enemy_config = match enemy {
        core_api::SpawnableEnemy::Slime => &enemy_configs.slime,
        core_api::SpawnableEnemy::RatKing => &enemy_configs.rat_king,
        core_api::SpawnableEnemy::Rat => &enemy_configs.rat,
    };

    let mut enemy_commands = commands.spawn(Name::new(enemy_config.name.clone()));

    let new_entity_pos = bad_guy_map.random_movement_unblocked_tile();
    let new_entity_texture = enemy_config.texture;
    enemy_commands
        .insert(MapPosition {
            pos: new_entity_pos.clone(),
            map_id: bad_guy_map.id(),
        })
        .insert(Renderable {
            texture: new_entity_texture,
        })
        .insert(enemy_config.hp.clone())
        .insert(enemy_config.combat_stats.clone())
        .insert(BlocksMovement)
        .insert(Enemy)
        .insert(Ai { action: None })
        .insert(Cooldown {
            time_remaining: 0.0,
            attack_time: enemy_config.attack_time,
            move_time: enemy_config.move_time,
        })
        .insert(Eyes::new(bad_guy_map, enemy_config.visibility));

    let new_entity_idx = enemy_commands.id().index();

    current_user_maps
        .0
        .iter()
        .for_each(|(user_id, user_map_pos)| {
            if user_map_pos.map_id == bad_guy_map.id() {
                sender_single_client
                    .0
                    .send((
                        *user_id,
                        ServerMessageSingleClient::AddSprite(SpriteUpdate {
                            entity: EntityIndex {
                                idx: new_entity_idx,
                            },
                            pos: new_entity_pos.clone(),
                            sprite: new_entity_texture,
                        }),
                    ))
                    .ok();
            }
        });

    let log_message = LogMessage(format!(
        "{} has spawned!",
        String::from(enemy_config.name.clone())
    ));
    sender_all_clients
        .0
        .send(ServerMessageAllClients::Log(log_message))
        .ok();
}

/// Allows enemies to be spawned dynamically
pub fn spawn_enemy_system(
    mut spawnable_enemy_buffer: ResMut<SpawnableEnemyBuffer>,
    sender_single_client: Res<MessageSenderSingleClient>,
    mut commands: Commands,
    current_user_maps: Res<CurrentUserMaps>,
    game_world: Res<GameWorld>,
    enemy_configs: Res<EnemyConfigs>,
    mut spawn_stopwatch: ResMut<SpawnStopWatch>,
    time: Res<Time>,
    enemy_query: Query<(Entity, &Renderable), With<Enemy>>,
    sender_all_clients: Res<MessageSenderAllClients>,
) {
    let bad_guy_map = game_world
        .game_maps
        .get(&MapId(BAD_GUY_MAP_ID))
        .expect("Somehow the primary map does not exist");

    if spawn_stopwatch.0.elapsed_secs() < 5.0 {
        spawn_stopwatch.0.tick(time.delta());
    } else {
        spawn_stopwatch.0.reset();

        let mut rat_kings: i32 = 0;
        let mut slimes: i32 = 0;
        let mut rats: i32 = 0;

        for (_enemy_entity, enemy_renderable) in enemy_query.iter() {
            match enemy_renderable.texture {
                SpriteTexture::NpcKingRatFrames4 => rat_kings += 1,
                SpriteTexture::NpcSlime => slimes += 1,
                SpriteTexture::NpcRatFrames4 => rats += 1,
                _ => {
                    // Nada
                }
            }
        }

        let mut rng = rand::thread_rng();

        let d20 = rng.gen_range(0..20) + 1;

        match d20 {
            1..=15 if rats < 10 && rat_kings == 1 => spawn_enemy_and_communicate(
                &bad_guy_map,
                &current_user_maps,
                &SpawnableEnemy::Rat,
                &enemy_configs,
                &mut commands,
                &sender_single_client,
                &sender_all_clients,
            ),
            16..=18 if slimes < 4 => spawn_enemy_and_communicate(
                &bad_guy_map,
                &current_user_maps,
                &SpawnableEnemy::Slime,
                &enemy_configs,
                &mut commands,
                &sender_single_client,
                &sender_all_clients,
            ),
            19..=20 if rat_kings < 1 => spawn_enemy_and_communicate(
                &bad_guy_map,
                &current_user_maps,
                &SpawnableEnemy::Slime,
                &enemy_configs,
                &mut commands,
                &sender_single_client,
                &sender_all_clients,
            ),
            _ => {
                // Nada
            }
        }
    }

    while let Some((_user_id, enemy)) = spawnable_enemy_buffer.0.pop_front() {
        // [TODO] Right now it's slime only but in the future it could be others
        spawn_enemy_and_communicate(
            &bad_guy_map,
            &current_user_maps,
            &enemy,
            &enemy_configs,
            &mut commands,
            &sender_single_client,
            &sender_all_clients,
        )
    }
}
