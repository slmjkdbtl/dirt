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
	pub framelist: Vec<Quad>,
	pub image_data: ImageData,

}

#[derive(Debug, Clone)]
pub struct Quad {

	pub x: u32,
	pub y: u32,
	pub w: u32,
	pub h: u32,

}

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
			a: opacity,
		};

	}

	fn append(&self, pixels: &mut Vec<u8>) {

		pixels.push(self.r);
		pixels.push(self.g);
		pixels.push(self.b);
		pixels.push(self.a);

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
	Frame(u32, Vec<String>),

}

#[derive(Debug)]
pub enum Error {
	Parse(String),
	Io(std::io::Error),
}

// impl From<pom::result::Error> for Error {
// 	fn from(err: pom::result::Error) -> Error {
// 		return Error::Parse("parsing error".to_owned());
// 	}
// }

impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Error {
		return Error::Io(err);
	}
}

enum Layout {

	Horizontal,
	Vertical,

}

struct CharFrame {
	data: Vec<String>,
	width: u32,
	height: u32,
}

impl CharFrame {

	fn new(data: Vec<String>) -> Result<Self, Error> {

		let h = data.len() as u32;

		if h == 0 {
			return Err(Error::Parse(format!("failed to parse frame")));
		}

		let w = data[0].len() as u32;

		if w == 0 {
			return Err(Error::Parse(format!("failed to parse frame")));
		} else {
			return Ok(Self {
				data: data,
				width: w,
				height: h,
			});
		}

	}

}

struct CharFramelist {

	frames: Vec<CharFrame>,
	palette: HashMap<char, Color>,

}

impl CharFramelist {

	fn new(palette: HashMap<char, Color>) -> Self {
		return Self {
			frames: Vec::new(),
			palette: palette,
		};
	}

	fn add(&mut self, frame: CharFrame) -> Result<(), Error> {

		let frames = &mut self.frames;

		if frames.is_empty() {
			return Ok(frames.push(frame));
		}

		if let Some(last) = frames.get(frames.len() - 1) {
			if frame.width != last.width || frame.height != last.height {
				return Err(Error::Parse(format!("failed to parse frames")));
			} else {
				return Ok((frames.push(frame)));
			}
		} else {
			return Err(Error::Parse(format!("failed to parse")))
		}

	}

	fn size(&self) -> Option<(u32, u32)> {
		if let Some(frame) = self.frames.get(0) {
			return Some((frame.width, frame.height));
		} else {
			return None;
		}
	}

	fn to_pixels(&self, layout: Layout) -> Result<(ImageData, Vec<Quad>), Error> {

		let palette = &self.palette;
		let nothing = Color::from_hex(0x000000, 0);
		let mut pixel_frames = Vec::with_capacity(self.frames.len());

		for frame in &self.frames {

			let data = &frame.data;
			let mut pixels = Vec::new();

			for line in data {

				for ch in line.chars() {

					let color;

					if ch == '.' {
						color = &nothing;
					} else {
						if let Some(c) = palette.get(&ch) {
							color = c;
						} else {
							return Err(Error::Parse(format!("cannot find color {}", ch)));
						}
					}

					color.append(&mut pixels);

				}

			}

			pixel_frames.push(pixels);

		}

		match layout {

			Layout::Horizontal => {
				unimplemented!();
			},

			Layout::Vertical => {
				unimplemented!();
			},

		}

		unimplemented!();

	}

}

#[derive(Debug, Clone)]
pub struct ImageData {
	pub width: u32,
	pub height: u32,
	pub data: Vec<u8>
}

impl ImageData {

	pub fn save_png(&self, fname: &str) -> Result<(), Error> {

		image::save_buffer(
			fname,
			&self.data,
			self.width,
			self.height,
			image::ColorType::RGBA(8),
		)?;

		return Ok(());

	}

}

impl Dirt {

	pub fn from_str(code: &'static str) -> Result<Self, Error> {

		let statements = all().parse(code.as_bytes()).expect("failed to parse");

		let mut anims = HashMap::new();
		let mut palette = HashMap::new();
		let mut frames = Vec::new();
		let mut cur_frame = 0;

		for s in statements {

			match s {

				Statement::Anim(name, anim) => {
					anims.insert(name, anim);
				}

				Statement::Color(ch, color) => {
					palette.insert(ch, color);
				}

				Statement::Frame(n, f) => {

					cur_frame += 1;

					if (n != cur_frame) {
						return Err(Error::Parse(format!("frames need to be in order")));
					}

					frames.push(CharFrame::new(f)?);

				}

				_ => {}

			}

		}

		let mut char_framelist = CharFramelist::new(palette);

		for f in frames {
			char_framelist.add(f);
		}

		let (image_data, framelist) = char_framelist.to_pixels(Layout::Horizontal)?;

		return Ok(Self {
			anims: anims,
			image_data: image_data,
			framelist: framelist,
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

