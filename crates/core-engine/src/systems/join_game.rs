use bevy::prelude::*;
use core_api::SpriteTexture;

use crate::{
    components::{
        ai::Ai, eyes::Eyes, paths::Paths, speaks::Speaks, BlocksLight, BlocksMovement, Item,
        MapPosition, Renderable, User,
    },
    data::{
        dialogue_contents::DialogueContents, enemy_configs::EnemyConfigs,
        player_config::PlayerConfig,
    },
    events::ShouldSendFullMapUpdateToClient,
    resources::{
        map::PEACEFUL_MAP_ID,
        world::{GameWorld, MapId},
        ConnectBuffer, CurrentUserMaps,
    },
};

/// Adds an entity to the game when the user connects
pub fn join_game_system(
    game_world: Res<GameWorld>,
    mut commands: Commands,
    mut connect_buffer: ResMut<ConnectBuffer>,
    mut ev_update_client: EventWriter<ShouldSendFullMapUpdateToClient>,
    mut current_user_maps: ResMut<CurrentUserMaps>,
    player_config: Res<PlayerConfig>,
    // enemy_configs: Res<EnemyConfigs>,
) {
    let map = game_world
        .game_maps
        .get(&MapId(PEACEFUL_MAP_ID))
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
        player_commands
            .insert(Name::new(player_name))
            .insert(player_map_position.clone())
            .insert(Renderable {
                texture: SpriteTexture::PcKidZilla,
            })
            .insert(player_config.combat_stats.clone())
            .insert(player_config.hp.clone());

        // Track the current map the new user is on
        current_user_maps.0.insert(user_id, player_map_position);

        // // Spawn a slime every time a new player joins
        // let mut slime_commands = commands.spawn(Item);
        // let slime_config = &enemy_configs.slime;
        // slime_commands
        //     .insert(Name::new(slime_config.name.clone()))
        //     // A walking slime...?
        //     .insert(MapPosition {
        //         pos: map.random_movement_unblocked_tile(),
        //         map_id: map.id(),
        //     })
        //     .insert(Renderable {
        //         texture: slime_config.texture,
        //     })
        //     .insert(slime_config.hp.clone())
        //     .insert(slime_config.combat_stats.clone())
        //     .insert(BlocksMovement)
        //     .insert(Ai {
        //         action: None,
        //         cooldown: 0.0,
        //     })
        //     .insert(Eyes::new(map, slime_config.visibility));

        // // Spawn a test NPC
        // let mut npc_commands = commands.spawn(
        //     Name::new("Npc Rat".to_string()), // Speaks(DialogueContents)
        // );

        // npc_commands
        //     .insert(MapPosition {
        //         pos: map.random_movement_unblocked_tile(),
        //         map_id: map.id(),
        //     })
        //     .insert(Renderable {
        //         texture: SpriteTexture::NpcSewerKidFrames6,
        //     })
        //     .insert(BlocksLight)
        //     .insert(BlocksMovement);

        // Refresh the full map of all clients when a player joins
        // [TODO] This is probably overkill -- could just send the new player sprite
        ev_update_client.send(ShouldSendFullMapUpdateToClient(map.id()));
    }
}
