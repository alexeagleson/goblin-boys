// use crate::{
//     api::{ClientMessage, ServerMessageAllClients},
//     database::DatabaseLock, engine::components::UserId,
// };

use core_api::{ClientMessage, UserId};
use log::trace;
use tokio::sync::mpsc::UnboundedSender;
use warp::ws::Message;

// use super::connections::ConnectionsLock;

pub async fn handle_message(
    id: UserId,
    msg: Message,
    // connections: &ConnectionsLock,
    sender: &UnboundedSender<(UserId, ClientMessage)>,
) {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    trace!("{}", msg);

    let request = serde_json::from_str::<ClientMessage>(msg);

    if let Ok(ref request) = request {
        sender.send((id, request.clone())).ok();
    }

   
}
