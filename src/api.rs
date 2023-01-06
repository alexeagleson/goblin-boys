use ae_direction::BodyRelative;
use ae_position::Position;
use bevy::prelude::Component;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Clone, Copy, Serialize, Debug, PartialEq, Eq)]
pub struct UserId {
    pub id: i32,
}

#[typeshare]
#[derive(Clone, Copy, Serialize, Debug, PartialEq, Eq)]
pub struct EntityIndex {
    pub index: u32,
}

#[typeshare]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Information about a specific player's current position
pub struct EntityPositionChange {
    pub entity_index: EntityIndex,
    pub pos: Position,
}

#[typeshare]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Information about a specific player's current position
pub struct GameEntity {
    pub entity_position: EntityPositionChange,
    pub sprite: SpriteTexture,
}

#[typeshare]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Information about a player
pub struct EntityInfo {
    pub name: String,
    pub blocks_light: bool,
}

#[typeshare]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A single entry in the game log
pub struct LogMessage(pub String);

#[typeshare]
#[derive(Component, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A sprite to render that represents a visible entity
pub enum SpriteTexture {
    Bunny,
    Carrot,
}

#[typeshare]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
/// An input interaction from the client
pub enum ClientMessage {
    TileHover(Position),
    TileClick(Position),
    Initialize,
    Keypress(BodyRelative),
    Disconnect,
}

#[typeshare]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
/// Communicates information about the active game to the client
pub enum ServerMessage {
    RemoveEntity(EntityIndex),
    AllGameEntities(Vec<GameEntity>),
    EntityPositionChange(EntityPositionChange),
    TileHover(Option<EntityInfo>),
    TileClick(LogMessage),
    MoveCount(i32),
}
