extern crate termbox;

use self::termbox::{
	BOLD,
	BLUE,
	WHITE,
	MAGENTA,
};

#[derive(PartialEq,Eq,Hash)]
pub struct Item {
	class: ItemClass,
	symbol: char,
	weight: i32,
	passable: bool,
	color: self::termbox::Attribute,
}

#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum Effect { Healing, Poison }

#[derive(Clone,PartialEq,Eq,Hash)]
pub enum ItemClass { Wall, Fountain, Street, Door(i32),
                     Potion(Effect)
                   }

impl ToString for ItemClass {
	fn to_string(&self) -> String {
		match *self {
			ItemClass::Wall => "Wall", ItemClass::Fountain => "Fountain",
			ItemClass::Street => "Street",
			ItemClass::Door(_) => "Door", ItemClass::Potion(_) => "Potion"
		}.to_string()
	}
}

impl Item {
	pub fn wall() -> Item { Item { class: ItemClass::Wall, symbol: '#', weight: 0, passable: false, color: WHITE } }
	pub fn fountain() -> Item { Item { class: ItemClass::Fountain, symbol: '~', weight: 0, passable: false, color: BLUE | BOLD } }
	pub fn street() -> Item { Item { class: ItemClass::Street, symbol: '.', weight: ::std::i32::MAX, passable: true, color: WHITE } }
	pub fn door_to(level: i32) -> Item { Item { class: ItemClass::Door(level), symbol: '+', weight: ::std::i32::MAX, passable: true, color: MAGENTA } }
	pub fn potion(effect: Effect, color: self::termbox::Attribute) -> Item { Item { class: ItemClass::Potion(effect), symbol: '!', weight: 1, passable: true, color: color } }
	pub fn get_symbol(&self) -> char { self.symbol }
	pub fn is_passable(&self) -> bool { self.passable }
	pub fn get_color(&self) -> u16 { self.color }
	pub fn get_class<'a>(&'a self) -> &'a ItemClass { &self.class }
}
