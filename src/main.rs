use core_api::{
    ClientMessage, DatabaseRequest, DatabaseResponse, ServerMessageAllClients,
    ServerMessageSingleClient, UserId,
};
use core_database::{database_setup, increment_db_move_count_and_get_total};
use core_engine::start_game_engine;
use core_server::{connections::ConnectionsLock, new_connection::handle_new_connection};
use log::info;
use tokio::sync::mpsc::{self, UnboundedSender};
use warp::{ws::Message, Filter};

// hello!

// hi

fn main() {
    let (client_sender, client_receiver) = mpsc::unbounded_channel::<(UserId, ClientMessage)>();
    let (server_sender_single_client, mut server_receiver_single_client) =
        mpsc::unbounded_channel::<(UserId, ServerMessageSingleClient)>();
    let (server_sender_all_clients, mut server_receiver_all_clients) =
        mpsc::unbounded_channel::<ServerMessageAllClients>();

    let (engine_to_db_sender, mut engine_to_db_receiver) =
        mpsc::unbounded_channel::<(UserId, DatabaseRequest)>();

    let (db_to_engine_sender, db_to_engine_receiver) =
        mpsc::unbounded_channel::<(UserId, DatabaseResponse)>();

    // Initialize the Bevy game engine
    std::thread::spawn(move || {
        start_game_engine(
            client_receiver,
            server_sender_single_client,
            server_sender_all_clients,
            engine_to_db_sender,
            db_to_engine_receiver,
        );
    });

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            pretty_env_logger::init();

            let db = database_setup().await;
            // let db = warp::any().map(move || db.clone());

            let sender = warp::any().map(move || client_sender.clone());

            // Websocket setup
            let connections = ConnectionsLock::default();
            let connections_2 = connections.clone();
            let connections_3 = connections.clone();

            let connections_filter = warp::any().map(move || connections.clone());

            // Database listener
            tokio::task::spawn(async move {
                while let Some((user_id, db_response)) = engine_to_db_receiver.recv().await {
                    match db_response {
                        DatabaseRequest::Placeholder => {
                            let move_count = increment_db_move_count_and_get_total(&db).await;

                            db_to_engine_sender
                                .send((user_id, DatabaseResponse::MoveCount(move_count)))
                                .ok();
                        }
                    }
                }
            });

            // Listener for messages to communicate to all clients
            tokio::task::spawn(async move {
                while let Some(all_clients_message) = server_receiver_all_clients.recv().await {
                    let serialized_message: String =
                        serde_json::to_string(&all_clients_message).expect("Serialize should work");

                    info!("Sending to all: {}", serialized_message);
                    for (&_uid, sender) in connections_2.read().await.0.iter() {
                        sender.send(Message::text(&serialized_message)).ok();
                    }
                }
            });

            // Listener for messages to communicate to specific clients
            tokio::task::spawn(async move {
                while let Some((user_id, single_client_message)) =
                    server_receiver_single_client.recv().await
                {
                    let serialized_message: String = serde_json::to_string(&single_client_message)
                        .expect("Serialize should work");

                    info!("Sending only to user {}: {}", user_id.0, serialized_message);

                    for (&uid, sender) in connections_3.read().await.0.iter() {
                        if uid == user_id.0 {
                            sender.send(Message::text(&serialized_message)).ok();
                        }
                    }
                }
            });

            // GET /game -> websocket upgrade
            let game = warp::path!("api" / "game")
                // The `ws()` filter will prepare Websocket handshake...
                .and(warp::ws())
                .and(connections_filter)
                // .and(db)
                .and(sender)
                .map(
                    |ws: warp::ws::Ws,
                     connections: ConnectionsLock,
                     //  db: DatabaseLock,
                     sender: UnboundedSender<(UserId, ClientMessage)>| {
                        // This will call our function if the handshake succeeds.
                        ws.on_upgrade(move |socket| {
                            handle_new_connection(
                                socket,
                                connections,
                                //  db,
                                sender,
                            )
                        })
                    },
                );

            // If you need to set REST endpoints you can use the example below

            // let any_origin_get = warp::cors().allow_any_origin().allow_method("GET");

            // // GET /game-config returns a `200 OK` with a JSON array of ids:
            // let game_config = warp::path!("api" / "game-config")
            //     .map(|| {
            //         warp::reply::json(&Dimensions2d {
            //             width: MAP_WIDTH,
            //             height: MAP_HEIGHT,
            //         })
            //     })
            //     .with(any_origin_get);

            // // GET / -> index html
            // let index = warp::path::end()
            //     .map(|| warp::reply::html(r#"<html>There is nothing to see here.</html>"#));

            // Serve static directory -- not currently used
            let index = warp::fs::dir("client/dist");

            let routes = index
                // .or(game_config)
                .or(game);

            warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
        });
}
