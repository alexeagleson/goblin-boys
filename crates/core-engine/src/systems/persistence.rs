use bevy::prelude::*;
use core_api::{DatabaseRequest, ServerMessageAllClients, UserId};

use crate::{
    components::MapPosition,
    resources::{DatabaseReceiver, DatabaseSender, MessageSenderAllClients},
};

/// Send any request for data, or send data to save in the SQLite database
pub fn database_sender_system(
    db_sender: Res<DatabaseSender>,
    query: Query<Entity, Changed<MapPosition>>,
) {
    for _ in query.iter() {
        // Send a message to the DB any time any entity changes position for any reason
        db_sender
            .0
            .send((
                UserId(-1), // No specific user, but there could be
                DatabaseRequest::Placeholder,
            ))
            .ok();
    }
}

/// Receive a response from the SQLite database message system
pub fn database_receiver_system(
    mut db_receiver: ResMut<DatabaseReceiver>,
    sender_all_clients: Res<MessageSenderAllClients>,
) {
    while let Ok((_user_id, db_response)) = db_receiver.0.try_recv() {
        match db_response {
            core_api::DatabaseResponse::MoveCount(move_count) => {
                sender_all_clients
                    .0
                    .send(ServerMessageAllClients::MoveCount(move_count))
                    .ok();
            }
        }
    }
}
