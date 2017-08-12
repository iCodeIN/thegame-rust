#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]

//-------------------------------Constants------------------------------------//

const LOCAL_MAP_WIDTH: i32 = 8;
const LOCAL_MAP_HEIGHT: i32 = 8;

const MAP_WIDTH: i32 = 2 + LOCAL_MAP_WIDTH*2;
const MAP_HEIGHT: i32 = 2 + LOCAL_MAP_HEIGHT*2;

type Tile = i32;

const tileGrass: Tile = 1;
const tileGround: Tile = 2;
const tileStairsUp: Tile = 3;
const tileStairsDown: Tile = 4;

const tileFirstStopTile: Tile = 5;
const tileTree: Tile = tileFirstStopTile;
const tileStone: Tile = tileFirstStopTile + 1;
const tileLast: Tile = tileFirstStopTile + 1;

const MaxDungeonLevel: i32 = 7;

//-------------------------------Data types-----------------------------------//

#[derive(Debug)]
pub struct TMapCell {
   pub Tile: Tile,
   pub IsVisible: bool
}

#[derive(Debug)]
pub struct TMap {
 	pub Cells: Vec<Vec<TMapCell>>,
	pub LocalMapLeft: i32,
	pub LocalMapTop: i32
}

type TGameMap = Vec<TMap>;
pub fn gen_map() -> TGameMap {
	let mut tmp_vec = vec!();
	for i in 0..MaxDungeonLevel {
		let mut col = vec!();
		for j in 0..MAP_WIDTH {
			let mut row = vec!();
			for k in 0..MAP_HEIGHT {
				row.push(TMapCell {
					Tile: tileGrass,
					IsVisible: false
				});
			}
			col.push(row);
		}
		tmp_vec.push(
			TMap {
				Cells: col,
				LocalMapLeft: 0,
				LocalMapTop: 0
			}
		);
	}
	tmp_vec
}

#[derive(Debug)]
pub struct Game {
	pub GameMap: TGameMap,
	pub CurMap: i32
}

impl Game {
	pub fn MapGeneration(&mut self, MapLevel: i32) {
		self.CurMap = MapLevel;
		let mut cur_map = &mut self.GameMap[self.CurMap as usize];
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
				println!("{:?}", &cell);
			}
		}
	
		cur_map.LocalMapLeft = MAP_WIDTH/2;
		cur_map.LocalMapTop = MAP_HEIGHT/2;
	}
}

//-------------------------------Functions------------------------------------//

fn random(end_interval: i32) -> i32 {
	use rand::{thread_rng, sample};
	let mut rng = thread_rng();
	sample(&mut rng, 0..end_interval, 1)[0]
}

fn FreeTile(tile: Tile) -> bool {
	tile < tileFirstStopTile
}

//----------------------------------------------------------------------------//

fn main() {
}