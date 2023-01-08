use ae_direction::{Cardinal, Direction, Ordinal};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::Position;

/// A single unit change in relative position meant to be added to a `Position`
/// values intended to be either 1, 0 or -1 and transformed from a `Direction`
#[typeshare]
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Delta {
    pub x: i8,
    pub y: i8,
}

pub const CARDINAL_DELTAS: [Delta; 4] = [
    Delta { x: 0, y: 1 },
    Delta { x: 1, y: 0 },
    Delta { x: 0, y: -1 },
    Delta { x: -1, y: 0 },
];

pub const ORDINAL_DELTAS: [Delta; 4] = [
    Delta { x: 1, y: 1 },
    Delta { x: 1, y: -1 },
    Delta { x: -1, y: 1 },
    Delta { x: -1, y: -1 },
];

impl From<Cardinal> for Delta {
    fn from(dir: Cardinal) -> Self {
        match dir {
            Cardinal::North => Delta { x: 0, y: -1 },
            Cardinal::East => Delta { x: 1, y: 0 },
            Cardinal::South => Delta { x: 0, y: 1 },
            Cardinal::West => Delta { x: -1, y: 0 },
        }
    }
}

impl From<Ordinal> for Delta {
    fn from(dir: Ordinal) -> Self {
        match dir {
            Ordinal::Northeast => Delta { x: 1, y: -1 },
            Ordinal::Southeast => Delta { x: 1, y: 1 },
            Ordinal::Southwest => Delta { x: -1, y: 1 },
            Ordinal::Northwest => Delta { x: -1, y: -1 },
        }
    }
}

impl From<Direction> for Delta {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Cardinal(dir) => match dir {
                Cardinal::North => Delta { x: 0, y: -1 },
                Cardinal::East => Delta { x: 1, y: 0 },
                Cardinal::South => Delta { x: 0, y: 1 },
                Cardinal::West => Delta { x: -1, y: 0 },
            },
            Direction::Ordinal(dir) => match dir {
                Ordinal::Northeast => Delta { x: 1, y: -1 },
                Ordinal::Southeast => Delta { x: 1, y: 1 },
                Ordinal::Southwest => Delta { x: -1, y: 1 },
                Ordinal::Northwest => Delta { x: -1, y: -1 },
            },
        }
    }
}

impl Delta {
    /// Get the next [`Cardinal`] direction [`Delta`] value toward a [`Position`] or None if the positions are the same
    /// Will prioritize X directional movement over Y directional movement
    pub fn next_cardinal_delta_to_position(from: &Position, to: &Position) -> Option<Self> {
        // East
        if to.x > from.x {
            Some(Self { x: 1, y: 0 })
        // West
        } else if to.x < from.x {
            Some(Self { x: -1, y: 0 })
        // South
        } else if to.y > from.y {
            Some(Self { x: 0, y: 1 })
        // North
        } else if to.y < from.y {
            Some(Self { x: 0, y: -1 })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn next_cardinal_delta_to_position_works() {
        // Should suggest moving east
        let from = Position { x: 1, y: 1 };
        let to = Position { x: 3, y: 3 };

        let delta = Delta::next_cardinal_delta_to_position(&from, &to);

        assert_eq!(delta, Some(Delta { x: 1, y: 0 }));

        // Should suggest moving south (increase in y value presuming starting from the upper left)
        let from = Position { x: 3, y: 3 };
        let to = Position { x: 3, y: 7 };

        let delta = Delta::next_cardinal_delta_to_position(&from, &to);

        assert_eq!(delta, Some(Delta { x: 0, y: 1 }));

        // Should suggest None as the positions are the same
        let from = Position { x: 1, y: 1 };
        let to = Position { x: 1, y: 1 };

        let delta = Delta::next_cardinal_delta_to_position(&from, &to);

        assert_eq!(delta, None);
    }
}
