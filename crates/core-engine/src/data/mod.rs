pub mod dialogue_contents;
pub mod enemy_config;
pub mod enemy_configs;
pub mod map_data;
pub mod player_config;

pub const player_config_str: &str = include_str!("./player_config.ron");
pub const enemy_configs_str: &str = include_str!("./enemy_configs.ron");
pub const dialogue_contents_str: &str = include_str!("./dialogue_contents.ron");
