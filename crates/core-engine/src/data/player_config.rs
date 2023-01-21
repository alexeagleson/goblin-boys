use crate::components::combat_stats::CombatStats;
use bevy::prelude::Resource;
use core_api::SpriteTexture;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Resource)]
pub struct PlayerConfig {
    pub visibility: u32,
    pub blocks_movement: bool,
    pub blocks_light: bool,
    pub texture: SpriteTexture,
    pub combat_stats: CombatStats,
}
