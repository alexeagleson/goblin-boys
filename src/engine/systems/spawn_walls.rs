use bevy::prelude::*;

use crate::{
    api::{EntityIndex, EntityPosition, EntityRenderData, ServerMessageAllClients, SpriteTexture},
    engine::{
        components::{BlocksLight, BlocksMovement},
        resources::{map::Map, MessageSenderAllClients},
    },
};

/// Adds the all tiles to the map on initial load
pub fn spawn_walls_system(
    sender: Res<MessageSenderAllClients>,
    map: Res<Map>,
    mut commands: Commands,
) {
    let mut wall_entities = Vec::new();

    for pos in map.perimeter_positions().iter() {
        // Spawn a wall
        let wall_entity_index = commands
            .spawn(Name::new("Wall"))
            .insert(pos.clone())
            .insert(BlocksLight)
            .insert(BlocksMovement)
            .insert(SpriteTexture::Wall)
            .id()
            .index();

        let wall_game_entity = EntityRenderData {
            entity_position: EntityPosition {
                entity_index: EntityIndex {
                    index: wall_entity_index,
                },
                pos: pos.clone(),
            },
            sprite: SpriteTexture::Wall,
        };

        wall_entities.push(wall_game_entity);
    }

    // Communicate to all clients the positions of all entities including the new ones
    sender
        .0
        .send(ServerMessageAllClients::AllEntityRenderData(wall_entities))
        .ok();
}
