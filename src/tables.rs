use hero;

pub const MaxPlayerLevel: usize = 8;

pub const ExpLevel_Table: [usize; MaxPlayerLevel as usize] =
    [10, 20, 50, 100, 250, 500, 1000, 3000];

pub const HPLevel_Table: [usize; MaxPlayerLevel as usize] = [10, 20, 30, 50, 80, 130, 210, 340];

// percents
pub const BaseSkill_Table: [usize; hero::MaxSkills as usize] = [40, 20, 25, 35];

pub const Race_Table: [[(usize, usize); hero::chrCHA + 1]; hero::raceHobbit + 1] = [
    [(12, 4), (12, 4), (12, 4), (12, 4), (12, 4), (10, 4)],
    [(11, 3), (16, 2), (11, 3), (14, 3), (13, 3), (13, 2)],
    [(16, 2), (12, 3), (16, 2), (9, 1), (10, 2), (7, 3)],
    [(10, 2), (13, 1), (9, 2), (16, 2), (15, 3), (15, 3)],
];

pub const ClassSkill_Table: [[(usize, usize); hero::skillMax + 1]; hero::classWizard + 1] = [
    [(80, 15), (50, 20), (25, 10), (20, 20)],
    [(20, 20), (80, 15), (60, 20), (80, 20)],
    [(70, 20), (50, 20), (15, 10), (35, 20)],
];