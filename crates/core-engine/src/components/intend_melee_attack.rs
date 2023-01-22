use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct IntendMeleeAttack {
    pub target: Entity,
}
