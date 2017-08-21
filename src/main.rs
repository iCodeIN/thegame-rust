#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![feature(drop_types_in_const)]
#![feature(const_fn)]

extern crate rand;
extern crate cursive;
#[macro_use]
mod map;
mod low_level;
mod hero;
mod tables;

fn main() {
	low_level::log("start");
	let mut app: cursive::Cursive = cursive::Cursive::new();
	low_level::VideoInitialize();
	map::MapGeneration(0);
	low_level::InitApp(&mut app);
	hero::InitHeroes();
	map::ShowMap(&mut app);
	app.run();
	}
