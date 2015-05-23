extern crate termbox;

use self::termbox::{
	BOLD,
	BLUE,
	WHITE,
};

pub struct Item {
	class: ItemClass,
	symbol: char,
	passable: bool,
	color: self::termbox::Attribute,
}

#[derive(Clone)]
pub enum ItemClass { Wall, Fountain }

impl Item {
	pub fn wall() -> Item { Item { class: ItemClass::Wall, symbol: '#', passable: false, color: WHITE } }
	pub fn fountain() -> Item { Item { class: ItemClass::Fountain, symbol: '~', passable: false, color: BLUE | BOLD } }
	pub fn get_symbol(&self) -> char { self.symbol }
	pub fn is_passable(&self) -> bool { self.passable }
	pub fn get_color(&self) -> u16 { self.color }
}
