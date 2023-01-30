use bevy::prelude::*;
use core_api::ClientMessage;

use crate::resources::{
    ConnectBuffer, DisconnectBuffer, KeypressBuffer, MessageReceiver, MouseClickBuffer,
    MouseHoverBuffer, SpawnableEnemyBuffer,
};

/// Handles all messages received from the client and places them into separate resource
/// buffers so they can be handled by separate systems independently
pub fn message_system(
    mut receiver: ResMut<MessageReceiver>,
    mut keypress_buffer: ResMut<KeypressBuffer>,
    mut disconnect_buffer: ResMut<DisconnectBuffer>,
    mut connect_buffer: ResMut<ConnectBuffer>,
    mut mouse_hover_buffer: ResMut<MouseHoverBuffer>,
    mut mouse_click_buffer: ResMut<MouseClickBuffer>,
    mut spawnable_enemy_buffer: ResMut<SpawnableEnemyBuffer>,
) {
    while let Ok((id, message)) = receiver.0.try_recv() {
        match message {
            ClientMessage::Initialize => {
                connect_buffer.0.push_back(id);
            }
            ClientMessage::Keypress(k) => {
                keypress_buffer.0.push_back((id, k));
            }
            ClientMessage::Disconnect => {
                disconnect_buffer.0.push_back(id);
            }
            ClientMessage::TileHover(pos) => {
                mouse_hover_buffer.0.push_back((id, pos));
            }
            ClientMessage::TileClick(pos) => {
                mouse_click_buffer.0.push_back((id, pos));
            }
            ClientMessage::Spawn(enemy) => {
                spawnable_enemy_buffer.0.push_back((id, enemy));
            },
            ClientMessage::KeepAlive => {
                // No action
            }
        }
    }
}
