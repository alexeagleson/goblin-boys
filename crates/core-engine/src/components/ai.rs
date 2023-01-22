use ae_position::Position;
use bevy::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum AiAction {
    Attack(Entity),
    Chase(Entity),
    Wander(Position),
}

#[derive(Component, Debug)]
pub struct Ai {
    pub action: Option<AiAction>,
    pub cooldown: f32
}
