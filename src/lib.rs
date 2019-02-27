// wengwengweng

use std::collections::HashMap;

pub struct Dirt {
	pub anims: HashMap<String, Anim>,
	pub colors: HashMap<char, Color>,
	pub frames: Vec<Frame>,
}

pub struct Frame;

pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
}

pub struct Anim {
	pub start: u8,
	pub end: u8,
}

impl Dirt {

	pub fn from_file(fname: &str) -> Self {
		return Self::from_str("");
	}

	pub fn from_str(code: &str) -> Self {
		return Self {
			anims: HashMap::new(),
			colors: HashMap::new(),
			frames: Vec::new(),
		}
	}

}

impl Color {

	fn from_hex(hex: u32, opacity: f32) -> Self {

		return Self {
			r: (hex >> 16) as f32 / 255.0,
			g: ((hex >> 8) & 0xff) as f32 / 255.0,
			b: (hex & 0xff) as f32 / 255.0,
		};

	}

}

