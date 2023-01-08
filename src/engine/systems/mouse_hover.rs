use ae_position::Position;
use bevy::prelude::*;

use crate::{
    api::{EntityData, ServerMessageSingleClient},
    engine::{
        components::{eyes::Eyes, BlocksLight, User},
        resources::{MessageSenderSingleClient, MouseHoverBuffer},
    },
};

/// Looks for an entity at a tile position being hovered
pub fn mouse_hover_system(
    sender: Res<MessageSenderSingleClient>,
    mut mouse_hover_buffer: ResMut<MouseHoverBuffer>,
    mut set: ParamSet<(
        Query<(&Position, &Name, Option<&BlocksLight>)>,
        Query<(&User, &Eyes)>,
    )>,
) {
    if let Some((id, hover_pos)) = mouse_hover_buffer.0.pop_front() {
        let mut hover_entity_info: Option<EntityData> = None;
        for (ent_pos, name, blocks_light) in set.p0().iter() {
            if hover_pos == *ent_pos {
                hover_entity_info = Some(EntityData {
                    name: name.into(),
                    blocks_light: blocks_light.is_some(),
                    visible_to_player: false,
                });
                break;
            }
        }

        if let Some(hover_entity_info) = &mut hover_entity_info {
            for (user, eyes) in set.p1().iter() {
                if user.0 == id {
                    if eyes.position_visible(&hover_pos) {
                        hover_entity_info.visible_to_player = true;
                    }

                    break;
                }
            }
        }

        // Communicate the entity at the hover position to the client that requested it
        // It's important to specifically communicate `None` if there is no entity to handle
        // the case where the user hovers from a tile with an entity to a tile without one
        sender
            .0
            .send((id, ServerMessageSingleClient::TileHover(hover_entity_info)))
            .ok();
    }
}
