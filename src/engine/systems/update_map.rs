use ae_position::Position;
use bevy::prelude::*;

use crate::engine::{
    components::{eyes::Eyes, BlocksLight, BlocksMovement, MapPosition},
    events::ShouldUpdateMap,
    resources::world::{GameWorld, MapId},
};

/// Updates visible and blocking tiles on map, and visibility arrays of entities that can see
pub fn update_map_system(
    mut game_world: ResMut<GameWorld>,
    mut ev_update_maps: EventReader<ShouldUpdateMap>,
    mut query: Query<(
        &MapPosition,
        Option<&BlocksLight>,
        Option<&BlocksMovement>,
        Option<&mut Eyes>,
    )>,
) {
    let mut updated_maps: Vec<MapId> = vec![];
    if !ev_update_maps.is_empty() {
        ev_update_maps.iter().for_each(|event| {
            let map_id = event.0;
            if !updated_maps.contains(&map_id) {
                updated_maps.push(map_id);

                let map = game_world.game_maps.get_mut(&map_id);

                if let Some(map) = map {
                    // Update the map's array of movement blocking indexes
                    map.reset_movement_blocking_idxs();
                    for (map_pos, blocks_movement, _, _) in query.iter() {
                        if blocks_movement.is_some() {
                            map.set_blocks_movement(&map_pos.pos);
                        }
                    }

                    // Update the map's array of light blocking indexes
                    map.reset_light_blocking_idxs();
                    for (map_pos, blocks_light, _, _) in query.iter() {
                        if blocks_light.is_some() {
                            map.set_blocks_light(&map_pos.pos);
                        }
                    }

                    // Update the visible tiles for the entities that can see
                    for (map_pos, _, _, eyes) in query.iter_mut() {
                        if let Some(mut eyes) = eyes {
                            eyes.set_visibility(&map_pos.pos, &map);
                        }
                    }
                }
            }
        });
    }
}
