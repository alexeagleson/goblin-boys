use crate::{
    components::{cooldown::Cooldown, eyes::Eyes, BlocksMovement, MapPosition, Renderable, User},
    data::{player_config::PlayerConfig, player_configs::PlayerConfigs},
    events::ShouldSendFullMapUpdateToClient,
    resources::{
        map::PEACEFUL_MAP_ID,
        world::{GameWorld, MapId},
        ConnectBuffer, CurrentUserMaps,
    },
};
use bevy::prelude::*;

/// Adds an entity to the game when the user connects
pub fn join_game_system(
    game_world: Res<GameWorld>,
    mut commands: Commands,
    mut connect_buffer: ResMut<ConnectBuffer>,
    mut ev_update_client: EventWriter<ShouldSendFullMapUpdateToClient>,
    mut current_user_maps: ResMut<CurrentUserMaps>,
    player_configs: Res<PlayerConfigs>,
    // enemy_configs: Res<EnemyConfigs>,
) {
    let map = game_world
        .game_maps
        .get(&MapId(PEACEFUL_MAP_ID))
        .expect("Somehow the primary map does not exist");

    if let Some((player_user_id, player_name, player_sprite)) = connect_buffer.0.pop_front() {
        let player_name = format!("{}", player_name);
        let player_map_position = MapPosition {
            pos: map.random_movement_unblocked_tile(),
            map_id: map.id(),
        };
        let mut player_commands = commands.spawn(User(player_user_id));

        let player_config = &player_configs.ghost_boy;

        player_commands.insert(Eyes::new(map, player_config.visibility));

        if player_config.blocks_movement {
            player_commands.insert(BlocksMovement);
        }
        player_commands
            .insert(Name::new(player_name))
            .insert(player_map_position.clone())
            .insert(Renderable {
                texture: player_sprite,
            })
            .insert(player_config.combat_stats.clone())
            .insert(player_config.hp.clone())
            .insert(Cooldown {
                time_remaining: 0.0,
                move_time: player_config.move_time,
                attack_time: player_config.attack_time,
            });

        // Track the current map the new user is on
        current_user_maps
            .0
            .insert(player_user_id, player_map_position);

        // Refresh the full map of all clients when a player joins
        // [TODO] This is probably overkill -- could just send the new player sprite
        ev_update_client.send(ShouldSendFullMapUpdateToClient(map.id()));
    }
}
