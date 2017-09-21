use texts;

const MaxItemInt: u32 = 20;
const MaxRealInt: u32 = 20; 

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
    pub x: u32,
    pub y: u32,
    pub IType: TGameItemType,
    Name : &'tgi str,
    Ints : [u32; MaxItemInt as usize],
    Reals: [f32; MaxRealInt as usize],
    pub IsVisible: bool
}

impl<'tgi> Copy for TGameItem<'tgi> {}

impl<'tgi> Clone for TGameItem<'tgi> {
    fn clone(&self) -> TGameItem<'tgi> {
        *self
    }
}

const intAttack_d1: u32    = 0;
const intAttack_d2: u32    = 1;
const intAttackHit: u32    = 2;
const intArmorDefence: u32 = 0;

pub const MaxItemTypes: u32 = 4;
type Items<'tgi> = [TGameItem<'tgi>; MaxItemTypes as usize];
pub const ItemTypes: Items = [
    TGameItem {
        ID: 0,
        x: 0,
        y: 0,
        IType: TGameItemType::ItemHandWeapon,
        Name : texts::STR_AXE,
        // [intAttack_d1, intAttack_d2, intAttackHit(in percents), ...]
        Ints : [1u32,6,50,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        Reals: [0.0; 20],
        IsVisible: false
    },
    TGameItem {
        ID: 1,
        x: 0,
        y: 0,
        IType: TGameItemType::ItemHandWeapon,
        Name : texts::STR_SWORD,
        Ints : [2u32,4,80,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        Reals: [0.0; 20],
        IsVisible: false
    },
    TGameItem {
        ID: 2,
        x: 0,
        y: 0,
        IType: TGameItemType::ItemArmor,
        Name : texts::STR_AXE,
        Ints : [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        Reals: [0.0; 20],
        IsVisible: false
    },
    TGameItem {
        ID: 3,
        x: 0,
        y: 0,
        IType: TGameItemType::ItemArmor,
        Name : texts::STR_AXE,
        // [intArmorDefence, ...]
        Ints : [5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        Reals: [0.0; 20],
        IsVisible: false
    },
];

pub const MaxItems: usize = 40;
pub static mut ITEMS: [TGameItem; MaxItems] = [
    TGameItem {
        ID: 0,
        x: 0,
        y: 0,
        IType: TGameItemType::ItemHandWeapon,
        Name : texts::STR_AXE,
        // [intAttack_d1, intAttack_d2, intAttackHit(in percents), ...]
        Ints : [0u32,0,50,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        Reals: [0.0; 20],
        IsVisible: false
    }; MaxItems
];
