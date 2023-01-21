use bevy::prelude::Resource;
use core_api::SpriteTexture;
use serde::{Serialize, Deserialize};

use crate::components::{combat_stats::CombatStats, hp::Hp};

#[derive(Debug, Deserialize, Serialize, Resource)]
pub struct EnemyConfig {
    pub name: String,
    pub blocks_movement: bool,
    pub paths: bool,
    pub texture: SpriteTexture,
    pub hp: Hp,
    pub combat_stats: CombatStats,
}
