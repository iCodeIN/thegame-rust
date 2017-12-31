//! This module describes any game items such as artifacts, amulets, etc.

use texts;

const MaxItemInt: usize = 20;
const MaxRealInt: usize = 20; 

#[derive(Debug)]
pub enum TGameItemType {
    ItemHandWeapon,
    ItemArmor
}

impl Copy for TGameItemType {}

impl Clone for TGameItemType {
    fn clone(&self) -> TGameItemType {
        *self
    }
}

pub struct TGameItem<'tgi> {
    ID: u32,
    pub x: usize,
    pub y: usize,
    pub IType: Option<TGameItemType>,
    pub Name : &'tgi str,
    Ints : [Option<u32>; MaxItemInt],
    Reals: [Option<f32>; MaxRealInt],
    pub IsVisible: bool
}

impl<'tgi> Copy for TGameItem<'tgi> {}

impl<'tgi> Clone for TGameItem<'tgi> {
    fn clone(&self) -> TGameItem<'tgi> {
        *self
    }
}

const intAttack_d1: usize  = 0;
const intAttack_d2: usize  = 1;
const intAttackHit: usize  = 2;
const intArmorDefence: u32 = 0;

pub const MaxItemTypes: usize = 4;
type Items<'tgi> = [TGameItem<'tgi>; MaxItemTypes as usize];
pub const ItemTypes: Items = [
    TGameItem {
        ID: 0,
        x: 0,
        y: 0,
        IType: Some(TGameItemType::ItemHandWeapon),
        Name : texts::STR_AXE,
        // [intAttack_d1, intAttack_d2, intAttackHit(in percents), ...]
        Ints : [Some(1u32),Some(6),Some(50),None,None,None,None,None,None,None,
                None,None,None,None,None,None,None,None,None,None],
        Reals: [None; MaxRealInt],
        IsVisible: false
    },
    TGameItem {
        ID: 1,
        x: 0,
        y: 0,
        IType: Some(TGameItemType::ItemHandWeapon),
        Name : texts::STR_SWORD,
        Ints : [Some(2u32),Some(4),Some(80),None,None,None,None,None,None,None,
                None,None,None,None,None,None,None,None,None,None],
        Reals: [None; MaxRealInt],
        IsVisible: false
    },
    TGameItem {
        ID: 2,
        x: 0,
        y: 0,
        IType: Some(TGameItemType::ItemArmor),
        Name : texts::STR_HELM,
        Ints : [Some(1u32),None,None,None,None,None,None,None,None,None,None,
                None,None,None,None,None,None,None,None,None],
        Reals: [None; MaxRealInt],
        IsVisible: false
    },
    TGameItem {
        ID: 3,
        x: 0,
        y: 0,
        IType: Some(TGameItemType::ItemArmor),
        Name : texts::STR_BODYARMOR,
        // [intArmorDefence, ...]
        Ints : [Some(5u32),None,None,None,None,None,None,None,None,None,None,
                None,None,None,None,None,None,None,None,None],
        Reals: [None; MaxRealInt],
        IsVisible: false
    },
];

pub const MaxItems: usize = 40;
pub static mut ITEMS: [TGameItem; MaxItems] = [
    TGameItem {
        ID: 0,
        x: 0,
        y: 0,
        IType: Some(TGameItemType::ItemHandWeapon),
        Name : texts::STR_AXE,
        // [intAttack_d1, intAttack_d2, intAttackHit(in percents), ...]
        Ints : [None; MaxItemInt],
        Reals: [None; MaxRealInt],
        IsVisible: false
    }; MaxItems
];
