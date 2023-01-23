use bevy::prelude::Resource;
use core_api::SpriteTexture;
use serde::{Deserialize, Serialize};

use crate::components::{combat_stats::CombatStats, cooldown::Cooldown, hp::Hp};

#[derive(Debug, Deserialize, Serialize, Resource)]
pub struct EnemyConfig {
    pub name: String,
    pub visibility: u32,
    pub blocks_movement: bool,
    pub paths: bool,
    pub texture: SpriteTexture,
    pub hp: Hp,
    pub combat_stats: CombatStats,
    pub attack_time: f32,
    pub move_time: f32,
}
