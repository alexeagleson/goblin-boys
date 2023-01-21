use bevy::prelude::*;
use core_api::SpriteTexture;

use crate::{
    components::{
        eyes::Eyes, paths::Paths, BlocksLight, BlocksMovement, Item, MapPosition, Renderable, User,
    },
    data::{enemy_configs::EnemyConfigs, player_config::PlayerConfig},
    events::ShouldSendFullMapUpdateToClient,
    resources::{
        map::PRIMARY_MAP_ID,
        world::{GameWorld, MapId},
        ConnectBuffer, CurrentUserMaps,
    },
};

/// Adds an entity to the game when the user connects
pub fn join_game_system(
    game_world: Res<GameWorld>,
    mut commands: Commands,
    mut connect_buffer: ResMut<ConnectBuffer>,
    // query: Query<(Entity, &MapPosition, &Renderable)>,
    mut ev_update_client: EventWriter<ShouldSendFullMapUpdateToClient>,
    mut current_user_maps: ResMut<CurrentUserMaps>,
    player_config: Res<PlayerConfig>,
    enemy_configs: Res<EnemyConfigs>,
) {
    let map = game_world
        .game_maps
        .get(&MapId(PRIMARY_MAP_ID))
        .expect("Somehow the primary map does not exist");

    if let Some(user_id) = connect_buffer.0.pop_front() {
        let player_name = format!("Player {}", user_id.0);
        let player_map_position = MapPosition {
            pos: map.random_movement_unblocked_tile(),
            map_id: map.id(),
        };
        let mut player_commands = commands.spawn(User(user_id));
        player_commands.insert(Eyes::new(map, player_config.visibility));
        if player_config.blocks_movement {
            player_commands.insert(BlocksMovement);
        }
        if player_config.blocks_movement {
            player_commands.insert(BlocksLight);
        }
        player_commands
            .insert(Name::new(player_name))
            .insert(player_map_position.clone())
            .insert(Renderable {
                texture: SpriteTexture::Bunny,
            })
            .insert(player_config.combat_stats.clone());

        // Track the current map the new user is on
        current_user_maps.0.insert(user_id, player_map_position);

        // Spawn a carrot every time a new player joins
        let mut carrot_commands = commands.spawn(Item);
        carrot_commands
            .insert(Name::new(enemy_configs.carrot.name.clone()))
            // A walking carrot...
            .insert(MapPosition {
                pos: map.random_movement_unblocked_tile(),
                map_id: map.id(),
            })
            .insert(Renderable {
                texture: enemy_configs.carrot.texture,
            })
            .insert(enemy_configs.carrot.hp.clone())
            .insert(enemy_configs.carrot.combat_stats.clone());
        if enemy_configs.carrot.blocks_movement {
            carrot_commands.insert(Paths::default());
        }
        if enemy_configs.carrot.paths {
            carrot_commands.insert(BlocksMovement);
        }
        // Refresh the full map of all clients when a player joins
        // [TODO] This is probably overkill -- could just send the new player sprite
        ev_update_client.send(ShouldSendFullMapUpdateToClient(map.id()));
    }
}
