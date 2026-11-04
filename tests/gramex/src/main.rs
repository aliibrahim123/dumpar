use gramex::{
	Matcher,
	bits::LBits,
	bytes::word_le,
	cursor::{Cursor, match_map, test, try_eat},
	derive::derive_enum_matcher,
	derive_slice_matchable,
	general::{an, list},
	matcher,
	result::{Expected, MatchError},
	str::{alpha, dec},
};
use rand::{
	distr::{Alphanumeric, SampleString},
	rng,
};

mod a;
mod claude {
	mod a;
	mod b;
	mod c;
	mod d;
	mod e;
}
mod gemini {
	mod a;
	mod b;
}
mod b;
mod showcase;
#[derive(Debug)]
struct Cur<'src> {
	pub off: usize,
	pub input: &'src str,
	pub errors: Vec<MatchError>,
}

impl<'src> Cursor<'src> for Cur<'src> {
	type MatchAble = str;
	type Error = ();
	fn input(&self) -> &'src Self::MatchAble {
		self.input
	}
	fn off(&self) -> usize {
		self.off
	}
	fn off_mut(&mut self) -> &mut usize {
		&mut self.off
	}
	fn map_error(&mut self, err: MatchError) -> Self::Error {
		self.errors.push(err);
		()
	}
}

#[derive_enum_matcher(for S<'slice>, field = .kind, expected = A::kind_to_expected)]
#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
enum A<'src> {
	U(&'src str, u8),
	D,
	F = 100,
	T { a: u16, b: u8 },
	EE3,
	I,
	O(Box<A<'src>>),
}
impl A<'_> {
	fn kind_to_expected(kind: &str) -> Expected {
		Expected::A(kind.into())
	}
}

#[derive(Debug, PartialEq, PartialOrd)]
struct T<'src> {
	off: usize,
	kind: A<'src>,
}
#[derive(Debug)]
struct S<'src>(&'src [T<'src>]);
derive_slice_matchable!(S(&[T<'slice>]), matchers: WithRange, expected: Debug);

fn main() {
	let a = "abc123def";
	let mut cur = Cur { off: 0, input: a, errors: Vec::new() };
	cur.eat("abc").unwrap();
	_ = cur.eat("t");
	let (one, two, three): (_, u8, _) =
		try_eat!(&mut cur, one:'1' (two = _ => two.parse().unwrap()) three*:"3").unwrap();
	assert!(test!(&mut cur, "d" 'e'? ~'f'));
	cur.eat(list(an(1, |c| c != "c"), 'e')).unwrap();
	println!("({one}, {two}, {three:?}) {:?}", cur.errors);

	let a = Alphanumeric.sample_string(&mut rng(), 4);
	let mut cur = Cur { off: 0, input: &a, errors: Vec::new() };
	cur.skip();
	let n = match_map!(cur, {
		'a'..'z' (a = _[2]) => Ok((&a[0..1], &a[1..2])),
		'A'..'D' => Err(2),
		'D'..'Z' a:alpha b:dec => Ok((a, b)),
		else => Err(3),
	});
	println!("{n:?} cursor: {cur:?}");

	println!(
		"{:?}",
		word_le(2, matcher!(for LBits, 0x12u8 0x34))
			.parse(&[0x12u8, 0x32], &mut 0)
			.map_err(|e| e.to_string())
	);
}

enum Y {
	A,
	B(u8),
	C(u8, u16),
	D { a: u8 },
}
fn h() {
	match Y::A {
		Y::A { .. } => (),
		Y::B { .. } => (),
		Y::C(a, b) => (),
		Y::D { a } => (),
	}
}

mod Z {}
