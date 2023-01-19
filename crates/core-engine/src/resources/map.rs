use std::sync::atomic::{AtomicI32, Ordering};

use ae_position::{Dimensions2d, Position};
use bevy::prelude::Resource;
use rand::seq::SliceRandom;
use simple_astar::astar;
use tv_shadowcasting::get_visible_idxs;

use super::world::MapId;

pub const MAP_WIDTH: i32 = 7;
pub const MAP_HEIGHT: i32 = 7;

pub const PRIMARY_MAP_ID: i32 = 1;

pub static MAP_ID_COUNTER: AtomicI32 = AtomicI32::new(PRIMARY_MAP_ID);

type IndexGrid = Vec<u8>;

#[derive(Debug)]
pub struct VisibilityGrid {
    grid: IndexGrid,
    width: usize,
}

impl VisibilityGrid {
    pub fn new(map: &GameMap) -> Self {
        Self {
            grid: empty_index_grid(map.width(), map.height()),
            width: map.width() as usize,
        }
    }

    pub fn position_visible(&self, pos: &Position) -> bool {
        self.grid[pos.to_idx(self.width)] == 1
    }

    #[allow(dead_code)]
    pub fn pretty_print(&self) {
        pretty_print_idx_map(&self.grid)
    }
}

#[derive(Debug)]
struct LightBlockingGrid(IndexGrid);

impl LightBlockingGrid {
    pub fn new(width: i32, height: i32) -> Self {
        Self(perimeter_index_grid(width, height))
    }
}

#[derive(Debug)]
struct MovementBlockingGrid(IndexGrid);

impl MovementBlockingGrid {
    pub fn new(width: i32, height: i32) -> Self {
        Self(perimeter_index_grid(width, height))
    }
}

fn empty_index_grid(width: i32, height: i32) -> IndexGrid {
    vec![0; (width * height) as usize]
}

/// Create an [`IndexGrid`] from perimeter values set to 1
fn perimeter_index_grid(width: i32, height: i32) -> IndexGrid {
    let mut idxs: IndexGrid = empty_index_grid(width, height);

    for i in 0..width {
        let first_row = i as usize;
        let last_row = (height - 1) as usize * width as usize + i as usize;
        idxs[first_row] = 1;
        idxs[last_row] = 1;
    }

    for i in 0..height {
        let first_column = (i * width) as usize;
        let last_column = first_column + (width - 1) as usize;
        idxs[first_column] = 1;
        idxs[last_column] = 1;
    }

    idxs
}

/// Returns the [`Position`] of all tiles that are either blocking or not blocking
fn index_grid_to_positions(grid: &IndexGrid, grid_width: usize, blocking: bool) -> Vec<Position> {
    let check_val: u8 = if blocking { 1 } else { 0 };

    grid.iter()
        .enumerate()
        .filter_map(|(idx, val)| (*val == check_val).then(|| Position::from_idx(idx, grid_width)))
        .collect()
}

/// Can use to print either an array of light blocking entities or
/// a map of visible tiles, both use the same format
fn pretty_print_idx_map(idxs: &Vec<u8>) {
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
pub struct GameMap {
    map_id: MapId,
    dimensions: Dimensions2d,
    light_blocking_grid: LightBlockingGrid,
    movement_blocking_grid: MovementBlockingGrid,
}

impl GameMap {
    fn inside_map_bounds(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.x < self.dimensions.width && pos.y >= 0 && pos.y < self.dimensions.height
    }

    fn assert_in_bounds(&self, pos: &Position) {
        if !self.inside_map_bounds(pos) {
            panic!("Attempted to use a position outside map bounds {:?} but map ID {} bounds are x: {} y: {}", pos, self.id().0, self.width(), self.height());
        }
    }

    pub fn id(&self) -> MapId {
        self.map_id
    }

    pub fn new(width: i32, height: i32) -> Self {
        Self {
            map_id: MapId(MAP_ID_COUNTER.fetch_add(1, Ordering::Relaxed)),
            dimensions: Dimensions2d { width, height },
            light_blocking_grid: LightBlockingGrid::new(width, height),
            movement_blocking_grid: MovementBlockingGrid::new(width, height),
        }
    }

    pub fn width(&self) -> i32 {
        self.dimensions.width
    }

    pub fn height(&self) -> i32 {
        self.dimensions.height
    }

    pub fn reset_light_blocking_idxs(&mut self) {
        self.light_blocking_grid = LightBlockingGrid::new(self.width(), self.height());
    }

    pub fn reset_movement_blocking_idxs(&mut self) {
        self.movement_blocking_grid = MovementBlockingGrid::new(self.width(), self.height());
    }

    /// Mark a [`Position`] on the map as light being unable to pass through
    pub fn set_blocks_light(&mut self, pos: &Position) {
        self.assert_in_bounds(pos);
        let map_width = self.width() as usize;
        self.light_blocking_grid.0[pos.to_idx(map_width)] = 1;
    }

    /// Mark a [`Position`] on the map as being unable to move to
    pub fn set_blocks_movement(&mut self, pos: &Position) {
        self.assert_in_bounds(pos);
        let map_width = self.width() as usize;
        self.movement_blocking_grid.0[pos.to_idx(map_width)] = 1;
    }

    #[allow(dead_code)]
    pub fn light_blocked(&self, pos: &Position) -> bool {
        self.assert_in_bounds(pos);
        let map_width = self.width() as usize;
        self.light_blocking_grid.0[pos.to_idx(map_width)] == 1
    }

    pub fn movement_blocked(&self, pos: &Position) -> bool {
        self.assert_in_bounds(pos);
        let map_width = self.width() as usize;
        self.movement_blocking_grid.0[pos.to_idx(map_width)] == 1
    }

    pub fn generate_astar(&self, from: &Position, to: &Position) -> Vec<u32> {
        let width = self.width() as usize;
        let grid = self
            .movement_blocking_grid
            .0
            .iter()
            // This is what astar wants (0 and 1 values inverted from how movement grid stores them)
            // Can be optimized easily or stored in a different way just need to decide on a standard
            .map(|val| !*val as u32)
            .collect::<Vec<_>>();

        let path = astar(
            from.to_idx(width) as u32,
            to.to_idx(width) as u32,
            &grid,
            width as u32,
            true,
        );
        path
    }

    // Calculates all visible tiles on a grid of light blocking tiles from a given position
    pub fn visibility_grid_from_position(&self, pos: &Position, radius: u32) -> VisibilityGrid {
        self.assert_in_bounds(pos);
        let map_width = self.width() as usize;
        VisibilityGrid {
            grid: get_visible_idxs(
                pos.to_idx(map_width),
                &self.light_blocking_grid.0,
                self.width() as usize,
                radius,
            ),
            width: map_width,
        }
    }

    /// Returns the position of every time around the perimeter of the map
    pub fn perimeter_positions(&self) -> Vec<Position> {
        let perimeter = perimeter_index_grid(self.width(), self.height());
        let map_width = self.width() as usize;
        index_grid_to_positions(&perimeter, map_width, true)
    }

    /// Returns a random position on the map that doesn't block movement
    pub fn random_movement_unblocked_tile(&self) -> Position {
        let map_width = self.width() as usize;
        let unblocked_positions =
            index_grid_to_positions(&self.movement_blocking_grid.0, map_width, false);

        let position = unblocked_positions
            .choose(&mut rand::thread_rng())
            .expect("Attempted to find a random unblocked tile, but apparently there are none")
            .clone();

        position
    }
}

impl Default for GameMap {
    fn default() -> Self {
        Self::new(MAP_WIDTH, MAP_HEIGHT)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn perimeter_index_grid_works() {
        let row_len: usize = 10;
        let col_len: usize = 12;
        let len = row_len * col_len;
        let grid = perimeter_index_grid(row_len as i32, col_len as i32);

        assert_eq!(grid[0], 1);
        assert_eq!(grid[len - 1], 1);
        assert_eq!(grid[row_len], 1);
        assert_eq!(grid[row_len + 1], 0);
    }
}
