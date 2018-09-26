use cursive;
use game_item;
use low_level;
use map;
use std::cmp::{max, min};
use tables;
use texts;

const MaxChars: usize = 4;
const chrSTR: usize = 0;
const chrDEX: u32 = 1;
const chrCON: u32 = 2;
const chrIQ: u32 = 3;

pub const MaxSkills: usize = 2;
pub const skillHandWeapon: usize = 0;
pub const skillTrapSearch: usize = 1;

pub const MaxHeroItems: usize = 12;

pub const MaxSlots: usize = 2;
pub const slotBody: usize = 0;
pub const slotHands: usize = 1;

pub struct THero<'tgi> {
    pub Chars: [usize; MaxChars],
    pub Skills: [usize; MaxSkills],
    pub Items: [Option<game_item::TGameItem<'tgi>>; MaxHeroItems],
    pub Slots: [Option<game_item::TGameItem<'tgi>>; MaxSlots],
    pub x: usize,
    pub y: usize,
    pub HP: usize,
    pub MaxHP: usize,
    pub Exp: usize,
    pub MaxExp: usize,
    pub Level: usize,
    pub VisLong: usize,
    pub CurItem: usize,
}

const MaxHeroes: usize = 1;

pub type Heroes<'tgi> = [THero<'tgi>; MaxHeroes];
pub static mut HEROES: Heroes = [THero {
    Chars: [0, 0, 0, 0],
    Skills: [0, 0],
    Items: [None; MaxHeroItems],
    Slots: [None; MaxSlots],
    x: 0,
    y: 0,
    HP: 0,
    MaxHP: 0,
    Exp: 0,
    MaxExp: 0,
    Level: 0,
    VisLong: 0,
    CurItem: 0,
}; MaxHeroes];
pub static mut CUR_HERO: usize = 0;

fn InitHero(HeroNum: usize) {
    use low_level::CURSOR;
    let hero: &mut THero = get_mut_ref_curhero!(HeroNum);

    for i in 0..MaxChars {
        hero.Chars[i] = 0;
    }
    for j in 0..MaxSkills {
        hero.Skills[j] = tables::BaseSkill_Table[j];
    }
    for i in 0..MaxHeroItems {
        hero.Items[i] = None;
    }
    for i in 0..MaxSlots {
        hero.Slots[i] = None;
    }
    hero.Items[0] = Some(game_item::ItemTypes[0]);
    hero.Items[1] = Some(game_item::ItemTypes[3]);

    hero.Level = 0;
    hero.MaxHP = tables::HPLevel_Table[hero.Level];
    hero.HP = hero.MaxHP;
    hero.Exp = 0;
    hero.MaxExp = tables::ExpLevel_Table[hero.Level];
    hero.VisLong = 2;

    let cur_map = get_ref_curmap!();
    let coords = loop {
        let x = cur_map.LocalMapLeft + map::random(
            map::SCROLL_DELTA,
            map::LOCAL_MAP_WIDTH - map::SCROLL_DELTA - 1,
        );
        let y = cur_map.LocalMapTop + map::random(
            map::SCROLL_DELTA,
            map::LOCAL_MAP_HEIGHT - map::SCROLL_DELTA - 1,
        );
        if map::FreeTile(&cur_map.Cells[x][y].Tile) {
            break (x, y);
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
    }
}

pub fn SetHeroVisible(HeroNum: usize) {
    let hero: &mut THero = get_mut_ref_curhero!(HeroNum);
    //let cur_map: &mut map::TMap = get_mut_ref_curmap!();
    for x in max(0, hero.x - hero.VisLong)..min(map::MAP_WIDTH, hero.x + hero.VisLong + 1) {
        for y in max(0, hero.y - hero.VisLong)..min(map::MAP_HEIGHT, hero.y + hero.VisLong + 1) {
            get_mut_ref_cell!(x, y).IsVisible = true;
        }
    }
}

pub fn SkillTest(app: &mut cursive::Cursive, H: &mut THero, skl: usize) -> bool {
    if map::random(0, 100) > H.Skills[skl] as usize {
        return false;
    }
    match skl {
        skillHandWeapon => SuccessSkillTest(app, H, skillHandWeapon),
        skillTrapSearch => {
            low_level::ShowInfo(app, texts::STR_TRAPOK.to_owned());
            let H_Level = H.Level;
            IncXP(app, H, max(1, H_Level + map::random(0, H_Level + 1)));
            SuccessSkillTest(app, H, skillTrapSearch)
        }
        _ => unreachable!(),
    };
    true
}

fn SuccessSkillTest(app: &mut cursive::Cursive, H: &mut THero, skl: usize) {
    match skl {
        skillHandWeapon => {
            if map::random(0, 50) == 0 {
                low_level::ShowInfo(app, texts::STR_HANDWEAPONSKILL_OK.to_owned());
                H.Skills[skillHandWeapon] += 1;
            }
        }
        skillTrapSearch => {
            let rnd = f32::round(6. / 100. * map::MaxDungeonLevel as f32) as usize;
            if map::random(0, 100) + 1 <= rnd {
                low_level::ShowInfo(app, texts::STR_TRAPSKILL_OK.to_owned());
                H.Skills[skillTrapSearch] += 1;
            }
        }
        _ => unreachable!(),
    };
}

pub fn DecHP(mut app: &mut cursive::Cursive, hero: &mut THero, dam: usize) {
    hero.HP = if hero.HP >= dam { hero.HP - dam } else { 0 };
    if hero.HP == 0 {
        low_level::HeroDied(&mut app);
    }
}

pub fn IncHP(hero: &mut THero, inc: usize) {
    hero.HP = min(hero.HP + inc, hero.MaxHP);
}

pub fn IncXP(app: &mut cursive::Cursive, H: &mut THero, axp: usize) {
    H.Exp = min(H.Exp + axp, H.MaxExp);
    low_level::ShowInfo(
        app,
        texts::STR_ADD_EXP.to_owned() + &axp.to_string() + " points",
    );
}

pub fn GetFreeBag(H: &THero) -> Option<usize> {
    for i in 0..MaxHeroItems {
        if H.Items[i].is_none() {
            return Some(i);
        }
    }
    None
}

pub fn GoodSlot(Slot: usize, Itm: game_item::TGameItem) -> bool {
    use game_item::TGameItemType::*;
    match Slot {
        slotBody => vec![ItemArmor].contains(&Itm.IType),
        slotHands => vec![ItemHandWeapon].contains(&Itm.IType),
        _ => panic!("{:?}", "Error in `GoodSlot`"),
    }
}

pub fn GetFreeSlot(H: &THero, Itm: game_item::TGameItem) -> Option<usize> {
    for i in 0..MaxSlots {
        if H.Slots[i].is_none() && GoodSlot(i, Itm) {
            return Some(i);
        }
    }
    None
}

pub fn GetHeroWeapon(H: &THero) -> Option<usize> {
    match H.Slots[slotHands] {
        None => None,
        _ => Some(slotHands),
    }
}
