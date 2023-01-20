use ae_position::Position;
use bevy::prelude::*;
use core_api::{EntityIndex, SpriteTexture, SpriteUpdate};

use crate::{
    components::{BlocksLight, BlocksMovement, MapPosition, Renderable},
    resources::world::{GameWorld, MapId},
};

/// Adds the all tiles to the maps on initial load
pub fn spawn_walls_system(game_world: Res<GameWorld>, mut commands: Commands) {
    for map in game_world.game_maps.values() {
        let mut wall_entities = Vec::new();

        if map.id() == (MapId(1)) {
            for pos in map.perimeter_positions().iter() {
                let sprite = Renderable {
                    texture: SpriteTexture::Wall,
                };

                let idx = commands
                    .spawn(Name::new("Wall"))
                    .insert(MapPosition {
                        pos: pos.clone(),
                        map_id: map.id(),
                    })
                    .insert(BlocksLight)
                    .insert(BlocksMovement)
                    .insert(sprite)
                    .id()
                    .index();

                wall_entities.push(SpriteUpdate {
                    entity: EntityIndex { idx },
                    pos: pos.clone(),
                    sprite: SpriteTexture::Wall,
                });
            }
        } else {
            // [TODO] Turn this working prototype into something more permanent
            let map_chars = r#"
                ##########
                #..#######
                #.......##
                ##......##
                ##...#..##
                ##...#..##
                ##...#..##
                ##...#..##
                ##########
                ##########
            "#;

            let lines = map_chars.lines().filter_map(|line| {
                let trimmed_line = line.trim();

                (!trimmed_line.is_empty()).then_some(trimmed_line)
            });

            for (y, line) in lines.enumerate() {
                let trimmed_line = line.trim();

                for (x, character) in trimmed_line.chars().enumerate() {
                    if character == '#' {
                        let pos = Position {
                            x: x as i32,
                            y: y as i32,
                        };

                        let sprite = Renderable {
                            texture: SpriteTexture::Wall,
                        };

                        let idx = commands
                            .spawn(Name::new("Wall"))
                            .insert(MapPosition {
                                pos: pos.clone(),
                                map_id: map.id(),
                            })
                            .insert(BlocksLight)
                            .insert(BlocksMovement)
                            .insert(sprite)
                            .id()
                            .index();

                        wall_entities.push(SpriteUpdate {
                            entity: EntityIndex { idx },
                            pos,
                            sprite: SpriteTexture::Wall,
                        });
                    }
                }
            }
        }
    }
}
