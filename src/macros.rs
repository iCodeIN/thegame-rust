//----------------------------------Macros------------------------------------//

#[macro_export]
macro_rules! get_ref_curmap {
    () => ({
        use map;
        unsafe { &map::GAME_MAP[map::CUR_MAP as usize] }
    })
}

#[macro_export]
macro_rules! get_ref_curmap_wo_unsafe {
    () => ({
        use map;
        &map::GAME_MAP[map::CUR_MAP as usize]
    })
}

#[macro_export]
macro_rules! get_mut_ref_curmap {
    () => ({
        use map;
        unsafe { &mut map::GAME_MAP[map::CUR_MAP as usize] }
    })
}

#[macro_export]
macro_rules! get_ref_cell {
    ( $x:ident, $y:ident ) => ({
        use map;
        unsafe {
            &map::GAME_MAP[map::CUR_MAP as usize]
                .Cells[$x as usize][$y as usize]
        }
    });
    ( $x:path, $y:path ) => ({
        use map;
        unsafe {
            &map::GAME_MAP[map::CUR_MAP as usize]
                .Cells[$x as usize][$y as usize]
        }
    });
    ( $x:expr, $y:expr ) => ({
        use map;
        unsafe {
            &map::GAME_MAP[map::CUR_MAP as usize]
                .Cells[$x as usize][$y as usize]
        }
    })
}

#[macro_export]
macro_rules! get_mut_ref_cell_wo_unsafe {
    ( $x:ident, $y:ident ) => ({
        use map;
        &mut map::GAME_MAP[map::CUR_MAP as usize].Cells[$x as usize][$y as usize]
    });
    ( $x:path, $y:path ) => ({
        use map;
        &mut map::GAME_MAP[map::CUR_MAP as usize].Cells[$x as usize][$y as usize]
    });
    ( $x:expr, $y:expr ) => ({
        use map;
        &mut map::GAME_MAP[map::CUR_MAP as usize].Cells[$x as usize][$y as usize]
    });
}

#[macro_export]
macro_rules! get_mut_ref_cell {
    ( $x:ident, $y:ident ) => ({
        use map;
        unsafe {
            &mut map::GAME_MAP[map::CUR_MAP as usize]
                .Cells[$x as usize][$y as usize]
        }
    })
}

#[macro_export]
macro_rules! get_ref_curhero {
    () => ({
        use hero;
        unsafe {
            &hero::HEROES[hero::CUR_HERO as usize]
        }
    });
    ( $HeroNum:path ) => ({
        use hero;
        unsafe {
            &hero::HEROES[$HeroNum as usize]
        }
    });
    ( $HeroNum:ident ) => ({
        use hero;
        unsafe {
            &hero::HEROES[$HeroNum as usize]
        }
    })
}

#[macro_export]
macro_rules! get_mut_ref_curhero {
    () => ({
        use hero;
        unsafe {
            &mut hero::HEROES[hero::CUR_HERO as usize]
        }
    });
    ( $HeroNum:ident ) => ({
        use hero;
        unsafe {
            &mut hero::HEROES[$HeroNum as usize]
        }
    });
    ( $HeroNum:path ) => ({
        use hero;
        unsafe {
            &mut hero::HEROES[$HeroNum as usize]
        }
    })
}

#[macro_export]
macro_rules! get_mut_ref_curhero_wo_unsafe {
    () => ({
        use hero;
        unsafe {
            &mut hero::HEROES[hero::CUR_HERO as usize]
        }
    });
    ( $HeroNum:ident ) => ({
        use hero;
        &mut hero::HEROES[$HeroNum as usize]
    });
    ( $HeroNum:path ) => ({
        use hero;
        &mut hero::HEROES[$HeroNum as usize]
    })
}

