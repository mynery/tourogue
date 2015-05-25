extern crate termbox;

use world::World;
use character::CharacterType::*;

use self::termbox::{
	Termbox,
	Event,
	Cell,
	BLACK,
	WHITE,
	GREEN,
	BOLD,
};

pub fn run(world: &mut World) {
	let mut tb = Termbox::open().unwrap();
	draw_level(&mut tb, &world);
	draw_inventory(&mut tb, &world);
	loop {
		match tb.poll_event() {
			Event::Key(event) => {
				match event.ch {
					Some('h') | Some('j') | Some('k') | Some('l') => {
						let position = world.get_position();
						world.move_player(event.ch.unwrap());
						draw_pos(&mut tb, &world, position);
						draw_pos(&mut tb, &world, world.get_position());
						tb.present();
					}
					Some('>') => { if world.descend() { draw_level(&mut tb, &world); } }
					Some('<') => { if world.ascend() { draw_level(&mut tb, &world); } }
					Some('q') => { break; }
					_ => {}
				}
			},
			_ => {},
		}
	}
}

fn draw_level(tb: &mut Termbox, world: &World) {
	tb.set_clear_attributes(BLACK, BLACK);
	tb.clear();
	let (x, y) = world.get_level().end_pos();
	tb.put_cell(x, y, Cell { ch: '>' as u32, fg: WHITE, bg: BLACK });
	let (x, y) = world.get_level().start_pos();
	tb.put_cell(x, y, Cell { ch: '<' as u32, fg: WHITE, bg: BLACK });
	for (position, character) in world.get_chars() {
		let (x, y) = *position;
		let foreground = match character.get_type() {
		                   Player => { WHITE | BOLD },
		                   _ => { WHITE }
		                 };
		tb.put_cell(x, y, Cell { ch: character.get_symbol() as u32,
		                         fg: foreground, bg: BLACK });
	}
	for (position, item) in world.get_items() {
		let (x, y) = *position;
		tb.put_cell(x, y, Cell { ch: item.get_symbol() as u32,
	                           fg: item.get_color(), bg: BLACK });
	}
	tb.present();
}

fn draw_inventory(tb: &mut Termbox, world: &World) {
	let border = '#' as u32;
	for x in 66..81 {
		tb.put_cell(x, 0, Cell { ch: border, fg: WHITE, bg: BLACK});
		tb.put_cell(x, 24, Cell { ch: border, fg: WHITE, bg: BLACK});
	}
	for y in 1..24 {
		tb.put_cell(80, y, Cell { ch: border, fg: WHITE, bg: BLACK});
	}
	for (pos, inventoryslot) in world.get_player().get_inventory().iter().enumerate() {
		let item = inventoryslot.get_item();
		tb.put_cell(66, pos as i32 + 1, Cell { ch: 'a' as u32 + pos as u32, fg: GREEN, bg: BLACK });
		tb.put_cell(67, pos as i32 + 1, Cell { ch: ')' as u32, fg: GREEN, bg: BLACK });
		tb.put_cell(70, pos as i32 + 1, Cell { ch: item.get_symbol() as u32, fg: item.get_color(), bg: BLACK });
		tb.put_str(68, pos as i32 + 1, &format!("{:2}", inventoryslot.get_count()), WHITE, BLACK);
		tb.put_str(71, pos as i32 + 1, &inventoryslot.get_title(), WHITE, BLACK);
	}
	tb.present();
}

fn clear_cell(tb: &mut Termbox, position: (i32, i32)) {
	let (x, y) = position;
	tb.put_cell(x, y, Cell { ch: ' ' as u32, fg: WHITE, bg: BLACK });
}

/*fn debug(tb: &mut Termbox, str: String) {
	let mut i = 0;
	for c in str.chars() {
		tb.put_cell(10 + i, 10, Cell { ch: c as u32, fg: WHITE, bg: BLACK });
		i = i + 1;
	}
}*/

fn draw_pos(tb: &mut Termbox, world: &World, position: (i32, i32)) {
	clear_cell(tb, position);
	let (x, y) = position;
	if position == world.get_level().end_pos() {
		tb.put_cell(x, y, Cell { ch: '>' as u32, fg: WHITE, bg: BLACK });
	}
	if world.depth() > 0 && position == world.get_level().start_pos() {
		tb.put_cell(x, y, Cell { ch: '<' as u32, fg: WHITE, bg: BLACK });
	}
	match world.get_item_at_pos(position) {
		None => { },
		Some(item) => tb.put_cell(x, y, Cell { ch: item.get_symbol() as u32, fg: item.get_color(), bg: BLACK })
	}
	match world.get_char_at_pos(position) {
		None => { },
		Some(character) => tb.put_cell(x, y, Cell { ch: character.get_symbol() as u32, fg: WHITE | BOLD, bg: BLACK })
	}
}
