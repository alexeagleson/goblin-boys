use ae_position::Position;
use bevy::prelude::Component;

use crate::resources::map::{GameMap, VisibilityGrid};

// use crate::engine::resources::map::{Map, VisibilityGrid};

#[derive(Component)]
pub struct Eyes {
    visibility_grid: VisibilityGrid,
    visible_distance: u32,
}

impl Eyes {
    pub fn new(map: &GameMap, visible_distance: u32) -> Self {
        Self {
            visibility_grid: VisibilityGrid::new(map),
            visible_distance,
        }
    }

    pub fn set_visibility(&mut self, pos: &Position, map: &GameMap) {
        self.visibility_grid = map.visibility_grid_from_position(pos, self.visible_distance);
    }

    pub fn position_visible(&self, pos: &Position) -> bool {
        self.visibility_grid.position_visible(pos)
    }

    // pub fn pretty_print(&self) {
    //     self.visibility_grid.pretty_print();
    // }
}
