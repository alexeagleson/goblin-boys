use ae_position::Position;
use bevy::prelude::*;
use core_api::SpriteTexture;

use crate::{
    components::{BlocksLight, BlocksMovement, MapPosition, Renderable},
    resources::{
        map::GameMap,
        raw_maps::{example_map_1_legend, example_map_2_legend, EXAMPLE_MAP_1, EXAMPLE_MAP_2},
        world::{GameWorld, MapId},
    },
};

fn str_map_to_game_map(
    string_map: &str,
    legend: &dyn Fn(char) -> SpriteTexture,
    commands: &mut Commands,
    map: &GameMap,
) {
    let lines = string_map.lines().filter_map(|line| {
        let trimmed_line = line.trim();

        (!trimmed_line.is_empty()).then_some(trimmed_line)
    });

    for (y, line) in lines.enumerate() {
        let trimmed_line = line.trim();

        for (x, character) in trimmed_line.chars().enumerate() {
            let pos = Position {
                x: x as i32,
                y: y as i32,
            };

            // Every tile needs a floor at minimum
            commands
                .spawn(Name::new("Floor"))
                .insert(MapPosition {
                    pos: pos.clone(),
                    map_id: map.id(),
                })
                .insert(BlocksLight)
                // .insert(BlocksMovement)
                .insert(Renderable {
                    texture: SpriteTexture::FloorConcrete,
                })
                .id()
                .index();

            // Check if there is stuff on top of the floor
            let sprite = legend(character);

            if sprite != SpriteTexture::Empty {
                commands
                    .spawn(Name::new("Wall"))
                    .insert(MapPosition {
                        pos: pos.clone(),
                        map_id: map.id(),
                    })
                    .insert(BlocksLight)
                    .insert(BlocksMovement)
                    .insert(Renderable { texture: sprite })
                    .id()
                    .index();
            }
        }
    }
}

/// Adds the all tiles to the maps on initial load
pub fn spawn_walls_system(game_world: Res<GameWorld>, mut commands: Commands) {
    for map in game_world.game_maps.values() {
        if map.id() == (MapId(1)) {
            str_map_to_game_map(EXAMPLE_MAP_1, &example_map_1_legend, &mut commands, map);
        } else {
            str_map_to_game_map(EXAMPLE_MAP_2, &example_map_2_legend, &mut commands, map);
        }
    }
}
