use item::Item;
use item::ItemClass::*;
use item::Effect::*;

use std::collections::HashMap;

#[derive(Eq,Hash)]
pub struct InventorySlot {
	item: Item,
	count: i32,
	equipped: bool,
	known: bool,
}

impl InventorySlot {
	pub fn class(&self) -> String { self.item.get_class().to_string() }
	pub fn count(&self) -> i32 { self.count }
	pub fn add(&mut self, count: i32) { self.count = self.count + count; }
}

impl PartialEq for InventorySlot {
	fn eq(&self, other: &InventorySlot) -> bool {
		self.item == other.item && self.known == other.known
	}
}

pub struct Character {
	chartype: CharacterType,
	symbol: char,
	inventory: HashMap<String, Vec<InventorySlot>>,
}

#[derive(Clone)]
pub enum CharacterType { Player, Tourist, Monster }

impl Character {
	pub fn add_item(&mut self, additional: InventorySlot) {
		let class = self.inventory.entry(additional.class()).or_insert(Vec::new());
		let mut found = false;
		for is in class.iter_mut() {
			if is == &additional {
				is.add(additional.count());
				found = true;
				break;
			}
		}
		if !found { class.push(additional); }
	}
	pub fn player() -> Character {
		Character { chartype: CharacterType::Player, symbol: '@', inventory: HashMap::new() }
	}
	pub fn tourist() -> Character {
		Character { chartype: CharacterType::Tourist, symbol: '@', inventory: HashMap::new() }
	}
	pub fn monster() -> Character {
		Character { chartype: CharacterType::Monster, symbol: 'd', inventory: HashMap::new() }
	}
	pub fn get_type(&self) -> CharacterType { self.chartype.clone() }
	pub fn get_symbol(&self) -> char { self.symbol }
	pub fn get_inventory<'a>(&'a self) -> &'a HashMap<String, Vec<InventorySlot>> { &self.inventory }
}

impl InventorySlot {
	pub fn start(item: Item, count: i32) -> InventorySlot {
		InventorySlot { item: item, count: count, equipped: false, known: true }
	}
	pub fn new(item: Item, count: i32) -> InventorySlot {
		InventorySlot { item: item, count: count, equipped: false, known: false }
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
