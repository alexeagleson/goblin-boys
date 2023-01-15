use std::{
    collections::HashMap,
    sync::atomic::{AtomicI32, Ordering},
};

use bevy::prelude::*;

use super::map::Map;

pub type MapId = i32;



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

        Self { game_maps }
    }
}
