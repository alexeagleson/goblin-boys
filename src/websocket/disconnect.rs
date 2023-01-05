use super::connections::ConnectionsLock;
use crate::api::{ClientMessage, ServerMessage, UserId};
use log::info;
use tokio::sync::mpsc::UnboundedSender;
use warp::ws::Message;

pub async fn handle_disconnect(
    id: UserId,
    connections: &ConnectionsLock,
    sender: &UnboundedSender<(UserId, ClientMessage)>,
) {
    info!("User disconnected: {}", id);

    let remove_player = ServerMessage::RemovedPlayer(id);
    let remove_player_serialized: String =
        serde_json::to_string(&remove_player).expect("Serialize should work");

    // Remove player's connection from the list of active connections
    connections.write().await.0.remove(&id);
    info!("Removing player connection: {}", id);

    // Tell Bevy that the player needs to be removed
    sender.send((id, ClientMessage::Disconnect)).ok();

    // Tell any remaining clients that the player has been removed
    for (&_uid, sender) in connections.read().await.0.iter() {
        sender.send(Message::text(&remove_player_serialized)).ok();
    }
}
