
use lykoi_data::rng;

const FULL: &[&str] = &[
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

const VOWEL: &[&str] = &[
	"a",
	"e",
	"i",
	"o",
	"u",
];

const FILL: &[&str] = &[
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

const LAC: &[&str] = &[
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

fn word_raw(len: u32, rng: &mut rng::XorShift64) -> String {
	let mut out = String::new();

	let mut state = Kind::None;

	let go = |a: &[&'static str], r: f64| {
		a[(r * a.len() as f64) as usize]
	};

	for _ in 0..len {
		out += match state {
			Kind::None => {
				state = Kind::Consonant;
				go(FULL, rng.nextf())
			},
			Kind::Consonant => if rng.nextf() < 0.33 {
				state = Kind::Lac;
				go(LAC, rng.nextf())
			} else {
				state = Kind::Vowel;
				go(VOWEL, rng.nextf())
			},
			Kind::Lac => {
				state = Kind::Vowel;
				go(VOWEL, rng.nextf())
			},
			Kind::Vowel => if rng.nextf() < 0.25 {
				state = Kind::Vowel;
				go(VOWEL, rng.nextf())
			} else {
				state = Kind::Consonant;
				go(FILL, rng.nextf())
			}
		}
	}

	out
}

fn util_gen_time() -> u64 {
	std::time::SystemTime::now()
		.duration_since(std::time::UNIX_EPOCH)
		.unwrap_or(std::time::Duration::from_millis(0x6969696969696969))
		.as_millis()
		.wrapping_pow(7)
		.wrapping_pow(5) as u64
}

fn util_gen_rand() -> rng::XorShift64 {
	rng::XorShift64::new(util_gen_time())
}

/// generates a word of a specified length.
pub fn word_len(len: u32) -> String {
	let mut rng = util_gen_rand();
	word_raw(len, &mut rng)
}
/// generates a word of a random length.
pub fn word() -> String {
	let mut rng = util_gen_rand();
	word_raw((rng.nextf() * 10.0 + 2.0) as u32, &mut rng)
}


fn sentence_raw(len: u32, rng: &mut rng::XorShift64) -> String {
	let mut out = String::new();

	for i in 0..len {
		out += word_raw((rng.nextf() * 10.0 + 2.0) as u32, rng).as_str();
		if i == len - 1 {
			match rng.nextf() {
				0.00..0.05 => out += "!",
				0.05..0.07 => out += "!!",
				0.07..0.14 => out += "?",
				0.14..0.25 => out += " :3",
				0.25..0.27 => out += "...",
				_ => out += ".",
			}
		} else if rng.nextf() <= 0.08 {
			out += ", ";
		} else {
			out += " ";
		}
	}

	out
}

/// generates `len` words to form a "sentence".
/// the sentence will end with some form of punctuation,
/// and may contain commas.
pub fn sentence_len(len: u32) -> String {
	let mut rng = util_gen_rand();
	sentence_raw(len, &mut rng)
}
/// generates a random amount of words to form a "sentence".
/// the sentence will end with some form of punctuation,
/// and may contain commas.
pub fn sentence() -> String {
	let mut rng = util_gen_rand();
	sentence_raw((rng.nextf() * 12.0 + 2.0) as u32, &mut rng)
}


fn paragraph_raw(len: u32, rng: &mut rng::XorShift64) -> String {
	let mut out = String::new();

	for _ in 0..len {
		out += sentence_raw((rng.nextf() * 12.0 + 2.0) as u32, rng).as_str();
		out += " ";
	}

	out
}

/// generates `len` sentences to form a "paragraph".
pub fn paragraph_len(len: u32) -> String {
	let mut rng = util_gen_rand();
	paragraph_raw(len, &mut rng)
}
/// generates a random amount of sentences to form a "paragraph".
pub fn paragraph() -> String {
	let mut rng = util_gen_rand();
	paragraph_raw((rng.nextf() * 12.0 + 4.0) as u32, &mut rng)
}


