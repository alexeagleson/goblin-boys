use bevy::prelude::Resource;
use core_api::UserId;

#[derive(Debug, Resource)]
pub struct UserIdResource {
    pub user_id: UserId,
}
