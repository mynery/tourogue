pub struct Character {
	chartype: CharacterType,
	symbol: char,
}

#[derive(Clone)]
pub enum CharacterType { Player, Tourist, Monster }

impl Character {
	pub fn player() -> Character {
		Character { chartype: CharacterType::Player, symbol: '@' }
	}
	pub fn tourist() -> Character {
		Character { chartype: CharacterType::Tourist, symbol: '@' }
	}
	pub fn monster() -> Character {
		Character { chartype: CharacterType::Monster, symbol: 'd' }
	}
	pub fn get_type(&self) -> CharacterType { self.chartype.clone() }
	pub fn get_symbol(&self) -> char { self.symbol }
}
