
pub trait Rand {
	fn next(&mut self) -> f64;
}

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

fn word_raw(len: u32, rng: &mut impl Rand) -> String {
	let mut out = String::new();

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

/// generates a word of a specified length.
pub fn word_len(rng: &mut impl Rand, len: u32) -> String {
	word_raw(len, rng)
}
/// generates a word of a random length.
pub fn word(rng: &mut impl Rand) -> String {
	word_raw((rng.next() * 10.0 + 2.0) as u32, rng)
}


fn sentence_raw(len: u32, rng: &mut impl Rand) -> String {
	let mut out = String::new();

	for i in 0..len {
		out += word_raw((rng.next() * 10.0 + 2.0) as u32, rng).as_str();
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

/// generates `len` words to form a "sentence".
/// the sentence will end with some form of punctuation,
/// and may contain commas.
pub fn sentence_len(rng: &mut impl Rand, len: u32) -> String {
	sentence_raw(len, rng)
}
/// generates a random amount of words to form a "sentence".
/// the sentence will end with some form of punctuation,
/// and may contain commas.
pub fn sentence(rng: &mut impl Rand) -> String {
	sentence_raw((rng.next() * 12.0 + 2.0) as u32, rng)
}


fn paragraph_raw(len: u32, rng: &mut impl Rand) -> String {
	let mut out = String::new();

	for _ in 0..len {
		out += sentence_raw((rng.next() * 12.0 + 2.0) as u32, rng).as_str();
		out += " ";
	}

	out
}

/// generates `len` sentences to form a "paragraph".
pub fn paragraph_len(rng: &mut impl Rand, len: u32) -> String {
	paragraph_raw(len, rng)
}
/// generates a random amount of sentences to form a "paragraph".
pub fn paragraph(rng: &mut impl Rand) -> String {
	paragraph_raw((rng.next() * 12.0 + 4.0) as u32, rng)
}


