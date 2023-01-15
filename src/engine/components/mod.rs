pub mod eyes;
pub mod paths;


use ae_position::Position;
use bevy::prelude::Component;

use crate::api::{SpriteTexture, UserId};

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

#[derive(Component, Clone)]
pub struct MapPosition {
    pub pos: Position,
    pub map_id: MapId,
}