#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

extern crate rand;
#[macro_use]
mod map;

fn main() {
	let mut game = map::Game {
		GameMap: map::gen_map(),
		CurMap: 0
	};
	game.MapGeneration(0);
	println!("{:?}", game);
}
