mod character;
mod item;
mod level;
mod world;
mod ui;

use character::Character;
use world::World;

fn main() {
	let mut world = World::new();
	world.place(Character::player(), (0, 0));
	world.place(Character::monster(), (20, 20));
	ui::run(&mut world);
}
