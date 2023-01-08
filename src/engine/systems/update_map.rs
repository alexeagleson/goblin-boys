use ae_position::Position;
use bevy::prelude::*;

use crate::engine::{
    components::{eyes::Eyes, BlocksLight, BlocksMovement},
    events::ShouldUpdateMap,
    resources::map::Map,
};

/// Updates visible and blocking tiles on map, and visibility arrays of entities that can see
pub fn update_map_system(
    mut map: ResMut<Map>,
    ev_update_map: EventReader<ShouldUpdateMap>,
    mut query: Query<(
        &Position,
        Option<&BlocksLight>,
        Option<&BlocksMovement>,
        Option<&mut Eyes>,
    )>,
) {
    if !ev_update_map.is_empty() {
        // Update the map's array of movement blocking indexes
        map.reset_movement_blocking_idxs();
        for (pos, blocks_movement, _, _) in query.iter() {
            if blocks_movement.is_some() {
                map.set_blocks_movement(pos);
            }
        }

        // Update the map's array of light blocking indexes
        map.reset_light_blocking_idxs();
        for (pos, blocks_light, _, _) in query.iter() {
            if blocks_light.is_some() {
                map.set_blocks_light(pos);
            }
        }

        // Update the visible tiles for the entities that can see
        for (pos, _, _, eyes) in query.iter_mut() {
            if let Some(mut eyes) = eyes {
                eyes.set_visibility(pos, &map);
            }
        }
    }
}
