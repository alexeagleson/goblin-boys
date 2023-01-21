pub mod eyes;
pub mod paths;
pub mod hp;
pub mod combat_stats;

use ae_position::Position;
use bevy::prelude::Component;
use core_api::{SpriteTexture, UserId};

use super::resources::world::MapId;

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

/// The minimum information required to fully describe the location of an entity
#[derive(Component, Clone, PartialEq, Eq)]
pub struct MapPosition {
    pub pos: Position,
    pub map_id: MapId,
}
