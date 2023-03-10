pub mod components;
pub mod data;
pub mod events;
pub mod resources;
pub mod systems;

use bevy::{
    prelude::{App, IntoSystemDescriptor},
    time::Time,
    MinimalPlugins,
};

use components::cooldown;
use core_api::{
    ClientMessage, DatabaseRequest, DatabaseResponse, ServerMessageAllClients,
    ServerMessageSingleClient, UserId,
};
use data::{
    dialogue_contents::DialogueContents, dialogue_contents_str, enemy_configs::EnemyConfigs,
    enemy_configs_str, player_configs::PlayerConfigs, player_configs_str,
};
use resources::{DatabaseReceiver, DatabaseSender, SpawnStopWatch, SpawnableEnemyBuffer};
use systems::{
    ai::ai_system,
    cooldown::cooldown_system,
    death::death_system,
    debug::debug_system,
    persistence::{database_receiver_system, database_sender_system},
    resolve_consume::resolve_consume_system,
    resolve_melee_attack::resolve_melee_attack_system,
    resolve_move::resolve_move_system,
    resolve_speak::resolve_speak_system,
    spawn_enemy::spawn_enemy_system,
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::{
    events::{ShouldSendFullMapUpdateToClient, ShouldUpdateMap},
    resources::{
        world::GameWorld, ConnectBuffer, CurrentUserMaps, DebugStopwatch, DisconnectBuffer,
        KeypressBuffer, MessageReceiver, MessageSenderAllClients, MessageSenderSingleClient,
        MouseClickBuffer, MouseHoverBuffer,
    },
    systems::{
        build_maps::build_maps_system, change_map::change_map_system, join_game::join_game_system,
        leave_game::leave_game_system, message::message_system, mouse_click::mouse_click_system,
        mouse_hover::mouse_hover_system, movement_keys::movement_keys_system,
        update_client::update_client_system, update_map::update_map_system,
    },
};

pub fn start_game_engine(
    client_receiver: UnboundedReceiver<(UserId, ClientMessage)>,
    server_sender_single_client: UnboundedSender<(UserId, ServerMessageSingleClient)>,
    server_sender_all_clients: UnboundedSender<ServerMessageAllClients>,
    db_sender: UnboundedSender<(UserId, DatabaseRequest)>,
    db_receiver: UnboundedReceiver<(UserId, DatabaseResponse)>,
) {
    App::new()
        .insert_resource(MessageReceiver(client_receiver))
        .insert_resource(DatabaseSender(db_sender))
        .insert_resource(DatabaseReceiver(db_receiver))
        .insert_resource(MessageSenderSingleClient(server_sender_single_client))
        .insert_resource(MessageSenderAllClients(server_sender_all_clients))
        .insert_resource(GameWorld::default())
        .insert_resource(KeypressBuffer::default())
        .insert_resource(DisconnectBuffer::default())
        .insert_resource(ConnectBuffer::default())
        .insert_resource(MouseHoverBuffer::default())
        .insert_resource(MouseClickBuffer::default())
        .insert_resource(SpawnableEnemyBuffer::default())
        .insert_resource(DebugStopwatch::new())
        .insert_resource(SpawnStopWatch::new())
        .insert_resource(Time::default())
        .insert_resource(CurrentUserMaps::default())
        .insert_resource(ron::from_str::<PlayerConfigs>(player_configs_str).unwrap())
        .insert_resource(ron::from_str::<EnemyConfigs>(enemy_configs_str).unwrap())
        .insert_resource(ron::from_str::<DialogueContents>(dialogue_contents_str).unwrap())
        .add_event::<ShouldUpdateMap>()
        .add_event::<ShouldSendFullMapUpdateToClient>()
        .add_startup_system(build_maps_system)
        .add_system(update_client_system.before(message_system))
        .add_system(message_system)
        .add_system(cooldown_system)
        .add_system(join_game_system.after(message_system))
        .add_system(spawn_enemy_system.after(message_system))
        .add_system(movement_keys_system.after(message_system))
        .add_system(ai_system.after(movement_keys_system))
        .add_system(resolve_move_system.after(message_system))
        // .add_system(pathing_system.after(message_system))
        // THIS SYSTEM WILL PANIC
        // .add_system(mouse_hover_system.after(message_system))
        .add_system(mouse_click_system.after(message_system))
        .add_system(leave_game_system.after(message_system))
        .add_system(change_map_system.after(message_system))
        // Don't run the map updater until after entities have moved
        .add_system(
            update_map_system.after(movement_keys_system), // .after(combat_system), // .after(pathing_system),
        )
        .add_system(resolve_melee_attack_system.after(update_map_system))
        .add_system(death_system.after(resolve_melee_attack_system))
        .add_system(resolve_speak_system.after(update_map_system))
        .add_system(resolve_consume_system.after(update_map_system))
        .add_system(database_sender_system.after(update_map_system))
        .add_system(database_receiver_system.after(update_map_system))
        .add_system(debug_system.after(database_receiver_system))
        .add_plugins(MinimalPlugins)
        .run();
}
