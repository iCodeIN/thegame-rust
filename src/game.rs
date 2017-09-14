use monster;
use map;
use hero;
use low_level;

pub fn GenerateAll() {
    map::MapGeneration(0);
    monster::GenerateMonsters();
}

pub fn ShowGame(app: &mut low_level::Cursive) {
    map::ShowMap(app);
    monster::ShowMonsters(app);
    unsafe {
        low_level::ShowHero(app, hero::CUR_HERO);
        low_level::ShowHeroInfo(app, hero::CUR_HERO);
        low_level::ShowMinimap(app);
    }
}

pub fn StartGame(app: &mut low_level::Cursive) {
    app.pop_layer();
    low_level::create_main_screen(app);
    hero::InitHeroes();
    ShowGame(app);
}
