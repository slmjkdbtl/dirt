// wengwengweng

use pom::Parser;
use pom::char_class::alphanum;
use pom::char_class::digit;
use pom::char_class::hex_digit;
use pom::parser::*;
use std::collections::HashMap;

use std::str::FromStr;

pub struct Dirt {

	pub anims: HashMap<String, Anim>,
	pub colors: HashMap<char, Color>,
	pub frames: Vec<Frame>,
	pub size: (u32, u32)

}

pub struct Frame;

pub struct Color {

	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,

}

pub struct Anim {

	pub start: u8,
	pub end: u8,

}

impl Dirt {

	pub fn from_file(fname: &str) -> Self {
		unimplemented!()
	}

	pub fn from_str(code: &str) -> Self {
		unimplemented!()
	}

}

impl Color {

	pub fn from_hex(hex: u32, opacity: u8) -> Self {

		return Self {
			r: (hex >> 16) as u8,
			g: ((hex >> 8) & 0xff) as u8,
			b: (hex & 0xff) as u8,
			a: 255,
		};

	}

}

fn space() -> Parser<u8, ()> {
	return sym(b' ').discard();
}

fn spaces() -> Parser<u8, ()> {
	return sym(b' ').repeat(0..).discard();
}

fn line() -> Parser<u8, ()> {
	return one_of(b"\n\r").discard();
}

fn lines() -> Parser<u8, ()> {
	return one_of(b"\n\r").repeat(0..).discard();
}

fn blank() -> Parser<u8, ()> {
	return spaces() - lines();
}

fn label() -> Parser<u8, String> {
	return is_a(alphanum).repeat(1..).convert(String::from_utf8);
}

fn pixels() -> Parser<u8, String> {
	return is_a(|b| alphanum(b) || b == b'.').repeat(1..).convert(String::from_utf8);
}

fn num() -> Parser<u8, u32> {

	let num = is_a(digit).repeat(1..)
		.collect()
		.convert(|s| String::from_utf8(s.to_vec()))
		.convert(|s| u32::from_str(&s));

	return num;

}

fn sep() -> Parser<u8, ()> {
	return sym(b'=').repeat(1..).discard();
}

fn img() -> Parser<u8, Vec<String>> {
	return sep() * line() * list(pixels(), sym(b'\n')) - line() - sep();
}

fn title() -> Parser<u8, String> {
	return (sym(b'[') * label()) - sym(b']');
}

fn span() -> Parser<u8, (u32, u32)> {
	return (num() - (seq(b"->") | seq(b"<->"))) + num();
}

// fn color() -> Parser<u8, String> {
// 	return seq("0x") * is_a(hex_digit).repeat(6).convert(|s| String::from_utf8(s.to_vec()));
// }

fn anim() {
	let anim = title() - space() + span();
}

