extern crate termbox;

use self::termbox::{
	BOLD,
	BLUE,
	WHITE,
	MAGENTA,
};

pub struct Item {
	class: ItemClass,
	symbol: char,
	passable: bool,
	color: self::termbox::Attribute,
}

#[derive(Clone)]
pub enum ItemClass { Wall, Fountain, Street, Door(i32) }

impl Item {
	pub fn wall() -> Item { Item { class: ItemClass::Wall, symbol: '#', passable: false, color: WHITE } }
	pub fn fountain() -> Item { Item { class: ItemClass::Fountain, symbol: '~', passable: false, color: BLUE | BOLD } }
	pub fn street() -> Item { Item { class: ItemClass::Street, symbol: '.', passable: true, color: WHITE } }
	pub fn door_to(level: i32) -> Item { Item { class: ItemClass::Door(level), symbol: '+', passable: true, color: MAGENTA } }
	pub fn get_symbol(&self) -> char { self.symbol }
	pub fn is_passable(&self) -> bool { self.passable }
	pub fn get_color(&self) -> u16 { self.color }
}
