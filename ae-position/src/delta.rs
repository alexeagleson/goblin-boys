use ae_direction::{Cardinal, Direction, Ordinal};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

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
