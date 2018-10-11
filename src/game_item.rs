//! This module describes any game items such as artifacts, amulets, etc.

use loggers::log;
use texts;

const MaxItemInt: usize = 20;
pub const MaxRealInt: usize = 20;

#[derive(Debug, PartialEq)]
pub enum TGameItemType {
    ItemHandWeapon,
    ItemArmor,
    ItemAmmo,
    ItemRangedWeapon,
}

impl Copy for TGameItemType {}

impl Clone for TGameItemType {
    fn clone(&self) -> TGameItemType {
        *self
    }
}

#[derive(Debug)]
pub struct TGameItem<'tgi> {
    pub ID: usize,
    pub x: usize,
    pub y: usize,
    pub IType: TGameItemType,
    pub Name: &'tgi str,
    pub Ints: [Option<usize>; MaxItemInt],
    pub Reals: [Option<f32>; MaxRealInt],
    pub IsVisible: bool,
}

impl<'tgi> Copy for TGameItem<'tgi> {}

impl<'tgi> Clone for TGameItem<'tgi> {
    fn clone(&self) -> TGameItem<'tgi> {
        *self
    }
}

pub const intAttack_d1: usize = 0;
pub const intAttack_d2: usize = 1;
pub const intAttackHit: usize = 2;
pub const intArmorDefence: usize = 3;
pub const intAmmo: usize = 0;
pub const intRangedAmmo: usize = 0;
pub const intRangedRange: usize = 1;
pub const intRangedDices: usize = 2;
pub const intRangedDiceNum: usize = 3; 

pub const MaxItemTypes: usize = 6;
type Items<'tgi> = [TGameItem<'tgi>; MaxItemTypes as usize];
pub const ItemTypes: Items = [
    TGameItem {
        ID: 0,
        x: 0,
        y: 0,
        IType: TGameItemType::ItemHandWeapon,
        Name: texts::STR_AXE,
        // [intAttack_d1, intAttack_d2, intAttackHit(in percents), ...]
        Ints: [
            Some(1usize),
            Some(6),
            Some(50),
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
        Reals: [None; MaxRealInt],
        IsVisible: false,
    },
    TGameItem {
        ID: 1,
        x: 0,
        y: 0,
        IType: TGameItemType::ItemHandWeapon,
        Name: texts::STR_SWORD,
        Ints: [
            Some(2usize),
            Some(4),
            Some(80),
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
        Reals: [None; MaxRealInt],
        IsVisible: false,
    },
    TGameItem {
        ID: 2,
        x: 0,
        y: 0,
        IType: TGameItemType::ItemArmor,
        Name: texts::STR_HELM,
        Ints: [
            Some(1usize),
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
        Reals: [None; MaxRealInt],
        IsVisible: false,
    },
    TGameItem {
        ID: 3,
        x: 0,
        y: 0,
        IType: TGameItemType::ItemArmor,
        Name: texts::STR_BODYARMOR,
        // [intArmorDefence, ...]
        Ints: [
            Some(5usize),
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
        Reals: [None; MaxRealInt],
        IsVisible: false,
    },
    TGameItem {
        ID: 4,
        x: 0,
        y: 0,
        IType: TGameItemType::ItemAmmo,
        Name: texts::STR_AMMO,
        Ints: [
            Some(100usize),
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
        Reals: [None; MaxRealInt],
        IsVisible: false,
    },
    TGameItem {
        ID: 5,
        x: 0,
        y: 0,
        IType: TGameItemType::ItemRangedWeapon,
        Name: texts::STR_CROSS,
        Ints: [
            Some(0usize),
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
        Reals: [None; MaxRealInt],
        IsVisible: false,
    },
];

pub const MaxItems: usize = 256;
pub static mut ITEMS: [Option<TGameItem>; MaxItems] = [None; MaxItems];

pub fn GetFreeItemNum() -> Option<usize> {
    for i in 0..MaxItems {
        unsafe {
            if ITEMS[i].is_none() {
                return Some(i);
            }
        }
    }
    None
}

pub fn GetItemName(itm: TGameItem) -> String {
    use self::TGameItemType::*;
    match itm.IType {
        ItemRangedWeapon => itm.Name.to_owned() + "(" + &itm.Ints[intRangedAmmo].unwrap().to_string() + ")",
        ItemAmmo => itm.Name.to_owned() + "(" + &itm.Ints[intAmmo].unwrap().to_string() + ")",
        _ => itm.Name.to_string(),
    }
}