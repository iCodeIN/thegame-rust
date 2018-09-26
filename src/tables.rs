use hero;

const MaxPlayerLevel: usize = 8;

pub const ExpLevel_Table: [usize; MaxPlayerLevel as usize] =
    [10, 20, 50, 100, 250, 500, 1000, 3000];

pub const HPLevel_Table: [usize; MaxPlayerLevel as usize] = [10, 20, 30, 50, 80, 130, 210, 340];

// percents
pub const BaseSkill_Table: [usize; hero::MaxSkills as usize] = [30, 20];
