use std::{
	borrow::Cow,
	ops::{Deref, Range},
};

use gramex::{
	MatchAble, Matcher, Mode,
	cursor::{SimpleCursor, eat, match_map},
	derive::derive_enum_matcher,
	derive_slice_matchable,
	general::{MatchFn, delim_list, list},
	grammar, matcher, matches,
	modes::{Capture, Parse},
	parse,
	result::{Expected, IntoResult, MatchError, MatchResult},
	str::{alpha, alphanum, dec, ws},
	try_match,
};
use lean_string::LeanString;

#[test]
fn a() {
	let a = 'a';
	assert!(matches!("abc".to_string(), a {|v, o| true} 'b' {Box::new('c')} ));
	assert!(matches!(A(1), _ & 'a'));
	assert_eq!(try_match!("a", (_ & (~(~_ allowed:_) -> _) | _)?), Some((Some("a"),)));
	assert_eq!(
		try_match!("abc", (abc: Cow<str> = 'a' 'b' 'c')),
		Some((Cow::Borrowed("abc"),))
	);

	let tokens = Tokens(&[Token::Nb(1), Token::Add, Token::Nb(2)]);

	assert_eq!(
		matcher!(for str, 'a' | 'b' | 'c').expected(),
		Expected::OneOf(vec!["\"a\"".into(), "\"b\"".into(), "\"c\"".into()])
	);
	assert_eq!(Capture::wrap_success(1), 1);
	let (name, value) = try_match!("abc = 123",
		(name = alphanum+) ws* '=' ws* (value = dec+ => value.parse::<u64>().unwrap())
	)
	.unwrap();
	assert_eq!((name, value), ("abc", 123));
	assert_eq!("abc".do_match::<Parse>("abc", &mut 0), Ok("abc"));
	assert_eq!(
		"abc".do_match::<Parse>("abd", &mut 0),
		Err(MatchError::mismatch(Expected::A("\"abc\"".into()), 0))
	);

	let matcher = matcher!(for str,
		(name = alphanum+) ws* '=' ws* (value: u64 = dec+ => value.parse().unwrap())
	);
	assert_eq!(parse("abc = 123", matcher), Ok(("abc", 123)));
	assert_eq!(
		parse("abc = ", matcher),
		Err(MatchError::incomplete(Expected::A("a decimal digit".into()), 6))
	);
	let nb = matcher!(for str, Capture: i64, '-'? ('0'..'9')+ => root.parse().unwrap());
	assert_eq!(try_match("-123", nb), Some(-123));
	assert_eq!(try_match("123", nb), Some(123));
	let b = 'b';
	assert_eq!(parse!("abc", a:'a' b? (c = !'d'+)), Ok(("a", "c")));
	assert!(parse!("abd", a:'a' b? (c = !'d'+)).is_err());
	assert_eq!(parse!("abcd", a:'a' b? (c = !'d'+)), Err(MatchError::excess(3)));

	assert_eq!(true.into_result::<Parse>(0), Ok(()));
	assert_eq!(
		false.into_result::<Parse>(0),
		Err(MatchError::mismatch(Expected::None, 0))
	);

	assert_eq!(Some(1).into_result::<Parse>(1), Ok(1));
	assert_eq!(
		None::<u8>.into_result::<Parse>(1),
		Err(MatchError::mismatch(Expected::None, 1))
	);

	assert_eq!(Ok(1).into_result::<Parse>(1), Ok(1));
	assert_eq!(
		Err::<u8, _>(MatchError::mismatch("thing".into(), 1)).into_result::<Parse>(1),
		Err(MatchError::mismatch("thing".into(), 1))
	);

	let alpha_list = list(alpha, ',');
	assert_eq!(try_match("a,b,c", alpha_list), Some(vec!["a", "b", "c"]));
	assert_eq!(try_match("a", alpha_list), Some(vec!["a"]));
	assert_eq!(try_match("a,1,c", alpha_list), None);
	assert_eq!(try_match("a,", alpha_list), None);
	assert_eq!(try_match("", alpha_list), None);
	assert_eq!(
		try_match!(for str, "a,b,c;", list:list<alpha, ','> ';'),
		Some((vec!["a", "b", "c"],))
	);
	let alpha_list = delim_list('(', alpha, ',', ')');
	assert_eq!(try_match("(a,b,c)", alpha_list), Some(vec!["a", "b", "c"]));
	assert_eq!(try_match("(a)", alpha_list), Some(vec!["a"]));
	assert_eq!(try_match("()", alpha_list), Some(vec![]));
	assert_eq!(try_match("(a,1,c)", alpha_list), None);
	assert_eq!(try_match("(a,)", alpha_list), None);
	assert_eq!(try_match("(a,b,c", alpha_list), None);
	assert_eq!(try_match("a,b,c)", alpha_list), None);
	assert_eq!(
		try_match!(for str, "(a,b,c);", list:delim_list<'(', alpha, ',', ')'> ';'),
		Some((vec!["a", "b", "c"],)),
	);
	let lt_100 = MatchFn::new(|matched: &str, off: &mut usize| {
		let len = matched[*off..]
			.find(|char: char| !char.is_digit(10))
			.unwrap_or(matched.len() - *off);
		matched[*off..*off + len]
			.parse::<u8>()
			.ok()
			.filter(|nb| *nb < 100)
			.inspect(|_| *off += len)
	});

	assert_eq!(try_match("12", lt_100), Some(12));
	assert_eq!(try_match("123", lt_100), None);
	assert_eq!(try_match("abc", lt_100), None);

	matches("abc", MatchFn::new(|matched: &str, off: &mut _| &matched[*off..] == "a"));
}
#[derive(Clone, PartialEq, Debug)]
enum Token {
	Nb(i64),
	Add,
	Sub,
	Eq,
	// ...
}
#[derive(Debug)]
struct Tokens<'src>(&'src [Token]);
grammar! {
	for str;
	let list2<item, sep>: Vec<Item::Capture<'src>> = first:item (items* = sep item:item => item) =>
		std::iter::chain(Some(first), items).collect();
}
/*impl MatchAble for Tokens<'_> {
	type Token<'src>
		= Token
	where
		Self: 'src;
	type Slice<'src>
		= Tokens<'src>
	where
		Self: 'src;
	fn len(&self) -> usize {
		self.0.len()
	}
	fn slice(&self, range: std::ops::Range<usize>) -> Option<Tokens<'_>> {
		self.0.get(range).map(Tokens)
	}
	fn get_token(&self, off: usize) -> Option<Token> {
		self.0.get(off).cloned()
	}
}*/
gramex::derive::derive_slice_matchable!(Tokens(&[Token]), matchers: true, expected: Debug);
struct A(usize);
impl Deref for A {
	type Target = usize;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl MatchAble for A {
	type Slice<'a> = A;
	type Token<'a> = A;
	fn len(&self) -> usize {
		self.0
	}
	fn get_token(&self, _: usize) -> Option<A> {
		None
	}
	fn slice(&self, _: Range<usize>) -> Option<A> {
		Some(A(0))
	}
}
impl Matcher<A> for char {
	type Capture<'a> = ();
	fn do_match<'a, M: Mode>(&self, _: &A, _: &mut usize) -> MatchResult<(), M> {
		Ok(M::wrap_success(()))
	}
}

#[derive_enum_matcher(for Tokens2<'slice>)]
#[derive(Debug, PartialEq, PartialOrd)]
enum Token2 {
	Nb1(i64),
	Add1,
	Eq, // ..
}
#[derive(Debug)]
struct Tokens2<'src>(&'src [Token2]);
derive_slice_matchable!(Tokens2(&[Token2]), matchers: true, expected: Debug);
#[test]
fn g() -> Result<(), MatchError> {
	use gramex::cursor::Cursor;

	type MyCursor<'src> = SimpleCursor<'src, Tokens2<'src>>;

	let mut cur =
		MyCursor::new(&Tokens2(&[Token2::Nb1(1), Token2::Add1, Token2::Nb1(2)]));
	let mut exprs = vec![*cur.eat(nb1)?];
	while cur.try_eat(add1).is_some() {
		exprs.push(*cur.eat(nb1)?);
	}
	assert_eq!(exprs, vec![1, 2]);

	let mut cur = SimpleCursor::new("abc");
	assert_eq!(eat!(cur, a:'a' 'b'? c*:'c'), Ok(("a", vec!["c"])));
	assert!(eat!(&mut cur, 'a').is_err());

	#[derive(Debug, PartialEq)]
	enum Case<'a> {
		A,
		B(&'a str, &'a str),
		D_E,
		F,
	}
	fn map<'a>(cur: &mut SimpleCursor<'a, str>) -> Case<'a> {
		match_map!(cur, {
			'a' => Case::A,
			'b' c1:_ ',' c2:_ => Case::B(c1, c2),
			'd' | 'e' => Case::D_E,
			else => Case::F,
		})
	}
	assert_eq!(map(&mut SimpleCursor::new("a")), Case::A);
	assert_eq!(map(&mut SimpleCursor::new("b1,2")), Case::B("1", "2"));
	assert_eq!(map(&mut SimpleCursor::new("d")), Case::D_E);
	assert_eq!(map(&mut SimpleCursor::new("e")), Case::D_E);
	assert_eq!(map(&mut SimpleCursor::new("f")), Case::F);

	Ok(())
}

mod t {
	use gramex::{
		MatchAble, Matcher, derive_slice_matchable,
		modes::Test,
		result::{Expected, MatchError},
	};

	#[derive(Debug, PartialEq, PartialOrd)]
	enum Token {
		Nb(i64),
		Add,
		Sub,
		Eq,
		// ...
	}
	#[derive(Debug, PartialEq)]
	struct Tokens<'src>(&'src [Token]);
	derive_slice_matchable!(Tokens(&[Token]), matchers: true, expected: Debug);
	#[test]
	fn a() {
		let tokens = Tokens(&[Token::Nb(1), Token::Add, Token::Nb(2)]);
		assert_eq!(tokens.len(), 3);
		assert_eq!(tokens.get_token(1), Some(&Token::Add));
		assert_eq!(tokens.slice(1..3), Some(Tokens(&[Token::Add, Token::Nb(2)])));

		let mut off = 1;
		_ = tokens.skip_n::<Test>(&mut off, 2);
		assert_eq!(off, 3);

		assert_eq!(Token::Nb(1).parse(&tokens, &mut 0), Ok(Tokens(&[Token::Nb(1)])));
		assert_eq!(
			Token::Nb(2).parse(&tokens, &mut 0),
			Err(MatchError::mismatch(Expected::A("Nb(2)".into()), 0))
		);

		assert!([Token::Nb(1), Token::Add].test(&tokens, &mut 0));
		assert!(Tokens(&[Token::Nb(1), Token::Add]).test(&tokens, &mut 0));
	}
}

mod e {
	use gramex::{
		MatchAble, Matcher,
		bits::{BBits, LBits, b},
		bytes::{a_be, a_le, be, f32_le, i16_le, i64_be, i128_le, le, word_be, word_le},
		derive::derive_enum_matcher,
		derive_slice_matchable, matcher, matches,
		modes::Test,
		parse,
		result::{Expected, MatchError},
		try_match,
	};
	#[derive(Debug, PartialEq)]
	enum Sign {
		Pos,
		Neg,
	}
	#[derive_enum_matcher(for Tokens<'slice>, field = .kind, expected = TokenKind::to_expected)]
	#[derive(Debug, PartialEq)]
	enum TokenKind<'src> {
		Ident(&'src str),
		Nb(Sign, u64),
		Add,
		Sub,
		Eq,
		// ...
	}
	impl TokenKind<'_> {
		fn to_expected(var: &str) -> Expected {
			Expected::A(var.to_lowercase().into())
		}
	}
	#[derive(Debug, PartialEq)]
	struct Token<'src> {
		kind: TokenKind<'src>,
		off: usize,
	}
	struct Tokens<'src>(&'src [Token<'src>]);
	derive_slice_matchable!(Tokens(&[Token<'slice>]), matchers: false);
	#[test]
	fn a() {
		let tokens = Tokens(&[
			Token { kind: TokenKind::Ident("abc"), off: 0 },
			Token { kind: TokenKind::Add, off: 4 },
			Token { kind: TokenKind::Nb(Sign::Pos, 123), off: 6 },
		]);

		assert_eq!(
			add.parse(&tokens, &mut 1),
			Ok(&Token { kind: TokenKind::Add, off: 4 })
		);
		assert_eq!(
			add.parse(&tokens, &mut 0),
			Err(MatchError::mismatch("add".into(), 0))
		);
		assert_eq!(
			add.parse(&tokens, &mut 3),
			Err(MatchError::incomplete("add".into(), 3))
		);

		assert_eq!(ident.parse(&tokens, &mut 0), Ok(&"abc"));
		assert_eq!(nb.parse(&tokens, &mut 2), Ok((&Sign::Pos, &123)));

		assert_eq!(parse(&[1, 2, 3][..], &[1, 2, 3]), Ok(&[1, 2, 3][..]));
		assert_eq!(
			parse(&[1, 2, 4][..], &[1, 2, 3]),
			Err(MatchError::mismatch("`01 02 03`".into(), 0))
		);
		assert_eq!(
			parse(&[1, 2][..], &[1, 2, 3]),
			Err(MatchError::incomplete("`01 02 03`".into(), 0))
		);

		assert!(matches(&[1][..], 1));
		assert!(matches(&[1][..], 1..=2));

		assert!(matches(&[1, 2, 3][..], vec![1, 2, 3]));
		assert!(matches(&[1, 2, 3][..], [1, 2, 3]));
		assert!(matches(&[1, 2, 3][..], Box::new([1, 2, 3])));
		let bytes: &[u8] = &[1, 2, 3];
		assert_eq!(MatchAble::len(bytes), 3);

		assert_eq!(MatchAble::slice(bytes, 1..3), Some(&[2, 3][..]));
		assert_eq!(MatchAble::slice(bytes, 1..4), None);
		assert_eq!(MatchAble::get_token(bytes, 2), Some(3));
		assert_eq!(MatchAble::get_token(bytes, 3), None);

		let mut off = 0;
		assert!(MatchAble::skip_n::<Test>(bytes, &mut off, 2).is_ok());
		assert_eq!(off, 2);
		assert_eq!(parse(&[1, 2, 3, 4][..], be(0x1020304u32)), Ok(&[1, 2, 3, 4][..]));
		assert_eq!(try_match(&[1, 2][..], i16_le), Some(0x201));
		assert_eq!(
			try_match(&[1, 2, 3, 4, 5, 6, 7, 8][..], a_le(|n: u64| n > 0)),
			Some(0x80706050_4030201)
		);

		assert_eq!(try_match(&[1, 2, 3, 0][..], le(0x30201u32)), Some(&[1, 2, 3, 0][..]));
		assert!(!matches(&[1, 2, 3, 4][..], le(0x30201u32)));
		assert!(!matches(&[1, 2, 3][..], le(0x30201u32)));
		assert!(matches(&[255, 255][..], le(-1i16)));

		assert_eq!(try_match(&[0, 1, 2, 3][..], be(0x10203u32)), Some(&[0, 1, 2, 3][..]));
		assert!(!matches(&[0, 1, 2, 4][..], be(0x10203u32)));
		assert!(!matches(&[1, 2, 3][..], be(0x10203u32)));
		assert!(matches(&[255, 255][..], be(-1i16)));
		let gt_100 = a_le(|v: u32| v > 100);
		assert_eq!(try_match(&[1, 2, 3, 0][..], gt_100), Some(0x30201));
		assert!(!matches(&[1, 0, 0, 0][..], gt_100));
		assert!(!matches(&[1, 2, 3][..], gt_100));
		assert!(matches(&[255, 255][..], a_le(|v: i16| v < 0)));

		let gt_100 = a_be(|v: u32| v > 100);
		assert_eq!(try_match(&[0, 1, 2, 3][..], gt_100), Some(0x10203));
		assert!(!matches(&[0, 0, 0, 1][..], gt_100));
		assert!(!matches(&[1, 2, 3][..], gt_100));
		assert!(matches(&[255, 255][..], a_be(|v: i16| v < 0)));

		assert_eq!(
			try_match(
				&[0x23, 0x1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0][..],
				i128_le
			),
			Some(0x123)
		);
		assert!(!matches(&[1][..], i128_le));
		assert_eq!(try_match(&[0, 0, 0, 0, 0, 0, 0x1, 0x23][..], i64_be), Some(0x123));
		assert!(!matches(&[1][..], i64_be));
		let value = 0x123 as f32;
		assert_eq!(try_match(&value.to_le_bytes()[..], f32_le), Some(value));
		assert!(!matches(&[1][..], f32_le));
		let bit_matcher = matcher!(for LBits, (a = _[4]) {b(8, 0xaa)} (b = _[4]));
		assert_eq!(
			try_match(&[0xa1, 0x2a][..], word_le(2, bit_matcher)),
			Some((LBits::new(4, 1), LBits::new(4, 2)))
		);
		let bit_matcher = matcher!(for BBits, (a = _[4]) {b(8, 0xaa)} (b = _[4]));
		println!("{:x}", BBits::from_be_bytes([0x1a, 0xa2]).0.value);
		println!("{:?}", bit_matcher.capture(&BBits::new(16, 0x1aa2), &mut 0));
		assert_eq!(
			parse(&[0x1a, 0xa2][..], word_be(2, bit_matcher)),
			Ok((BBits::new(4, 1), BBits::new(4, 2)))
		);
		assert!(!matches(&[0x1b, 0xa2][..], word_be(2, bit_matcher)));
		assert!(!matches(&[0xa1][..], word_be(2, bit_matcher)));
	}
}
