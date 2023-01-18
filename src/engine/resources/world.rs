use std::collections::HashMap;

use bevy::prelude::*;

use super::map::Map;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MapId(pub i32);

type GameMaps = HashMap<MapId, Map>;

#[derive(Resource)]
pub struct GameWorld {
    pub game_maps: GameMaps,
}

impl Default for GameWorld {
    fn default() -> Self {
        let default_map = Map::default();

        let mut game_maps: GameMaps = HashMap::new();

        game_maps.insert(default_map.id(), default_map);

        let second_map = Map::new(20, 20);

        game_maps.insert(second_map.id(), second_map);

        Self { game_maps }
    }
}
