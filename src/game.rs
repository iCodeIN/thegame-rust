use monster;
use map;
use hero;
use low_level;

pub fn ShowGame(app: &mut low_level::Cursive) {
    map::ShowMap(app);
    monster::ShowMonsters(app);
    unsafe {
        low_level::ShowHero(app, hero::CUR_HERO);
        low_level::ShowHeroInfo(app, hero::CUR_HERO);
    }
}
