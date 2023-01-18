use super::connections::ConnectionsLock;
use crate::{api::ClientMessage, engine::components::UserId};
use log::info;
use tokio::sync::mpsc::UnboundedSender;

pub async fn handle_disconnect(
    user_id: UserId,
    connections: &ConnectionsLock,
    sender: &UnboundedSender<(UserId, ClientMessage)>,
) {
    info!("User disconnected: {}", user_id.0);

    // Remove player's connection from the list of active connections
    connections.write().await.0.remove(&user_id.0);
    info!("Removing player connection: {}", user_id.0);

    // Tell Bevy that the player needs to be removed
    sender.send((user_id, ClientMessage::Disconnect)).ok();
}
