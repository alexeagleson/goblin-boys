use bevy::prelude::*;

use crate::{
    api::{EntityIndex, EntityPosition, EntityRenderData, ServerMessageAllClients, SpriteTexture},
    engine::{
        components::{BlocksLight, BlocksMovement, Renderable},
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
        let sprite = Renderable {
            texture: SpriteTexture::Wall,
        };

        let index = commands
            .spawn(Name::new("Wall"))
            .insert(pos.clone())
            .insert(BlocksLight)
            .insert(BlocksMovement)
            .insert(sprite)
            .id()
            .index();

        wall_entities.push(EntityRenderData {
            entity_position: EntityPosition {
                entity_index: EntityIndex { index },
                pos: pos.clone(),
            },
            sprite: SpriteTexture::Wall,
        });
    }

    // Communicate to all clients the positions of all entities including the new ones
    sender
        .0
        .send(ServerMessageAllClients::NewEntities(wall_entities))
        .ok();
}
