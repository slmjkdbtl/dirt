// wengwengweng

use pom::Parser;
use pom::char_class::alphanum;
use pom::char_class::digit;
use pom::char_class::hex_digit;
use pom::parser::*;
use std::collections::HashMap;

use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Dirt {

	pub anims: HashMap<String, Anim>,
	pub size: (u32, u32),
	pub frames: Vec<Quad>,
	pub pixels: Vec<u8>,

}

#[derive(Debug, Clone)]
pub struct Quad {
	pub x: u32,
	pub y: u32,
	pub w: u32,
	pub h: u32,
}

type Frame = Vec<String>;

#[derive(Debug, Clone)]
pub struct Color {

	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,

}

impl Color {

	fn from_hex(hex: u32, opacity: u8) -> Self {

		return Self {
			r: (hex >> 16) as u8,
			g: ((hex >> 8) & 0xff) as u8,
			b: (hex & 0xff) as u8,
			a: 255,
		};

	}

}

#[derive(Debug, Clone)]
pub struct Anim {

	pub start: u32,
	pub end: u32,

}

enum Statement {

	Comment,
	Anim(String, Anim),
	Color(char, Color),
	Frame(u32, Frame),

}

#[derive(Debug)]
pub enum Error {
	Parse(String),
}

// impl From<pom::result::Error> for Error {
// 	fn from(err: pom::result::Error) -> Error {
// 		return Error::Parse("parsing error".to_owned());
// 	}
// }

impl Dirt {

	pub fn from_str(code: &'static str) -> Result<Self, Error> {

		let statements = all().parse(code.as_bytes()).expect("failed to parse");

		let mut anims = HashMap::new();
		let mut colors = HashMap::new();
		let mut images = Vec::new();
		let mut frames = Vec::new();
		let mut cur_frame = 0;

		for s in statements {

			match s {

				Statement::Anim(name, anim) => {
					anims.insert(name, anim);
				}

				Statement::Color(ch, color) => {
					colors.insert(ch, color);
				}

				Statement::Frame(n, f) => {

					cur_frame += 1;

					if (n != cur_frame) {
						return Err(Error::Parse(format!("frames need to be in order")));
					}

					if (f.len() == 0) {
						return Err(Error::Parse(format!("nothing in frame {}", n)))
					}

					images.push(f);

				}

				_ => {}

			}

		}

		let mut pixels = Vec::new();
		let o = Color::from_hex(0xffffff, 0);

		for f in images {

			for line in f {

				for ch in line.chars() {

					let color;

					if ch == '.' {
						color = &o;
					} else {
						if let Some(c) = colors.get(&ch) {
							color = c;
						} else {
							return Err(Error::Parse(format!("cannot find color {}", ch)));
						}
					}

				}

			}

		}

		return Ok(Self {

			anims: anims,
			size: (0, 0),
			frames: frames,
			pixels: pixels,

		});

	}

}

fn space() -> Parser<u8, ()> {
	return sym(b' ').opt().discard();
}

fn line() -> Parser<u8, ()> {
	return one_of(b"\n\r").discard();
}

fn blank() -> Parser<u8, ()> {
	return one_of(b" \n\r\t").repeat(0..).discard();
}

fn label() -> Parser<u8, String> {
	return is_a(alphanum).repeat(1..).convert(String::from_utf8);
}

fn pixels() -> Parser<u8, String> {
	return is_a(|b| alphanum(b) || b == b'.').repeat(1..).convert(String::from_utf8);
}

fn num() -> Parser<u8, u32> {

	let num = is_a(digit).repeat(1..)
		.convert(String::from_utf8)
		.convert(|s| u32::from_str(&s));

	return num;

}

fn sep() -> Parser<u8, ()> {
	return sym(b'=').repeat(1..).discard();
}

fn title() -> Parser<u8, String> {
	return
		sym(b'[')
		* label()
		- sym(b']');
}

fn span() -> Parser<u8, Anim> {

	let parsed =
		num()
		- (seq(b"->") | seq(b"<->"))
		+ num();

	return parsed.map(|(a, b)| Anim {
		start: a,
		end: b,
	});

}

fn frame() -> Parser<u8, Statement> {

	let rule =
		num()
		- line()
		- sep()
		- line()
		+ list(pixels(), sym(b'\n'))
		- line()
		- sep();

	return rule.map(|(n, f)| Statement::Frame(n, f));

}

fn color() -> Parser<u8, Statement> {

	let rule =
		is_a(alphanum).repeat(1).convert(String::from_utf8).map(|s| s.chars().next().unwrap())
		- sym(b':')
		- space()
		+ is_a(hex_digit)
			.repeat(6)
			.convert(String::from_utf8)
			.convert(|s| u32::from_str_radix(&s, 16))
			.map(|s| Color::from_hex(s, 255));

	return rule.map(|(s, c)| Statement::Color(s, c));

}

fn anim() -> Parser<u8, Statement> {

	let rule =
		title()
		- space()
		+ span();

	return rule.map(|(s, a)| Statement::Anim(s, a));

}

fn comment() -> Parser<u8, Statement> {

	let rule =
		sym(b'#')
		* none_of(b"\n\r").repeat(0..).discard();

	return rule.map(|_| Statement::Comment);

}

fn all() -> Parser<u8, Vec<Statement>> {
	return
		blank()
		* list(comment() | anim() | color() | frame(), blank())
		- blank();
}

