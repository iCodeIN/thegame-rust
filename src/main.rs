//! `The Game` is the yet another rogue-like game.
//!
//! Author: virtuos86
//!
//! I use TurboPascal's naming convention ("camel-case"). It's OK =)

#![feature(use_extern_macros)]
#![feature(proc_macro)]

#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

extern crate rand;
extern crate cursive;

extern crate decorators;
use decorators::decorators;

#[macro_use]
mod macros; 
mod game;
mod game_item;
mod hero;
mod loggers;
mod low_level;
mod map;
mod monster;
mod tables;
mod texts;

fn main() {
    let mut app: cursive::Cursive = cursive::Cursive::new();
    app.load_theme_file("../../src/theme.toml")
       .unwrap_or(());
    low_level::VideoInitialize();
    low_level::InitApp(&mut app);
    app.run();
}
