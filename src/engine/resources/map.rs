use ae_position::{Position, Dimensions2d};
use bevy::prelude::Resource;

pub const DEFAULT_MAP_WIDTH: i32 = 12;
pub const DEFAULT_MAP_HEIGHT: i32 = 12;

#[derive(Debug, Resource)]
pub struct Map {
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn valid_position(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
    }
}

impl Default for Map {
    fn default() -> Self {
        Self {
            width: DEFAULT_MAP_WIDTH,
            height: DEFAULT_MAP_HEIGHT,
        }
    }
}

impl From<&Map> for Dimensions2d {
    fn from(map: &Map) -> Self {
        Self {
            width: map.width,
            height: map.height,
        }
    }
}
