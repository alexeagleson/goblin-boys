pub mod map;

use std::collections::VecDeque;

use ae_position::Position;
use bevy::prelude::Resource;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::api::{ClientMessage, Key, ServerMessage, UserId};

#[derive(Resource)]
pub struct MessageReceiver(pub UnboundedReceiver<(UserId, ClientMessage)>);

#[derive(Resource)]
pub struct MessageSender(pub UnboundedSender<(UserId, ServerMessage)>);

#[derive(Resource, Default)]
pub struct KeypressBuffer(pub VecDeque<(UserId, Key)>);

#[derive(Resource, Default)]
pub struct DisconnectBuffer(pub VecDeque<UserId>);

#[derive(Resource, Default)]
pub struct ConnectBuffer(pub VecDeque<UserId>);

#[derive(Resource, Default)]
pub struct MouseHoverBuffer(pub VecDeque<(UserId, Position)>);

#[derive(Resource, Default)]
pub struct MouseClickBuffer(pub VecDeque<(UserId, Position)>);
