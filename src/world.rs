extern crate noise;
use self::noise::Seed;

use character::Character;
use character::CharacterType::*;
use level::Level;
use item::Item;

use std::collections::HashMap;

#[derive(Default)]
pub struct World {
	width: i32, height: i32,
	seed: Option<Seed>,
	levels: HashMap<i32, Level>,
	characters: HashMap<(i32, i32), Character>,
	tourist: Option<Character>,
	tourpos: (i32, i32),
	tourlvl: i32,
	player: Option<Character>,
	position: (i32, i32),
	level: i32,
}

impl World {
	pub fn new(width: i32, height: i32) -> World {
		let level = Level::surface(width, height);
		let mut world = World { position: level.start_pos(), ..Default::default() };
		world.width = width; world.height = height;
		world.seed = Some(Seed::new(12));
		world.levels.insert(0, level);
		world
	}
	pub fn place(&mut self, character: Character, position: (i32, i32)) {
		let (x, y) = position;
		match character.get_type() {
			Monster => { self.characters.insert((x, y), character); }
			Tourist => { self.tourist = Some(character); }
			Player => { self.player = Some(character); }
		}
	}
	pub fn get_position(&self) -> (i32, i32) { self.position }
	pub fn get_player<'a>(&'a self) -> &'a Character {
		match self.player {
			Some(ref character) => character,
			None => panic!()
		}
	}
	pub fn get_level<'a>(&'a self) -> &'a Level { self.levels.get(&self.level).unwrap() }
	pub fn get_chars<'a>(&'a self) -> ::std::iter::Chain<::std::collections::hash_map::Iter<'a, (i32, i32), Character>, ::std::option::IntoIter<(&(i32, i32), &'a Character)>> {
		self.characters.iter().chain(Some((&self.position, self.get_player())).into_iter())
	}
	pub fn get_items<'a>(&'a self) -> ::std::collections::hash_map::Iter<'a, (i32, i32), Item> {
		self.levels.get(&self.level).unwrap().get_items()
	}
	pub fn get_item_at_pos<'a>(&'a self, position: (i32, i32)) -> Option<&'a Item> {
		self.levels.get(&self.level).unwrap().get_at_pos(position)
	}
	pub fn get_char_at_pos<'a>(&'a self, position: (i32, i32)) -> Option<&'a Character> {
		if position == self.position {
			match self.player {
				Some(ref character) => Some(character),
				None => None
			}
		} else {
			self.characters.get(&position)
		}
	}
	pub fn passable(&self, position: (i32, i32)) -> bool {
		match self.get_item_at_pos(position) {
			Some(item) => { item.is_passable() },
			None => { self.get_char_at_pos(position).is_none() }
		}
	}
	pub fn move_player(&mut self, direction: char) {
		let (x, y) = self.position;
		let newpos = match direction {
			'h' => (x - 1, y),
			'j' => (x, y + 1),
			'k' => (x, y - 1),
			'l' => (x + 1, y),
			_ => (x, y)
		};
		if self.passable(newpos) { self.position = newpos; }
	}
	pub fn depth(&self) -> i32 { self.level }
	pub fn descend(&mut self) -> bool {
		let stairs = self.get_level().end_pos();
		if stairs != self.position { return false; }
		self.level = self.level + 1;
		if !self.levels.contains_key(&self.level) {
			self.levels.insert(self.level, Level::new(self.level, self.width, self.height, stairs, &self.seed.as_ref().unwrap()));
		}
		true
	}
	pub fn ascend(&mut self) -> bool {
		if self.level == 0 { return false; }
		self.level = self.level - 1;
		true
	}
}
