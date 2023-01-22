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
###############################################
#q.#######........#...........................#
#.......##...ww...#...........................#
##...........www..#.......@...........o.......#
##......##........#..........y.z..............#
##......##........#..hh...............n.......#
##......##........#..h........................#
##.............s..#............x......m.......#
##########....................................#
#########################.............l.......#
#.............................................#
#.................a.b.c.d.e.f.g.h.i.j.k.......#
#.........p...r.s.t.u.v.......................#
#.............................................#
###############################################
"#;

pub fn example_map_1_legend(character: char) -> (SpriteTexture, Option<DialogueMap>) {
    match character {
        '#' => (SpriteTexture::WallBrick, None),
        's' => (SpriteTexture::ObjectSewerGrate, None),
        '.' => (SpriteTexture::Empty, None),
        'w' => (SpriteTexture::ObjectWater, None),
        'a' => (SpriteTexture::NpcFatherNeilFrames6, None),
        'b' => (SpriteTexture::NpcFootballFrames4, None),
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
        'l' => (SpriteTexture::NpcRatFrames4, None),
        'm' => (
            SpriteTexture::NpcSewerKidFrames6,
            Some(
                ron::from_str::<DialogueContents>(dialogue_contents_str)
                    .unwrap()
                    .sewer_kid,
            ),
        ),
        'n' => (SpriteTexture::NpcSmallRatFrames6, None),
        'o' => (SpriteTexture::ObjectLadderDown, None),
        'p' => (SpriteTexture::ObjectLadderUp, None),
        'q' => (SpriteTexture::ObjectWarpTeeveeFrames3, None),
        'r' => (SpriteTexture::ObjectWindow, None),
        'x' => (SpriteTexture::PcAntBoiFrames4, None),
        't' => (SpriteTexture::PcBoneyBoiFrames4, None),
        'u' => (SpriteTexture::PcGhostBoyFrames8, None),
        'v' => (SpriteTexture::FloorSlime, None),
        'y' => (SpriteTexture::NpcRealEstateDickFrames21, None),
        'z' => (SpriteTexture::ObjectNewspaper, None),
        '@' => (SpriteTexture::ObjectRedSoda, None),
        _ => panic!(
            "Encountered an unrecognized character on map 1: {}",
            character
        ),
    }
}

// EXAMPLE_MAP_2

pub const DEFAULT_FLOOR_MAP_2: SpriteTexture = SpriteTexture::FloorGrass;

pub const EXAMPLE_MAP_2: &str = r#"
###########################
#t.#######................#
#.......##................#
##........................#
##...#..##................#
##...#..##................#
##...#..##................#
##...#..##................#
##########................#
###########################
"#;

pub fn example_map_2_legend(character: char) -> (SpriteTexture, Option<DialogueMap>) {
    match character {
        '#' => (SpriteTexture::WallBrick, None),
        't' => (SpriteTexture::ObjectWarpTeeveeFrames3, None),
        '.' => (SpriteTexture::Empty, None),
        _ => panic!(
            "Encountered an unrecognized character on map 2: {}",
            character
        ),
    }
}
