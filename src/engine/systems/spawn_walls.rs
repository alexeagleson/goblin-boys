use bevy::prelude::*;

use crate::{
    api::{EntityIndex, EntityPositionChange, GameEntity, ServerMessage, SpriteTexture, UserId},
    engine::{
        components::{BlocksLight, BlocksMovement},
        resources::{map::Map, MessageSender},
    },
};

/// Adds the all tiles to the map on initial load
pub fn spawn_walls_system(
    sender: Res<MessageSender>,
    map: Res<Map>,
    mut commands: Commands,
    // query: Query<(Entity, &Position, &SpriteTexture)>,
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

        let wall_game_entity = GameEntity {
            entity_position: EntityPositionChange {
                entity_index: EntityIndex {
                    index: wall_entity_index,
                },
                pos: pos.clone(),
            },
            // The other entity is a carrot
            sprite: SpriteTexture::Wall,
        };

        wall_entities.push(wall_game_entity);
    }

    // Communicate to all clients the positions of all entities including the new ones
    sender
        .0
        .send((
            UserId { id: 0 }, // User 0 does not exist but this will send to all
            ServerMessage::AllGameEntities(wall_entities),
        ))
        .ok();
}
