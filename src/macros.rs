//! A lot of macros for hiding ugly unsafe blocks by changing most useful global static variables.

#[macro_export]
macro_rules! get_ref_curmap {
    () => ({
        use map;
        unsafe { &map::GAME_MAP[map::CUR_MAP] }
    })
}

#[macro_export]
macro_rules! get_ref_curmap_wo_unsafe {
    () => ({
        use map;
        &map::GAME_MAP[map::CUR_MAP]
    })
}

#[macro_export]
macro_rules! get_mut_ref_curmap {
    () => ({
        use map;
        unsafe { &mut map::GAME_MAP[map::CUR_MAP] }
    })
}

#[macro_export]
macro_rules! get_ref_cell {
    ( $x:ident, $y:ident ) => ({
        use map;
        unsafe {
            &map::GAME_MAP[map::CUR_MAP].Cells[$x][$y]
        }
    });
    ( $x:path, $y:path ) => ({
        use map;
        unsafe {
            &map::GAME_MAP[map::CUR_MAP].Cells[$x][$y]
        }
    });
    ( $x:expr, $y:expr ) => ({
        use map;
        unsafe {
            &map::GAME_MAP[map::CUR_MAP].Cells[$x][$y]
        }
    })
}

#[macro_export]
macro_rules! get_mut_ref_cell_wo_unsafe {
    ( $x:ident, $y:ident ) => ({
        use map;
        &mut map::GAME_MAP[map::CUR_MAP].Cells[$x][$y]
    });
    ( $x:path, $y:path ) => ({
        use map;
        &mut map::GAME_MAP[map::CUR_MAP].Cells[$x][$y]
    });
    ( $x:expr, $y:expr ) => ({
        use map;
        &mut map::GAME_MAP[map::CUR_MAP].Cells[$x][$y]
    });
}

#[macro_export]
macro_rules! get_mut_ref_cell {
    ( $x:ident, $y:ident ) => ({
        use map;
        unsafe {
            &mut map::GAME_MAP[map::CUR_MAP].Cells[$x][$y]
        }
    })
}

#[macro_export]
macro_rules! get_ref_curhero {
    () => ({
        use hero;
        unsafe {
            &hero::HEROES[hero::CUR_HERO]
        }
    });
    ( $HeroNum:path ) => ({
        use hero;
        unsafe {
            &hero::HEROES[$HeroNum]
        }
    });
    ( $HeroNum:ident ) => ({
        use hero;
        unsafe {
            &hero::HEROES[$HeroNum]
        }
    })
}

#[macro_export]
macro_rules! get_mut_ref_curhero {
    () => ({
        use hero;
        unsafe {
            &mut hero::HEROES[hero::CUR_HERO]
        }
    });
    ( $HeroNum:ident ) => ({
        use hero;
        unsafe {
            &mut hero::HEROES[$HeroNum]
        }
    });
    ( $HeroNum:path ) => ({
        use hero;
        unsafe {
            &mut hero::HEROES[$HeroNum]
        }
    })
}

#[macro_export]
macro_rules! get_mut_ref_curhero_wo_unsafe {
    () => ({
        use hero;
        unsafe {
            &mut hero::HEROES[hero::CUR_HERO]
        }
    });
    ( $HeroNum:ident ) => ({
        use hero;
        &mut hero::HEROES[$HeroNum]
    });
    ( $HeroNum:path ) => ({
        use hero;
        &mut hero::HEROES[$HeroNum]
    })
}

#[macro_export]
macro_rules! log {
    ($message:expr) => ({
        use ::DEBUG;
        if DEBUG {
            use loggers::log;
            log($message);
        } else {}
    })
}

#[macro_export]
macro_rules! strict_log {
    ($message:expr) => ({
        use ::DEBUG;
        if DEBUG {
            use loggers::strict_log;
            strict_log($message);
        } else {}
    })
}
