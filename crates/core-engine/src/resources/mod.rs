pub mod map;
pub mod world;
pub mod user_id_resource;

use std::collections::{HashMap, VecDeque};

use ae_direction::BodyRelative;
use ae_position::Position;
use bevy::{prelude::Resource, time::Stopwatch};
use core_api::{
    ClientMessage, DatabaseRequest, DatabaseResponse, ServerMessageAllClients,
    ServerMessageSingleClient, UserId, SpawnableEnemy,
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::components::MapPosition;

#[derive(Resource)]
pub struct MessageReceiver(pub UnboundedReceiver<(UserId, ClientMessage)>);

#[derive(Resource)]
pub struct MessageSenderSingleClient(pub UnboundedSender<(UserId, ServerMessageSingleClient)>);

#[derive(Resource)]
pub struct DatabaseSender(pub UnboundedSender<(UserId, DatabaseRequest)>);

#[derive(Resource)]
pub struct DatabaseReceiver(pub UnboundedReceiver<(UserId, DatabaseResponse)>);

#[derive(Resource)]
pub struct MessageSenderAllClients(pub UnboundedSender<ServerMessageAllClients>);

#[derive(Resource, Default)]
pub struct KeypressBuffer(pub VecDeque<(UserId, BodyRelative)>);

#[derive(Resource, Default)]
pub struct DisconnectBuffer(pub VecDeque<UserId>);

#[derive(Resource, Default)]
pub struct ConnectBuffer(pub VecDeque<UserId>);

#[derive(Resource, Default)]
pub struct MouseHoverBuffer(pub VecDeque<(UserId, Position)>);

#[derive(Resource, Default)]
pub struct MouseClickBuffer(pub VecDeque<(UserId, Position)>);

#[derive(Resource, Default)]
pub struct SpawnableEnemyBuffer(pub VecDeque<(UserId, SpawnableEnemy)>);


#[derive(Resource)]
pub struct DebugStopwatch(pub Stopwatch);

impl DebugStopwatch {
    pub fn new() -> Self {
        Self(Stopwatch::new())
    }
}


#[derive(Resource, Default)]

pub struct CurrentUserMaps(pub HashMap<UserId, MapPosition>);
