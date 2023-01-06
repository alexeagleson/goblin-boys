use serde::Serialize;
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Information about the size of a 2D space
pub struct Dimensions2d {
    pub width: i32,
    pub height: i32,
}