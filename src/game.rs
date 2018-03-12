use cursive;
use game_item;
use hero;
use low_level;
use map;
use monster;

use decorators::decorators;
use loggers::{log, logger};

pub fn GenerateAll() {
    map::MapGeneration(0);
    monster::GenerateMonsters();
}

pub fn ShowItems(app: &mut cursive::Cursive) {
    for i in 0..game_item::MaxItems {
        unsafe {
            let item = game_item::ITEMS[i];
            if item.is_some() {
                if map::VisiblePoint(item.unwrap().x, item.unwrap().y) {
                    low_level::ShowItem(app, &item.unwrap());
                }
            }
        }
    }
}

pub fn ShowGame(app: &mut cursive::Cursive) {
    map::ShowMap(app);
    log!("ShowMap");
    ShowItems(app);
    log!("ShowItems");
    monster::ShowMonsters(app);
    log!("ShowMonsters");
    unsafe {
        low_level::ShowHero(app, hero::CUR_HERO);
        low_level::ShowHeroInfo(app, hero::CUR_HERO);
        low_level::ShowMinimap(app);
    }
}

pub fn StartGame(app: &mut cursive::Cursive) {
    app.pop_layer();
    log!("First");
    low_level::create_main_screen(app);
    log!("create_main_screen");
    hero::InitHeroes();
    log!("InitHeroes");
    ShowGame(app);
    log!("ShowGame");
}
