use bevy::prelude::*;

#[derive(Component)]
pub struct CombatStats {
    pub attack: i32,
    pub defense: i32,
}
