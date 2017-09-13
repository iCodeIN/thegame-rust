pub const STR_HERO_HP: &str = "Health/Max Health: ";
pub const STR_HERO_XY: &str = "Coordinates: ";

pub const STR_MONSTER1: &str = "Rat";
pub const STR_MONSTER2: &str = "Wolf";
pub const STR_MONSTER3: &str = "Giant Spider";
pub const STR_MONSTER4: &str = "Skeleton";
pub const STR_MONSTER5: &str = "Orc";
pub const STR_MONSTER6: &str = "Troll";
pub const STR_MONSTER7: &str = "Giant Snake";

pub const STR_TRAP: &str = "A trap works and damages your hero ";
pub const STR_LIVE: &str = "You regained your hero's health in the Source!";

pub const STR_HERO_DIED: &str = "Hero dies!";

pub const INIT_DIALOG: &str = "Select `Start` or press <Space> key to start playing.\
                               \nSelect `Quit` to exit.";
pub const HELP_EXIT_DIALOG: &str = "Select `Help`to get help.\n\
                              Select `Quit` or press <Esc> key to exit.";

pub fn help() -> String {
    format!("`@`: Hero (use <W-A-S-D> keys for moving)\n\
             \n\
             Tiles:\n\
             `.`: Grass\n\
             `:`: Ground\n\
             `+`: Stairs Up\n\
             `-`: Stairs Down\n\
             `^`: Tree\n\
             `X`: Stone\n\
             \n\
             Monsters:\n\
             `p`: {}\n\
             `%`: {}\n\
             `!`: {}\n\
             `#`: {}\n\
             `&`: {}\n\
             `j`: {}\n\
             `A`: {}",
             STR_MONSTER1,
             STR_MONSTER2,
             STR_MONSTER3,
             STR_MONSTER4,
             STR_MONSTER5,
             STR_MONSTER6,
             STR_MONSTER7
    )
}
