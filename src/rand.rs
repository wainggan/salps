
// https://en.wikipedia.org/wiki/Wichmann%E2%80%93Hill

pub struct Rng {
	seed0: u32,
	seed1: u32,
	seed2: u32,
}
impl Rng {
	#[inline]
	pub fn next(&mut self) -> f32 {
		self.seed0 = self.seed0 * 171 % 30269;
		self.seed1 = self.seed1 * 172 % 30307;
		self.seed2 = self.seed2 * 170 % 30323;
		(
			self.seed0 as f32 / 30269.0 +
			self.seed1 as f32 / 30307.0 +
			self.seed2 as f32 / 30323.0
		) % 1.0
	}
}

fn time() -> u64 {
	use std::time::SystemTime;
	match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
		Ok(n) => n.as_nanos() as u64,
		Err(_) => 0,
	}
}

pub fn rng_raw(seed0: u32, seed1: u32, seed2: u32) -> Rng {
	Rng {
		seed0: seed0 % 30269,
		seed1: seed1 % 30307,
		seed2: seed2 % 30323,
	}
}
pub fn rng_sim(seed: u32) -> Rng {
	rng_raw(seed, seed, seed)
}
pub fn rng() -> Rng {
	rng_sim(time() as u32)
}


