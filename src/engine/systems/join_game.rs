use ae_position::Position;
use bevy::prelude::*;

use crate::{
    api::{
        EntityIndex, EntityPosition, EntityRenderData, ServerMessageAllClients,
        ServerMessageSingleClient, SpriteTexture,
    },
    engine::{
        components::{
            eyes::Eyes, paths::Paths, BlocksLight, BlocksMovement, Item, MapPosition, Renderable,
            User,
        },
        events::ShouldSendFullMapUpdateToClient,
        resources::{
            map::{Map, PRIMARY_MAP_ID},
            world::GameWorld,
            ConnectBuffer, CurrentUserMaps, MessageSenderAllClients, MessageSenderSingleClient,
        },
    },
};

/// Adds an entity to the game when the user connects
pub fn join_game_system(
    // sender_single_client: Res<MessageSenderSingleClient>,
    // sender_all_clients: Res<MessageSenderAllClients>,
    game_world: Res<GameWorld>,
    mut commands: Commands,
    mut connect_buffer: ResMut<ConnectBuffer>,
    query: Query<(Entity, &MapPosition, &Renderable)>,
    mut ev_update_client: EventWriter<ShouldSendFullMapUpdateToClient>,
    mut current_user_maps: ResMut<CurrentUserMaps>,
) {
    let map = game_world
        .game_maps
        .get(&PRIMARY_MAP_ID)
        .expect("Somehow the primary map does not exist");

    if let Some(user_id) = connect_buffer.0.pop_front() {
        let player_map_position: MapPosition = MapPosition {
            pos: map.random_movement_unblocked_tile(),
            map_id: map.id(),
        };
        let player_name = format!("Player {}", user_id.id);
        let player_texture = SpriteTexture::Bunny;

        let player_index = commands
            .spawn(User(user_id))
            .insert(Eyes::new(&map, 10))
            .insert(BlocksMovement)
            .insert(BlocksLight)
            .insert(Paths::default())
            .insert(Name::new(player_name))
            .insert(player_map_position.clone())
            .insert(Renderable {
                texture: player_texture,
            })
            .id()
            .index();

        current_user_maps.0.insert(user_id, player_map_position);

        let carrot_map_position: MapPosition = MapPosition {
            pos: map.random_movement_unblocked_tile(),
            map_id: map.id(),
        };
        let carrot_texture = SpriteTexture::Carrot;

        // Spawn a carrot every time a new player joins
        let carrot_entity_index = commands
            .spawn(Item)
            .insert(Name::new("Carrot"))
            .insert(carrot_map_position.clone())
            .insert(Renderable {
                texture: carrot_texture,
            })
            .id()
            .index();

        // sender_all_clients
        //     .0
        //     .send(ServerMessageAllClients::NewEntity(EntityRenderData {
        //         entity_position: EntityPosition {
        //             entity_index: EntityIndex {
        //                 index: carrot_entity_index,
        //             },
        //             pos: carrot_map_position.pos,
        //         },
        //         sprite: carrot_texture,
        //     }))
        //     .ok();

        // Let the new client get copies of all the entities in the existing world

        ev_update_client.send(ShouldSendFullMapUpdateToClient(map.id()));

        // let existing_render_entities = query
        //     .iter()
        //     .map(|(entity, map_pos, sprite)| EntityRenderData {
        //         entity_position: EntityPosition {
        //             entity_index: EntityIndex {
        //                 index: entity.index(),
        //             },
        //             pos: map_pos.pos.clone(),
        //         },
        //         sprite: sprite.texture,
        //     })
        //     .collect::<Vec<_>>();

        // // Communicate to the new client all existing render entities
        // sender_single_client
        //     .0
        //     .send((
        //         user_id,
        //         ServerMessageSingleClient::ExistingEntities(existing_render_entities),
        //     ))
        //     .ok();
    }
}
