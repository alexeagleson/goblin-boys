use bevy::prelude::Component;

use crate::api::UserId;

use super::resources::map::default_light_blocking_idxs;

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
pub struct Eyes {
    pub visible_tiles: Vec<u8>,
    pub visible_distance: u32,
}

impl Eyes {
    pub fn new(visible_distance: u32) -> Self {
        Self {
            visible_tiles: default_light_blocking_idxs(),
            visible_distance,
        }
    }
}
