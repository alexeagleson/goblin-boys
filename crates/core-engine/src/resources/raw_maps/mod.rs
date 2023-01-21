use ae_position::Dimensions2d;
use core_api::SpriteTexture;

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

pub fn example_map_1_legend(character: char) -> SpriteTexture {
    match character {
        '#' => SpriteTexture::WallBrick,
        's' => SpriteTexture::ObjectSewerGrate,
        '.' => SpriteTexture::None,
        'w' => SpriteTexture::EnvironmentWater,
        'h' => SpriteTexture::EnvironmentGrass,
        _ => SpriteTexture::Unrecognized,
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

pub fn example_map_2_legend(character: char) -> SpriteTexture {
    match character {
        '#' => SpriteTexture::WallBrick,
        'r' => SpriteTexture::ObjectRedSoda,
        '.' => SpriteTexture::None,
        _ => SpriteTexture::Unrecognized,
    }
}
