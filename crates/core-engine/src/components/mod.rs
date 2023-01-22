pub mod combat_stats;
pub mod eyes;
pub mod hp;
pub mod intend_melee_attack;
pub mod intend_move;
pub mod intend_speak;
pub mod paths;
pub mod speaks;
use ae_position::Position;
use bevy::prelude::Component;
use core_api::{SpriteTexture, UserId};
pub mod ai;

use super::resources::world::MapId;

#[derive(Component)]
pub struct User(pub UserId);

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
