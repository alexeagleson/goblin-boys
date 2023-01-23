use bevy::prelude::*;

use crate::components::cooldown::Cooldown;

pub fn cooldown_system(mut query: Query<&mut Cooldown>, time: Res<Time>) {
    for mut cooldown in query.iter_mut() {
        if cooldown.time_remaining > 0.0 {
            cooldown.time_remaining -= time.delta().as_secs_f32();
        }
    }
}
