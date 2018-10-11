//! `The Game` is the yet another rogue-like game.
//!
//! Author: virtuos86
//!
//! I use TurboPascal's naming convention ("camel-case"). It's OK =)

//#![feature(use_extern_macros)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

extern crate cursive;
extern crate rand;

//extern crate decorators;
//use decorators::decorators;

#[macro_use]
mod macros;
mod combat;
mod game;
mod game_item;
mod hero;
mod loggers;
mod low_level;
mod map;
mod monster;
mod tables;
mod texts;

const DEBUG: bool = false;

fn main() -> Result<(), cursive::theme::Error> {
    let mut app: cursive::Cursive = cursive::Cursive::default();
    app.load_theme_file("assets/theme.toml")?;
    low_level::VideoInitialize();
    low_level::InitApp(&mut app);
    app.run();
    Ok(())
}
