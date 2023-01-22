use ae_position::Dimensions2d;
use core_api::{DialogueMap, SpriteTexture};

use crate::data::{dialogue_contents::DialogueContents, dialogue_contents_str};

// Utility

pub fn str_map_dimensions(str_map: &str) -> Dimensions2d {
    let lines = str_map
        .lines()
        .filter_map(|line| {
            let trimmed_line = line.trim();

            (!trimmed_line.is_empty()).then_some(trimmed_line)
        })
        .collect::<Vec<_>>();

    let height = lines.len();
    let width = lines[0].chars().count();

    Dimensions2d {
        width: width as i32,
        height: height as i32,
    }
}

// EXAMPLE_MAP_1

pub const DEFAULT_FLOOR_MAP_1: SpriteTexture = SpriteTexture::FloorConcrete;

pub const EXAMPLE_MAP_1: &str = r#"
##########################
#..#######........#......#
#.......##...ww...#......#
##...........www..#......#
##......##........#......#
##......##........#..hh..#
##......##........#..h...#
##.............s..#......#
##########...............#
##########################
"#;

pub fn example_map_1_legend(character: char) -> (SpriteTexture, Option<DialogueMap>) {
    match character {
        '#' => (SpriteTexture::WallBrick, None),
        's' => (SpriteTexture::ObjectSewerGrate, None),
        '.' => (SpriteTexture::Empty, None),
        'w' => (SpriteTexture::ObjectWater, None),
        'h' => (SpriteTexture::FloorGrass, None),
        _ => (SpriteTexture::Unrecognized, None),
    }
}

// EXAMPLE_MAP_2

pub const DEFAULT_FLOOR_MAP_2: SpriteTexture = SpriteTexture::FloorGrass;

pub const EXAMPLE_MAP_2: &str = r#"
##########
#..#######
#....r..##
##......##
##...#..##
##...#..##
##...#..##
##...#..##
##########
##########
"#;

pub fn example_map_2_legend(character: char) -> (SpriteTexture, Option<DialogueMap>) {
    match character {
        '#' => (SpriteTexture::WallBrick, None),
        'r' => (
            SpriteTexture::ObjectRedSoda,
            Some(
                ron::from_str::<DialogueContents>(dialogue_contents_str)
                    .unwrap()
                    .sewer_kid,
            ),
        ),
        '.' => (SpriteTexture::Empty, None),
        _ => (SpriteTexture::Unrecognized, None),
    }
}
