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
	let mut player = Character::player();
	player.add_item(InventorySlot::start(Item::potion(Poison, RED), 2));
	world.place(player, (0, 0));
	world.place(Character::monster(), (20, 20));
	ui::run(&mut world);
}
