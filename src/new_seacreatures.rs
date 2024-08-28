use std::{ops::Add};

use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

const NEW_AMOUNT_SEA_CREATURE_TYPES: usize = 67; //13..=79

// The purpose of Species(usize) is forcing me to be correct about what range I am referring to when I write a number for a species
// Within a Species() is always a number in the 0-66 range, outside it is always a number in the 13-79 range.
pub struct Species(usize);
impl Species {
	pub fn from(given_species: usize) -> Result<Self, String> {
		if (13..=79).contains(&given_species) {
			Ok( Self(given_species - 13) )
		} else {
			Err(format!("Species::from() called with given species of {}, eg {}, which is not a species!",given_species,((given_species as i32)-13)))
		}
	}
	pub fn from_array_index(given_species: usize) -> Result<Self, String> {
		if (13..=79).contains(&given_species) {
			Ok( Self(given_species - 13) )
		} else {
			Err(format!("Species::from() called with given species of {}, eg {}, which is not a species!",given_species,((given_species as i32)-13)))
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewSeaCreatures {
	#[serde(with = "BigArray")]
	pub creatures: [u32; NEW_AMOUNT_SEA_CREATURE_TYPES]
}

impl NewSeaCreatures {
	pub fn new() -> Self {
		Self {
			creatures: [0; NEW_AMOUNT_SEA_CREATURE_TYPES],
		}
	}
	pub fn set_species_quantity(&mut self, species: Species, new_quantity: u32) {
		self.creatures[species.0] = new_quantity;
	}
	pub fn get_species_quantity(&self, species: Species) -> u32 {
		self.creatures[species.0]
	}
	pub fn sum(&self) -> u32 {
		self.creatures.iter().sum()
	}
}

impl Add for NewSeaCreatures {
	type Output = NewSeaCreatures;
	fn add(self, rhs: NewSeaCreatures) -> NewSeaCreatures {
		let mut res = NewSeaCreatures::new();
		for i in 0..=66 {
			res.creatures[i] = self.creatures[i] + rhs.creatures[i];
		}
		res
	}
}