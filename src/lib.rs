
pub trait Rand {
	fn next(&mut self) -> f64;
}

fn util_range(rng: &mut impl Rand, range: Option<(u32, Option<u32>)>, default_min: u32, default_max: u32) -> u32 {
	match range.unwrap_or((default_min, Some(default_max))) {
		(x, None) => x,
		(x, Some(y)) => {
			let y = y.max(x);

			let x = x as f64;
			let y = y as f64;

			(rng.next() * (y - x) + x) as u32
		}
	}
}

pub struct Config {
	pub paragraph_len: Option<(u32, Option<u32>)>,
	pub sentence_len: Option<(u32, Option<u32>)>,
	pub word_len: Option<(u32, Option<u32>)>,
}
impl Default for Config {
	fn default() -> Self {
		Self {
			paragraph_len: None,
			sentence_len: None,
			word_len: None,
		}
	}
}


const FULL: &'static [&str] = &[
	"a",
	"b",
	"c",
	"d",
	"e",
	"f",
	"g",
	"h",
	"i",
	"j",
	"k",
	"l",
	"m",
	"n",
	"o",
	"p",
	"q",
	"r",
	"s",
	"t",
	"u",
	"v",
	"w",
	"x",
	"y",
	"z",
];

const VOWEL: &'static [&str] = &[
	"a",
	"e",
	"i",
	"o",
	"u",
];

const FILL: &'static [&str] = &[
	"b",
	"c",
	"d",
	"f",
	"g",
	"h",
	"j",
	"k",
	"l",
	"m",
	"n",
	"p",
	"q",
	"r",
	"s",
	"t",
	"v",
	"w",
	"x",
	"y",
	"z",
];

const LAC: &'static [&str] = &[
	"s",
	"l",
	"w",
];

enum Kind {
	None,
	Consonant,
	Vowel,
	Lac,
}

fn word_raw(rng: &mut impl Rand, config: &Config) -> String {
	let mut out = String::new();

	let len = util_range(rng, config.word_len, 2, 12);

	let mut state = Kind::None;

	let go = |a: &[&'static str], r: f64| {
		a[(r * a.len() as f64) as usize]
	};

	for _ in 0..len {
		out += match state {
			Kind::None => {
				state = Kind::Consonant;
				go(FULL, rng.next())
			},
			Kind::Consonant => if rng.next() < 0.33 {
				state = Kind::Lac;
				go(LAC, rng.next())
			} else {
				state = Kind::Vowel;
				go(VOWEL, rng.next())
			},
			Kind::Lac => {
				state = Kind::Vowel;
				go(VOWEL, rng.next())
			},
			Kind::Vowel => if rng.next() < 0.25 {
				state = Kind::Vowel;
				go(VOWEL, rng.next())
			} else {
				state = Kind::Consonant;
				go(FILL, rng.next())
			}
		}
	}

	out
}

/// generates a word.
pub fn word(rng: &mut impl Rand, config: &Config) -> String {
	word_raw(rng, config)
}


fn sentence_raw(rng: &mut impl Rand, config: &Config) -> String {
	let mut out = String::new();

	let len = util_range(rng, config.sentence_len, 2, 14);

	for i in 0..len {
		out += word_raw(rng, config).as_str();
		if i == len - 1 {
			match rng.next() {
				0.00..0.05 => out += "!",
				0.05..0.07 => out += "!!",
				0.07..0.14 => out += "?",
				0.14..0.25 => out += " :3",
				0.25..0.27 => out += "...",
				_ => out += ".",
			}
		} else if rng.next() <= 0.08 {
			out += ", ";
		} else {
			out += " ";
		}
	}

	out
}

/// generates words to form a "sentence".
/// the sentence will end with some form of punctuation,
/// and may contain commas.
pub fn sentence(rng: &mut impl Rand, config: &Config) -> String {
	sentence_raw(rng, config)
}


fn paragraph_raw(rng: &mut impl Rand, config: &Config) -> String {
	let mut out = String::new();

	let len = util_range(rng, config.paragraph_len, 6, 12);

	for _ in 0..len {
		out += sentence_raw(rng, config).as_str();
		out += " ";
	}

	out
}

/// generates a sentences to form a "paragraph".
pub fn paragraph(rng: &mut impl Rand, config: &Config) -> String {
	paragraph_raw(rng, config)
}


#[cfg(test)]
mod test {
	struct Fake;
	impl crate::Rand for Fake {
		fn next(&mut self) -> f64 {
			0.0
		}
	}
	
	#[test]
	fn test() {
		let word = crate::word(&mut Fake, &crate::Config {
			word_len: Some((4, None)),
			..Default::default()
		});
		assert_eq!(word.len(), 4);
	}
}

