use low_level;
use map;
use tables;
use texts;
use std::cmp::{min, max};

const MaxChars: u32 = 4;
const chrSTR: u32 = 0;
const chrDEX: u32 = 1;
const chrCON: u32 = 2;
const chrIQ: u32 = 3;

pub const MaxSkills: u32 = 2;
pub const skillHandWeapon: usize = 0;
pub const skillTrapSearch: usize = 1; 

pub struct THero {
    pub Chars: [u32; MaxChars as usize],
    pub Skills: [u32; MaxSkills as usize],
    pub x: u32,
    pub y: u32,
    pub HP: u32,
    pub MaxHP: u32,
    pub Exp: u32,
    pub MaxExp: u32,
    pub Level: u32,
    pub VisLong: u32
}

const MaxHeroes: u32 = 1;

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
pub static mut CUR_HERO: u32 = 0;

fn InitHero(HeroNum: u32) {
    use low_level::CURSOR;
    let hero: &mut THero = get_mut_ref_curhero!(HeroNum);

    for i in 0..MaxChars {
        hero.Chars[i as usize] = 0;
    }
    for j in 0..MaxSkills as usize {
        hero.Skills[j] = tables::BaseSkill_Table[j];
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
              + map::random(
                    map::SCROLL_DELTA,
                    map::LOCAL_MAP_WIDTH - map::SCROLL_DELTA - 1);
        let y = cur_map.LocalMapTop
              + map::random(
                    map::SCROLL_DELTA,
                    map::LOCAL_MAP_HEIGHT - map::SCROLL_DELTA - 1);
        if map::FreeTile(cur_map.Cells[x as usize][y as usize].Tile) {
            break (x, y)
        };
    };
    hero.x = coords.0 as u32;
    hero.y = coords.1 as u32;
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

pub fn SetHeroVisible(HeroNum: u32) {
    let hero: &mut THero = get_mut_ref_curhero!(HeroNum);
    let cur_map: &mut map::TMap = get_mut_ref_curmap!();
    for x in max(0, hero.x - hero.VisLong)..min(map::MAP_WIDTH, hero.x + hero.VisLong + 1) {
        for y in max(0, hero.y - hero.VisLong)..min(map::MAP_HEIGHT, hero.y + hero.VisLong + 1) {
            get_mut_ref_cell!(x, y).IsVisible = true;
        }
    }
}

pub fn SkillTest(app: &mut low_level::Cursive, H: &mut THero, skl: usize ) -> bool {
    if map::random(0, 100) > H.Skills[skl] { return false }
    match skl {
        skillTrapSearch => {
            low_level::ShowInfo(app, texts::STR_TRAPOK.to_owned());
            let H_Level = H.Level;
            IncXP(app, H, 
                  max(1, H_Level + map::random(0, H_Level + 1)));
            SuccessSkillTest(app, H, skillTrapSearch)
        },
        _ => unreachable!()
    };
    true
}

fn SuccessSkillTest(app: &mut low_level::Cursive, H:  &mut THero, skl: usize) {
    match skl {
        skillTrapSearch => {
            let rnd = f32::round(20./100.*map::MaxDungeonLevel as f32) as u32;
            if map::random(0, 100) + 1 <= rnd {
                low_level::ShowInfo(app, texts::STR_TRAPSKILL_OK.into());
                H.Skills[skillTrapSearch] += 1; 
            }
        },
        _ => unreachable!()
    };
}

pub fn DecHP(mut app: &mut low_level::Cursive, hero: &mut THero, dam: u32) {
    hero.HP = if hero.HP >= dam {hero.HP - dam} else {0};
    if hero.HP == 0 {
        low_level::HeroDied(&mut app);
    }
}

pub fn IncHP(app: &mut low_level::Cursive, hero: &mut THero, inc: u32) {
    hero.HP = min(hero.HP + inc, hero.MaxHP);
}

pub fn IncXP(app: &mut low_level::Cursive, H: &mut THero, axp: u32) {
    H.Exp = min(H.Exp + axp, H.MaxExp);
    low_level::ShowInfo(app, texts::STR_ADD_EXP.to_owned()
                        + &axp.to_string() + " points");
}
