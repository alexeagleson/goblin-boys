use ae_position::Position;
use bevy::prelude::*;
use core_api::{EntityIndex, ServerMessageSingleClient};

use crate::{
    components::{
        combat_stats::{self, CombatStats},
        hp::Hp,
        MapPosition, User,
    },
    events::{ShouldSendFullMapUpdateToClient, ShouldUpdateMap, TryAttack},
    resources::{world::MapId, CurrentUserMaps, MessageSenderSingleClient},
};

pub fn combat_system(
    mut query: Query<(Entity, &Name, &mut Hp, &MapPosition, &CombatStats)>,

    mut ev_try_attack: EventReader<TryAttack>,
    mut commands: Commands,
    current_user_maps: ResMut<CurrentUserMaps>,
    sender_single_client: Res<MessageSenderSingleClient>,
    mut ev_update_map: EventWriter<ShouldUpdateMap>,
) {
    if ev_try_attack.is_empty() {
        return;
    }

    let mut killed_guys: Vec<(Entity, MapId)> = vec![];

    ev_try_attack.iter().for_each(|attack_event| {
        // Find out if an attackable target exists on the tile
        let target = query
            .iter_mut()
            .find(|(_, _, _, map_pos, _)| &attack_event.map_position == *map_pos);

        if let Some((ent, _, mut hp, map_pos, combat_stats)) = target {
            hp.current -= (attack_event.attack_value - combat_stats.defense).max(0);
            println!("OW");
            if hp.current <= 0 {
                killed_guys.push((ent, map_pos.map_id));
                println!("oh no shit I'm dead");
            }
        }
    });

    killed_guys.iter().for_each(|(ent, dead_entity_map_id)| {
        commands.entity(*ent).despawn();
        // Need to update the map if something dies
        ev_update_map.send(ShouldUpdateMap(*dead_entity_map_id));

        current_user_maps
            .0
            .iter()
            .for_each(|(user_id, user_map_pos)| {
                // Communicate to any users on the old map that the sprite should be removed
                if &user_map_pos.map_id == dead_entity_map_id {
                    sender_single_client
                        .0
                        .send((
                            *user_id,
                            ServerMessageSingleClient::RemoveSprite(EntityIndex {
                                idx: ent.index(),
                            }),
                        ))
                        .ok();
                }
            });
    });
}
