use ae_position::Position;
use bevy::prelude::*;

use crate::{
    api::{
        EntityIndex, EntityPosition, EntityRenderData, ServerMessageAllClients,
        ServerMessageSingleClient, SpriteTexture,
    },
    engine::{
        components::{BlocksLight, BlocksMovement, Eyes, Item, User},
        resources::{ConnectBuffer, MessageSenderAllClients, MessageSenderSingleClient},
    },
};

/// Adds an entity to the game when the user connects
pub fn join_game_system(
    sender_single_client: Res<MessageSenderSingleClient>,
    sender_all_clients: Res<MessageSenderAllClients>,
    mut commands: Commands,
    mut connect_buffer: ResMut<ConnectBuffer>,
    query: Query<(Entity, &Position, &SpriteTexture)>,
) {
    if let Some(user_id) = connect_buffer.0.pop_front() {
        let player_position: Position = Position { x: 1, y: 1 };
        let player_name = format!("Player {}", user_id.id);
        let player_sprite = SpriteTexture::Bunny;

        let player_index = commands
            .spawn(User(user_id))
            .insert(Eyes::new(10))
            .insert(BlocksMovement)
            .insert(BlocksLight)
            .insert(Name::new(player_name))
            .insert(player_position.clone())
            .insert(player_sprite)
            .id()
            .index();

        sender_all_clients
            .0
            .send(ServerMessageAllClients::NewEntity(EntityRenderData {
                entity_position: EntityPosition {
                    entity_index: EntityIndex {
                        index: player_index,
                    },
                    pos: player_position,
                },
                sprite: player_sprite,
            }))
            .ok();

        let carrot_position = Position { x: 5, y: 5 };
        let carrot_sprite = SpriteTexture::Carrot;

        // Spawn a carrot every time a new player joins
        let carrot_entity_index = commands
            .spawn(Item)
            .insert(Name::new("Carrot"))
            .insert(carrot_position.clone())
            .insert(carrot_sprite)
            .id()
            .index();

        sender_all_clients
            .0
            .send(ServerMessageAllClients::NewEntity(EntityRenderData {
                entity_position: EntityPosition {
                    entity_index: EntityIndex {
                        index: carrot_entity_index,
                    },
                    pos: carrot_position,
                },
                sprite: carrot_sprite,
            }))
            .ok();

        // Let the new client get copies of all the entities in the existing world

        let existing_render_entities = query
            .iter()
            .map(|(entity, pos, sprite)| EntityRenderData {
                entity_position: EntityPosition {
                    entity_index: EntityIndex {
                        index: entity.index(),
                    },
                    pos: pos.clone(),
                },
                sprite: sprite.to_owned(),
            })
            .collect::<Vec<_>>();

        // Communicate to the new client all existing render entities
        sender_single_client
            .0
            .send((
                user_id,
                ServerMessageSingleClient::ExistingEntities(existing_render_entities),
            ))
            .ok();
    }
}
