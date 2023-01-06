use bevy::{
    prelude::{App, IntoSystemDescriptor},
    MinimalPlugins,
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::api::{ClientMessage, ServerMessage, UserId};

use super::{
    resources::{
        map::Map, ConnectBuffer, DisconnectBuffer, KeypressBuffer, MessageReceiver, MessageSender,
        MouseClickBuffer, MouseHoverBuffer,
    },
    systems::{
        join_game::join_game_system, leave_game::leave_game_system, message::message_system,
        mouse_click::mouse_click_system, mouse_hover::mouse_hover_system,
        movement_keys::movement_keys_system,
    },
};

pub fn start_game_engine(
    client_receiver: UnboundedReceiver<(UserId, ClientMessage)>,
    server_sender: UnboundedSender<(UserId, ServerMessage)>,
) {
    App::new()
        .insert_resource(MessageReceiver(client_receiver))
        .insert_resource(MessageSender(server_sender))
        .insert_resource(Map::default())
        .insert_resource(KeypressBuffer::default())
        .insert_resource(DisconnectBuffer::default())
        .insert_resource(ConnectBuffer::default())
        .insert_resource(MouseHoverBuffer::default())
        .insert_resource(MouseClickBuffer::default())
        .add_system(message_system)
        .add_system(join_game_system.after(message_system))
        .add_system(movement_keys_system.after(message_system))
        .add_system(mouse_hover_system.after(message_system))
        .add_system(mouse_click_system.after(message_system))
        .add_system(leave_game_system.after(message_system))
        .add_plugins(MinimalPlugins)
        .run();
}
