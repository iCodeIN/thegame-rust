//! This module describes all text information showing in the game.

use hero;

pub const STR_HERO_RACE: &str = "Race: ";
pub const STR_HERO_CLASS: &str = "Class: ";
pub const STR_HERO_LEVEL: &str = "Level: ";
pub const STR_HERO_EXP: &str = "Experience: ";
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

pub const INIT_DIALOG: &str = "Select <Start> or press [Space] key to start playing.\
                               \nSelect <Quit> or press [Esc] key to exit.";
pub const HELP_EXIT_DIALOG: &str = "Select <Help> to get help.\n\
                                    Select <Quit> to exit.";

pub fn help() -> String {
    format!(
        "[@]: Hero\n\
         \n\
         Tiles:\n\
         [.]: Grass\n\
         [:]: Ground\n\
         [+]: Stairs Up\n\
         [-]: Stairs Down\n\
         [^]: Tree\n\
         [X]: Stone\n\
         \n\
         Monsters:\n\
         [p]: {}\n\
         [%]: {}\n\
         [!]: {}\n\
         [#]: {}\n\
         [&]: {}\n\
         [j]: {}\n\
         [A]: {}\n\
         \n\
         Gameplay keyboard shortcuts:\n\
         [w], [a], [s], [d]: keys for moving of Hero\n\
         [e]: Hero's slots\n\
         [i]: Hero's items\n\
         [Backspace]: to clear an info area on the right pane\n\
         [Insert]: to take an item (you need stand on a tile with the item)",
        STR_MONSTER1,
        STR_MONSTER2,
        STR_MONSTER3,
        STR_MONSTER4,
        STR_MONSTER5,
        STR_MONSTER6,
        STR_MONSTER7
    )
}

pub const STR_MOVE_ITEM_TO_SLOTS : &str = "You move an item to your slots: ";
pub const STR_MOVE_SLOT_TO_ITEMS : &str = "You move an item from slots to items: ";
pub const STR_CANNOT_THROW_ITEM: &str = "There is the busy tile! Cannot throw the item.";
pub const STR_YOU_THROW_ITEM: &str = "You throw the item";
pub const STR_AXE: &str = "Axe";
pub const STR_SWORD: &str = "Sword";
pub const STR_BODYARMOR: &str = "Bodyarmor";
pub const STR_HELM: &str = "Helm";

pub const STR_TRAPOK: &str = "You have neutralized a trap!";
pub const STR_ADD_EXP: &str = "You increased your experience by ";
pub const STR_HANDWEAPONSKILL_OK: &str = "Now melee skills are increased.";
pub const STR_BAD_ATTACK: &str = "You missed…";
pub const STR_BAD_RANGED_ATTACK: &str = "You missed…";
pub const STR_TRAPSKILL_OK: &str = "Your skill of neutralizing traps is increased.";
pub const STR_NONE_WEAPONS: &str = "You have no weapons.";
pub const STR_NONE_RANGED_WEAPONS: &str = "You have no bow.";
pub const STR_NONE_AMMO: &str = "You have no arrows.";
pub const STR_BIG_SKIN: &str = "Monster's skin stood your blow…";
pub const STR_ATTACK: &str = "You do several points damage: ";
pub const STR_MON_KILL: &str = " is dead!";
pub const STR_MON_STOP: &str = ": attack repulsed!";
pub const STR_MON_DEF: &str = " strikes, but doesn't penetrate your defense…";
pub const STR_DEFENCESKILL_OK: &str = "You repulsed a blow!";
pub const STR_MON_ATTACK: &str = " attacks and damages your hero by";

pub const STR_HERO_ITEMS: &str = "Hero's items";
pub const STR_EMPTY_ITEM: &str = "<empty>";
pub const STR_HERO_ITEMINFO: &str = "Press [Enter] or [<key>] to move the item to `Slots`.\n\
                                     Press [Backspace] to throw the item from `Items`.\n\
                                     Press [q] to close the dialog.";

pub const STR_HERO_SLOTITEMS: &str = "Used items";
pub const SlotName: [&str; hero::MaxSlots] = ["Body:    ", "In hand: "];
pub const STR_HERO_SLOTINFO: &str = "Press [Enter] or [<key>] to move the slot to `Items`.\n\
                                     Press [q] to close the dialog.";

pub const STR_GAME_OVER: &str = "









            ########          #####        ##       ##     ##########
           #       ##        #     #       ###     ###     #       #
          #                 #       #      ## #   # ##     # 
         ##        ####    #         #     #   # #   #     ######
          #          #     ###########     #    #    #     # 
           #        #      #         #     #         #     #       #
            ########       #         #     #         #     ##########
 
 
 
 
 
 
 
 
 
 
            #######        ##       ##     ##########     ######## 
           #       #       #         #     #       #      #       #
          #         #      #         #     #              #       #
          #         #      #         #     #####          #######
          #         #       #       #      #              #      #
           #       #         #     #       #       #      #       #
            #######           #####        ##########     ##       ##";

pub const MaxRace: usize = 4;
pub const RaceName: [&str; MaxRace] = [
    "Human", "Elf", "Dwarf", "Hobbit"];

pub const MaxClass: usize = 3;
pub const ClassName: [&str; MaxClass] = [
    "Warrior", "Archer", "Wizard"];

pub const STR_NEXTLEVEL: &str = "Level up!";
pub const STR_RANGEDWEAPONSKILL_OK: &str = "Archery skill has been improved!";

pub const STR_AMMO: &str = "Arrows";
pub const STR_N_TAKED_ARROWS: &str = " arrows taked!";
pub const STR_CROSS: &str = "Bow";
pub const STR_TAKED_BOW: &str = "A bow taked!";
pub const STR_TAKED_ITEM: &str = "You take the item";