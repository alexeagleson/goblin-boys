use std::collections::VecDeque;

use ae_position::{Delta, Position};
use bevy::prelude::Component;

use crate::engine::resources::map::Map;

type Path = VecDeque<Position>;

#[derive(Component)]
pub struct Paths {
    path: Path,
}

impl Paths {
    pub fn set(&mut self, path: Path) {
        self.path = path
    }

    pub fn generate_direct_to_position(from: &Position, to: &Position) -> Path {
        let mut path: Path = VecDeque::new();
        let mut current_pos = from.clone();
        while let Some(delta) = Delta::next_cardinal_delta_to_position(&current_pos, to) {
            current_pos = current_pos.add_delta(&delta);
            path.push_back(current_pos.clone());
        }
        path
    }

    pub fn generate_astar(from: &Position, to: &Position, map: &Map) -> Path {
        let u32_path = map.generate_astar(from, to);
        u32_path
            .into_iter()
            .map(|val| Position::from_idx(val as usize, map.width() as usize))
            .collect()
    }

    pub fn get_next(&mut self) -> Option<Position> {
        self.path.pop_front()
    }
}

impl Default for Paths {
    fn default() -> Self {
        Self {
            path: Default::default(),
        }
    }
}
