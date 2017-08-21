#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]

use low_level;
use hero;

//-------------------------------Constants------------------------------------//

pub const LOCAL_MAP_WIDTH: i32 = 8;
pub const LOCAL_MAP_HEIGHT: i32 = 8;

pub const MAP_WIDTH: i32 = 32 + LOCAL_MAP_WIDTH*2;
pub const MAP_HEIGHT: i32 = 32 + LOCAL_MAP_HEIGHT*2;

type Tile = i32;

const tileGrass: Tile = 0;
const tileGround: Tile = 1;
const tileStairsUp: Tile = 2;
const tileStairsDown: Tile = 3;

const tileFirstStopTile: Tile = 4;
const tileTree: Tile = tileFirstStopTile;
const tileStone: Tile = tileFirstStopTile + 1;
pub const tileLast: Tile = tileFirstStopTile + 1;

const MaxDungeonLevel: i32 = 7;

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

impl Copy for TMap { }

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

pub fn MapGeneration(MapLevel: i32) {
	unsafe {
		CUR_MAP = MapLevel;
		let cur_map = &mut GAME_MAP[CUR_MAP as usize];
		for x in 0..MAP_WIDTH {
			for y in 0..MAP_HEIGHT {
				let mut cell = &mut cur_map.Cells[x as usize][y as usize];
		  		if (x <= LOCAL_MAP_WIDTH)
		  			&& (x >= MAP_WIDTH-LOCAL_MAP_WIDTH)
		  			&& (y <= LOCAL_MAP_HEIGHT)
		  			&& (y >= MAP_HEIGHT-LOCAL_MAP_HEIGHT) {
		    		cell.Tile = tileStone;
		    	} else if random(100) < 35 {
		    		cell.Tile = tileTree;
		    	} else if random(2) == 1 {
		    		cell.Tile = tileGrass;
		    	} else {
		    		cell.Tile = tileGround;
		    	}
				cell.IsVisible = false;
			}
		}

		cur_map.LocalMapLeft = MAP_WIDTH/2;
		cur_map.LocalMapTop = MAP_HEIGHT/2;

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
	};
}

pub fn ShowMap(mut app: &mut low_level::Cursive) {
	let cur_map = unsafe { &GAME_MAP[CUR_MAP as usize] };
	low_level::PrepareMap();
	for x in cur_map.LocalMapLeft..cur_map.LocalMapLeft + LOCAL_MAP_WIDTH {
		for y in cur_map.LocalMapTop..cur_map.LocalMapTop + LOCAL_MAP_HEIGHT {
	    	low_level::ShowCell(app, &cur_map.Cells[x as usize][y as usize], x, y);
	    }
	}
}


//-------------------------------Functions------------------------------------//

pub fn random(end_interval: i32) -> i32 {
	use rand::{thread_rng, sample};
	let mut rng = thread_rng();
	sample(&mut rng, 0..end_interval, 1)[0]
}

pub fn FreeTile(tile: Tile) -> bool {
	tile < tileFirstStopTile
}

fn FreeMapPoint(cur_map: &TMap) -> (i32, i32) {
	loop {
    	let (x, y) = (
    		random(MAP_WIDTH - LOCAL_MAP_WIDTH*2) + LOCAL_MAP_WIDTH,
            random(MAP_HEIGHT - LOCAL_MAP_HEIGHT*2) + LOCAL_MAP_HEIGHT
        );
		if FreeTile(cur_map
			.Cells[x as usize][y as usize].Tile) {break (x, y)};
  	}
}
