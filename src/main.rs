#![allow(unused_variables)]
//#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![feature(drop_types_in_const)]
#![feature(const_fn)]
#![feature(use_extern_macros)]

extern crate rand;
extern crate cursive;

#[macro_use]
mod macros;
mod game;
mod hero;
mod low_level;
mod map;
mod monster;
mod tables;
mod texts;

fn main() {
    let mut app: cursive::Cursive = cursive::Cursive::new();
    low_level::VideoInitialize();
    map::MapGeneration(0);
    monster::GenerateMonsters();
    low_level::InitApp(&mut app);
    app.run();
}
