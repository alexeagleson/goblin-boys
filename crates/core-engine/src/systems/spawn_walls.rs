use ae_position::Position;
use bevy::prelude::*;
use core_api::{DialogueMap, SpriteTexture};

use crate::{
    components::{speaks::Speaks, BlocksLight, BlocksMovement, MapPosition, Renderable},
    data::map_data::{
        example_map_1_legend, example_map_2_legend, DEFAULT_FLOOR_MAP_1, DEFAULT_FLOOR_MAP_2,
        EXAMPLE_MAP_1, EXAMPLE_MAP_2,
    },
    resources::{
        map::GameMap,
        world::{GameWorld, MapId},
    },
};

fn str_map_to_game_map(
    string_map: &str,
    legend: &dyn Fn(char) -> (SpriteTexture, Option<DialogueMap>),
    commands: &mut Commands,
    map: &GameMap,
    floor: SpriteTexture,
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
                .insert(Renderable { texture: floor })
                .id()
                .index();

            // Check if there is stuff on top of the floor
            let (sprite, maybe_dialogue_map) = legend(character);

            if sprite != SpriteTexture::Empty {
                let mut sprite_command = commands.spawn(Renderable { texture: sprite });

                if let Some(maybe_dialogue_map) = maybe_dialogue_map {
                    sprite_command.insert(Speaks(maybe_dialogue_map));
                }

                let blocks_movement_and_light = match sprite {
                    SpriteTexture::WallBrick => true,
                    SpriteTexture::PcBoneyBoi => true,
                    SpriteTexture::PcKidZilla => true,
                    SpriteTexture::ObjectRedSoda => true,
                    SpriteTexture::ObjectSewerGrate => false,
                    SpriteTexture::ObjectWindow => true,
                    SpriteTexture::ObjectLadderUp => false,
                    SpriteTexture::ObjectLadderDown => false,
                    SpriteTexture::ObjectWater => true,
                    SpriteTexture::FloorGrass => false,
                    SpriteTexture::FloorConcrete => false,
                    SpriteTexture::FloorSlime => false,
                    SpriteTexture::NpcFatherNeilFrames6 => true,

                    SpriteTexture::NpcKingRatFrames4 => true,
                    SpriteTexture::NpcSewerKidFrames6 => true,
                    SpriteTexture::NpcSlime => true,
                    SpriteTexture::Empty => false,

                    SpriteTexture::NpcFootballFrames4 => true,
                    SpriteTexture::NpcGoon1Frames4 => true,
                    SpriteTexture::NpcGoon2Frames4 => true,
                    SpriteTexture::NpcGoon3Frames4 => true,
                    SpriteTexture::NpcGoon4Frames4 => true,
                    SpriteTexture::NpcGraceJonesFrames6 => true,
                    SpriteTexture::NpcMallChick1Frames6 => true,
                    SpriteTexture::NpcMallChick2Frames6 => true,
                    SpriteTexture::NpcPersonFrames2 => true,
                    SpriteTexture::NpcRatFrames4 => true,
                    SpriteTexture::NpcSmallRatFrames6 => true,

                    SpriteTexture::ObjectWarpTeeveeFrames3 => false,

                    SpriteTexture::PcAntBoi => true,
                    SpriteTexture::PcAntBoiFrames4 => true,
                    SpriteTexture::PcBoneyBoiFrames4 => true,
                    SpriteTexture::PcGhostBoyFrames8 => true,
                    SpriteTexture::ObjectNewspaper => true,
                    SpriteTexture::NpcRealEstateDickFrames21 => true,
                    
                };

                let name = match sprite {
                    SpriteTexture::WallBrick => "Brick Wall".to_string(),
                    SpriteTexture::PcBoneyBoi => "Boney Boi".to_string(),
                    SpriteTexture::PcKidZilla => "Kidzilla".to_string(),
                    SpriteTexture::ObjectRedSoda => "Soda".to_string(),
                    SpriteTexture::ObjectSewerGrate => "Sewer Grate".to_string(),
                    SpriteTexture::ObjectWindow => "Window".to_string(),
                    SpriteTexture::ObjectLadderUp => "Ladder (Up)".to_string(),
                    SpriteTexture::ObjectLadderDown => "Ladder (Down)".to_string(),
                    SpriteTexture::ObjectWater => "Water".to_string(),
                    SpriteTexture::FloorGrass => "Grass".to_string(),
                    SpriteTexture::FloorConcrete => "Concrete".to_string(),
                    SpriteTexture::FloorSlime => "Slime Floor".to_string(),
                    SpriteTexture::NpcFatherNeilFrames6 => "Father Neil".to_string(),
                    SpriteTexture::NpcKingRatFrames4 => "King Rat".to_string(),
                    SpriteTexture::NpcSewerKidFrames6 => "Sewer Kid".to_string(),
                    SpriteTexture::NpcSlime => "Slime".to_string(),
                    SpriteTexture::Empty => "XXX EMPTY XXX".to_string(),

                    SpriteTexture::NpcFootballFrames4 => "Football".to_string(),
                    SpriteTexture::NpcGoon1Frames4 => "Goon".to_string(),
                    SpriteTexture::NpcGoon2Frames4 => "Goon".to_string(),
                    SpriteTexture::NpcGoon3Frames4 => "Goon".to_string(),
                    SpriteTexture::NpcGoon4Frames4 => "Goon".to_string(),
                    SpriteTexture::NpcGraceJonesFrames6 => "Grace Jones".to_string(),
                    SpriteTexture::NpcMallChick1Frames6 => "Mall Chick".to_string(),
                    SpriteTexture::NpcMallChick2Frames6 => "Mall Chick".to_string(),
                    SpriteTexture::NpcPersonFrames2 => "Person".to_string(),
                    SpriteTexture::NpcRatFrames4 => "Rat".to_string(),
                    SpriteTexture::NpcSmallRatFrames6 => "Small Rat".to_string(),

                    SpriteTexture::ObjectWarpTeeveeFrames3 => "Warp Teevee".to_string(),
                    SpriteTexture::PcAntBoi => "Ant Boi".to_string(),
                    SpriteTexture::PcAntBoiFrames4 => "Ant Boi".to_string(),
                    SpriteTexture::PcBoneyBoiFrames4 => "Boney Boi".to_string(),
                    SpriteTexture::PcGhostBoyFrames8 => "Ghost Boy".to_string(),
                    SpriteTexture::ObjectNewspaper => "Newspaper".to_string(),
                    SpriteTexture::NpcRealEstateDickFrames21 => "Real Estate Dick".to_string(),

                    
                };

                sprite_command.insert(Name::new(name));

                if blocks_movement_and_light {
                    sprite_command.insert(BlocksMovement);
                    sprite_command.insert(BlocksLight);
                };

                sprite_command
                    .insert(MapPosition {
                        pos: pos.clone(),
                        map_id: map.id(),
                    })
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
            str_map_to_game_map(
                EXAMPLE_MAP_1,
                &example_map_1_legend,
                &mut commands,
                map,
                DEFAULT_FLOOR_MAP_1,
            );
        } else {
            str_map_to_game_map(
                EXAMPLE_MAP_2,
                &example_map_2_legend,
                &mut commands,
                map,
                DEFAULT_FLOOR_MAP_2,
            );
        }
    }
}
