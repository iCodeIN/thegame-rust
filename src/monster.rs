use low_level;
use map;
use texts;

pub struct TMonster<'tm> {
    pub Name: &'tm str,
    pub ID: u32,
    pub x: u32,
    pub y: u32,
    pub HP: i32,
    pub MaxHP: i32,
    pub XP: u32,
    pub Level: u32,
    pub Ad1: i32,
    pub Ad2: i32,
    pub Dd1: i32,
    pub Dd2: i32,
    pub ViewZone: u32,
    pub RandomStep: u32
}

impl<'tm> Copy for TMonster<'tm> {}

impl<'tm> Clone for TMonster<'tm> {
    fn clone(&self) -> TMonster<'tm> {
        *self
    }
}

pub const MaxMonsterTypes: u32 = 7;
type Monsters<'tm> = [TMonster<'tm>; MaxMonsterTypes as usize];
pub const MonsterTypes: Monsters = [
    TMonster {
        Name: texts::STR_MONSTER1,
        ID: 1, x: 0, y: 0,
        HP: 1, MaxHP: 1, XP: 1, Level: 0,
        Ad1: 1, Ad2: 3, Dd1: 1, Dd2: 2,
        ViewZone: 4,
        RandomStep: 3
    },
    TMonster {
        Name: texts::STR_MONSTER2,
        ID: 2, x: 0, y: 0,
        HP: 2, MaxHP: 2, XP: 2, Level: 0,
        Ad1: 1, Ad2: 6, Dd1: 1, Dd2: 2,
        ViewZone: 3,
        RandomStep: 4
    },
    TMonster {
        Name: texts::STR_MONSTER3,
        ID: 3, x: 0, y: 0,
        HP: 5, MaxHP: 5, XP: 3, Level: 0,
        Ad1: 1, Ad2: 2, Dd1: 2, Dd2: 2,
        ViewZone: 4,
        RandomStep: 5
    },
    TMonster {
        Name: texts::STR_MONSTER4,
        ID: 4, x: 0, y: 0,
        HP: 9, MaxHP: 9, XP: 7, Level: 1,
        Ad1: 2, Ad2: 4, Dd1: 1, Dd2: 6,
        ViewZone: 3,
        RandomStep: 5
    },
    TMonster {
        Name: texts::STR_MONSTER5,
        ID: 5, x: 0, y: 0,
        HP: 3, MaxHP: 3, XP: 3, Level: 1,
        Ad1: 12, Ad2: 12, Dd1: 9, Dd2: 2,
        ViewZone: 3,
        RandomStep: 4
    },
    TMonster {
        Name: texts::STR_MONSTER6,
        ID: 6, x: 0, y: 0,
        HP: 20, MaxHP: 20, XP: 15, Level: 2,
        Ad1: 2, Ad2: 6, Dd1: 1, Dd2: 10,
        ViewZone: 4,
        RandomStep: 4
    },
    TMonster {
        Name: texts::STR_MONSTER7,
        ID: 7, x: 0, y: 0,
        HP: 35, MaxHP: 35, XP: 30, Level: 3,
        Ad1: 4, Ad2: 10, Dd1: 2, Dd2: 6,
        ViewZone: 5,
        RandomStep: 3
    }
];

const MaxMonsters: u32 = (map::LOCAL_MAP_WIDTH + map::LOCAL_MAP_HEIGHT)*5/6;
pub static mut MONSTERS: [TMonster; MaxMonsters as usize] = [
    TMonster {
        Name: texts::STR_MONSTER1,
        ID: 1, x: 0, y: 0,
        HP: 1, MaxHP: 1, XP: 1, Level: 0,
        Ad1: 1, Ad2: 3, Dd1: 1, Dd2: 2,
        ViewZone: 4,
        RandomStep: 3
    }; MaxMonsters as usize
];

pub fn GenerateMonsters() {
    let mut v = vec!();
    for mt in MonsterTypes.iter() {
        if mt.Level == unsafe {map::CUR_MAP} { v.push(*mt); };
    }
    for i in 0..MaxMonsters - 1 {
        let mut m: TMonster = v[map::random(0, v.len() as u32) as usize];
        let (x, y) = map::FreeMapPoint(unsafe { &map::GAME_MAP[map::CUR_MAP as usize] });
        m.x = x;
        m.y = y;
        unsafe { MONSTERS[i as usize] = m; }
    }
}

pub fn ShowMonsters(app: &mut low_level::Cursive) {
    for i in 0..MaxMonsters - 1 {
        let m = unsafe { &MONSTERS[i as usize] };
        if m.HP > 0 && map::VisiblePoint(m.x, m.y) {
            low_level::ShowMonster(app, &m);
        };
    }
}
