use bevy::{
    prelude::{Name, Query, Res, ResMut, Without},
    time::Time,
};
use core_api::{DebugData, ServerMessageAllClients};

use crate::{
    components::{combat_stats::CombatStats, hp::Hp, MapPosition, User},
    resources::{DebugStopwatch, MessageSenderAllClients},
};

pub fn debug_system(
    time: Res<Time>,
    mut debug_stopwatch: ResMut<DebugStopwatch>,
    sender_all_clients: Res<MessageSenderAllClients>,
    target_query: Query<(&CombatStats, &mut Hp, &Name, &MapPosition), Without<User>>,
) {
    if debug_stopwatch.0.elapsed_secs() < 0.5 {
        debug_stopwatch.0.tick(time.delta());
    } else {
        debug_stopwatch.0.reset();

        let num_enemies = target_query.iter().count() as i32;

        sender_all_clients
            .0
            .send(ServerMessageAllClients::Debug(DebugData { num_enemies }))
            .ok();
    }
}
