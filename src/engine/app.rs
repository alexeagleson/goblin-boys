use bevy::{
    prelude::{App, IntoSystemDescriptor},
    MinimalPlugins, time::Time,
};


use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::api::{ClientMessage, ServerMessageAllClients, ServerMessageSingleClient, UserId};

use super::{
    events::ShouldUpdateMap,
    resources::{
        map::Map, ConnectBuffer, DisconnectBuffer, KeypressBuffer, MessageReceiver,
        MessageSenderAllClients, MessageSenderSingleClient, MouseClickBuffer, MouseHoverBuffer,
        MoveStopwatch,
    },
    systems::{
        join_game::join_game_system, leave_game::leave_game_system, message::message_system,
        mouse_click::mouse_click_system, mouse_hover::mouse_hover_system,
        move_timer::move_timer_system, movement_keys::movement_keys_system,
        spawn_walls::spawn_walls_system, update_map::update_map_system,
    },
};

pub fn start_game_engine(
    client_receiver: UnboundedReceiver<(UserId, ClientMessage)>,
    server_sender_single_client: UnboundedSender<(UserId, ServerMessageSingleClient)>,
    server_sender_all_clients: UnboundedSender<ServerMessageAllClients>,
) {
    App::new()
        .insert_resource(MessageReceiver(client_receiver))
        .insert_resource(MessageSenderSingleClient(server_sender_single_client))
        .insert_resource(MessageSenderAllClients(server_sender_all_clients))
        .insert_resource(Map::default())
        .insert_resource(KeypressBuffer::default())
        .insert_resource(DisconnectBuffer::default())
        .insert_resource(ConnectBuffer::default())
        .insert_resource(MouseHoverBuffer::default())
        .insert_resource(MouseClickBuffer::default())
        .insert_resource(MoveStopwatch::new())
        .insert_resource(Time::default())
        .add_event::<ShouldUpdateMap>()
        .add_startup_system(spawn_walls_system)
        .add_system(message_system)
        .add_system(join_game_system.after(message_system))
        .add_system(movement_keys_system.after(message_system))
        .add_system(move_timer_system.after(message_system))
        .add_system(mouse_hover_system.after(message_system))
        .add_system(mouse_click_system.after(message_system))
        .add_system(leave_game_system.after(message_system))
        // Don't run the map updater until after entities have moved
        .add_system(
            update_map_system
                .after(movement_keys_system)
                .after(move_timer_system),
        )
        .add_plugins(MinimalPlugins)
        .run();
}
