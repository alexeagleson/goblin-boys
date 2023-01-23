use bevy::prelude::*;
use core_api::{EntityIndex, ServerMessageSingleClient, SpriteUpdate};

use crate::{
    components::{ai::Ai, cooldown::Cooldown, eyes::Eyes, BlocksMovement, MapPosition, Renderable},
    data::enemy_configs::EnemyConfigs,
    resources::{
        map::BAD_GUY_MAP_ID,
        world::{GameWorld, MapId},
        CurrentUserMaps, MessageSenderSingleClient, SpawnableEnemyBuffer,
    },
};

/// Allows enemies to be spawned dynamically
pub fn spawn_enemy_system(
    mut spawnable_enemy_buffer: ResMut<SpawnableEnemyBuffer>,
    sender_single_client: Res<MessageSenderSingleClient>,
    mut commands: Commands,
    current_user_maps: Res<CurrentUserMaps>,
    game_world: Res<GameWorld>,
    enemy_configs: Res<EnemyConfigs>,
) {
    let bad_guy_map = game_world
        .game_maps
        .get(&MapId(BAD_GUY_MAP_ID))
        .expect("Somehow the primary map does not exist");

    while let Some((_user_id, enemy)) = spawnable_enemy_buffer.0.pop_front() {
        // [TODO] Right now it's slime only but in the future it could be others
        let enemy_config = match enemy {
            core_api::SpawnableEnemy::Slime => &enemy_configs.slime,
        };

        let mut slime_commands = commands.spawn(Name::new(enemy_config.name.clone()));
        let enemy_config = &enemy_configs.slime;

        let new_entity_pos = bad_guy_map.random_movement_unblocked_tile();
        let new_entity_texture = enemy_config.texture;
        slime_commands
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
            .insert(Ai { action: None })
            .insert(Cooldown {
                time_remaining: 0.0,
                attack_time: enemy_config.attack_time,
                move_time: enemy_config.move_time,
            })
            .insert(Eyes::new(bad_guy_map, enemy_config.visibility));

        let new_entity_idx = slime_commands.id().index();

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
    }
}
