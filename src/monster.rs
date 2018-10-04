//! This module describes Monsters.

use cursive;
use combat;
use hero;
use low_level;
use map;
use texts;

//use decorators::decorators;
use loggers::{log, logger};

/// Ladys and gentlmens, it is a monster!
///
/// `Ad1` and `Ad2` determines an attacking capabilities of a monster.
/// For this we throw `Ad1` virtual dices with `Ad2` sides.
/// Note, it's not real game dice, it's ok if dice has 3 sides, for example :)
///
/// `Dd1` and `Dd2` determines a thickness of a skin.
pub struct TMonster<'tm> {
    pub Name: &'tm str,
    pub ID: usize,
    /// Coordinate: `x`
    pub x: usize,
    /// Coordinate: `y`
    pub y: usize,
    /// Health
    pub HP: usize,
    pub MaxHP: usize,
    /// Experience.
    pub XP: usize,
    pub Level: usize,
    pub Ad1: usize,
    pub Ad2: usize,
    pub Dd1: usize,
    pub Dd2: usize,
    pub ViewZone: usize,
    pub RandomStep: usize,
}

impl<'tm> Copy for TMonster<'tm> {}

impl<'tm> Clone for TMonster<'tm> {
    fn clone(&self) -> TMonster<'tm> {
        *self
    }
}

pub const MaxMonsterTypes: usize = 7;
type Monsters<'tm> = [TMonster<'tm>; MaxMonsterTypes];
pub const MonsterTypes: Monsters = [
    TMonster {
        Name: texts::STR_MONSTER1,
        ID: 1_usize,
        x: 0_usize,
        y: 0_usize,
        HP: 1_usize,
        MaxHP: 1_usize,
        XP: 1_usize,
        Level: 0_usize,
        Ad1: 1_usize,
        Ad2: 3_usize,
        Dd1: 1_usize,
        Dd2: 2_usize,
        ViewZone: 4_usize,
        RandomStep: 3_usize,
    },
    TMonster {
        Name: texts::STR_MONSTER2,
        ID: 2_usize,
        x: 0_usize,
        y: 0_usize,
        HP: 2_usize,
        MaxHP: 2_usize,
        XP: 2_usize,
        Level: 0_usize,
        Ad1: 1_usize,
        Ad2: 6_usize,
        Dd1: 1_usize,
        Dd2: 2_usize,
        ViewZone: 3_usize,
        RandomStep: 4_usize,
    },
    TMonster {
        Name: texts::STR_MONSTER3,
        ID: 3_usize,
        x: 0_usize,
        y: 0_usize,
        HP: 5_usize,
        MaxHP: 5_usize,
        XP: 3_usize,
        Level: 0_usize,
        Ad1: 1_usize,
        Ad2: 2_usize,
        Dd1: 2_usize,
        Dd2: 2_usize,
        ViewZone: 4_usize,
        RandomStep: 5_usize,
    },
    TMonster {
        Name: texts::STR_MONSTER4,
        ID: 4_usize,
        x: 0_usize,
        y: 0_usize,
        HP: 9_usize,
        MaxHP: 9_usize,
        XP: 7_usize,
        Level: 1_usize,
        Ad1: 2_usize,
        Ad2: 4_usize,
        Dd1: 1_usize,
        Dd2: 6_usize,
        ViewZone: 3_usize,
        RandomStep: 5_usize,
    },
    TMonster {
        Name: texts::STR_MONSTER5,
        ID: 5_usize,
        x: 0_usize,
        y: 0_usize,
        HP: 3_usize,
        MaxHP: 3_usize,
        XP: 3_usize,
        Level: 1_usize,
        Ad1: 12_usize,
        Ad2: 12_usize,
        Dd1: 9_usize,
        Dd2: 2_usize,
        ViewZone: 3_usize,
        RandomStep: 4_usize,
    },
    TMonster {
        Name: texts::STR_MONSTER6,
        ID: 6_usize,
        x: 0_usize,
        y: 0_usize,
        HP: 20_usize,
        MaxHP: 20_usize,
        XP: 15_usize,
        Level: 2_usize,
        Ad1: 2_usize,
        Ad2: 6_usize,
        Dd1: 1_usize,
        Dd2: 10_usize,
        ViewZone: 4_usize,
        RandomStep: 4_usize,
    },
    TMonster {
        Name: texts::STR_MONSTER7,
        ID: 7_usize,
        x: 0_usize,
        y: 0_usize,
        HP: 35_usize,
        MaxHP: 35_usize,
        XP: 30_usize,
        Level: 3_usize,
        Ad1: 4_usize,
        Ad2: 10_usize,
        Dd1: 2_usize,
        Dd2: 6_usize,
        ViewZone: 5_usize,
        RandomStep: 3_usize,
    },
];

/// The number of monsters depends on the size of the game map.
pub const MaxMonsters: usize = ((map::LOCAL_MAP_WIDTH + map::LOCAL_MAP_HEIGHT) / 6) * 5; // default is `… * 5`
pub static mut MONSTERS: [TMonster; MaxMonsters] = [TMonster {
    Name: texts::STR_MONSTER1,
    ID: 1_usize,
    x: 0_usize,
    y: 0_usize,
    HP: 1_usize,
    MaxHP: 1_usize,
    XP: 1_usize,
    Level: 0_usize,
    Ad1: 1_usize,
    Ad2: 3_usize,
    Dd1: 1_usize,
    Dd2: 2_usize,
    ViewZone: 4_usize,
    RandomStep: 3_usize,
}; MaxMonsters];

pub fn GenerateMonsters() {
    let mut v = vec![];
    for mt in MonsterTypes.iter() {
        if mt.Level == unsafe { map::CUR_MAP } {
            v.push(*mt);
        };
    }
    for i in 0..MaxMonsters - 1 {
        let mut m: TMonster = v[map::random(0, v.len())];
        let (x, y) = map::FreeMapPoint(get_ref_curmap!());
        m.x = x;
        m.y = y;
        unsafe {
            MONSTERS[i] = m;
        }
    }
}

pub fn ShowMonsters(app: &mut cursive::Cursive) {
    for i in 0..MaxMonsters - 1 {
        let m = unsafe { &MONSTERS[i] };
        if m.HP > 0 && map::VisiblePoint(m.x, m.y) {
            low_level::ShowMonster(app, &m);
        };
    }
}

pub fn IsMonsterOnTile(x: usize, y: usize) -> Option<usize> {
    for i in 0..MaxMonsters {
        if unsafe { (MONSTERS[i].HP > 0) && (MONSTERS[i].x == x) && (MONSTERS[i].y == y) } {
            return Some(i);
        };
    }
    None
}

pub fn CanTrace(m: &TMonster, h: &hero::THero) -> bool {
    let d = combat::Distance((h.x, h.y), (m.x, m.y));
    (d <= m.ViewZone) && (d > 1) 
}

pub fn StepMonster(m: &mut TMonster, dx: i32, dy: i32 ) {
    if dx > 0 { m.x += dx as usize; } else {
        let dx = dx.abs() as usize;
        if m.x >= dx { m.x -= dx; } else { m.x = 0; }
    }
    if dy > 0 { m.y += dy as usize; } else {
        let dy = dy.abs() as usize;
        if m.y >= dy { m.y -= dy; } else { m.y = 0; }
    }
}

pub fn MonsterStep(m: &mut TMonster, h: &hero::THero) {
    let curmap = get_ref_curmap!();
    let mut dx;
    let mut dy;
    if h.x > m.x {
        dx = 1;
    } else if h.x < m.x {
        dx = -1;
    } else { dx = 0; }

    if h.y > m.y {
        dy = 1;
    } else if h.y < m.y {
        dy = -1;
    } else { dy = 0; }

    if dx != 0 {
        if (!map::FreeTile(&curmap.Cells[m.x + dx as usize][m.y].Tile)) ||
           (IsMonsterOnTile(m.x + dx as usize, m.y).is_some()) {
            dx = 0;
        }
    }

    if dy != 0 {
        if (!map::FreeTile(&curmap.Cells[m.x][m.y + dy as usize].Tile)) ||
           (IsMonsterOnTile(m.x, m.y + dy as usize).is_some()) {
            dy = 0;
        }
    }

    if dx == 0 && dy == 0 { return; }

    if dx == 0 {
        StepMonster(m, 0, dy)
    } else if dy == 0 {
        StepMonster(m, dx, 0)
    } else if map::random(0, 2) == 0 {
        StepMonster(m, 0, dy)
    } else { StepMonster(m, dx, 0); }
}

pub fn MonstersStep(app: &mut cursive::Cursive) {
    let curhero = get_ref_curhero!();
    for i in 0..MaxMonsters {
        unsafe {
            let m = &mut MONSTERS[i];
            if m.HP > 0 && CanTrace(m, curhero) {
                low_level::ShowInfo(app, format!("{} moves to you!", m.Name));
                MonsterStep(m, curhero);
            }
        }
    }
    combat::MonstersAttack(app);
}