use ae_position::Position;
use bevy::prelude::*;

use crate::{
    api::{EntityIndex, EntityPositionChange, GameEntity, ServerMessage, SpriteTexture},
    engine::{
        components::{Item, User},
        resources::{ConnectBuffer, MessageSender},
    },
};

/// Adds an entity to the game when the user connects
pub fn join_game_system(
    sender: Res<MessageSender>,
    mut commands: Commands,
    mut connect_buffer: ResMut<ConnectBuffer>,
    query: Query<(Entity, &Position, &SpriteTexture)>,
) {
    if let Some(connected_user_id) = connect_buffer.0.pop_front() {
        let initial_position: Position = Position { x: 0, y: 0 };
        let player_name = format!("Player {}", connected_user_id.id);

        info!("Adding new player named {}", &player_name);

        let player_entity_index = commands
            .spawn(User(connected_user_id))
            .insert(Name::new(player_name))
            .insert(initial_position.clone())
            .insert(SpriteTexture::Bunny)
            .id()
            .index();

        let new_player_entity = GameEntity {
            entity_position: EntityPositionChange {
                entity_index: EntityIndex {
                    index: player_entity_index,
                },
                pos: initial_position,
            },
            // The player is a bunny
            sprite: SpriteTexture::Bunny,
        };

        let carrot_position = Position { x: 5, y: 5 };

        // Spawn a carrot every time a new player joins
        let carrot_entity_index = commands
            .spawn(Item)
            .insert(Name::new("Carrot"))
            .insert(carrot_position.clone())
            .insert(SpriteTexture::Carrot)
            .id()
            .index();

        let carrot_entity = GameEntity {
            entity_position: EntityPositionChange {
                entity_index: EntityIndex {
                    index: carrot_entity_index,
                },
                pos: carrot_position,
            },
            // The other entity is a carrot
            sprite: SpriteTexture::Carrot,
        };

        let mut all_game_entities = query
            .iter()
            .map(|(entity, pos, sprite)| GameEntity {
                entity_position: EntityPositionChange {
                    entity_index: EntityIndex {
                        index: entity.index(),
                    },
                    pos: pos.clone(),
                },
                sprite: sprite.to_owned(),
            })
            .collect::<Vec<_>>();

        all_game_entities.push(new_player_entity);
        all_game_entities.push(carrot_entity);

        // Communicate to all clients the positions of all entities including the new ones
        sender
            .0
            .send((
                connected_user_id,
                ServerMessage::AllGameEntities(all_game_entities),
            ))
            .ok();
    }
}
