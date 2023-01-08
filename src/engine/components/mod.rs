pub mod eyes;
pub mod paths;


use bevy::prelude::Component;

use crate::api::{SpriteTexture, UserId};

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
