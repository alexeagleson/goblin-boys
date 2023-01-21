use bevy::prelude::*;
use core_api::SpriteTexture;

use crate::{
    components::{
        combat_stats::CombatStats, eyes::Eyes, hp::Hp, paths::Paths, BlocksLight, BlocksMovement,
        Item, MapPosition, Renderable, User,
    },
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
        commands
            .spawn(User(user_id))
            .insert(Eyes::new(map, 10))
            .insert(BlocksMovement)
            .insert(BlocksLight)
            .insert(Name::new(player_name))
            .insert(player_map_position.clone())
            .insert(Renderable {
                texture: SpriteTexture::Bunny,
            })
            .insert(CombatStats {
                attack: 3,
                defense: 1,
            });

        // Track the current map the new user is on
        current_user_maps.0.insert(user_id, player_map_position);

        // Spawn a carrot every time a new player joins
        commands
            .spawn(Item)
            .insert(Name::new("Carrot"))
            // A walking carrot...
            .insert(Paths::default())
            .insert(BlocksMovement)
            .insert(MapPosition {
                pos: map.random_movement_unblocked_tile(),
                map_id: map.id(),
            })
            .insert(Renderable {
                texture: SpriteTexture::Carrot,
            })
            .insert(Hp {
                current: 10,
                max: 10,
            })
            .insert(CombatStats {
                attack: 2,
                defense: 1,
            });

        // Refresh the full map of all clients when a player joins
        // [TODO] This is probably overkill -- could just send the new player sprite
        ev_update_client.send(ShouldSendFullMapUpdateToClient(map.id()));
    }
}
