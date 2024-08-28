use std::ops::Add;

use serde::{Deserialize, Serialize};

const OLD_AMOUNT_SEA_CREATURE_TYPES: usize = 72;

// The purpose of OldSpecies(usize) is forcing me to be correct about what range I am referring to when I write a number for a species
// Within a OldSpecies() is always a number in the 0-66 range, outside it is always a number in the 13-79 range.
pub struct OldSpecies(usize);
impl OldSpecies {
	pub fn from(given_species: usize) -> Result<Self, String> {
		if (13..=79).contains(&given_species) {
			Ok( Self(given_species - 13) )
		} else {
			Err(format!("OldSpecies::from() called with given species of {}, eg {}, which is not a species!",given_species,((given_species as i32)-13)))
		}
	}
	pub fn from_array_index(given_species: usize) -> Result<Self, String> {
		if (13..=79).contains(&given_species) {
			Ok( Self(given_species - 13) )
		} else {
			Err(format!("OldSpecies::from() called with given species of {}, eg {}, which is not a species!",given_species,((given_species as i32)-13)))
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OldSeaCreatures {
	pub creatures: Vec<u32>
}

impl OldSeaCreatures {
	pub fn new() -> Self {
		Self {
			creatures: Vec::with_capacity(OLD_AMOUNT_SEA_CREATURE_TYPES),
		}
	}
	pub fn set_species_quantity(&mut self, species: OldSpecies, new_quantity: u32) {
		self.creatures[species.0] = new_quantity;
	}
	pub fn get_species_quantity(&self, species: OldSpecies) -> u32 {
		self.creatures[species.0]
	}
	pub fn sum(&self) -> u32 {
		self.creatures.iter().sum()
	}
}

impl Add for OldSeaCreatures {
	type Output = OldSeaCreatures;
	fn add(self, rhs: OldSeaCreatures) -> OldSeaCreatures {
		let mut res = OldSeaCreatures::new();
		for i in 0..=res.creatures.len() {
			res.creatures[i] = self.creatures[i] + rhs.creatures[i];
		}
		res
	}
}