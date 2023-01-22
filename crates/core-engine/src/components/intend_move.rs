use ae_position::Position;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct IntendMove {
    pub position: Position,
}
