use bevy::prelude::*;
use core_api::{DatabaseRequest, UserId};

use crate::{
    components::MapPosition,
    resources::{DatabaseReceiver, DatabaseSender},
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
pub fn database_receiver_system(mut db_receiver: ResMut<DatabaseReceiver>) {
    while let Ok((_user_id, db_response)) = db_receiver.0.try_recv() {
        println!("{:?}", db_response);
    }
}
