use bevy::prelude::*;

use crate::{
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
    if ev_update_maps.is_empty() {
        return;
    }

    let mut updated_maps: Vec<MapId> = vec![];
    ev_update_maps.iter().for_each(|event| {
        let update_map_id = event.0;
        if !updated_maps.contains(&update_map_id) {
            updated_maps.push(update_map_id);

            let map = game_world.game_maps.get_mut(&update_map_id);

            if let Some(map) = map {
                // Update the map's array of blockers
                map.reset_movement_blocking_idxs();
                map.reset_light_blocking_idxs();

                for (map_pos, blocks_light, blocks_movement, _) in query.iter() {
                    // Entities don't block tiles unless they are on the same map!
                    if map_pos.map_id != update_map_id {
                        continue;
                    }

                    if blocks_movement.is_some() {
                        map.set_blocks_movement(&map_pos.pos);
                    }

                    if blocks_light.is_some() {
                        map.set_blocks_light(&map_pos.pos);
                    }
                }

                // Update the visible tiles for the entities that can see
                for (map_pos, _, _, eyes) in query.iter_mut() {
                    if map_pos.map_id != update_map_id {
                        continue;
                    }

                    if let Some(mut eyes) = eyes {
                        eyes.set_visibility(&map_pos.pos, &map);
                    }
                }
            }
        }
    });
}
