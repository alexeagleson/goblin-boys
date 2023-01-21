use bevy::prelude::*;

#[derive(Component)]
pub struct Hp {
    pub current: i32,
    pub max: i32,
}
