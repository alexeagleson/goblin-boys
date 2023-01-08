use ae_position::Position;
use bevy::prelude::Component;

use crate::api::{SpriteTexture, UserId};

use super::resources::map::{Map, VisibilityGrid};

#[derive(Component)]
pub struct User(pub UserId);

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct BlocksLight;

#[derive(Component)]
pub struct BlocksMovement;

#[derive(Component)]
pub struct Renderable {
    pub texture: SpriteTexture,
}

#[derive(Component)]
pub struct Eyes {
    visibility_grid: VisibilityGrid,
    visible_distance: u32,
}

impl Eyes {
    pub fn new(map: &Map, visible_distance: u32) -> Self {
        Self {
            visibility_grid: VisibilityGrid::new(&map),
            visible_distance,
        }
    }

    pub fn set_visibility(&mut self, pos: &Position, map: &Map) {
        self.visibility_grid = map.visibility_grid_from_position(pos, self.visible_distance);
    }

    pub fn position_visible(&self, pos: &Position) -> bool {
        self.visibility_grid.position_visible(pos)
    }
}
