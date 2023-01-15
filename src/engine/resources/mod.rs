pub mod map;
pub mod world;

use std::collections::VecDeque;

use ae_direction::BodyRelative;
use ae_position::Position;
use bevy::{prelude::Resource, time::Stopwatch};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::api::{ClientMessage, ServerMessageAllClients, ServerMessageSingleClient, UserId};

#[derive(Resource)]
pub struct MessageReceiver(pub UnboundedReceiver<(UserId, ClientMessage)>);

#[derive(Resource)]
pub struct MessageSenderSingleClient(pub UnboundedSender<(UserId, ServerMessageSingleClient)>);

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

#[derive(Resource)]
pub struct MoveStopwatch(pub Stopwatch);

impl MoveStopwatch {
    pub fn new() -> Self {
        Self(Stopwatch::new())
    }
}
