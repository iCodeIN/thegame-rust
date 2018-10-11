use game;
use game_item;
use hero;
use low_level;
use map;
use monster;
use texts;

pub fn HeroShot(app: &mut ::cursive::Cursive, direction: map::Direction) {
    use map::Direction::*;
    let curhero = get_mut_ref_curhero!();
    if let Some(item) = curhero.Slots[hero::slotHands] {
        if item.IType != game_item::TGameItemType::ItemRangedWeapon {
            low_level::ShowInfo(app, texts::STR_NONE_WEAPONS.to_string());
            return;
        }
    }
    if curhero.Slots[hero::slotHands].unwrap().Ints[game_item::intRangedAmmo] == Some(0) {
        low_level::ShowInfo(app, texts::STR_NONE_AMMO.to_string());
        return;
    }

    let mut old_item = curhero.Slots[hero::slotHands].unwrap();
    let n = old_item.Ints[game_item::intRangedAmmo];
    old_item.Ints[game_item::intRangedAmmo] = Some(n.unwrap() - 1);
    curhero.Slots[hero::slotHands] = if n.unwrap() > 0 { Some(old_item) } else { None };

    if !hero::SkillTest(app, curhero, hero::skillRangedWeapon) {
        low_level::ShowInfo(app, texts::STR_BAD_RANGED_ATTACK.to_string());
        return;
    }

    let mut n = curhero.Slots[hero::slotHands].unwrap().Ints[game_item::intRangedRange].unwrap();
    let (mut x, mut y) = (curhero.x, curhero.y);
    while n >= 1 {
        match direction {
            Up => {
                y += 1;
            }
            Down => {
                y -= 1;
            }
            Left => {
                x -= 1;
            }
            Right => {
                x += 1;
            }
        }
        if !map::FreeTile(&get_ref_curmap!().Cells[x][y].Tile) {
            return;
        }

        let m = monster::IsMonsterOnTile(x, y);
        if m.is_some() {low_level::ShowInfo(app, m.unwrap().to_string());
            let dam = game::RollDice(
                curhero.Slots[hero::slotHands].unwrap().Ints[game_item::intRangedDices].unwrap(),
                curhero.Slots[hero::slotHands].unwrap().Ints[game_item::intRangedDiceNum].unwrap());
low_level::ShowInfo(app, "damage: ".to_string() + &dam.to_string());
            HeroAttackFin(app, curhero, m.unwrap(), dam);
            monster::MonstersStep(app);
            // Don't change an order of operations!
            hero::SetHeroVisible(unsafe { hero::CUR_HERO });
            game::ShowGame(app);
            return;
        }
        n -= 1;
    }
}

fn HeroAttackFin(app: &mut ::cursive::Cursive, h: &mut hero::THero, m: usize, dam: usize) {
//    let mut mnstr = unsafe { monster::MONSTERS[m] };
//    let skin = game::RollDice(mnstr.Dd1, mnstr.Dd2);
//    if skin >= dam {
//        low_level::ShowInfo(app, texts::STR_BIG_SKIN.to_string());
//        return;
//    }
//    mnstr.HP -= dam - skin;
//    low_level::ShowInfo(app, texts::STR_ATTACK.to_string() + &(dam-skin).to_string());
//
//    if mnstr.HP <= 0 {
//        low_level::ShowInfo(app, mnstr.Name.to_string() + texts::STR_MON_KILL);
//        hero::IncXP(app, h, mnstr.XP);
//    }
    let skin = unsafe { game::RollDice(monster::MONSTERS[m].Dd1, monster::MONSTERS[m].Dd2) };
    if skin > dam {
        low_level::ShowInfo(app, texts::STR_BIG_SKIN.to_string());
        return;
    }

    unsafe {
        if monster::MONSTERS[m].HP >= (dam - skin) {
            monster::MONSTERS[m].HP -= dam - skin;
        } else {
            monster::MONSTERS[m].HP = 0;
        }
    };
    low_level::ShowInfo(app, format!("{}{}", texts::STR_ATTACK, dam - skin));

    unsafe {
        if monster::MONSTERS[m].HP == 0 {
            low_level::ShowInfo(
                app,
                format!("{}{}", monster::MONSTERS[m].Name, texts::STR_MON_KILL),
            );
            // Don't change an order of operations!
            hero::SetHeroVisible(hero::CUR_HERO);
            game::ShowGame(app);
        }
        hero::IncXP(app, h, monster::MONSTERS[m].XP);
    }
}

pub fn HeroAttack(app: &mut ::cursive::Cursive, h: &mut hero::THero, m: usize) {
    let w = hero::GetHeroWeapon(h);
    if w.is_none() {
        low_level::ShowInfo(app, texts::STR_NONE_WEAPONS.to_string());
        return;
    }
    if let Some(item) = h.Slots[hero::slotHands] {
        if item.IType != game_item::TGameItemType::ItemRangedWeapon {
            low_level::ShowInfo(app, texts::STR_NONE_WEAPONS.to_string());
            return;
        }
    }
    if !hero::SkillTest(app, h, hero::skillHandWeapon) {
        low_level::ShowInfo(app, texts::STR_BAD_ATTACK.to_string());
        return;
    }
    let mut dam = WeaponDamage(h.Slots[w.unwrap()].unwrap());

    let d = game::RollDice(3, 6);
    if d <= h.Chars[hero::chrSTR] {
        dam += dam / 2; // If the hero is strong, then the damage from his impact will be increased by one and a half times.
    }

    HeroAttackFin(app, h, m, dam); 
}

fn WeaponDamage(itm: game_item::TGameItem) -> usize {
    if map::random(0, 100) + 1_usize > itm.Ints[game_item::intAttackHit].unwrap() as usize {
        return 0;
    }
    game::RollDice(
        itm.Ints[game_item::intAttack_d1].unwrap(),
        itm.Ints[game_item::intAttack_d2].unwrap(),
    )
}

pub fn MonstersAttack(app: &mut ::cursive::Cursive) {
    let curhero = get_mut_ref_curhero!();
    for i in 0..monster::MaxMonsters {
        if unsafe { monster::MONSTERS[i].HP > 0 && CanAttack(i, hero::CUR_HERO) } {
            MonsterAttack(app, i, curhero);
        }
    }
    low_level::ShowHeroInfo(app, unsafe { hero::CUR_HERO });
}

fn CanAttack(MonsterNum: usize, HeroNum: usize) -> bool {
    unsafe {
        Distance(
            (hero::HEROES[HeroNum].x, hero::HEROES[HeroNum].y),
            (
                monster::MONSTERS[MonsterNum].x,
                monster::MONSTERS[MonsterNum].y,
            ),
        ) == 1
    }
}

pub fn Distance(hero_xy: (usize, usize), monster_xy: (usize, usize)) -> usize {
    use std::cmp::{min, max};
    max(hero_xy.0, monster_xy.0) - min(hero_xy.0, monster_xy.0)
    + max(hero_xy.1, monster_xy.1) - min(hero_xy.1, monster_xy.1)
}

fn MonsterAttack(app: &mut ::cursive::Cursive, MonsterNum: usize, h: &mut hero::THero) {
    if hero::SkillTest(app, h, hero::skillDefence) {
        unsafe {
            low_level::ShowInfo(
                app,
                format!(
                    "{}{}",
                    monster::MONSTERS[MonsterNum].Name,
                    texts::STR_MON_STOP
                ),
            );
        }
        return;
    }
    let dam = unsafe {
        game::RollDice(
            monster::MONSTERS[MonsterNum].Ad1,
            monster::MONSTERS[MonsterNum].Ad2,
        )
    };
    let def = hero::GetHeroDefence(h);
    if dam <= def {
        unsafe {
            low_level::ShowInfo(
                app,
                format!(
                    "{}{}",
                    monster::MONSTERS[MonsterNum].Name,
                    texts::STR_MON_DEF
                ),
            );
        }
        return;
    }
    unsafe {
        low_level::ShowInfo(
            app,
            format!(
                "{}{} {} points!",
                monster::MONSTERS[MonsterNum].Name,
                texts::STR_MON_ATTACK,
                dam - def
            ),
        );
    }
    hero::DecHP(app, h, dam - def);
}
