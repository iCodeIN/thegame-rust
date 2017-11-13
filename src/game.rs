use cursive;
use game_item;
use hero;
use low_level;
use map;
use monster;


use decorators::decorators;
use loggers::{logger, log}; 

pub fn GenerateAll() {
    map::MapGeneration(0);
    monster::GenerateMonsters();
}

pub fn ShowItems(app: &mut cursive::Cursive) {
    for i in 0..game_item::MaxItems {
        unsafe {
            if map::VisiblePoint(game_item::ITEMS[i].x, game_item::ITEMS[i].y) {
                low_level::ShowItem(app, &game_item::ITEMS[i]);
            }
        }
    }
}

pub fn ShowGame(app: &mut cursive::Cursive) {
    map::ShowMap(app);
    ShowItems(app);
    monster::ShowMonsters(app);
    unsafe {
        low_level::ShowHero(app, hero::CUR_HERO);
        low_level::ShowHeroInfo(app, hero::CUR_HERO);
        low_level::ShowMinimap(app);
    }
}

pub fn StartGame(app: &mut cursive::Cursive) {
    app.pop_layer();
    low_level::create_main_screen(app);
    hero::InitHeroes();
    ShowGame(app);
}
