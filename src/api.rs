use std::fmt::Display;

use ae_position::Position;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
pub type UserId = i32;

#[typeshare]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Information about a specific player's current position
pub struct PlayerPosition {
    pub id: UserId,
    pub pos: Position,
}

#[typeshare]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Information about a player
pub struct PlayerDetails {
    pub name: String,
}

#[typeshare]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Information about the game map
pub struct MapDimensions {
    pub width: i32,
    pub height: i32,
}

#[typeshare]
#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
/// Supported key inputs
pub enum Key {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::Up => write!(f, "up"),
            Key::Down => write!(f, "down"),
            Key::Left => write!(f, "left"),
            Key::Right => write!(f, "right"),
        }
    }
}

#[typeshare]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A single entry in the game log
pub struct LogMessage(pub String);

#[typeshare]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
/// An input interaction from the client
pub enum ClientMessage {
    TileHover(Position),
    TileClick(Position),
    Initialize,
    Keypress(Key),
    Disconnect,
}

#[typeshare]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
/// Communicates information about the active game to the client
pub enum ServerMessage {
    RemovedPlayer(UserId),
    AllPlayerPositions(Vec<PlayerPosition>),
    PlayerPosition(PlayerPosition),
    TileHover(Option<PlayerDetails>),
    TileClick(LogMessage),
    MoveCount(i32),
    Initialize(UserId)
}
