use crate::components::MapPosition;

use super::resources::world::MapId;

pub struct ShouldUpdateMap(pub MapId);

pub struct ShouldSendFullMapUpdateToClient(pub MapId);

pub struct TryAttack(pub MapPosition);