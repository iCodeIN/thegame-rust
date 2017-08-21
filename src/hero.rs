use low_level;
use map;
use tables;

const MaxChars: i32 = 4;
const chrSTR: i32 = 0;
const chrDEX: i32 = 1;
const chrCON: i32 = 2;
const chrIQ: i32 = 3;

const MaxSkills: i32 = 2;
const skillHandWeapon: i32 = 0;
const skillTrapSearch: i32 = 1; 

#[derive(Debug)]
pub struct THero {
    Chars: [i32; MaxChars as usize],
    Skills: [i32; MaxSkills as usize],
    x: i32,
    y: i32,
    HP: i32,
    MaxHP: i32,
    Exp: i32,
    MaxExp: i32,
    Level: i32,
    VisLong: i32
}

const MaxHeroes: i32 = 1;

pub type Heroes = [THero; MaxHeroes as usize];
static mut HEROES: Heroes = [
	THero {
		Chars: [0, 0, 0, 0],
		Skills: [0, 0],
		x: 0,
		y: 0,
		HP: 0,
		MaxHP: 0,
		Exp: 0,
		MaxExp: 0,
		Level: 0,
		VisLong: 0}; MaxHeroes as usize];
static mut CUR_HERO: i32 = 0;

fn InitHero(HeroNum: i32) {
	unsafe {
		let hero: &mut THero = &mut HEROES[HeroNum as usize];

		for i in 0..MaxChars {
			hero.Chars[i as usize] = 0;
		}
		for j in 0..MaxSkills {
			hero.Skills[j as usize] = 0;
		}

		hero.Level = 0;
		hero.MaxHP = tables::HPLevel_Table[hero.Level as usize];
		hero.HP = hero.MaxHP;
		hero.Exp = 0;
		hero.MaxExp = tables::ExpLevel_Table[hero.Level as usize];
		hero.VisLong = 2;

		let map_ = map::GAME_MAP[map::CUR_MAP as usize];
		let coords = loop {
    	    let x = map_.LocalMapLeft + map::LOCAL_MAP_WIDTH / 3 +
    	    	map::random(map::LOCAL_MAP_WIDTH / 3);
    	    let y = map_.LocalMapTop + map::LOCAL_MAP_HEIGHT / 3 +
    	        map::random(map::LOCAL_MAP_HEIGHT / 3);
    		if map::FreeTile(map_.Cells[x as usize][y as usize].Tile) {
    			break (x, y)
    		};
		};
		hero.x = coords.0;
		hero.y = coords.1;
		low_level::log(&hero.x.to_string());
		low_level::log(&hero.y.to_string());
		SetHeroVisible(HeroNum);
	}
}

pub fn InitHeroes() {
	for i in 0..MaxHeroes {
		InitHero(i);
	};
}

fn SetHeroVisible(HeroNum: i32) {
	unsafe {
		let hero: &mut THero = &mut HEROES[HeroNum as usize];
		let cur_map: &mut map::TMap = &mut map::GAME_MAP[map::CUR_MAP as usize];
		for i in hero.x - hero.VisLong..hero.x + hero.VisLong {
			for j in hero.y - hero.VisLong..hero.y + hero.VisLong {
		    	cur_map.Cells[i as usize][j as usize].IsVisible = true;
			}
		}
	}
}
