use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};


use super::player_config::PlayerConfig;

#[derive(Debug, Deserialize, Serialize, Resource)]
pub struct PlayerConfigs {
    pub ghost_boy: PlayerConfig,
    pub kidzilla: PlayerConfig,
    pub sewer_kid: PlayerConfig,
    pub boney_boy: PlayerConfig,
    pub ant_boy: PlayerConfig,
}
