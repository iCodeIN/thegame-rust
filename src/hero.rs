use low_level;
use map;
use tables;
use std::cmp::{min, max};

const MaxChars: i32 = 4;
const chrSTR: i32 = 0;
const chrDEX: i32 = 1;
const chrCON: i32 = 2;
const chrIQ: i32 = 3;

const MaxSkills: i32 = 2;
const skillHandWeapon: i32 = 0;
const skillTrapSearch: i32 = 1; 

pub struct THero {
    pub Chars: [i32; MaxChars as usize],
    pub Skills: [i32; MaxSkills as usize],
    pub x: i32,
    pub y: i32,
    pub HP: i32,
    pub MaxHP: i32,
    pub Exp: i32,
    pub MaxExp: i32,
    pub Level: i32,
    pub VisLong: i32
}

const MaxHeroes: i32 = 1;

pub type Heroes = [THero; MaxHeroes as usize];
pub static mut HEROES: Heroes = [
    THero {
        Chars: [0, 0, 0, 0],
        Skills: [0, 0],
        x: 0,
        y: 0,
        HP: 0,
        MaxHP: 0,
        Exp: 0,
        MaxExp: 0,
        Level: 0,
        VisLong: 0}; MaxHeroes as usize];
pub static mut CUR_HERO: i32 = 0;

fn InitHero(HeroNum: i32) {
    use low_level::CURSOR;
    let hero: &mut THero = get_mut_ref_curhero!(HeroNum);

    for i in 0..MaxChars {
        hero.Chars[i as usize] = 0;
    }
    for j in 0..MaxSkills {
        hero.Skills[j as usize] = 0;
    }

    hero.Level = 0;
    hero.MaxHP = tables::HPLevel_Table[hero.Level as usize];
    hero.HP = hero.MaxHP;
    hero.Exp = 0;
    hero.MaxExp = tables::ExpLevel_Table[hero.Level as usize];
    hero.VisLong = 2;

    let cur_map = get_ref_curmap!();
    let coords = loop {
        let x = cur_map.LocalMapLeft
              + map::random(map::SCROLL_DELTA,
                            map::LOCAL_MAP_WIDTH - map::SCROLL_DELTA - 1);
        let y = cur_map.LocalMapTop
              + map::random(map::SCROLL_DELTA,
                            map::LOCAL_MAP_HEIGHT - map::SCROLL_DELTA - 1);
        if map::FreeTile(cur_map.Cells[x as usize][y as usize].Tile) {
            break (x, y)
        };
    };
    hero.x = coords.0;
    hero.y = coords.1;
    unsafe {
        CURSOR.x = hero.x - cur_map.LocalMapLeft;
        CURSOR.y = hero.y - cur_map.LocalMapTop;
    }
    SetHeroVisible(HeroNum);
}

pub fn InitHeroes() {
    for i in 0..MaxHeroes {
        InitHero(i);
    };
}

pub fn SetHeroVisible(HeroNum: i32) {
    let hero: &mut THero = get_mut_ref_curhero!(HeroNum);
    let cur_map: &mut map::TMap = get_mut_ref_curmap!();
    for x in max(0, hero.x - hero.VisLong)..min(map::MAP_WIDTH, hero.x + hero.VisLong + 1) {
        for y in max(0, hero.y - hero.VisLong)..min(map::MAP_HEIGHT, hero.y + hero.VisLong + 1) {
            get_mut_ref_cell!(x, y).IsVisible = true;
        }
    }
}

pub fn IncHP(mut app: &mut low_level::Cursive, hero: &mut THero, dam: i32) {
    hero.HP = max(0, hero.HP + dam);
    if hero.HP <= 0 {
        low_level::HeroDied(&mut app);
    } else if hero.HP > hero.MaxHP {
        hero.HP = hero.MaxHP;
    }
}
