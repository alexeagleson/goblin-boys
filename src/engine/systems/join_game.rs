use ae_position::Position;
use bevy::prelude::*;

use crate::{
    api::{
        EntityIndex, EntityPosition, EntityRenderData, ServerMessageAllClients,
        ServerMessageSingleClient, SpriteTexture,
    },
    engine::{
        components::{BlocksLight, BlocksMovement, Item, Renderable, User, eyes::Eyes, paths::Paths},
        resources::{map::Map, ConnectBuffer, MessageSenderAllClients, MessageSenderSingleClient},
    },
};

/// Adds an entity to the game when the user connects
pub fn join_game_system(
    sender_single_client: Res<MessageSenderSingleClient>,
    sender_all_clients: Res<MessageSenderAllClients>,
    map: Res<Map>,
    mut commands: Commands,
    mut connect_buffer: ResMut<ConnectBuffer>,
    query: Query<(Entity, &Position, &Renderable)>,
) {
    if let Some(user_id) = connect_buffer.0.pop_front() {
        let player_position = map.random_movement_unblocked_tile();
        let player_name = format!("Player {}", user_id.id);
        let player_texture = SpriteTexture::Bunny;

        let player_index = commands
            .spawn(User(user_id))
            .insert(Eyes::new(&map, 10))
            .insert(BlocksMovement)
            .insert(BlocksLight)
            .insert(Paths::default())
            .insert(Name::new(player_name))
            .insert(player_position.clone())
            .insert(Renderable {
                texture: player_texture,
            })
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
                sprite: player_texture,
            }))
            .ok();

        let carrot_position = map.random_movement_unblocked_tile();
        let carrot_texture = SpriteTexture::Carrot;

        // Spawn a carrot every time a new player joins
        let carrot_entity_index = commands
            .spawn(Item)
            .insert(Name::new("Carrot"))
            .insert(carrot_position.clone())
            .insert(Renderable {
                texture: carrot_texture,
            })
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
                sprite: carrot_texture,
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
                sprite: sprite.texture,
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
