use bevy::prelude::Component;

use crate::api::UserId;

#[derive(Component)]
pub struct User(pub UserId);

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Item;
