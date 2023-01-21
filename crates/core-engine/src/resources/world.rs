use std::collections::HashMap;

use bevy::prelude::*;

use super::{
    map::GameMap,
    raw_maps::{str_map_dimensions, EXAMPLE_MAP_1, EXAMPLE_MAP_2},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MapId(pub i32);

type GameMaps = HashMap<MapId, GameMap>;

#[derive(Resource)]
pub struct GameWorld {
    pub game_maps: GameMaps,
}

impl Default for GameWorld {
    fn default() -> Self {
        let dimensions = str_map_dimensions(EXAMPLE_MAP_1);
        dbg!("{:?}", &dimensions);
        let default_map = GameMap::new(dimensions);

        let mut game_maps: GameMaps = HashMap::new();

        game_maps.insert(default_map.id(), default_map);

        let dimensions = str_map_dimensions(EXAMPLE_MAP_2);
        let second_map = GameMap::new(dimensions);

        game_maps.insert(second_map.id(), second_map);

        Self { game_maps }
    }
}
