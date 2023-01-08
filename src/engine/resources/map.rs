use ae_position::{Dimensions2d, Position};
use bevy::prelude::Resource;
use tv_shadowcasting::get_visible_idxs;

pub const MAP_WIDTH: i32 = 12;
pub const MAP_HEIGHT: i32 = 12;
pub const MAX_MAP_INDEX: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;

const DEFAULT_LIGHT_BLOCKING_IDXS: [u8; MAX_MAP_INDEX] = [0; MAX_MAP_INDEX];

pub fn default_light_blocking_idxs() -> Vec<u8> {
    let mut idxs: Vec<u8> = DEFAULT_LIGHT_BLOCKING_IDXS.into();

    for i in 0..MAP_WIDTH {
        let first_row = i as usize;
        let last_row = (MAP_HEIGHT - 1) as usize * MAP_WIDTH as usize + i as usize;
        idxs[first_row] = 1;
        idxs[last_row] = 1;
    }

    for i in 0..MAP_HEIGHT {
        let first_column = (i * MAP_WIDTH) as usize;
        let last_column = first_column + (MAP_WIDTH - 1) as usize;
        idxs[first_column] = 1;
        idxs[last_column] = 1;
    }

    idxs
}

pub fn default_movement_blocking_idxs() -> fn() -> Vec<u8> {
    default_light_blocking_idxs
}

/// Can use to print either an array of light blocking entities or
/// a map of visible tiles, both use the same format
#[allow(dead_code)]
pub fn pretty_print_idx_map(idxs: &Vec<u8>) {
    println!("");
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            print!("{}", idxs[(MAP_WIDTH * y) as usize + x as usize])
        }
        println!("");
    }
    println!("");
}

#[derive(Debug, Resource)]
pub struct Map {
    dimensions: Dimensions2d,
    light_blocking_idxs: Vec<u8>,
    movement_blocking_idxs: Vec<u8>,
}

impl Map {
    fn inside_map_bounds(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.x < self.dimensions.width && pos.y >= 0 && pos.y < self.dimensions.height
    }

    pub fn width(&self) -> i32 {
        self.dimensions.width
    }

    pub fn height(&self) -> i32 {
        self.dimensions.height
    }

    pub fn reset_light_blocking_idxs(&mut self) {
        self.light_blocking_idxs = default_light_blocking_idxs();
    }

    pub fn reset_movement_blocking_idxs(&mut self) {
        self.movement_blocking_idxs = default_movement_blocking_idxs()();
    }

    pub fn set_blocks_light(&mut self, pos: &Position) {
        let map_width = self.width();
        self.light_blocking_idxs[pos.to_idx(map_width)] = 1;
    }

    pub fn set_blocks_movement(&mut self, pos: &Position) {
        let map_width = self.width();
        self.movement_blocking_idxs[pos.to_idx(map_width)] = 1;
    }

    // pub fn light_blocked(&self, pos: &Position) -> bool {
    //     if !self.inside_map_bounds(pos) {
    //         panic!(
    //             "Attempted to check light blocking outside map bounds {:?}",
    //             pos
    //         );
    //     }
    //     self.light_blocking_idxs[pos.to_idx(self.width())] == 1
    // }

    pub fn movement_blocked(&self, pos: &Position) -> bool {
        if !self.inside_map_bounds(pos) {
            panic!(
                "Attempted to check movement blocking outside map bounds {:?}",
                pos
            );
        }
        self.movement_blocking_idxs[pos.to_idx(self.width())] == 1
    }

    pub fn visible_idxs_from_position(&self, pos: &Position, radius: u32) -> Vec<u8> {
        let visible_idxs = get_visible_idxs(
            pos.to_idx(MAP_WIDTH),
            &self.light_blocking_idxs,
            MAP_WIDTH as usize,
            radius,
        );

        visible_idxs
    }

    // Returns the position of all tiles around the perimeter edge of the map
    pub fn perimeter_positions(&self) -> Vec<Position> {
        let mut positions: Vec<Position> =
            Vec::with_capacity((self.width() * 2 + self.height() * 2) as usize);

        for x in 0..self.width() {
            positions.push(Position { x, y: 0 });
            positions.push(Position {
                x,
                y: self.height() - 1,
            });
        }

        // Use of 1 is due to the first and last row already having the y positions set above
        for y in 1..self.height() - 1 {
            positions.push(Position { x: 0, y });
            positions.push(Position {
                x: self.width() - 1,
                y,
            });
        }

        positions
    }
}

impl Default for Map {
    fn default() -> Self {
        Self {
            dimensions: Dimensions2d {
                width: MAP_WIDTH,
                height: MAP_HEIGHT,
            },
            light_blocking_idxs: default_light_blocking_idxs(),
            movement_blocking_idxs: default_movement_blocking_idxs()(),
        }
    }
}
