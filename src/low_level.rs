use std;
use std::cmp::max;

use combat;
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
use cursive::views::{DummyView, Dialog, LinearLayout, SelectView, ScrollView, TextView, Checkbox};

//use decorators::decorators;
use loggers::{log, logger};

const CHARACTERS: [char; 48] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
    's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'W', 'A', 'S', 'D',
    ' ', 'й', 'ф', 'ы', 'в', 'а', 'у', 'ш',
];
#[derive(Debug)]
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
    LightBlue,
}

pub struct TTileRecord {
    pub C: char,
    pub Clr: Color,
}

pub const TileRecords: [TTileRecord; (map::tileLast + 1) as usize] = [
    TTileRecord {
        C: '.',
        Clr: Color::Green,
    }, // Grass
    TTileRecord {
        C: ':',
        Clr: Color::Black,
    }, // Ground
    TTileRecord {
        C: '+',
        Clr: Color::Brown,
    }, // StairsUp
    TTileRecord {
        C: '-',
        Clr: Color::Brown,
    }, // StairsDown
    TTileRecord {
        C: ':',
        Clr: Color::Brown,
    }, // Trap
    TTileRecord {
        C: '*',
        Clr: Color::Brown,
    }, // Live
    TTileRecord {
        C: '^',
        Clr: Color::LightGray,
    }, // Tree
    TTileRecord {
        C: 'X',
        Clr: Color::LightGreen,
    }, // Stone
];

pub const MonsterRecords: [TTileRecord; monster::MaxMonsterTypes as usize] = [
    TTileRecord {
        C: 'p',
        Clr: Color::LightRed,
    },
    TTileRecord {
        C: '%',
        Clr: Color::Yellow,
    },
    TTileRecord {
        C: '!',
        Clr: Color::LightGreen,
    },
    TTileRecord {
        C: '#',
        Clr: Color::LightMagenta,
    },
    TTileRecord {
        C: '&',
        Clr: Color::LightCyan,
    },
    TTileRecord {
        C: 'j',
        Clr: Color::LightGray,
    },
    TTileRecord {
        C: 'A',
        Clr: Color::LightBlue,
    },
];

pub const ItemRecords: [TTileRecord; 5] = [
    TTileRecord {
        C: '>',
        Clr: Color::LightCyan,
    },
    TTileRecord {
        C: '[',
        Clr: Color::LightGreen,
    },
    TTileRecord {
        C: '|',
        Clr: Color::LightCyan,
    },
    TTileRecord {
        C: '{',
        Clr: Color::LightGreen,
    },
    TTileRecord {
        C: 'e',
        Clr: Color::Black,
    },
];

pub fn InitApp(app: &mut Cursive) {
    create_init_screen(app);
}

fn disable_current_shortcuts(app: &mut Cursive) {
    for character in CHARACTERS.iter() {
        app.clear_global_callbacks(*character);
    }
    use map::Direction::*;
    app.clear_global_callbacks(Key::Backspace);
    app.clear_global_callbacks(Key::Ins);
    app.clear_global_callbacks(Key::Esc);
    app.clear_global_callbacks(Key::Up);
    app.clear_global_callbacks(Key::Down);
    app.clear_global_callbacks(Key::Left);
    app.clear_global_callbacks(Key::Right);
}

fn enable_main_shortcuts(app: &mut Cursive) {
    disable_current_shortcuts(app);
    use map::Direction::*;
    app.add_global_callback(Key::Backspace, |a| {
        ClearInfo(a);
    });
    app.add_global_callback(Key::Esc, |_| {});
    app.add_global_callback(Key::Ins, take_item);
    app.add_global_callback('W', |a| combat::HeroShot(a, Up));
    app.add_global_callback('S', |a| combat::HeroShot(a, Down));
    app.add_global_callback('A', |a| combat::HeroShot(a, Left));
    app.add_global_callback('D', |a| combat::HeroShot(a, Right));
    app.add_global_callback('e', |a| ShowHeroSlots(a));
    app.add_global_callback('i', |a| ShowHeroItems(a));
    app.add_global_callback('w', |a| move_cursor(a, Up));
    app.add_global_callback('s', |a| move_cursor(a, Down));
    app.add_global_callback('a', |a| move_cursor(a, Left));
    app.add_global_callback('d', |a| move_cursor(a, Right));
    // Special for Russian keyboard layout.
    app.add_global_callback('у', |a| ShowHeroSlots(a));
    app.add_global_callback('ш', |a| ShowHeroItems(a));
    app.add_global_callback('ц', |a| move_cursor(a, Up));
    app.add_global_callback('ы', |a| move_cursor(a, Down));
    app.add_global_callback('ф', |a| move_cursor(a, Left));
    app.add_global_callback('в', |a| move_cursor(a, Right));
}

fn enable_init_shortcuts(app: &mut Cursive) {
    disable_current_shortcuts(app);
    app.add_global_callback(' ', |mut a| {
        game::GenerateAll();
        game::StartGame(&mut a);
    });

    app.add_global_callback(Key::Esc, |a| {
        a.quit();
    });
}

pub fn create_main_screen(app: &mut Cursive) {
    let mut text: String = "".to_owned();
    for _ in 0..map::LOCAL_MAP_HEIGHT {
        for _ in 0..map::LOCAL_MAP_WIDTH {
            text.push_str(" ");
        }
        text.push_str("\n");
    }
    let sep = TextView::empty()
        .with_id("sep")
        .fixed_size((1, map::LOCAL_MAP_HEIGHT));
    app.add_layer(
        LinearLayout::horizontal()
            .child(
                TextView::new(text)
                    .with_id("area")
                    .fixed_size((map::LOCAL_MAP_WIDTH, map::LOCAL_MAP_HEIGHT)),
            ).child(
                LinearLayout::horizontal().child(sep).child(
                    LinearLayout::vertical()
                        .child(
                            LinearLayout::horizontal()
                                .child(
                                    TextView::empty()
                                        .center()
                                        .with_id("minimap")
                                        .fixed_size((12, 5)),
                                ).child(
                                    TextView::empty()
                                        .center()
                                        .with_id("compass")
                                        .fixed_size((9, 5)),
                                ),
                        ).child(TextView::empty().with_id("sep1").fixed_size((9, 1)))
                        .child(ScrollView::new(LinearLayout::vertical()
                            .child(TextView::empty().with_id("info")
                        )).with_id("history").fixed_size((24, 20))).child(TextView::empty().with_id("sep2").fixed_size((9, 1)))
                        .child(
                            TextView::empty()
                                .with_id("hero_info")
                                .fixed_size((24, map::LOCAL_MAP_HEIGHT - 5 - 1 - 20 - 1 - 9)),
                        ).child(
                            Dialog::around(TextView::new(texts::HELP_EXIT_DIALOG))
                                .button("Help", |a| a.add_layer(Dialog::info(texts::help())))
                                .button("Quit", |mut a| {
                                    a.pop_layer();
                                    create_init_screen(&mut a);
                                }).with_id("exit")
                                .fixed_size((24, 9)),
                        ),
                ),
            ),
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
        app.find_id::<TextView>("sep").unwrap().append("|\n");
    }
    disable_current_shortcuts(app);
    enable_main_shortcuts(app);
}

fn create_init_screen(app: &mut Cursive) {
    let (width, height) = (70, 18);
    app.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::empty().with_id("top").fixed_size((width, height)))
                .child(
                    TextView::new(texts::INIT_DIALOG)
                        .center()
                        .fixed_size((width, 4)),
                ).child(
                    TextView::empty()
                        .with_id("bottom")
                        .fixed_size((width, height)),
                ).fixed_size((width, height * 2 + 4)),
        ).title("THE GAME")
        .button("Start", |mut a| {
            game::GenerateAll();
            game::StartGame(&mut a);
        }).button("Quit", |a| a.quit())
        .with_id("init"),
    );

    let top = &mut *app.find_id::<TextView>("top").unwrap();
    let bottom = &mut *app.find_id::<TextView>("bottom").unwrap();
    for _ in 0..width * height {
        top.append(["^", ":", "."][map::random(0, 3) as usize]);
    }
    for _ in 0..width * height {
        bottom.append(["^", ":", "."][map::random(0, 3) as usize]);
    }
    disable_current_shortcuts(app);
    enable_init_shortcuts(app);
}

fn create_slots_screen(app: &mut Cursive) {
    disable_current_shortcuts(app);
    let hero = get_ref_curhero!();
    let mut list: SelectView<usize> = SelectView::new();
    for i in 0..hero::MaxSlots {
        let mut character: char;
        if i < 10 {
            character = i.to_string().chars().next().unwrap();
        } else if i > 9 && i < 38 {
            character = CHARACTERS[i];
        } else {
            panic!("Too many slots: {:?}!", i);
        }
        list.add_item(format!("[{}] {}", character, match hero.Slots[i] {
            None => texts::STR_EMPTY_ITEM.to_string(),
            Some(item) => game_item::GetItemName(item),
        }), i);
        app.add_global_callback(character, move |a| {
            move_slot_to_items(a, i);
        });
    }
    list.set_on_submit(|a, i| {
        move_slot_to_items(a, *i);
    });
    app.add_layer(Dialog::new().button("Back", |a| {
            a.pop_layer();
            disable_current_shortcuts(a);
            enable_main_shortcuts(a);
        })
        .title(texts::STR_HERO_SLOTITEMS)
        .content(LinearLayout::vertical()
            .child(TextView::new("\n"))
            .child(list.with_id("slots_list"))
            .child(TextView::new(format!("\n{}", texts::STR_HERO_SLOTINFO))))
    );
    app.add_global_callback('q', |a| {
        a.pop_layer();
        disable_current_shortcuts(a);
        enable_main_shortcuts(a);
    })
}

fn create_items_screen(app: &mut Cursive) {
    disable_current_shortcuts(app);
    let hero = get_ref_curhero!();
    let mut list: SelectView<usize> = SelectView::new();
    for i in 0..hero::MaxHeroItems {
        let mut character: char;
        if i < 10 {
            character = i.to_string().chars().next().unwrap();
        } else if i > 9 && i < 38 {
            character = CHARACTERS[i];
        } else {
            panic!("Too many items: {:?}!", i);
        }
        list.add_item(format!("[{}] {}", character, match hero.Items[i] {
            None => texts::STR_EMPTY_ITEM.to_string(),
            Some(item) => game_item::GetItemName(item),
        }), i);
        app.add_global_callback(character, move |a| {
            move_item_to_slots(a, i);
        });
    }
    list.set_on_submit(|a, i| {
        move_item_to_slots(a, *i);
    });
    app.add_layer(Dialog::new().button("Back", |a| {
            a.pop_layer();
            disable_current_shortcuts(a);
            enable_main_shortcuts(a);
        })
        .title(texts::STR_HERO_ITEMS)
        .content(LinearLayout::vertical()
            .child(TextView::new("\n"))
            .child(list.with_id("items_list"))
            .child(TextView::new(format!("\n{}", texts::STR_HERO_ITEMINFO))))
        //.with_id("d")
    );
    app.add_global_callback('q', |a| {
        a.pop_layer();
        disable_current_shortcuts(a);
        enable_main_shortcuts(a);
    });
    app.add_global_callback(Key::Backspace, throw_item);
}

fn move_slot_to_items(app: &mut Cursive, index: usize) {
    let hero = get_mut_ref_curhero!();
    let free_bag_index: Option<usize> = hero::GetFreeBag(hero);
    let slot: Option<game_item::TGameItem> = hero.Slots[index as usize];
    if slot.is_some() && free_bag_index.is_some() {
        hero.Items[free_bag_index.unwrap()] = slot;
        hero.Slots[index as usize] = None;
        ShowInfo(app, texts::STR_MOVE_SLOT_TO_ITEMS.to_owned() + slot.unwrap().Name);
    }
    app.pop_layer();
    create_slots_screen(app);
}

fn move_item_to_slots(app: &mut Cursive, index: usize) {
    let hero = get_mut_ref_curhero!();
    let item: Option<game_item::TGameItem> = hero.Items[index as usize];
    if item.is_some() {
        let free_slot_index: Option<usize> = hero::GetFreeSlot(hero, item.unwrap());
        if free_slot_index.is_some() {
            hero.Slots[free_slot_index.unwrap()] = item;
            hero.Items[index as usize] = None;
        ShowInfo(app, texts::STR_MOVE_ITEM_TO_SLOTS.to_owned() + item.unwrap().Name);
        }
    }
    app.pop_layer();
    create_items_screen(app);
}

fn throw_item(app: &mut Cursive) {
    use game_item::ITEMS;
    let selected_id = app.find_id::<SelectView<usize>>("items_list").unwrap().selected_id();
    let i = game_item::GetFreeItemNum();
    if let Some(i) = i {
        let mut curhero = get_mut_ref_curhero!();
        unsafe {
            let item = curhero.Items[selected_id.unwrap()];
            if item.is_none() { return; }
            let item = item.unwrap();

            let (x, y) = (curhero.x, curhero.y);
            for i in ITEMS.iter() {
                if let Some(itm) = i {
                    if itm.x == x && itm.y == y {
                        ShowInfo(app, "There is the busy tile! Cannot throw the item.".to_owned());
                        return;
                    }
                }
            }

            ITEMS[i] = Some(game_item::TGameItem {
                ID: item.ID,
                x: curhero.x,
                y: curhero.y,
                IType: item.IType,
                Name: item.Name,
                Ints: item.Ints,
                Reals: item.Reals,
                IsVisible: item.IsVisible,
            });
            ShowInfo(app, format!("You throw the item: {}", item.Name));
        }
        curhero.Items[selected_id.unwrap()] = None;
        app.pop_layer();
        create_items_screen(app);
    }
}

fn take_item(app: &mut Cursive) {
    use game_item::ITEMS;
    let curhero = get_mut_ref_curhero!();
    let index = hero::GetFreeItem(curhero);
    if index.is_none() { return; }
    unsafe {
        for (n, i) in ITEMS.iter().enumerate() {
            if let Some(mut itm) = i {
                if itm.x == curhero.x && itm.y == curhero.y {
                    match itm.IType {
                        // if the item is bow
                        game_item::TGameItemType::ItemRangedWeapon => {
                            let mut items = Vec::new();
                            for i in 0..hero::MaxHeroItems {
                                if curhero.Items[i].is_some() && curhero.Items[i].unwrap().IType == game_item::TGameItemType::ItemAmmo {
                                        items.push(curhero.Items[i].unwrap());
                                        curhero.Items[i] = None;
                                    }
                            }
                            let n = itm.Ints[game_item::intAmmo].unwrap() + items.iter().map(|ref i| i.Ints[game_item::intAmmo].unwrap()).sum::<usize>();
                            let index = hero::GetFreeItem(curhero).unwrap();
                            curhero.Items[index] = Some(game_item::TGameItem {
                                ID: 5,
                                x: 0,
                                y: 0,
                                IType: game_item::TGameItemType::ItemRangedWeapon,
                                Name: texts::STR_CROSS,
                                Ints: [
                                    Some(n),
                                    Some(5),
                                    Some(1),
                                    Some(4),
                                    None,
                                    None,
                                    None,
                                    None,
                                    None,
                                    None,
                                    None,
                                    None,
                                    None,
                                    None,
                                    None,
                                    None,
                                    None,
                                    None,
                                    None,
                                    None,
                                ],
                                Reals: [None; game_item::MaxRealInt],
                                IsVisible: false,
                            });
                            ShowInfo(app, "A bow taked!".to_string());
                        }
                        // if the item is ammo
                        game_item::TGameItemType::ItemAmmo => {
                            // if the hero has a bow in his hands
                            if let Some(mut hero_item) = curhero.Slots[hero::slotHands] {
                                // if this weapon is a bow
                                if hero_item.IType == game_item::TGameItemType::ItemRangedWeapon {
                                    if let Some(n) = hero_item.Ints[game_item::intRangedAmmo] { // if the bow has an arrows
                                        curhero.Slots[hero::slotHands] = Some(game_item::TGameItem {
                                            ID: 5,
                                            x: 0,
                                            y: 0,
                                            IType: game_item::TGameItemType::ItemRangedWeapon,
                                            Name: texts::STR_CROSS,
                                            Ints: [
                                                Some(n + itm.Ints[game_item::intAmmo].unwrap()),
                                                Some(5),
                                                Some(1),
                                                Some(4),
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                            ],
                                            Reals: [None; game_item::MaxRealInt],
                                            IsVisible: false,
                                        });
                                    } else { // if the bow hasn't any arrows
                                        curhero.Slots[hero::slotHands] = Some(game_item::TGameItem {
                                            ID: 5,
                                            x: 0,
                                            y: 0,
                                            IType: game_item::TGameItemType::ItemRangedWeapon,
                                            Name: texts::STR_CROSS,
                                            Ints: itm.Ints,
                                            Reals: [None; game_item::MaxRealInt],
                                            IsVisible: false,
                                        });
                                    }
                                    ShowInfo(app, itm.Ints[game_item::intAmmo].unwrap().to_string() + " arrows taked!");
                                }
                            } else if let Some(idx) = { // else if the hero hasn't a bow in his hands, but he has a bow in his `Items`
                                let mut idx = None;
                                for i in 0..hero::MaxHeroItems {
                                    if curhero.Items[i].is_some() && curhero.Items[i].unwrap().IType == game_item::TGameItemType::ItemRangedWeapon {
                                            idx = Some(i);
                                            break;
                                        }
                                }
                                idx
                            } {
                                // if the bow has an arrows
                                if let Some(n) = curhero.Items[idx].unwrap().Ints[game_item::intRangedAmmo] {
                                    curhero.Items[idx] = Some(game_item::TGameItem {
                                        ID: 5,
                                        x: 0,
                                        y: 0,
                                        IType: game_item::TGameItemType::ItemRangedWeapon,
                                        Name: texts::STR_CROSS,
                                        Ints: [
                                            Some(n + itm.Ints[game_item::intAmmo].unwrap()),
                                            Some(5),
                                            Some(1),
                                            Some(4),
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                        ],
                                        Reals: [None; game_item::MaxRealInt],
                                        IsVisible: false,
                                    });
                                } else { // else if the bow hasn't any arrows
                                    curhero.Items[idx] = Some(game_item::TGameItem {
                                        ID: 5,
                                        x: 0,
                                        y: 0,
                                        IType: game_item::TGameItemType::ItemRangedWeapon,
                                        Name: texts::STR_CROSS,
                                        Ints: itm.Ints,
                                        Reals: [None; game_item::MaxRealInt],
                                        IsVisible: false,
                                    });
                                }
                                ShowInfo(app, itm.Ints[game_item::intAmmo].unwrap().to_string() + " arrows taked!");
                            } else { // else if the hero hasn't any bow in `Items`
                                let mut flag = false;
                                for it in 0..hero::MaxHeroItems {
                                    if curhero.Items[it].is_some() && curhero.Items[it].unwrap().IType == game_item::TGameItemType::ItemAmmo {
                                            curhero.Items[it] = Some(game_item::TGameItem {
                                                ID: 4,
                                                x: 0,
                                                y: 0,
                                                IType: game_item::TGameItemType::ItemAmmo,
                                                Name: texts::STR_AMMO,
                                                Ints: [
                                                    Some(curhero.Items[it].unwrap().Ints[game_item::intAmmo].unwrap() + i.unwrap().Ints[game_item::intAmmo].unwrap()),
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                ],
                                                Reals: [None; game_item::MaxRealInt],
                                                IsVisible: false,
                                            });
                                            flag = true;
                                            break;
                                        }
                                }
                                if !flag { curhero.Items[index.unwrap()] = *i; }
                                ShowInfo(app, itm.Ints[game_item::intAmmo].unwrap().to_string() + " arrows taked!");
                            }
                        }
                        _ => { // if the item isn't ammo
                            curhero.Items[index.unwrap()] = *i;
                            ShowInfo(app, format!("You take the item: {}", itm.Name));
                        }
                    }
                    ITEMS[n] = None;
                    break;
                }
            }
        }
    }
}

pub fn VideoInitialize() {}

pub fn PrepareMap() {}

pub fn ShowCell(app: &mut Cursive, t: &map::TMapCell, x: usize, y: usize) {
    let c = TileRecords[t.Tile as usize].C;
    let mut text: String = app
        .find_id::<TextView>("area")
        .unwrap()
        .get_content()
        .source()
        .to_owned();
    let cur_map = get_ref_curmap!();
    let index = (map::LOCAL_MAP_WIDTH + 1) * (y - cur_map.LocalMapTop) + (x - cur_map.LocalMapLeft);
    text.remove(index);
    text.insert(index, if t.IsVisible { c } else { ' ' });
    app.find_id::<TextView>("area").unwrap().set_content(text);
}

pub fn ShowItem(app: &mut Cursive, itm: &game_item::TGameItem) {
    use game_item::TGameItemType::*;
    let mut text: String = app
        .find_id::<TextView>("area")
        .unwrap()
        .get_content()
        .source()
        .to_owned();
    let cur_map = get_ref_curmap!();
    if itm.y < cur_map.LocalMapTop || itm.x < cur_map.LocalMapLeft { return; }
    let index =
        (map::LOCAL_MAP_WIDTH + 1) * (itm.y - cur_map.LocalMapTop) + (itm.x - cur_map.LocalMapLeft);
    text.remove(index);
    text.insert(
        index,
        ItemRecords[match itm.IType {
                        ItemHandWeapon => 0,
                        ItemArmor => 1,
                        ItemAmmo => 2,
                        ItemRangedWeapon => 3,
                    } as usize]
            .C,
    );
    app.find_id::<TextView>("area").unwrap().set_content(text);
}

pub fn ShowHero(app: &mut Cursive, HeroNum: usize) {
    let mut text: String = app
        .find_id::<TextView>("area")
        .unwrap()
        .get_content()
        .source()
        .to_owned();
    let cur_map = get_ref_curmap!();
    let h = unsafe { &hero::HEROES[HeroNum] };
    if h.y < cur_map.LocalMapTop || h.x < cur_map.LocalMapLeft { return; }
    //let index = unsafe { ((map::LOCAL_MAP_WIDTH + 1) * CURSOR.y + CURSOR.x) as usize };
    let index =
        (map::LOCAL_MAP_WIDTH + 1) * (h.y - cur_map.LocalMapTop) + (h.x - cur_map.LocalMapLeft);
    text.remove(index);
    text.insert(index, '@');
    app.find_id::<TextView>("area").unwrap().set_content(text);
}

pub fn ShowHeroInfo(app: &mut Cursive, HeroNum: usize) {
    let hero: &hero::THero = get_ref_curhero!(HeroNum);
    let info = texts::STR_HERO_RACE.to_owned()
            + match hero.Race {
                hero::raceHuman => "Human",
                hero::raceElf => "Elf",
                hero::raceDwarf => "Dwarf",
                hero::raceHobbit => "Hobbit",
                _ => panic!("Error in `ShowHeroInfo`"),
            }
            + "\n"
            + &texts::STR_HERO_CLASS.to_owned()
            + match hero.Class {
                hero::classWarrior => "Warrior",
                hero::classArcher => "Archer",
                hero::classWizard => "Wizard",
                _ => panic!("Error in `ShowHeroInfo`"),
            }
            + "\n"
            + &texts::STR_HERO_LEVEL.to_owned()
            + &hero.Level.to_string()
            + "\n"
            + &texts::STR_HERO_EXP.to_owned()
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
            + &hero.y.to_string();
    app.find_id::<TextView>("hero_info").unwrap().set_content(
        info
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
    let mut text: String = app
        .find_id::<TextView>("area")
        .unwrap()
        .get_content()
        .source()
        .to_owned();
    let cur_map = get_ref_curmap!();
    let index = ((map::LOCAL_MAP_WIDTH + 1) * (m.y - cur_map.LocalMapTop)
        + (m.x - cur_map.LocalMapLeft)) as usize;
    text.remove(index);
    text.insert(index, MonsterRecords[m.ID as usize].C);
    app.find_id::<TextView>("area").unwrap().set_content(text);
}

pub fn ShowInfo(app: &mut Cursive, text: String) {
    let mut old_text = app
        .find_id::<TextView>("info")
        .unwrap()
        .get_content()
        .source()
        .to_string();
    if old_text.len() > 1024 {
        old_text = old_text.splitn(2, "\n").collect::<Vec<_>>()[1].into()
    };
    app.find_id::<TextView>("info").unwrap().set_content(
        old_text + "\n- " + &text
    );
    app.find_id::<ScrollView<LinearLayout>>("history").unwrap().scroll_to_bottom();
}

pub fn ClearInfo(app: &mut Cursive) {
    app.find_id::<TextView>("info").unwrap().set_content("");
}

pub fn ShowMinimap(app: &mut Cursive) {
    let mut text: String = "_|_|_\n_|_|_\n | | ".to_owned();
    let hero: &hero::THero = get_ref_curhero!();
    let x = if hero.x < map::MAP_WIDTH / 3 {
        0
    } else if hero.x < 2 * map::MAP_WIDTH / 3 {
        2
    } else {
        4
    };
    let y = if hero.y < map::MAP_HEIGHT / 3 {
        0
    } else if hero.y < 2 * map::MAP_HEIGHT / 3 {
        1
    } else {
        2
    };
    text.remove(6 * y + x);
    text.insert(6 * y + x, '@');
    app.find_id::<TextView>("minimap")
        .unwrap()
        .set_content(text);
}

fn ShowCompassInfo(app: &mut Cursive, direction: map::Direction) {
    use map::Direction::*;
    let mut text = app
        .find_id::<TextView>("compass")
        .unwrap()
        .get_content()
        .source()
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
        }
        Right => {
            text.remove(26);
            text.insert(26, '>');
        }
        Up => {
            text.remove(14);
            text.insert(14, '^');
        }
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
    pub y: usize,
}

pub static mut CURSOR: Cursor = Cursor { x: 0, y: 0 };

fn move_cursor(mut app: &mut Cursive, direction: map::Direction) {
    use map::Direction::*;
    unsafe {
        let (mut dx, mut dy) = (0i32, 0i32);
        match direction {
            Up => {
                dy = if CURSOR.y > 0 { -1 } else { 0 };
            }
            Down => {
                dy = if CURSOR.y < map::LOCAL_MAP_HEIGHT - 1 {
                    1
                } else {
                    0
                };
            }
            Left => {
                dx = if CURSOR.x > 0 { -1 } else { 0 };
            }
            Right => {
                dx = if CURSOR.x < map::LOCAL_MAP_WIDTH - 1 {
                    1
                } else {
                    0
                };
            }
        }
        //ShowInfo(app, format!("{:?}, {:?}", dx, dy));
        let cur_map = get_ref_curmap_wo_unsafe!();
        let hero: &mut hero::THero = get_mut_ref_curhero_wo_unsafe!(hero::CUR_HERO);

        // If hero died to stop his moving at all.
        if hero.HP <= 0 {
            GameOverAnimation(app);
            return;
        }

        if !map::FreeTile(
            &cur_map.Cells[(hero.x as i32 + dx) as usize][(hero.y as i32 + dy) as usize].Tile,
        ) {
            return;
        }

        // battle with monster
        let mnstr =
            monster::IsMonsterOnTile((hero.x as i32 + dx) as usize, (hero.y as i32 + dy) as usize);

        if mnstr.is_some() {
            combat::HeroAttack(app, hero, mnstr.unwrap());
            monster::MonstersStep(app);
            combat::MonstersAttack(app);
            return;
        }
        //

        let (prev_x, prev_y) = (CURSOR.x, CURSOR.y);
        if dx >= 0 {
            CURSOR.x += dx as usize;
            hero.x += dx as usize;
        } else {
            CURSOR.x = (CURSOR.x as i32 + dx) as usize;
            hero.x = (hero.x as i32 + dx) as usize;
        }
        if dy >= 0 {
            CURSOR.y += dy as usize;
            hero.y += dy as usize;
        } else {
            CURSOR.y = (CURSOR.y as i32 + dy) as usize;
            hero.y = (hero.y as i32 + dy) as usize;
        }
        //ShowInfo(&mut app, CURSOR.x.to_string() + "-" + &CURSOR.y.to_string());
        if prev_x != CURSOR.x || prev_y != CURSOR.y {
            let cur_cell = get_mut_ref_cell_wo_unsafe!(hero.x, hero.y);

            for trap in map::TrapTileSet.iter() {
                if &cur_cell.Tile == trap {
                    cur_cell.Tile = map::tileGrass;
                    if !hero::SkillTest(app, hero, hero::skillTrapSearch) {
                        let dam = map::random(1, hero.MaxHP as usize); //f32::round(hero.MaxHP * 1.1)) + 1;
                        ShowInfo(
                            app,
                            String::from(texts::STR_TRAP) + "(-" + &dam.to_string() + " points)",
                        );
                        hero::DecHP(app, hero, dam);
                    }
                }
            }
            for live in map::LiveTileSet.iter() {
                if &cur_cell.Tile == live {
                    ShowInfo(app, String::from(texts::STR_LIVE));
                    let inc = hero.MaxHP;
                    hero::IncHP(hero, inc, false);
                }
            }

            match dx {
                1 => ShowCompassInfo(app, Right),
                -1 => ShowCompassInfo(app, Left),
                0 | _ => (),
            }
            match dy {
                1 => ShowCompassInfo(app, Down),
                -1 => ShowCompassInfo(app, Up),
                0 | _ => (),
            }

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

            monster::MonstersStep(app);
            // Don't change an order of operations!
            hero::SetHeroVisible(hero::CUR_HERO);
            game::ShowGame(&mut app);
        };
    }
}

pub fn GenerateHero(app: &mut Cursive) {
    app.add_layer(
        Dialog::new()
            .title("Select a hero")
            .button("Create", |a| {
                let curhero = get_mut_ref_curhero!();
                curhero.Race = if a.find_id::<Checkbox>("race0")
                                    .unwrap()
                                    .is_checked() { hero::raceHuman }
                                else if a.find_id::<Checkbox>("race1")
                                    .unwrap()
                                    .is_checked() { hero::raceElf }
                                else if a.find_id::<Checkbox>("race2")
                                    .unwrap()
                                    .is_checked() { hero::raceDwarf }
                                else if a.find_id::<Checkbox>("race3")
                                    .unwrap()
                                    .is_checked() { hero::raceHobbit }
                                else { panic!("Error in hero creation dialog"); };
                curhero.Class = if a.find_id::<Checkbox>("class0")
                                    .unwrap()
                                    .is_checked() { hero::classWarrior }
                                else if a.find_id::<Checkbox>("class1")
                                    .unwrap()
                                    .is_checked() { hero::classArcher }
                                else if a.find_id::<Checkbox>("class2")
                                    .unwrap()
                                    .is_checked() { hero::classWizard }
                                else { panic!("Error in hero creation dialog"); };
                if curhero.Class == hero::classArcher {
                    curhero.Items[0] = Some(game_item::ItemTypes[5]);
                }
                ShowHeroInfo(a, unsafe { hero::CUR_HERO });
                a.pop_layer();
                game::ShowGame(a);
                enable_main_shortcuts(a);
            })
            .content(LinearLayout::vertical()
                .child(DummyView.fixed_size((45, 1)))
                .child(LinearLayout::horizontal()
                    .child(Dialog::new()
                        .title("Select a race")
                        .content(LinearLayout::vertical()
                            .child(DummyView.fixed_size((10, 1)))
                            .child(LinearLayout::horizontal()
                                .child(TextView::new(texts::RaceName[0]))
                                .child(Checkbox::new()
                                    .on_change(|a, flag| {
                                        if flag {
                                            a.find_id::<Checkbox>("race1")
                                                .unwrap()
                                                .uncheck();
                                            a.find_id::<Checkbox>("race2")
                                                .unwrap()
                                                .uncheck();
                                            a.find_id::<Checkbox>("race3")
                                                .unwrap()
                                                .uncheck();
                                        } else if !a.find_id::<Checkbox>("race1")
                                            .unwrap()
                                            .is_checked()
                                            && !a.find_id::<Checkbox>("race2")
                                                .unwrap()
                                                .is_checked()
                                            && !a.find_id::<Checkbox>("race3")
                                                .unwrap()
                                                .is_checked() {
                                                    a.find_id::<Checkbox>("race0")
                                                        .unwrap()
                                                        .check();
                                                } 
                                    })
                                    .with_id("race0")
                                    .fixed_size((10, 1))))
                            .child(LinearLayout::horizontal()
                                .child(TextView::new(texts::RaceName[1]))
                                .child(Checkbox::new()
                                    .on_change(|a, flag| {
                                        if flag {
                                            a.find_id::<Checkbox>("race0")
                                                .unwrap()
                                                .uncheck();
                                            a.find_id::<Checkbox>("race2")
                                                .unwrap()
                                                .uncheck();
                                            a.find_id::<Checkbox>("race3")
                                                .unwrap()
                                                .uncheck();
                                        } else if !a.find_id::<Checkbox>("race0")
                                            .unwrap()
                                            .is_checked()
                                            && !a.find_id::<Checkbox>("race2")
                                                .unwrap()
                                                .is_checked()
                                            && !a.find_id::<Checkbox>("race3")
                                                .unwrap()
                                                .is_checked() {
                                                    a.find_id::<Checkbox>("race1")
                                                        .unwrap()
                                                        .check();
                                                }
                                    })
                                    .with_id("race1")
                                    .fixed_size((10, 1))))
                            .child(LinearLayout::horizontal()
                                .child(TextView::new(texts::RaceName[2]))
                                .child(Checkbox::new()
                                    .on_change(|a, flag| {
                                        if flag {
                                            a.find_id::<Checkbox>("race0")
                                                .unwrap()
                                                .uncheck();
                                            a.find_id::<Checkbox>("race1")
                                                .unwrap()
                                                .uncheck();
                                            a.find_id::<Checkbox>("race3")
                                                .unwrap()
                                                .uncheck();
                                        } else if !a.find_id::<Checkbox>("race0")
                                            .unwrap()
                                            .is_checked()
                                            && !a.find_id::<Checkbox>("race1")
                                                .unwrap()
                                                .is_checked()
                                            && !a.find_id::<Checkbox>("race3")
                                                .unwrap()
                                                .is_checked() {
                                                    a.find_id::<Checkbox>("race2")
                                                        .unwrap()
                                                        .check();
                                                }
                                    })
                                    .with_id("race2")
                                    .fixed_size((10, 1))))
                            .child(LinearLayout::horizontal()
                                .child(TextView::new(texts::RaceName[3]))
                                .child(Checkbox::new()
                                    .on_change(|a, flag| {
                                        if flag {
                                            a.find_id::<Checkbox>("race0")
                                                .unwrap()
                                                .uncheck();
                                            a.find_id::<Checkbox>("race1")
                                                .unwrap()
                                                .uncheck();
                                            a.find_id::<Checkbox>("race2")
                                                .unwrap()
                                                .uncheck();
                                        } else if !a.find_id::<Checkbox>("race0")
                                            .unwrap()
                                            .is_checked()
                                            && !a.find_id::<Checkbox>("race1")
                                                .unwrap()
                                                .is_checked()
                                            && !a.find_id::<Checkbox>("race2")
                                                .unwrap()
                                                .is_checked() {
                                                    a.find_id::<Checkbox>("race3")
                                                        .unwrap()
                                                        .check();
                                                }
                                    })
                                    .with_id("race3")
                                    .fixed_size((10, 1))))))
                    .child(DummyView.fixed_size((1, 10)))
                    .child(Dialog::new()
                        .title("Select a class")
                        .content(LinearLayout::vertical()
                            .child(DummyView.fixed_size((10, 1)))
                            .child(LinearLayout::horizontal()
                                .child(TextView::new(texts::ClassName[0]))
                                .child(Checkbox::new()
                                    .on_change(|a, flag| {
                                        if flag {
                                            a.find_id::<Checkbox>("class1")
                                                .unwrap()
                                                .uncheck();
                                            a.find_id::<Checkbox>("class2")
                                                .unwrap()
                                                .uncheck();
                                        } else if !a.find_id::<Checkbox>("class1")
                                            .unwrap()
                                            .is_checked()
                                            && !a.find_id::<Checkbox>("class2")
                                                .unwrap()
                                                .is_checked() {
                                                    a.find_id::<Checkbox>("class0")
                                                        .unwrap()
                                                        .check();
                                                }
                                    })
                                    .with_id("class0")
                                    .fixed_size((12, 1))))
                            .child(LinearLayout::horizontal()
                                .child(TextView::new(texts::ClassName[1]))
                                .child(Checkbox::new()
                                    .on_change(|a, flag| {
                                        if flag {
                                            a.find_id::<Checkbox>("class0")
                                                .unwrap()
                                                .uncheck();
                                            a.find_id::<Checkbox>("class2")
                                                .unwrap()
                                                .uncheck();
                                        } else if !a.find_id::<Checkbox>("class0")
                                            .unwrap()
                                            .is_checked()
                                            && !a.find_id::<Checkbox>("class2")
                                                .unwrap()
                                                .is_checked() {
                                                    a.find_id::<Checkbox>("class1")
                                                        .unwrap()
                                                        .check();
                                                }
                                    })
                                    .with_id("class1")
                                    .fixed_size((12, 1))))
                            .child(LinearLayout::horizontal()
                                .child(TextView::new(texts::ClassName[2]))
                                .child(Checkbox::new()
                                    .on_change(|a, flag| {
                                        if flag {
                                            a.find_id::<Checkbox>("class0")
                                                .unwrap()
                                                .uncheck();
                                            a.find_id::<Checkbox>("class1")
                                                .unwrap()
                                                .uncheck();
                                        } else if !a.find_id::<Checkbox>("class0")
                                            .unwrap()
                                            .is_checked()
                                            && !a.find_id::<Checkbox>("class1")
                                                .unwrap()
                                                .is_checked() {
                                                    a.find_id::<Checkbox>("class2")
                                                        .unwrap()
                                                        .check();
                                                }
                                    })
                                    .with_id("class2")
                                    .fixed_size((12, 1)))))))
                .fixed_size((46, 10))));
    app.find_id::<Checkbox>("race0").unwrap().check();
    app.find_id::<Checkbox>("class0").unwrap().check();
    disable_current_shortcuts(app);
}

pub fn HeroDied(app: &mut Cursive) {
    ShowInfo(app, String::from(texts::STR_HERO_DIED));
}

fn GameOverAnimation(app: &mut Cursive) {
    disable_current_shortcuts(app);
    app.find_id::<TextView>("area")
        .unwrap()
        .set_content(texts::STR_GAME_OVER);
}
