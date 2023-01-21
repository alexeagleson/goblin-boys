use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Clone, Serialize, Deserialize, Debug)]
pub struct CombatStats {
    pub attack: i32,
    pub defense: i32,
}
