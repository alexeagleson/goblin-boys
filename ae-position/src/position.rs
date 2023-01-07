use bevy::prelude::Component;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::delta::{Delta, CARDINAL_DELTAS, ORDINAL_DELTAS};

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug, Component)]
#[serde(rename_all = "camelCase")]
/// Represents the location of something on a 2D grid
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    /// Combine with a Delta to get a new position relative to the old position
    pub fn add_delta(&self, delta: &Delta) -> Position {
        let new_x = self.x + delta.x as i32;
        let new_y = self.y + delta.y as i32;

        Position { x: new_x, y: new_y }
    }

    /// Get a vector of all four positions surrounding a position on the grid
    pub fn cardinal_positions(&self) -> Vec<Position> {
        CARDINAL_DELTAS
            .iter()
            .map(|delta| self.add_delta(delta))
            .collect()
    }

    /// Get a vector of all four ordinal surrounding a position on the grid
    pub fn ordinal_positions(&self) -> Vec<Position> {
        ORDINAL_DELTAS
            .iter()
            .map(|delta| self.add_delta(delta))
            .collect()
    }

    /// The position index on a grid of tiles
    pub fn to_idx(&self, grid_width: i32) -> usize {
        (self.y * grid_width + self.x) as usize
    }

    /// The position from the index on a grid of tiles
    pub fn from_idx(idx: usize, grid_width: i32) -> Self {
        Self {
            x: idx as i32 % grid_width,
            y: idx as i32 / grid_width,
        }
    }
}

#[cfg(test)]
mod tests {
    use ae_direction::{Cardinal, Direction, Ordinal};

    use super::*;

    #[test]
    fn add_delta() {
        let pos = Position { x: 1, y: 1 };
        assert_eq!(pos, Position { x: 1, y: 1 });

        let pos = pos.add_delta(&Delta::from(Direction::Cardinal(Cardinal::North)));
        assert_eq!(pos, Position { x: 1, y: 0 });

        let pos = pos.add_delta(&Delta::from(Direction::Cardinal(Cardinal::East)));
        assert_eq!(pos, Position { x: 2, y: 0 });

        let pos = pos.add_delta(&Delta::from(Direction::Ordinal(Ordinal::Southwest)));
        assert_eq!(pos, Position { x: 1, y: 1 });
    }

    #[test]
    fn cardinal_positions() {
        let pos = Position { x: 5, y: 5 };

        let cardinal_positions = pos.cardinal_positions();

        assert!(cardinal_positions.contains(&Position { x: 5, y: 4 }));
        assert!(cardinal_positions.contains(&Position { x: 5, y: 6 }));
        assert!(cardinal_positions.contains(&Position { x: 4, y: 5 }));
        assert!(cardinal_positions.contains(&Position { x: 6, y: 5 }));
    }

    #[test]
    fn ordinal_positions() {
        let pos = Position { x: 5, y: 5 };

        let cardinal_positions = pos.ordinal_positions();

        assert!(cardinal_positions.contains(&Position { x: 6, y: 4 }));
        assert!(cardinal_positions.contains(&Position { x: 6, y: 6 }));
        assert!(cardinal_positions.contains(&Position { x: 4, y: 4 }));
        assert!(cardinal_positions.contains(&Position { x: 4, y: 6 }));
    }

    #[test]
    fn to_idx() {
        let grid_width = 10;
        let pos = Position { x: 5, y: 5 };

        assert_eq!(pos.to_idx(grid_width), 55);

        let grid_width = 8;
        let pos = Position { x: 6, y: 2 };

        assert_eq!(pos.to_idx(grid_width), 22);
    }

    #[test]
    fn from_idx() {
        let grid_width = 10;
        let idx = 55;

        assert_eq!(Position::from_idx(idx, grid_width), Position { x: 5, y: 5 });
    }
}
