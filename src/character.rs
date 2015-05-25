use item::Item;
use item::ItemClass::*;
use item::Effect::*;

use std::collections::HashSet;

#[derive(PartialEq,Eq,Hash)]
pub struct InventorySlot {
	item: Item,
	count: i32,
	equipped: bool,
	known: bool,
}

pub struct Character {
	chartype: CharacterType,
	symbol: char,
	inventory: HashSet<InventorySlot>,
}

#[derive(Clone)]
pub enum CharacterType { Player, Tourist, Monster }

impl Character {
	pub fn create_inventory() -> HashSet<InventorySlot> { HashSet::new() }
	pub fn add_item(inventory: &mut HashSet<InventorySlot>, additional: InventorySlot) {
		inventory.insert(additional);
	}
	pub fn player(inventory: HashSet<InventorySlot>) -> Character {
		Character { chartype: CharacterType::Player, symbol: '@', inventory: inventory }
	}
	pub fn tourist() -> Character {
		Character { chartype: CharacterType::Tourist, symbol: '@', inventory: HashSet::new() }
	}
	pub fn monster() -> Character {
		Character { chartype: CharacterType::Monster, symbol: 'd', inventory: HashSet::new() }
	}
	pub fn get_type(&self) -> CharacterType { self.chartype.clone() }
	pub fn get_symbol(&self) -> char { self.symbol }
	pub fn get_inventory<'a>(&'a self) -> &'a HashSet<InventorySlot> { &self.inventory }
}

impl InventorySlot {
	pub fn start(item: Item, count: i32) -> InventorySlot {
		InventorySlot { item: item, count: count, equipped: false, known: true }
	}
	pub fn get_item<'a>(&'a self) -> &'a Item { &self.item }
	pub fn get_count(&self) -> i32 { self.count }
	pub fn get_title(&self) -> String {
		if !self.known { return "".to_string(); }
		match self.item.get_class() {
			&Potion(ref effect) => { format!("{:?}", effect) }
			_ => { panic!() }
		}
	}
}
