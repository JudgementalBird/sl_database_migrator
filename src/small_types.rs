use std::time::SystemTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SteamId(u64);
impl SteamId {
	pub const fn from_u64(u: u64) -> SteamId {
		SteamId(u)
	}
	pub fn to_u64(self) -> u64 {
		self.0
	}
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct UnixTime(u64);
impl UnixTime {
	pub const fn from_u64(u: u64) -> UnixTime {
		UnixTime(u)
	}
	pub fn to_u64(self) -> u64 {
		self.0
	}
	pub fn now() -> UnixTime {
		match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
			Ok(n) => UnixTime(n.as_secs()),
			Err(_) => panic!("SystemTime before UNIX EPOCH!"),
		}
	}
}