use game;
use game_item;
use hero;
use low_level;
use map;
use monster;
use texts;

pub fn HeroAttack(app: &mut ::cursive::Cursive, h: &mut hero::THero, m: usize) {
    let w = hero::GetHeroWeapon(h);
    if w.is_none() {
        low_level::ShowInfo(app, texts::STR_NONE_WEAPONS.to_string());
        return;
    }
    if !hero::SkillTest(app, h, hero::skillHandWeapon) {
        low_level::ShowInfo(app, texts::STR_BAD_ATTACK.to_string());
        return;
    }
    let dam = WeaponDamage(h.Slots[w.unwrap()].unwrap());
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
        low_level::ShowInfo(app, format!("{}, -{}", monster::MONSTERS[m].HP, dam - skin));
    };
    low_level::ShowInfo(app, format!("{}{}", texts::STR_ATTACK, dam - skin));

    unsafe {
        if monster::MONSTERS[m].HP == 0 {
            low_level::ShowInfo(
                app,
                format!("{}{}", monster::MONSTERS[m].Name, texts::STR_MON_KILL),
            );
        }
        hero::IncXP(app, h, monster::MONSTERS[m].XP);
    }
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

fn Distance(hero_xy: (usize, usize), monster_xy: (usize, usize)) -> usize {
    use std::cmp::{max, min};
    max(hero_xy.0, hero_xy.1) - min(hero_xy.0, hero_xy.1) + max(monster_xy.0, monster_xy.1)
        - min(monster_xy.0, monster_xy.1)
}

fn MonsterAttack(app: &mut ::cursive::Cursive, MonsterNum: usize, h: &mut hero::THero) {
    if hero::SkillTest(app, h, hero::skillDefence) {
        unsafe {
            low_level::ShowInfo(
                app,
                format!(
                    "{} {}",
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
                    "{} {}",
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
                "{} {}",
                monster::MONSTERS[MonsterNum].Name,
                texts::STR_MON_ATTACK
            ),
        );
    }
    hero::IncHP(h, dam - def, true);
}
