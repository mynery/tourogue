extern crate termbox;

use self::termbox::{
	RED,
};

mod character;
mod item;
mod level;
mod world;
mod ui;

use item::Item;
use item::Effect::*;
use character::{Character, InventorySlot};
use world::World;

fn main() {
	let mut world = World::new(65, 24);
	let mut inventory = Character::create_inventory();
	Character::add_item(&mut inventory, InventorySlot::start(Item::potion(Poison, RED), 2));
	world.place(Character::player(inventory), (0, 0));
	world.place(Character::monster(), (20, 20));
	ui::run(&mut world);
}
