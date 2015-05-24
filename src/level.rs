extern crate noise;
use self::noise::{Brownian2, Seed};
extern crate rand;
use self::rand::Rng;

use item::Item;

use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct Level {
	items: HashMap<(i32, i32), Item>,
	upstairs: (i32, i32),
	downstairs: (i32, i32),
}

impl Level {
	fn filled_rect(&mut self, start: (i32, i32), end: (i32, i32), object: (fn() -> Item)) {
		let (x1, y1) = start;
		let (x2, y2) = end;
		for i in x1..(x2+1) {
			for j in y1..(y2+1) {
				self.items.insert((i, j), object());
			}
		}
	}
	fn empty_rect(&mut self, start: (i32, i32), end: (i32, i32), object: (fn() -> Item)) {
		let (x1, y1) = start;
		let (x2, y2) = end;
		self.filled_rect((x1, y1), (x2, y1), object);
		self.filled_rect((x1, y2), (x2, y2), object);
		self.filled_rect((x1, y1 + 1), (x1, y2 - 1), object);
		self.filled_rect((x2, y1 + 1), (x2, y2 - 1), object);
	}
	pub fn shop(id: i32) -> Level {
		match id {
			-1 => { Level { ..Default::default() } }
			_ => { panic!("shop with unknown id") }
		}
	}
	fn building(&mut self, start: (i32, i32), end: (i32, i32), id: i32) {
		self.filled_rect(start, end, Item::wall);
		self.items.insert(((start.0 + end.0)/2, end.1), Item::door_to(id));
	}
	pub fn surface() -> Level {
		let mut level = Level { upstairs: (5, 5), downstairs: (1, 1), ..Default::default() };
		level.empty_rect((0, 0), (80, 24), Item::wall);
		level.filled_rect((1, 11), (79, 13), Item::street);
		level.filled_rect((39, 1), (41, 23), Item::street);
		level.filled_rect((38, 9), (42, 15), Item::street);
		level.filled_rect((37, 10), (43, 14), Item::street);
		level.filled_rect((40, 11), (40, 13), Item::fountain);
		level.filled_rect((39, 12), (41, 12), Item::fountain);
		level.building((44, 9), (46, 10), -2);
		level
	}
	pub fn new(depth: i32, upstairs: (i32, i32), seed: &Seed) -> Level {
		let mut level = Level { upstairs: upstairs, ..Default::default() };
		let mut floors = HashMap::new();
		level.empty_rect((0, 0), (80, 24), Item::wall);
		let noise = Brownian2::new(noise::perlin2, 4).wavelength(4.0);
		for x in 1..80 {
			for y in 1..24 {
				if noise.apply(seed, &[x as f32, y as f32]) > 0.1 && (x, y) != upstairs {
					level.items.insert((x, y), Item::wall());
				} else {
					floors.insert((x, y), (0, 0));
				}
			}
		}
		let mut rooms = Vec::new();
		level.find_rooms(&mut floors, &mut rooms);
		level.place_downstairs(&rooms, upstairs.0 < 40);
		let mut pathes = HashMap::new();
		level.calc_distances(&rooms, &mut pathes);
		level.join_rooms(rooms.len() as i32, &mut pathes);
		level
	}
	fn find_rooms(&mut self, floors: &mut HashMap<(i32, i32), (i32, i32)>, rooms: &mut Vec<Vec<(i32, i32)>>) {
		let mut current_room = Level::find_unprocessed_room(floors);
		while current_room != (0, 0) {
			floors.insert(current_room, current_room);
			let mut cells = Vec::new();
			cells.push(current_room);
			Level::fill_room(floors, current_room, &mut cells);
			if cells.len() > 9 {
				rooms.push(cells);
			}
			current_room = Level::find_unprocessed_room(floors);
		}
		if *floors.get(&self.upstairs).unwrap_or(&(0, 0)) == (0, 0) {
			floors.insert(self.upstairs, self.upstairs);
			rooms.push(vec![self.upstairs]);
		}
	}
	fn fill_room(floors: &mut HashMap<(i32, i32), (i32, i32)>, position: (i32, i32), cells: &mut Vec<(i32, i32)>) {
		let (x, y) = position;
		let &representative = floors.get(&position).unwrap();
		let neighbours = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
		for neighbour in neighbours {
			match floors.get(&neighbour) {
				Some(&candidate) if candidate == (0, 0) => {
					floors.insert(neighbour, representative);
					cells.push(neighbour);
					Level::fill_room(floors, neighbour, cells);
				}, _ => { }
			}
		}
	}
	fn find_unprocessed_room(floors: &HashMap<(i32, i32), (i32, i32)>) -> (i32, i32) {
		for x in 1..80 {
			for y in 1..24 {
				if *floors.get(&(x, y)).unwrap_or(&(x, y)) == (0, 0) { return (x, y); }
			}
		}
		(0, 0)
	}
	fn place_downstairs(&mut self, rooms: &[Vec<(i32, i32)>], right: bool) {
		let finalroom = if right { rooms.last() } else { rooms.first() }.unwrap();
		self.downstairs = finalroom.last().unwrap().clone();
	}
	fn calc_distances(&self, rooms: &[Vec<(i32, i32)>], pathes: &mut HashMap<(i32, i32), Vec<(i32, i32)>>) {
		let mut rng = rand::thread_rng();
		for (id1, room) in rooms.iter().enumerate() {
			for (id2, other) in rooms.iter().enumerate() {
				if id1 >= id2 { continue; }
				let mut shortest: Option<Vec<(i32, i32)>> = None;
				for _ in 0..20 {
					let walls = self.bresenham(*rng.choose(room).unwrap(),
					                           *rng.choose(other).unwrap());
					if shortest.is_none() || shortest.as_ref().unwrap().len() > walls.len() {
						shortest = Some(walls);
					}
				}
				pathes.insert((id1 as i32, id2 as i32), shortest.unwrap());
			}
		}
	}
	fn bresenham(&self, start: (i32, i32), end: (i32, i32)) -> Vec<(i32, i32)> {
		let (mut x1, mut y1) = start;
		let (x2, y2) = end;
		let mut result = Vec::new();
		let sx = if x1 < x2 { 1 } else { -1 };
		let dx = sx*(x2 - x1);
		let sy = if y1 < y2 { 1 } else { -1 };
		let dy = -sy*(y2 - y1);
		let mut err = dx + dy;
		loop {
			if self.items.get(&(x1, y1)).is_some() {
				result.push((x1, y1));
			}
			if x1 == x2 && y1 == y2 { break; }
			if 2*err > dy { err = err + dy; x1 = x1 + sx; } else {
			if 2*err < dx { err = err + dx; y1 = y1 + sy; } }
		}
		result
	}
	fn join_rooms(&mut self, roomcount: i32, pathes: &mut HashMap<(i32, i32), Vec<(i32, i32)>>) {
		let mut roomids = (0..roomcount).collect::<HashSet<i32>>();
		while roomids.len() > 1 {
			let mut min = ::std::usize::MAX;
			for path in pathes.values() {
				if min > path.len() { min = path.len(); }
			}
			let (mut sid1, mut sid2) = (0, 0);
			for (&(id1, id2), path) in pathes.iter() {
				if path.len() == min {
					for cell in path { self.items.remove(&cell); }
					sid1 = id1 as i32; sid2 = id2 as i32; break;
				}
			}
			pathes.remove(&(sid1, sid2));
			for &i in roomids.iter() {
				if i == sid1 || i == sid2 { continue; }
				let tup1 = if i < sid1 { (i, sid1) } else { (sid1, i) };
				let tup2 = if i < sid2 { (i, sid2) } else { (sid2, i) };
				{
					let diff1len = pathes.get(&tup1).unwrap().len();
					let diff2 = pathes.get(&tup2).unwrap().clone();
					if diff2.len() < diff1len { pathes.insert(tup1, diff2); }
				}
				pathes.remove(&tup2);
			}
			roomids.remove(&sid2);
		}
	}
	pub fn get_at_pos<'a>(&'a self, position: (i32, i32)) -> Option<&'a Item> {
		self.items.get(&position)
	}
	pub fn start_pos<'a>(&'a self) -> (i32, i32) { self.upstairs }
	pub fn end_pos<'a>(&'a self) -> (i32, i32) { self.downstairs }
	pub fn get_items<'a>(&'a self) -> ::std::collections::hash_map::Iter<'a, (i32, i32), Item> {
		self.items.iter()
	}
}
