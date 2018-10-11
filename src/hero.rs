use cursive;
use game;
use game_item;
use low_level;
use map;
use std::cmp::{max, min};
use tables;
use texts;

const MaxChars: usize = 6;
pub const CharsName: [&str; MaxChars] = [
    "Strength", "Dexterity", "Constitution", "IQ", "Wisdom", "Charisma",
];
pub const chrSTR: usize = 0; // strength
pub const chrDEX: usize = chrSTR + 1; // dexterity
const chrCON: usize = chrDEX + 1; // constitution
const chrIQ: usize = chrCON + 1; // wisdom
const chrWIS: usize = chrIQ + 1; // IQ
pub const chrCHA: usize = chrWIS + 1; // charisma

pub const MaxSkills: usize = 4;
pub const SkillsName: [&str; MaxSkills] = [
    "Hand weapon", "Trap search", "Defence", "Archery",
];
const skillMin: usize = 0;
pub const skillHandWeapon: usize = skillMin;
pub const skillTrapSearch: usize = skillHandWeapon + 1;
pub const skillDefence: usize = skillTrapSearch + 1;
pub const skillRangedWeapon: usize = skillDefence + 1;
pub const skillMax: usize = skillRangedWeapon;

pub const MaxHeroItems: usize = 12;

pub const MaxSlots: usize = 2;
pub const slotBody: usize = 0;
pub const slotHands: usize = slotBody + 1;

pub const raceHuman: usize = 0;
pub const raceElf: usize = raceHuman + 1;
pub const raceDwarf: usize = raceElf + 1;
pub const raceHobbit: usize = raceDwarf + 1;

pub const classWarrior: usize = 0;
pub const classArcher: usize = classWarrior + 1;
pub const classWizard: usize = classArcher + 1;

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
    pub Class: usize,
    pub Race: usize,
    pub Name: &'static str,
}

const MaxHeroes: usize = 1;

pub type Heroes<'tgi> = [THero<'tgi>; MaxHeroes];
pub static mut HEROES: Heroes = [THero {
    Chars: [0, 0, 0, 0, 0, 0],
    Skills: [0, 0, 0, 0],
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
    Class: 0,
    Race: 0,
    Name: "",
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

    for i in skillMin..=skillMax {
        hero.Skills[i] = tables::ClassSkill_Table[hero.Class][i].0
            + map::random(0, tables::ClassSkill_Table[hero.Class][i].1) + 1; 
    }

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
    let mut ss = H.Skills[skl];
    let default_skl = ss;
    if skl == skillDefence {
        let mut d = game::RollDice(3, 6);
        if d <= H.Chars[chrDEX] {
            ss += default_skl / 3;
        }
        d = game::RollDice(3, 6);
        if d <= H.Chars[chrCON] {
            ss -= default_skl / 5;
        }
        d = game::RollDice(3, 6);
        if d <= H.Chars[chrIQ] {
            ss += default_skl / 3;
        }
    }
    if map::random(0, 100) > ss {
        return false;
    }
    match skl {
        skillHandWeapon => SuccessSkillTest(app, H, skillHandWeapon),
        skillDefence => SuccessSkillTest(app, H, skillDefence),
        skillTrapSearch => {
            low_level::ShowInfo(app, texts::STR_TRAPOK.to_owned());
            let H_Level = H.Level;
            IncXP(app, H, max(1, H_Level + map::random(0, H_Level + 1)));
            SuccessSkillTest(app, H, skillTrapSearch)
        }
        skillRangedWeapon => {
            SuccessSkillTest(app, H, skillRangedWeapon)
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
        skillDefence => {
            if map::random(0, 100) == 0 {
                low_level::ShowInfo(app, texts::STR_DEFENCESKILL_OK.to_owned());
                H.Skills[skillDefence] += 1;
            }
        }
        skillTrapSearch => {
            let rnd = f32::round(6. / 100. * map::MaxDungeonLevel as f32) as usize;
            if map::random(0, 100) + 1 <= rnd {
                low_level::ShowInfo(app, texts::STR_TRAPSKILL_OK.to_owned());
                H.Skills[skillTrapSearch] += 1;
            }
        }
        skillRangedWeapon => {
            if map::random(0, 35) == 0 { // default value is 35
                low_level::ShowInfo(app, texts::STR_RANGEDWEAPONSKILL_OK.to_string());
                H.Skills[skillRangedWeapon] += 1;
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

pub fn IncHP(hero: &mut THero, inc: usize, sign: bool) {
    match sign {
        true => hero.HP = if hero.HP >= inc { hero.HP - inc } else { 0 },
        false => hero.HP = min(hero.HP + inc, hero.MaxHP),
    }
}

pub fn IncXP(app: &mut cursive::Cursive, H: &mut THero, axp: usize) {
    H.Exp = min(H.Exp + axp, H.MaxExp);
    low_level::ShowInfo(
        app,
        texts::STR_ADD_EXP.to_owned() + &axp.to_string() + " points",
    );
    if H.Exp == H.MaxExp {
        H.Exp = 0;
        if H.Level < tables::MaxPlayerLevel {
            low_level::ShowInfo(app, texts::STR_NEXTLEVEL.to_owned());
            H.Level += 1;
        }
        H.MaxExp = tables::ExpLevel_Table[H.Level];
        H.MaxHP = tables::HPLevel_Table[H.Level];
        H.HP = H.MaxHP;
    }
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
        slotHands => vec![ItemHandWeapon].contains(&Itm.IType)
            || vec![ItemRangedWeapon].contains(&Itm.IType),
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

pub fn GetFreeItem(H: &THero) -> Option<usize> {
    for i in 0..MaxHeroItems {
        if H.Items[i].is_none() {
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

pub fn GetHeroDefence(h: &THero) -> usize {
    let mut d = 0;
    if let Some(itm) = h.Slots[slotBody] {
        d += itm.Ints[game_item::intArmorDefence].unwrap();
    }
    d
}
