use core_api::{UserId, ClientMessage};
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use log::{error, info, trace};
use std::sync::atomic::Ordering;
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::WebSocket;

use crate::{connections::USER_ID_COUNTER, message::handle_message, disconnect::handle_disconnect};

use super::connections::ConnectionsLock;

pub async fn handle_new_connection(
    ws: WebSocket,
    connections: ConnectionsLock,
    sender: UnboundedSender<(UserId, ClientMessage)>,
) {
    let new_id = UserId(USER_ID_COUNTER.fetch_add(1, Ordering::Relaxed));

    info!("New connection: {}", new_id.0);

    // Split the socket into a sender and receive of messages.
    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages to the websocket
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            trace!("Received client message {:?}", message);
            user_ws_tx
                .send(message)
                .unwrap_or_else(|e| {
                    error!("websocket send error: {}", e);
                })
                .await;
        }
    });

    // Add user to the list of active connections
    connections.write().await.new_connection(new_id, tx);

    // Every time the user sends a message, broadcast it to all other users
    'listening: while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                error!("websocket error(uid={}): {}", new_id.0, e);
                break 'listening;
            }
        };
        handle_message(
            new_id,
            msg,
            // &connections,
            // &db,
            // &world,
            &sender,
        )
        .await;
    }

    // user_ws_rx stream will keep processing as long as the user stays
    // connected. Once they disconnect, then...
    handle_disconnect(new_id, &connections, &sender).await;
}
