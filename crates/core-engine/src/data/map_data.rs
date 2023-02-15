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

pub const DEFAULT_FLOOR_MAP_1: SpriteTexture = SpriteTexture::FloorGrass;

pub const OLD_EXAMPLE_MAP_1: &str = r#"
#################################################BBBBBBBBBBBBBBBB
#q.#######........#..........144.............CCABBBBBBBBBBBBBBBBB
#.......##...ww...#..........3...............CCABBBBBBBBBBBBBBBBB
##...........www..#.......@..244......o......CCABBBBBBBBBBBBBBBBB
##......##........#..........y.z.............CCABBBBBBBBBBBBBBBBB
##......##........#..hh...............n......CCABBBBBBBBBBBBBBBBB
##......##........#..h.......................CCABBBBBBBBBBBBBBBBB
##.............s..#............x......m......CCABBBBBBBBBBBBBBBBB
##########...................................CCABBBBBBBBBBBBBBBBB
#########################.............l......CCABBBBBBBBBBBBBBBBB
#............................................CCABBBBBBBBBBBBBBBBB
#.................a.b.c.d.e.f.g.h.i.j.k......CCABBBBBBBBBBBBBBBBB
#.........p...r.s.t.u.v......................CCABBBBBBBBBBBBBBBBB
#............................................CCABBBBBBBBBBBBBBBBB
############################################.CCABBBBBBBBBBBBBBBBB
#################################################BBBBBBBBBBBBBBBB
"#;

pub const EXAMPLE_MAP_1: &str = r#"
#################################################################
#qy########.....................................................#
#........##.....................................................#
#........##.....................................................#
#........##.........144...1444444444............................#
#.........#.........3.....3.....................................#
#...................3.....3...................CCCCCCCCCCCCCCCCCC#
#............................................CCABBBBBBBBBBBBBBBBB
#...................3........................CCABBBBBBBBBBBBBBBBB
#...................3........................CCABBBBBBBBBBBBBBBBB
#55555555...........244444444444.............CCABBBBBBBBBBBBBBBBB
#55555555....................................CCABBBBBBBBBBBBBBBBB
#55555555555.................................CCABBBBBBBBBBBBBBBBB
#55555555555.................................CCABBBBBBBBBBBBBBBBB
#55555555555.................................CCABBBBBBBBBBBBBBBBB
#################################################################
"#;

pub fn example_map_1_legend(character: char) -> (SpriteTexture, Option<DialogueMap>) {
    match character {
        '#' => (SpriteTexture::WallStone, None),
        's' => (SpriteTexture::ObjectSewerGrate, None),
        '.' => (SpriteTexture::Empty, None),
        'w' => (SpriteTexture::ObjectWater, None),
        'a' => (SpriteTexture::NpcFatherNeilFrames6, None),
        'b' => (
            SpriteTexture::NpcFootballFrames4,
            Some(
                ron::from_str::<DialogueContents>(dialogue_contents_str)
                    .unwrap()
                    .sewer_kid,
            ),
        ),
        'c' => (SpriteTexture::NpcGoon1Frames4, None),
        'd' => (SpriteTexture::NpcGoon2Frames4, None),
        'e' => (SpriteTexture::NpcGoon3Frames4, None),
        'f' => (SpriteTexture::NpcGoon4Frames4, None),
        'g' => (
            SpriteTexture::NpcGraceJonesFrames6,
            Some(
                ron::from_str::<DialogueContents>(dialogue_contents_str)
                    .unwrap()
                    .grace_jones,
            ),
        ),
        'h' => (SpriteTexture::NpcKingRatFrames4, None),
        'i' => (SpriteTexture::NpcMallChick1Frames6, None),
        'j' => (SpriteTexture::NpcMallChick2Frames6, None),
        'k' => (SpriteTexture::NpcPersonFrames2, None),
        'l' => (
            SpriteTexture::NpcRatFrames4,
            Some(
                ron::from_str::<DialogueContents>(dialogue_contents_str)
                    .unwrap()
                    .voidcat,
            ),
        ),
        'm' => (SpriteTexture::PcSewerKidFrames6, None),
        'n' => (
            SpriteTexture::NpcSmallRatFrames6,
            Some(
                ron::from_str::<DialogueContents>(dialogue_contents_str)
                    .unwrap()
                    .voidcat,
            ),
        ),
        'o' => (SpriteTexture::ObjectLadderDown, None),
        'p' => (SpriteTexture::ObjectLadderUp, None),
        'q' => (SpriteTexture::ObjectWarpTeeveeFrames3, None),
        'r' => (SpriteTexture::ObjectWindow, None),
        'x' => (SpriteTexture::PcAntBoiFrames4, None),
        't' => (SpriteTexture::PcBoneyBoiFrames4, None),
        'u' => (SpriteTexture::PcGhostBoyFrames8, None),
        'v' => (SpriteTexture::FloorSlime, None),
        'y' => (
            SpriteTexture::NpcRealEstateDickFrames21,
            Some(
                ron::from_str::<DialogueContents>(dialogue_contents_str)
                    .unwrap()
                    .real_estate_dick,
            ),
        ),
        'z' => (SpriteTexture::ObjectNewspaper, None),
        '@' => (SpriteTexture::ObjectRedSoda, None),
        'A' => (SpriteTexture::ObjectShoreFrames4, None),
        'B' => (SpriteTexture::ObjectWaterFrames4, None),
        'C' => (SpriteTexture::ObjectSand, None),
        '1' => (SpriteTexture::WallFenceCornerIn, None),
        '2' => (SpriteTexture::WallFenceCornerOut, None),
        '3' => (SpriteTexture::WallFenceVertical, None),
        '4' => (SpriteTexture::WallFenceHorizontal, None),
        '5' => (SpriteTexture::ObjectWood, None),
        _ => panic!(
            "Encountered an unrecognized character on map 1: {}",
            character
        ),
    }
}

// EXAMPLE_MAP_2

pub const DEFAULT_FLOOR_MAP_2: SpriteTexture = SpriteTexture::FloorConcrete;

pub const EXAMPLE_MAP_2: &str = r#"
#############################################
#t.#######..........................#.......#
#..................#.........g......#.......#
#...#######........#................#.......#
#...##....#......###........#########.......#
#.........#.....................#...........#
#....g....#.....................#...........#
#.........#.....................#......g....#
#.........#............g....................#
#...........................................#
#...................#............##.........#
#...................####.........##.........#
#..######..............#..........#.........#
#.......#####.................g.............#
#...........................................#
#############################################
"#;

pub fn example_map_2_legend(character: char) -> (SpriteTexture, Option<DialogueMap>) {
    match character {
        '#' => (SpriteTexture::WallBrick, None),
        't' => (SpriteTexture::ObjectWarpTeeveeFrames3, None),
        '.' => (SpriteTexture::Empty, None),
        'g' => (SpriteTexture::ObjectSewerGrate, None),
        _ => panic!(
            "Encountered an unrecognized character on map 2: {}",
            character
        ),
    }
}
