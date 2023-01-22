use bevy::prelude::*;
use core_api::DialogueMap;

#[derive(Component)]
pub struct Speaks(pub DialogueMap);
