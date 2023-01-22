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

pub fn example_map_1_legend(character: char) -> (String, SpriteTexture, Option<DialogueMap>) {
    match character {
        '#' => (String::from("Wall"), SpriteTexture::WallBrick, None),
        's' => (String::from("Sewer"), SpriteTexture::ObjectSewerGrate, None),
        '.' => (String::from("???"), SpriteTexture::Empty, None),
        'w' => (String::from("Water"), SpriteTexture::ObjectWater, None),
        'h' => (String::from("Grass"), SpriteTexture::FloorGrass, None),
        _ => (String::from("???"), SpriteTexture::Unrecognized, None),
    }
}

// EXAMPLE_MAP_2

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

pub fn example_map_2_legend(character: char) -> (String, SpriteTexture, Option<DialogueMap>) {
    match character {
        '#' => (String::from("Wall"), SpriteTexture::WallBrick, None),
        'r' => (
            String::from("Soda Can"),
            SpriteTexture::ObjectRedSoda,
            Some(
                ron::from_str::<DialogueContents>(dialogue_contents_str)
                    .unwrap()
                    .sewer_kid,
            ),
        ),
        '.' => (String::from("???"), SpriteTexture::Empty, None),
        _ => (String::from("???"), SpriteTexture::Unrecognized, None),
    }
}
