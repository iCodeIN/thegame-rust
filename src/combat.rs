use game;
use game_item;
use hero;
use low_level;
use map;
use monster;
use texts;

pub fn HeroAttack(app: &mut ::cursive::Cursive, h: &mut hero::THero, m: usize) {
    if !hero::SkillTest(app, h, hero::skillHandWeapon) {
        low_level::ShowInfo(app, texts::STR_BAD_ATTACK.to_string());
        return;
    }
    let w = hero::GetHeroWeapon(h);
    if w.is_none() {
        low_level::ShowInfo(app, texts::STR_NONE_WEAPONS.to_string());
        return;
    }
    let dam = WeaponDamage(h.Slots[w.unwrap()].unwrap());
    let skin = unsafe { game::RollDice(monster::MONSTERS[m].Dd1, monster::MONSTERS[m].Dd2) };
    if skin > dam {
        low_level::ShowInfo(app, texts::STR_BIG_SKIN.to_string());
        return;
    }

    unsafe { monster::MONSTERS[m].HP -= dam - skin };
    low_level::ShowInfo(app, format!("{}{}", texts::STR_ATTACK, dam - skin));

    unsafe {
        if monster::MONSTERS[m].HP <= 0 {
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
