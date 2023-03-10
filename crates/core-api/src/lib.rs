use std::collections::HashMap;

use ae_direction::BodyRelative;
use ae_position::Position;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UserId(pub i32);

#[typeshare]
#[derive(Clone, Copy, Serialize, Debug, PartialEq, Eq)]
pub struct EntityIndex {
    pub idx: u32,
}

#[typeshare]
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Information about a sprite to render
pub struct SpriteUpdate {
    pub entity: EntityIndex,
    pub pos: Position,
    pub sprite: SpriteTexture,
}

#[typeshare]
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Information about an entity to display to the user
pub struct EntityData {
    pub name: String,
    pub blocks_light: bool,
    pub visible_to_player: bool,
}

#[typeshare]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A single entry in the game log
pub struct LogMessage(pub String);

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// A sprite to render that represents a visible entity
pub enum SpriteTexture {
    Empty,
    ObjectBone,
    ObjectRedSoda,
    ObjectSewerGrate,
    ObjectWindow,
    ObjectLadderUp,
    ObjectLadderDown,
    ObjectNewspaper,
    ObjectWater,
    ObjectWaterFrames4,
    ObjectWarpTeeveeFrames3,
    ObjectShoreFrames4,
    ObjectSand,
    ObjectWood,
    FloorGrass,
    FloorConcrete,
    FloorSlime,
    NpcFatherNeilFrames6,
    NpcFootballFrames4,
    NpcGoon1Frames4,
    NpcGoon2Frames4,
    NpcGoon3Frames4,
    NpcGoon4Frames4,
    NpcGraceJonesFrames6,
    NpcKingRatFrames4,
    NpcMallChick1Frames6,
    NpcMallChick2Frames6,
    NpcPersonFrames2,
    NpcRatFrames4,
    NpcRealEstateDickFrames21,
    NpcSmallRatFrames6,
    NpcSlime,
    PcAntBoi,
    PcAntBoiFrames4,
    PcBoneyBoi,
    PcBoneyBoiFrames4,
    PcGhostBoyFrames8,
    PcSewerKidFrames6,
    PcKidZilla,
    WallBrick,
    WallStone,
    WallFenceCornerIn,
    WallFenceCornerOut,
    WallFenceHorizontal,
    WallFenceVertical,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// A sprite to render that represents a temporary animation to show
pub enum AnimationTexture {
    AttackBatFrames4,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// A sprite to render that represents a temporary animation to show
pub enum SpawnableEnemy {
    Slime,
    RatKing,
    Rat,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
// Intentionally does not serde renameAll due to some challenges with .ron files
pub struct DialogueContent {
    pub text: String,
    pub response_1_text: Option<String>,
    pub response_1_id: Option<i32>,
    pub response_2_text: Option<String>,
    pub response_2_id: Option<i32>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DialogueMap(pub HashMap<i32, DialogueContent>);

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
/// Tell client to play audio
pub enum Sound {
    Punch,
    EatBones,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
/// Tell client to play audio
pub struct DebugData {
    pub num_enemies: i32,
}

#[typeshare]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
/// An input interaction from the client
pub enum ClientMessage {
    TileHover(Position),
    TileClick(Position),
    #[serde(rename_all = "camelCase")]
    Initialize {
        name: String,
        sprite: SpriteTexture,
    },
    Keypress(BodyRelative),
    Disconnect,
    /// Clients should send every 30 seconds or so to
    /// keep from getting your socket closed when hosting on free services
    KeepAlive,
    Spawn(SpawnableEnemy),
}

#[typeshare]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
/// Communicates information about the active game to one client
pub enum ServerMessageSingleClient {
    TileHover(Option<EntityData>),
    // ExistingEntities(Vec<EntityRenderData>),
    EntityPositionChange(SpriteUpdate),
    AddSprite(SpriteUpdate),
    CentreCamera(Position),
    UpdateFullGameMap {
        camera: Position,
        entities: Vec<SpriteUpdate>,
    },
    RemoveSprite(EntityIndex),
    PlaySound(Sound),
    #[serde(rename_all = "camelCase")]
    ShowDialogue {
        entity_name: String,
        dialogue_map: DialogueMap,
    },
    #[serde(rename_all = "camelCase")]
    ShowAnimation {
        position: Position,
        animation: AnimationTexture,
        time: f32,
    },
    #[serde(rename_all = "camelCase")]
    ShowDamage {
        entity: EntityIndex,
        damage: i32,
        is_healing: bool,
        target_is_user: bool,
        target_is_me: bool,
        current_hp: i32,
        max_hp: i32,
    },
}

#[typeshare]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
/// Communicates information about the active game to one client
pub enum ServerMessageAllClients {
    // NewEntity(EntityRenderData),
    // NewEntities(Vec<EntityRenderData>),
    // RemovedEntity(EntityIndex),
    // EntityPositionChange(EntityPosition),
    TileClick(LogMessage),
    MoveCount(i32),
    Damage(LogMessage),
    Death(LogMessage),
    Debug(DebugData),
    Log(LogMessage),
}

#[derive(Debug)]
/// Communicates information from the game engine to the database
pub enum DatabaseRequest {
    Placeholder,
}

#[derive(Debug)]
/// Communicates information from the database back to the game engine
pub enum DatabaseResponse {
    MoveCount(i32),
}
