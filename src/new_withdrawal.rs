use serde::{Deserialize, Serialize};
use crate::{handy::contains_all_chars, new_seacreatures::{NewSeaCreatures, Species}, small_types::{SteamId, UnixTime}};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewWithdrawal {
	pub crane_id: u32,
	pub steam_id: SteamId,
	pub specific_withdrawn: NewSeaCreatures,
	pub received_at: UnixTime,
}
impl NewWithdrawal {
	pub fn from_request(crane_id: u32, steam_id: u64, encoded_specific_seacreatures: &String) -> Result<NewWithdrawal, String> {
		
		let mut final_seacreatures = NewSeaCreatures::new();
		
		for unparsed_seacreature in encoded_specific_seacreatures.split("x") {
			if !contains_all_chars(unparsed_seacreature, &['a', 'b', 'c', 'd']) && unparsed_seacreature.is_empty() {
				continue;
			} else if unparsed_seacreature.is_empty() {
				let err = String::from("Unparsed seacreature is not empty, and does not contain abcd!");
				println!("{}",&err);
				return Err(err)
			}

			let seacreature_type_start = unparsed_seacreature.find("a").unwrap()+1;
			let seacreature_type_end = unparsed_seacreature.find("b").unwrap()-1;
			let seacreature_type = String::from(
				&unparsed_seacreature[ seacreature_type_start..=seacreature_type_end ]
			).parse::<u32>().unwrap();


			let seacreature_amount_start = unparsed_seacreature.find("c").unwrap()+1;
			let seacreature_amount_end = unparsed_seacreature.find("d").unwrap()-1;
			let seacreature_amount = String::from(
				&unparsed_seacreature[ seacreature_amount_start..=seacreature_amount_end ]
			).parse::<u32>().unwrap();

			final_seacreatures.set_species_quantity(Species::from(seacreature_type as usize)?, seacreature_amount);
		}
		
		Ok(NewWithdrawal {
			crane_id: crane_id,
			steam_id: SteamId::from_u64(steam_id),
			specific_withdrawn: final_seacreatures,
			received_at: UnixTime::now(),
		})
	}
}