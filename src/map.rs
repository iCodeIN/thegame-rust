//! This module describes a game map.

use cursive;
use game_item;
use low_level;
use std::cmp::{max, min};

//use decorators::decorators;
use loggers::{log, logger};

//-------------------------------Constants------------------------------------//

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

// `LOCAL_MAP_WIDTH` and `LOCAL_MAP_HEIGHT` must be divisible by 6 without residue!
// Otherwise happy debugging ;)
pub const LOCAL_MAP_WIDTH: usize = 78;
pub const LOCAL_MAP_HEIGHT: usize = 48;

const MAP_BORDER: usize = 2;

pub const MAP_WIDTH: usize = LOCAL_MAP_WIDTH * 2 + MAP_BORDER * 2;
pub const MAP_HEIGHT: usize = LOCAL_MAP_HEIGHT * 2 + MAP_BORDER * 2;

type Tile = u32;

pub const tileGrass: Tile = 0;
const tileGround: Tile = 1;
const tileStairsUp: Tile = 2;
const tileStairsDown: Tile = 3;
const tileTrap: Tile = 4;
const tileLive: Tile = 5;

const tileFirstStopTile: Tile = 6;
const tileTree: Tile = tileFirstStopTile;
const tileStone: Tile = tileFirstStopTile + 1;
pub const tileLast: Tile = tileFirstStopTile + 1;

pub const TrapTileSet: [Tile; 1usize] = [tileTrap; 1usize];
pub const LiveTileSet: [Tile; 1usize] = [tileLive; 1usize];

pub const MaxDungeonLevel: usize = 7;

pub const SCROLL_DELTA: usize = 2;

//-------------------------------Data types-----------------------------------//

#[derive(Debug)]
pub struct TMapCell {
    pub Tile: Tile,
    pub IsVisible: bool,
}

impl Copy for TMapCell {}

impl Clone for TMapCell {
    fn clone(&self) -> TMapCell {
        *self
    }
}

type Cells = [[TMapCell; MAP_HEIGHT as usize]; MAP_WIDTH as usize];
pub struct TMap {
    pub Cells: Cells,
    pub LocalMapLeft: usize,
    pub LocalMapTop: usize,
}

impl Copy for TMap {}

impl Clone for TMap {
    fn clone(&self) -> TMap {
        *self
    }
}

pub type TGameMap = [TMap; MaxDungeonLevel as usize];
pub static mut GAME_MAP: TGameMap = [TMap {
    Cells: [[TMapCell {
        Tile: tileGrass,
        IsVisible: false,
    }; MAP_HEIGHT as usize]; MAP_WIDTH as usize],
    LocalMapLeft: 0,
    LocalMapTop: 0,
}; MaxDungeonLevel as usize];
pub static mut CUR_MAP: usize = 0;

//-------------------------------Functions------------------------------------//

pub fn MapGeneration(MapLevel: usize) {
    unsafe {
        CUR_MAP = MapLevel;
    }
    let cur_map = get_mut_ref_curmap!();
    let mut n = 0usize;
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let mut cell = get_mut_ref_cell!(x, y);
            if (x < MAP_BORDER)
                || (x > MAP_WIDTH - MAP_BORDER - 1)
                || (y < MAP_BORDER)
                || (y > MAP_HEIGHT - MAP_BORDER - 1)
            {
                cell.Tile = tileStone;
            } else {
                if random(0, 100) < 32 {
                    cell.Tile = tileTree;
                } else if random(0, 2) == 1 {
                    cell.Tile = tileGrass;
                } else {
                    cell.Tile = tileGround;
                }
                if random(0, 50) == 0 {
                    if random(0, 2) == 0 {
                        cell.Tile = tileTrap;
                    } else {
                        cell.Tile = tileLive;
                    }
                }

                if FreeTile(&get_ref_cell!(x, y).Tile) {
                    if random(0, 100) == 0 {
                        if n < game_item::MaxItems {
                            unsafe {
                                let mut item = game_item::ItemTypes[random(0, game_item::MaxItemTypes)];
                                item.x = x;
                                item.y = y;
                                item.IsVisible = false;
                                game_item::ITEMS[n] = Some(item);
                            }
                        }
                        n += 1;
                    }
                }
            };
            // All cells is invisible by default.
            cell.IsVisible = false;
        }
    }

    cur_map.LocalMapLeft = MAP_WIDTH / 3;
    cur_map.LocalMapTop = MAP_HEIGHT / 3;

    if MapLevel < MaxDungeonLevel {
        for i in 0..2 {
            let (x, y) = FreeMapPoint(&cur_map);
            cur_map.Cells[x][y].Tile = tileStairsDown;
        }
    };

    if MapLevel > 1 {
        let (x, y) = FreeMapPoint(&cur_map);
        cur_map.Cells[x][y].Tile = tileStairsUp;
    };
}

pub fn ShowMap(app: &mut cursive::Cursive) {
    let cur_map = get_ref_curmap!();
    low_level::PrepareMap();
    for x in cur_map.LocalMapLeft..cur_map.LocalMapLeft + LOCAL_MAP_WIDTH {
        for y in cur_map.LocalMapTop..cur_map.LocalMapTop + LOCAL_MAP_HEIGHT {
            low_level::ShowCell(app, get_ref_cell!(x, y), x, y);
        }
    }
}

pub fn ScrollMap(direction: Direction) {
    let cur_map = get_mut_ref_curmap!();
    let (mut new_local_map_left, mut new_local_map_top) =
        (cur_map.LocalMapLeft, cur_map.LocalMapTop);
    match direction {
        Direction::Left => {
            new_local_map_left = max(
                0,
                cur_map.LocalMapLeft as i32 - (LOCAL_MAP_WIDTH / 2) as i32,
            ) as usize;
        }
        Direction::Right => {
            new_local_map_left = min(
                MAP_WIDTH - LOCAL_MAP_WIDTH,
                cur_map.LocalMapLeft + LOCAL_MAP_WIDTH / 2,
            );
        }
        Direction::Up => {
            new_local_map_top = max(
                0,
                cur_map.LocalMapTop as i32 - (LOCAL_MAP_HEIGHT / 2) as i32,
            ) as usize;
        }
        Direction::Down => {
            new_local_map_top = min(
                MAP_HEIGHT - LOCAL_MAP_HEIGHT,
                cur_map.LocalMapTop + LOCAL_MAP_HEIGHT / 2,
            );
        }
    }

    let dx = cur_map.LocalMapLeft - new_local_map_left;
    let dy = cur_map.LocalMapTop - new_local_map_top;
    cur_map.LocalMapLeft = new_local_map_left;
    cur_map.LocalMapTop = new_local_map_top;
    unsafe {
        low_level::CURSOR.x += dx;
        low_level::CURSOR.y += dy;
    }
}

/// Generates a number from a range from `start` to `end` not including `end`.
///
/// # Examples
///
/// ```
/// random(0, 5); // -> 0 or 1 or 2 or 3 or 4
/// ```
pub fn random(start: usize, end: usize) -> usize {
    use rand::seq::sample_iter;
    use rand::thread_rng;
    if end <= start {
        panic!("End={} should be more than Start={}", end, start)
    };
    let mut rng = thread_rng();
    sample_iter(&mut rng, start..end, 1).unwrap()[0]
}

/// Checks the possibility of passing through a map cell.
///
/// # Examples
///
/// ```
/// FreeTile(tileLive); // -> true
/// ```
pub fn FreeTile(tile: &Tile) -> bool {
    tile < &tileFirstStopTile
}

/// Looks up and returns coordinates of a first available free map cell.
///
/// # Examples
///
/// ```
/// let (x, y) = FreeMapPoint(get_ref_curmap!());
/// ```
pub fn FreeMapPoint(cur_map: &TMap) -> (usize, usize) {
    let curhero = get_ref_curhero!();
    loop {
        let (x, y) = (
            random(MAP_BORDER, MAP_WIDTH - MAP_BORDER - 1),
            random(MAP_BORDER, MAP_HEIGHT - MAP_BORDER - 1),
        );
        if FreeTile(&cur_map.Cells[x][y].Tile) {
            if x != curhero.x || y != curhero.y { break (x, y); } else { continue; }
        };
    }
}

/// Checks a location of a map cell with the `x` and `y` coordinates in the visible part of a map.
///
/// # Examples
///
/// ```
/// VisiblePoint(5, 7); // -> bool
/// ```
pub fn VisiblePoint(x: usize, y: usize) -> bool {
    let cur_map = get_ref_curmap!();
    get_ref_cell!(x, y).IsVisible
        && x >= cur_map.LocalMapLeft
        && x < cur_map.LocalMapLeft + LOCAL_MAP_WIDTH
        && y >= cur_map.LocalMapTop
        && y < cur_map.LocalMapTop + LOCAL_MAP_HEIGHT
}
