pub mod enemy_config;
pub mod player_config;
pub mod enemy_configs;

pub const player_config_str: &str = include_str!("./player_config.ron");
pub const enemy_configs_str: &str = include_str!("./enemy_configs.ron");