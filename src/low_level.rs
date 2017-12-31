use std;

use game;
use game_item;
use hero;
use map;
use monster;
use texts;

pub use cursive::Cursive;
//use cursive::theme;
use cursive::event::Key;
//use cursive::menu::MenuTree;
use cursive::traits::*;
use cursive::views::{TextView, Dialog, LinearLayout};

use decorators::decorators;
use loggers::{logger, log};

const CHARACTERS: [char; 31] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', ' ', 'ф', 'ы', 'в', 'а'];

pub enum Color {
    Green,
    Black,
    Brown,
    LightGray,
    LightGreen,
    LightRed,
    Yellow,
    LightMagenta,
    LightCyan,
    LightBlue
}

pub struct TTileRecord {
    pub C: char,
    pub Clr: Color
}

pub const TileRecords: [TTileRecord; (map::tileLast + 1) as usize] = [
    TTileRecord {C: '.', Clr: Color::Green},     // Grass
    TTileRecord {C: ':', Clr: Color::Black},     // Ground
    TTileRecord {C: '+', Clr: Color::Brown},     // StairsUp
    TTileRecord {C: '-', Clr: Color::Brown},     // StairsDown
    TTileRecord {C: ':', Clr: Color::Brown},     // Trap
    TTileRecord {C: '*', Clr: Color::Brown},     // Live
    TTileRecord {C: '^', Clr: Color::LightGray}, // Tree
    TTileRecord {C: 'X', Clr: Color::LightGreen} // Stone
];

pub const MonsterRecords: [TTileRecord; monster::MaxMonsterTypes as usize] = [
    TTileRecord {C: 'p', Clr: Color::LightRed},
    TTileRecord {C: '%', Clr: Color::Yellow},
    TTileRecord {C: '!', Clr: Color::LightGreen},
    TTileRecord {C: '#', Clr: Color::LightMagenta},
    TTileRecord {C: '&', Clr: Color::LightCyan},
    TTileRecord {C: 'j', Clr: Color::LightGray},
    TTileRecord {C: 'A', Clr: Color::LightBlue},
];

pub const ItemRecords: [TTileRecord; 3] = [
    TTileRecord {C: '>', Clr: Color::LightCyan},
    TTileRecord {C: '[', Clr: Color::LightGreen},
    TTileRecord {C: 'e', Clr: Color::Black}
];

pub fn InitApp(app: &mut Cursive) {
    create_init_screen(app);
}

fn disable_current_shortcuts(app: &mut Cursive) {
    for character in CHARACTERS.iter() {
        app.add_global_callback(*character, |a| {});
    }
    use map::Direction::*;
    app.add_global_callback( Key::Esc,   |a| {});
    app.add_global_callback( Key::Up,    |a| {});
    app.add_global_callback( Key::Down,  |a| {});
    app.add_global_callback( Key::Left,  |a| {});
    app.add_global_callback( Key::Right, |a| {});
}

fn enable_main_shortcuts(app: &mut Cursive) {
    disable_current_shortcuts(app);
    use map::Direction::*;
    app.add_global_callback( Key::Esc,   |a| {});
    app.add_global_callback( Key::Up,    |a| move_cursor(a, Up));
    app.add_global_callback( Key::Down,  |a| move_cursor(a, Down));
    app.add_global_callback( Key::Left,  |a| move_cursor(a, Left));
    app.add_global_callback( Key::Right, |a| move_cursor(a, Right));
    app.add_global_callback( 'e',        |a| ShowHeroSlots(a));
    app.add_global_callback( 'i',        |a| ShowHeroItems(a));
    app.add_global_callback( 'w',        |a| move_cursor(a, Up));
    app.add_global_callback( 's',        |a| move_cursor(a, Down));
    app.add_global_callback( 'a',        |a| move_cursor(a, Left));
    app.add_global_callback( 'd',        |a| move_cursor(a, Right));
    // Special for Russian keyboard layout.
    app.add_global_callback( 'ц',        |a| move_cursor(a, Up));
    app.add_global_callback( 'ы',        |a| move_cursor(a, Down));
    app.add_global_callback( 'ф',        |a| move_cursor(a, Left));
    app.add_global_callback( 'в',        |a| move_cursor(a, Right));
}

fn enable_init_shortcuts(app: &mut Cursive) {
    disable_current_shortcuts(app);
    app.add_global_callback(' ', |mut a| { game::GenerateAll();
                                           game::StartGame(&mut a); });

    app.add_global_callback( Key::Esc,   |a| { a.quit(); });
}

pub fn create_main_screen(app: &mut Cursive) {
    let mut text: String = "".to_owned();
    for y in 0..map::LOCAL_MAP_HEIGHT {
        for x in 0..map::LOCAL_MAP_WIDTH {
            text.push_str(" ");
        }
        text.push_str("\n");
    }
    let sep = TextView::empty()
        .with_id("sep")
        .fixed_size((1, map::LOCAL_MAP_HEIGHT));
    app.add_layer(LinearLayout::horizontal()
        .child(
            TextView::new(text)
                .with_id("area")
                .fixed_size((map::LOCAL_MAP_WIDTH, map::LOCAL_MAP_HEIGHT)))
        .child(LinearLayout::horizontal()
            .child(sep)
            .child(LinearLayout::vertical()
                .child(
                    LinearLayout::horizontal()
                        .child(
                            TextView::empty()
                                .center()
                                .with_id("minimap")
                                .fixed_size((12, 5)))
                        .child(
                            TextView::empty()
                                .center()
                                .with_id("compass")
                                .fixed_size((9, 5))))
                .child(
                    TextView::empty()
                        .with_id("sep1")
                        .fixed_size((9, 1)))
                .child(
                    TextView::empty()
                        .center()
                        .with_id("info")
                        .fixed_size((9, 5)))
                .child(
                    TextView::empty()
                        .with_id("sep2")
                        .fixed_size((9, 1)))
                .child(
                    TextView::empty()
                        .with_id("hero_info")
                        .fixed_size(
                            (24, map::LOCAL_MAP_HEIGHT - 5 - 1 - 5 - 1 - 9)))
                .child(Dialog::around(TextView::new(texts::HELP_EXIT_DIALOG))
                    .button("Help", |a| a.add_layer(
                        Dialog::info(texts::help())))
                    .button("Quit", |mut a| {
                        a.pop_layer();
                        create_init_screen(&mut a);
                    })
                    .with_id("exit")
                    .fixed_size((24, 9)))
            )
        )
    );
    ShowMinimap(app);
    app.find_id::<TextView>("compass")
        .unwrap()
        .set_content("    N    \n         \n W  @  O \n         \n    S    ");
    app.find_id::<TextView>("sep1")
        .unwrap()
        .set_content("________________________");
    app.find_id::<TextView>("sep2")
        .unwrap()
        .set_content("________________________");
    for _ in 0..map::LOCAL_MAP_HEIGHT {
        app.find_id::<TextView>("sep")
            .unwrap()
            .append_content("|\n");
    }
    enable_main_shortcuts(app);
}

fn create_init_screen(app: &mut Cursive) {
    let (width, height) = (70, 18);
    app.add_layer(Dialog::around(LinearLayout::vertical()
        .child(TextView::empty()
            .with_id("top")
            .fixed_size((width, height)))
        .child(TextView::new(texts::INIT_DIALOG)
            .center()
            .fixed_size((width, 4)))
        .child(TextView::empty()
            .with_id("bottom")
            .fixed_size((width, height)))
        .fixed_size((width, height*2 + 4)))
        .title("THE GAME")
        .button("Start", |mut a| { game::GenerateAll();
                                   game::StartGame(&mut a); })
        .button("Quit", |a| a.quit())
        .with_id("init"));

    let mut top = app.find_id::<TextView>("top").unwrap();
    let mut bottom = app.find_id::<TextView>("bottom").unwrap();
    for i in 0..width*height {
        top.append_content(["^", ":", "."][map::random(0, 3) as usize]);
    }
    for i in 0..width*height {
        bottom.append_content(["^", ":", "."][map::random(0, 3) as usize]);
    }
    enable_init_shortcuts(app);
}

fn create_slots_screen(app: &mut Cursive) {
    disable_current_shortcuts(app);
    let hero = get_ref_curhero!();
    let mut text = String::from("");
    text.push_str(texts::STR_HERO_SLOTITEMS);
    text.push_str("\n\n");
    for i in 0..hero::MaxSlots {
        let index = CHARACTERS[i];
        text.push_str(texts::SlotName[i]);
        match hero.Slots[i] {
            None => {
                text.push_str(&*(format!("[{}] {}", index.to_string(), texts::STR_EMPTY_ITEM)));
                text.push_str("\n");
            },
            Some(item) => {
                text.push_str(&*(format!("[{}] {}", index.to_string(), item.Name)));
                text.push_str("\n");
            }
        };
        app.add_global_callback(index, |a| {});
    }
    text.push_str("\n\n");
    text.push_str(texts::STR_HERO_SLOTINFO);
    app.add_layer(LinearLayout::vertical()
        .child(Dialog::around(TextView::new(text))
            .button("Back", |a| { a.pop_layer();
                                  enable_main_shortcuts(a); }))
    )
}

fn create_items_screen(app: &mut Cursive) {
    disable_current_shortcuts(app);
    let hero = get_ref_curhero!();
    let mut text = String::from("");
    text.push_str(texts::STR_HERO_ITEMS);
    text.push_str("\n\n");
    for i in 0..hero::MaxHeroItems {
        match hero.Items[i] {
            None => {
                text.push_str(texts::STR_EMPTY_ITEM);
                text.push_str("\n");
            },
            Some(item) => {
                text.push_str(item.Name);
                text.push_str("\n");
            }
        };
    }
    app.add_layer(LinearLayout::vertical()
        .child(Dialog::around(TextView::new(text))
            .button("Back", |a| { a.pop_layer();
                                  enable_main_shortcuts(a); }))
    )
}

pub fn VideoInitialize() {}

pub fn PrepareMap() {}

pub fn ShowCell(app: &mut Cursive, t: &map::TMapCell, x: usize, y: usize) {
    let c = TileRecords[t.Tile as usize].C;
    let mut text: String = app.find_id::<TextView>("area")
        .unwrap()
        .get_content()
        .to_owned();
    let cur_map = get_ref_curmap!();
    let index = (map::LOCAL_MAP_WIDTH+1)*(y-cur_map.LocalMapTop)
                +(x-cur_map.LocalMapLeft);
    text.remove(index);
    text.insert(index,
        if t.IsVisible {c} else {' '});
    app.find_id::<TextView>("area").unwrap().set_content(text);
}

pub fn ShowItem(app: &mut Cursive, itm: &game_item::TGameItem) {
    use game_item::TGameItemType::*;
    let mut text: String = app.find_id::<TextView>("area")
        .unwrap()
        .get_content()
        .to_owned();
    let cur_map = get_ref_curmap!();
    let index = (map::LOCAL_MAP_WIDTH+1)*(itm.y-cur_map.LocalMapTop)
                +(itm.x-cur_map.LocalMapLeft);
    text.remove(index);
    text.insert(index,
                ItemRecords[match itm.IType {
                        Some(ItemHandWeapon) => 0,
                        Some(ItemArmor) => 1,
                        None => 2
                    } as usize].C);
    app.find_id::<TextView>("area").unwrap().set_content(text);
}

pub fn ShowHero(app: &mut Cursive, HeroNum: usize) {
    let hero: &hero::THero = get_ref_curhero!(HeroNum);
    let mut text: String = app.find_id::<TextView>("area")
        .unwrap()
        .get_content()
        .to_owned();
    let index = unsafe { ((map::LOCAL_MAP_WIDTH + 1)*CURSOR.y
        + CURSOR.x) as usize };
    text.remove(index);
    text.insert(index, '@');
    app.find_id::<TextView>("area").unwrap().set_content(text);
}

pub fn ShowHeroInfo(app: &mut Cursive, HeroNum: usize) {
    let hero: &hero::THero = get_ref_curhero!(HeroNum);
    app.find_id::<TextView>("hero_info")
        .unwrap()
        .set_content(texts::STR_HERO_EXP.to_owned()
            + &hero.Exp.to_string()
            + "\n"
            + &texts::STR_HERO_HP.to_owned()
            + &hero.HP.to_string()
            + "/"
            + &hero.MaxHP.to_string()
            + "\n"
            + &texts::STR_HERO_XY.to_owned()
            + &hero.x.to_string()
            + ", "
            + &hero.y.to_string()
        );
}

fn ShowHeroItems(app: &mut Cursive) {
    create_items_screen(app);
    game::ShowGame(app);
}

fn ShowHeroSlots(app: &mut Cursive) {
    create_slots_screen(app);
    game::ShowGame(app);
} 

pub fn ShowMonster(app: &mut Cursive, m: &monster::TMonster) {
    let mut text: String = app.find_id::<TextView>("area")
        .unwrap()
        .get_content()
        .to_owned();
    let cur_map = get_ref_curmap!();
    let index = ((map::LOCAL_MAP_WIDTH + 1)*(m.y - cur_map.LocalMapTop)
                  + (m.x - cur_map.LocalMapLeft)) as usize;
    text.remove(index);
    text.insert(index, MonsterRecords[m.ID as usize].C);
    app.find_id::<TextView>("area").unwrap().set_content(text);
}

pub fn ShowInfo(app: &mut Cursive, text: String) {
    app.find_id::<TextView>("info")
        .unwrap()
        .set_content(text);
}

pub fn ShowMinimap(app: &mut Cursive) {
    let mut text: String = "_|_|_\n_|_|_\n | | ".to_owned();
    let hero: &hero::THero = get_ref_curhero!();
    let x = if hero.x < map::MAP_WIDTH/3 {0}
            else if hero.x < 2*map::MAP_WIDTH/3 {2}
            else {4};
    let y = if hero.y < map::MAP_HEIGHT/3 {0}
            else if hero.y < 2*map::MAP_HEIGHT/3 {1}
            else {2};
    text.remove(6*y + x);
    text.insert(6*y + x, '@');
    app.find_id::<TextView>("minimap")
        .unwrap()
        .set_content(text);
}

fn ShowCompassInfo(app: &mut Cursive, direction: map::Direction) {
    use map::Direction::*;
    let mut text = app.find_id::<TextView>("compass")
        .unwrap()
        .get_content()
        .to_owned();
    text.remove(22);
    text.insert(22, ' ');
    text.remove(26);
    text.insert(26, ' ');
    text.remove(14);
    text.insert(14, ' ');
    text.remove(34);
    text.insert(34, ' ');
    match direction {
        Left => {
            text.remove(22);
            text.insert(22, '<');
        },
        Right => {
            text.remove(26);
            text.insert(26, '>');
        },
        Up => {
            text.remove(14);
            text.insert(14, '^');
        },
        Down => {
            text.remove(34);
            text.insert(34, 'v');
        }
    }
    app.find_id::<TextView>("compass")
        .unwrap()
        .set_content(text);
}

//------------------------------------------------------------------------------

pub struct Cursor {
    pub x: usize,
    pub y: usize
}

pub static mut CURSOR: Cursor = Cursor { x: 0, y: 0 };

fn move_cursor(mut app: &mut Cursive, direction: map::Direction) {
    use map::Direction::*;
    unsafe {
        let (mut dx, mut dy) = (0i32, 0i32);
        match direction {
            Up => {
                dy = if CURSOR.y > 0 {-1} else {0};
            },
            Down => {
                dy = if CURSOR.y < map::LOCAL_MAP_HEIGHT - 1 {1} else {0};
            },
            Left => {
                dx = if CURSOR.x > 0 {-1} else {0};
            }
            Right => {
                dx = if CURSOR.x < map::LOCAL_MAP_WIDTH - 1 {1} else {0};
            }
        }

        let cur_map = get_ref_curmap_wo_unsafe!();
        let hero: &mut hero::THero = get_mut_ref_curhero_wo_unsafe!(hero::CUR_HERO);

        // If hero died to stop his moving at all.
        if hero.HP <= 0 {
            GameOverAnimation(app);
            return;
        }

        if !map::FreeTile(
            &cur_map.Cells[(hero.x as i32 + dx) as usize]
                [(hero.y as i32 + dy) as usize].Tile) {
            return;
        }

        let (prev_x, prev_y) = (CURSOR.x, CURSOR.y);
        if dx >= 0 {
            CURSOR.x += dx as usize;
            hero.x   += dx as usize;
        } else {
            CURSOR.x = (CURSOR.x as i32 + dx) as usize;
            hero.x   = (hero.x as i32 + dx) as usize;
        }
        if dy >= 0 {
            CURSOR.y += dy as usize;
            hero.y   += dy as usize;
        } else {
            CURSOR.y = (CURSOR.y as i32 + dy) as usize;
            hero.y   = (hero.y as i32 + dy) as usize;
        }
        //ShowInfo(&mut app, CURSOR.x.to_string() + "-" + &CURSOR.y.to_string());
        if prev_x != CURSOR.x || prev_y != CURSOR.y {
            let cur_cell = get_mut_ref_cell_wo_unsafe!(hero.x, hero.y);

            for trap in map::TrapTileSet.iter() {
                if &cur_cell.Tile == trap {
                    cur_cell.Tile = map::tileGrass;
                    if !hero::SkillTest(app, hero, hero::skillTrapSearch) {
                        let dam = map::random(0, hero.MaxHP as usize) + 1;//f32::round(hero.MaxHP * 1.1)) + 1;
                        ShowInfo(app, String::from( texts::STR_TRAP)
                                                  + "(-"
                                                  + &dam.to_string()
                                                  + " points)");
                        hero::DecHP(app, hero, dam as u32);
                    }
                }
            }
            for live in map::LiveTileSet.iter() {
                if &cur_cell.Tile == live {
                    ShowInfo(app, String::from(texts::STR_LIVE));
                    let inc = hero.MaxHP;
                    hero::IncHP(app, hero, inc);
                }
            }

            match dx {
                 1 => ShowCompassInfo(app, Right),
                -1 => ShowCompassInfo(app, Left),
                 0 | _ => ()
            }
            match dy {
                 1 => ShowCompassInfo(app, Down),
                -1 => ShowCompassInfo(app, Up),
                 0 | _ => ()
            }
ShowInfo(app, (hero.y - cur_map.LocalMapTop + map::SCROLL_DELTA).to_string());
            if hero.x - cur_map.LocalMapLeft < map::SCROLL_DELTA {
                map::ScrollMap(Left);
            } else if hero.x - cur_map.LocalMapLeft + map::SCROLL_DELTA >= map::LOCAL_MAP_WIDTH {
                   map::ScrollMap(Right);
            }
            if hero.y - cur_map.LocalMapTop < map::SCROLL_DELTA {
                   map::ScrollMap(Up);
            } else if hero.y - cur_map.LocalMapTop + map::SCROLL_DELTA >= map::LOCAL_MAP_HEIGHT {
                   map::ScrollMap(Down);
            }

            // Don't change an order of operations!
            hero::SetHeroVisible(hero::CUR_HERO);
            game::ShowGame(&mut app);
        };
    }
}

pub fn HeroDied(app: &mut Cursive) {
    ShowInfo(app, String::from(texts::STR_HERO_DIED));
}

fn GameOverAnimation(app: &mut Cursive) {
    use std::{thread, time};
    let interval = time::Duration::from_millis(10);
    let cur_map = get_mut_ref_curmap!();
    for y in cur_map.LocalMapTop..cur_map.LocalMapTop + map::LOCAL_MAP_HEIGHT {
        thread::sleep(interval);
        for x in cur_map.LocalMapLeft..cur_map.LocalMapLeft + map::LOCAL_MAP_WIDTH {
            get_mut_ref_cell!(x, y).IsVisible = false;
            ShowCell(app, get_ref_cell!(x, y), x, y);
        }
    }
}
