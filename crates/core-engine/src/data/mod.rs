pub mod dialogue_contents;
pub mod enemy_config;
pub mod enemy_configs;
pub mod map_data;
pub mod player_config;
pub mod player_configs;


pub const player_configs_str: &str = include_str!("./player_configs.ron");
pub const enemy_configs_str: &str = include_str!("./enemy_configs.ron");
pub const dialogue_contents_str: &str = include_str!("./dialogue_contents.ron");
