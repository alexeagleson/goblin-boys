use crate::{
    api::{ClientMessage, ServerMessageAllClients},
    database::DatabaseLock, engine::components::UserId,
};

use log::trace;
use tokio::sync::mpsc::UnboundedSender;
use warp::ws::Message;

use super::connections::ConnectionsLock;

pub async fn handle_message(
    id: UserId,
    msg: Message,
    connections: &ConnectionsLock,
    db: &DatabaseLock,
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

    if let Ok(ClientMessage::Keypress(key)) = request {
        // [TODO] Move this SQL stuff somewhere else
        let key_string = key.to_string();
        let db = db.read().await;

        // Log the move in the database regardless of whether it succeeds because why not
        sqlx::query!("INSERT INTO moves (direction) VALUES (?)", key_string)
            .execute(&db.0)
            .await
            .unwrap();

        let count_results = sqlx::query!("SELECT COUNT(id) as count FROM moves")
            .fetch_all(&db.0)
            .await
            .unwrap();

        // Count should have a single record result with a count property of the number of found rows
        let move_count: ServerMessageAllClients =
            ServerMessageAllClients::MoveCount(count_results[0].count);
        let move_count_serialized: String =
            serde_json::to_string(&move_count).expect("that should work");

        for (&_uid, sender) in connections.read().await.0.iter() {
            sender.send(Message::text(&move_count_serialized)).ok();
        }
    }
}
