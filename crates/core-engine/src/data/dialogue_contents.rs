use bevy::prelude::Resource;
use core_api::DialogueMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Resource, Clone)]
pub struct DialogueContents {
    pub rat: DialogueMap,
    pub sewer_kid: DialogueMap,
    pub grace_jones: DialogueMap,
    pub voidcat: DialogueMap,
}
