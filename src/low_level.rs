use std::fs::OpenOptions;
use std::io::Write;

use map;
pub use cursive::Cursive;
use cursive::event::Key;
use cursive::menu::MenuTree;
use cursive::traits::*;
use cursive::views::{TextView, Dialog};
use cursive::utils::{LinesIterator, Row};

pub fn log(s: &str) {
	let mut file = OpenOptions::new()
		.append(true)
		.create(true)
		.open("log.txt")
		.unwrap();
	let mut s: String = s.to_owned();
	s.push_str("\n\n================================================================================\n\n");
	file.write_all(s.as_bytes()).unwrap();
	file.sync_all().unwrap();
}

enum Color {
	Green,
	Black,
	Brown,
	LightGray,
	LightGreen
}

struct TTileRecord {
    C: char,
    Clr: Color
}

const TileRecords: [TTileRecord; (map::tileLast + 1) as usize] = [
    TTileRecord {C: '.', Clr: Color::Green},
    TTileRecord {C: ':', Clr: Color::Black},
    TTileRecord {C: '+', Clr: Color::Brown},
    TTileRecord {C: '=', Clr: Color::Brown},
    TTileRecord {C: '^', Clr: Color::LightGray},
    TTileRecord {C: '!', Clr: Color::LightGreen}
];

// Координаты точки вывода карты. Не имеют смысла в данной реализации, т.к.
// используется не голый ncurses.
const WINDOW_LEFT: i32 = 0;
const WINDOW_TOP: i32 = 0; 

pub fn InitApp(app: &mut Cursive) {
	app.add_global_callback(Key::Esc, |s| s.quit());
	app.add_global_callback('q', |s| s.quit());
	app.add_global_callback(Key::Up, |a| move_cursor(a, "up"));
	app.add_global_callback(Key::Down, |a| move_cursor(a, "down"));
	app.add_global_callback(Key::Left, |a| move_cursor(a, "left"));
	app.add_global_callback(Key::Right, |a| move_cursor(a, "right"));
	
	let mut text: String = "".to_owned();
	for y in 0..map::MAP_HEIGHT {
		for x in 0..map::MAP_WIDTH {
			text.push_str(" ");
		}
		text.push_str("\n");
	}
	text.remove(0usize);
	text.insert(0usize, '@');
	app.add_layer(TextView::new(text)
		.with_id("area")
		.fixed_size((map::MAP_WIDTH, map::MAP_HEIGHT)));
}

pub fn  VideoInitialize() {}

pub fn PrepareMap() {}

pub fn ShowCell(app: &mut Cursive, t: &map::TMapCell, x: i32, y: i32) {
	//GoToXY(WINDOW_LEFT+x-GameMap[CurMap].LocalMapLeft,
	//WINDOW_TOP+y-GameMap[CurMap].LocalMapTop );
	//TextColor( TileRecords[t.Tile].Clr );
	//Write( TileRecords[t.Tile].C 
	let c = TileRecords[t.Tile as usize].C;
	let mut text: String = app.find_id::<TextView>("area")
	    .unwrap()
	    .get_content()
	    .to_owned();
	text.remove(((map::MAP_WIDTH + 1)*y + x) as usize);
	text.insert(((map::MAP_WIDTH + 1)*y + x) as usize,
		if t.IsVisible {c} else {' '});
	app.find_id::<TextView>("area").unwrap().set_content(text);
}


//------------------------------------------------------------------------------

struct Cursor {
    x: i32,
    y: i32,
    prev_pos: (i32, i32),
    prev_char: char
}

static mut CURSOR: Cursor = Cursor {
	x: 0, y: 0, prev_pos: (0, 0), prev_char: ' '};

fn move_cursor(mut app: &mut Cursive, direction: &str) {
	unsafe {
        CURSOR.prev_pos = (CURSOR.x, CURSOR.y);
    }
    match direction {
        "up" => unsafe {
            CURSOR.y = if CURSOR.y > 0 {
                CURSOR.y - 1 } else {0} },
        "down" => unsafe {
            CURSOR.y = if CURSOR.y < map::MAP_HEIGHT - 1 {
                CURSOR.y + 1} else {map::MAP_HEIGHT - 1} },
        "left" => unsafe {
            CURSOR.x = if CURSOR.x > 0 {
                CURSOR.x - 1} else {0} },
        "right" => unsafe {
            CURSOR.x = if CURSOR.x < map::MAP_WIDTH - 1 {
                CURSOR.x + 1} else {map::MAP_WIDTH - 1} },
        _ => unreachable!(),
    }
    unsafe {
    	if CURSOR.prev_pos.0 == CURSOR.x && CURSOR.prev_pos.1 == CURSOR.y {

    	} else {
    		redraw_map(&mut app)
    	};
    }
}

fn redraw_map(app: &mut Cursive) {
    let mut text: String = app.find_id::<TextView>("area")
        .unwrap()
        .get_content()
        .to_owned();
    unsafe {
    	let prev_pos = ((map::MAP_WIDTH + 1)*CURSOR.prev_pos.1
    		+ CURSOR.prev_pos.0) as usize;
        let cur_pos = ((map::MAP_WIDTH + 1)*CURSOR.y + CURSOR.x) as usize;
    	let prev_char = CURSOR.prev_char;
    	CURSOR.prev_char = text.chars().nth(cur_pos).unwrap();
        text.remove(prev_pos);
        text.insert(prev_pos, prev_char);
        text.remove(cur_pos);
        text.insert(cur_pos, '@');
    }
    app.find_id::<TextView>("area").unwrap().set_content(text);
}
