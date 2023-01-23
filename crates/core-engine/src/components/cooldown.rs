use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Cooldown {
    pub time_remaining: f32,
    pub attack_time: f32,
    pub move_time: f32,
}
