#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]

use low_level;
use std::cmp::{min, max};

//-------------------------------Constants------------------------------------//

pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

pub const LOCAL_MAP_WIDTH: i32 = 78;
pub const LOCAL_MAP_HEIGHT: i32 = 48;

const MAP_BORDER: i32 = 2;

pub const MAP_WIDTH: i32 = LOCAL_MAP_WIDTH*2 + MAP_BORDER*2;
pub const MAP_HEIGHT: i32 = LOCAL_MAP_HEIGHT*2 + MAP_BORDER*2;

type Tile = i32;

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

const MaxDungeonLevel: i32 = 7;

pub const SCROLL_DELTA: i32 = 3;

//-------------------------------Data types-----------------------------------//

#[derive(Debug)]
pub struct TMapCell {
   pub Tile: Tile,
   pub IsVisible: bool
}

impl Copy for TMapCell { }

impl Clone for TMapCell {
    fn clone(&self) -> TMapCell {
        *self
    }
}

type Cells = [[TMapCell; MAP_HEIGHT as usize]; MAP_WIDTH as usize];
pub struct TMap {
     pub Cells: Cells,
    pub LocalMapLeft: i32,
    pub LocalMapTop: i32
}

impl Copy for TMap {}

impl Clone for TMap {
    fn clone(&self) -> TMap {
        *self
    }
}

pub type TGameMap = [TMap; MaxDungeonLevel as usize];
pub static mut GAME_MAP: TGameMap = [
    TMap {
        Cells: [
            [TMapCell {
                Tile: tileGrass, IsVisible: false
            }; MAP_HEIGHT as usize]; MAP_WIDTH as usize],
        LocalMapLeft: 0,
        LocalMapTop: 0
    }; MaxDungeonLevel as usize];
pub static mut CUR_MAP: i32 = 0;

//-------------------------------Functions------------------------------------//

pub fn MapGeneration(MapLevel: i32) {
    unsafe { CUR_MAP = MapLevel; }
    let cur_map = get_mut_ref_curmap!();
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let mut cell = get_mut_ref_cell!(x, y);
            if (x < MAP_BORDER)
            || (x > MAP_WIDTH - MAP_BORDER - 1)
            || (y < MAP_BORDER)
            || (y > MAP_HEIGHT - MAP_BORDER - 1) {
                cell.Tile = tileStone;
            } else {
                if random(0, 100) < 32 {
                    cell.Tile = tileTree;
                } else if random(0, 2) == 1 {
                    cell.Tile = tileGrass;
                } else {
                    cell.Tile = tileGround;
                }
                if random(0, 100) == 0 {
                    if random(0, 2) == 0 {
                        cell.Tile = tileTrap;
                    } else {
                        cell.Tile = tileLive;
                    }
                }
            };
            cell.IsVisible = false;
        }
    }

    cur_map.LocalMapLeft = MAP_WIDTH/4;
    cur_map.LocalMapTop = MAP_HEIGHT/4;

    if MapLevel < MaxDungeonLevel {
        for i in 0..2 {
            let (x, y) = FreeMapPoint(&cur_map);
            cur_map.Cells[x as usize][y as usize].Tile = tileStairsDown;
        }
    };

    if MapLevel > 1 {
        let (x, y) = FreeMapPoint(&cur_map);
        cur_map.Cells[x as usize][y as usize].Tile = tileStairsUp;
    };
}

pub fn ShowMap(app: &mut low_level::Cursive) {
    let cur_map = get_ref_curmap!();
    low_level::PrepareMap();
    for x in cur_map.LocalMapLeft..cur_map.LocalMapLeft + LOCAL_MAP_WIDTH  {
        for y in cur_map.LocalMapTop..cur_map.LocalMapTop + LOCAL_MAP_HEIGHT  {
            low_level::ShowCell(app, get_ref_cell!(x, y), x, y);
        }
    }
}

pub fn ScrollMap(direction: Direction) {
    let cur_map = get_mut_ref_curmap!();
    let (mut new_local_map_left, mut new_local_map_top) = (
        cur_map.LocalMapLeft, cur_map.LocalMapTop);
    match direction {
        Direction::Left => {
            new_local_map_left = max(0, cur_map.LocalMapLeft - LOCAL_MAP_WIDTH/2);
        },
        Direction::Right => {
            new_local_map_left = min(MAP_WIDTH - LOCAL_MAP_WIDTH,
                                       cur_map.LocalMapLeft + LOCAL_MAP_WIDTH/2);
        },
        Direction::Up => {
            new_local_map_top = max(0, cur_map.LocalMapTop - LOCAL_MAP_HEIGHT/2);
        },
        Direction::Down => {
            new_local_map_top = min(MAP_HEIGHT - LOCAL_MAP_HEIGHT,
                                      cur_map.LocalMapTop + LOCAL_MAP_HEIGHT/2);
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

pub fn random(start: i32, end: i32) -> i32 {
    use rand::{thread_rng, sample};
       let mut rng = thread_rng();
    sample(&mut rng, start..end, 1)[0]
}

pub fn FreeTile(tile: Tile) -> bool {
    tile < tileFirstStopTile
}

pub fn FreeMapPoint(cur_map: &TMap) -> (i32, i32) {
    loop {
        let (x, y) = (
            random(MAP_BORDER, MAP_WIDTH - MAP_BORDER - 1),
            random(MAP_BORDER, MAP_HEIGHT - MAP_BORDER - 1)
        );
        if FreeTile(cur_map
            .Cells[x as usize][y as usize].Tile) {break (x, y)};
      }
}

pub fn VisiblePoint(x: i32, y: i32) -> bool {
    let cur_map = get_ref_curmap!();
    get_ref_cell!(x, y).IsVisible
        && x >= cur_map.LocalMapLeft
        && x < cur_map.LocalMapLeft + LOCAL_MAP_WIDTH
        && y >= cur_map.LocalMapTop
        && y < cur_map.LocalMapTop + LOCAL_MAP_HEIGHT
}
