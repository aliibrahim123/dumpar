use std::sync::LazyLock;
mod a;
use gramex::{MatchBy, MatchSignal, MatchStatus, by, gramex, matcher, matches, try_match};

use crate::matchers::g_captures::{Cap3, Cap4};

const c1: &str = "abc";
mod r {
	pub const c2: &str = "ali";
}
const c2: LazyLock<Box<str>> = LazyLock::new(|| String::from("abc").into_boxed_str());
gramex! {
	pub mod matchers for str;
	use std::string::String;
	let a = 'a' r::c2 | 'b' _ _ {&'1'} | ('c' "abc") | 'd'..'o';
	let b = 'a' {by(|v, i, _| {println!("{v:?}"); *i += 2; MatchSignal::Matched})} 'b';
	let c = 'd' 'a'? "b"? 'c' !'r' 'a' !~('a'..'z') _ 'b'; // aa
	let d = 'a'+ 'b'[3] 'c'[2..] 'd'[..3] /* example */ 'e'[2..4] 'e' ;
	let e = !'a'[3] & 'b' & (_ 'c' 'e'+) 'e' {by(|v, i, s| { println!("{s:?}"); MatchSignal::Matched})}[2..5];
	let f = 'a' (c: A = 'c'* 'd') (d1? = 'a'..'z' (e2[2..4]: B::<'a> = 'e' => {|v| B(v)}) ) '1';
	let g = 'a' (a = '1') | (b = 'b' => { |_| "b1" }) | (c: A = 'c' (d = 'd') => {|v| A::from(v.matched)});
	let h<a, b, c> = a ',' b ',' c;
	let i: String = 'a' a 'c' => {|v| v.to_string() + "ali"};
	let j = 't' crate::matchers::h<'a', 'b', 'c'?> 'e' (d1 = g);
	let k = '1' (a = 'a'..'z' -> ('a'..'z')[3] (b = 'a' | (c = 'c')));
	let l = "a";
}

#[derive(Debug, PartialEq)]
pub struct A(pub String);
#[derive(Debug, PartialEq)]
pub struct B<'a>(pub &'a str);
impl<'a> From<&'a str> for A {
	fn from(value: &'a str) -> Self {
		Self(value.to_string())
	}
}
impl<'a> From<&'a str> for B<'a> {
	fn from(value: &'a str) -> Self {
		B(value)
	}
}

#[test]
fn min() {
	println!("{:?}", matchers::match_b("a12b"));
	println!("{:?}", matchers::match_a("aali"));
	println!("{:?}", matchers::match_a("cabc"));
	println!("{:?}", matchers::match_a("b😀😀1"));
	println!("{:?}", matchers::match_a("o"));
	println!("{:?}", matchers::match_c("dbcta1b"));
	println!("{:?}", matchers::match_d("aabbbcccddeeeee"));
	println!("{:?}", matchers::match_e("bcee"));
	println!("{:?}", matchers::match_f("acccccdfeee1"));
	println!("{:?}", matchers::match_g("cd"));
	println!("{:?}", matchers::match_g("b"));
	println!("{:?}", matchers::match_i("aaalic"));
	println!(
		"{:?}",
		matchers::match_h(
			"a12b,bbb,c",
			matchers::b,
			matcher!(for str, 'b'[2..5]),
			by(|v: &str, i, s| v.match_by('c', i, s))
		)
	);
	println!("{:?}", matchers::match_j("ta,b,cecd"));

	let a = "a".to_string();
	let a = try_match!(a.as_str(): str, ("a" 'b' (a='1')));
	println!("{:?}", try_match!("ab1": str, ("a" 'b'[0..123] (a: &'a str='1' => {|v| v}))));
	println!("{:?}", matches!("ab1": str, ("a" 'b' ('1'))));
	println!("{:?}", try_match!("bbc": str, !~'b'[3] ~'b'[2] !'a'[3]));

	println!("{:?}", matchers::match_k("1abcc"));
}

#[test]
fn a() {
	// quick matching can be done using `matches` macro
	// matches agianst items by literals, path or blocks
	assert!(matches!("abc": str, "abc"));
	let pat3 = "abc";
	assert!(matches!("abc": str, pat3));
	assert!(matches!("bc": str, { &pat3[1..] }));

	// patterns are separated by whitespace
	assert!(matches!("abc": str, 'a' 'b' 'c'));

	// `?`: optional, `*`: +0 repetition, `+`: +1 repetition
	// `[count]`: exact repetition, `[min..max]`: ranged repetition
	assert!(matches!("abbccc": str, 'a'? 'b'+ 'c'[3]));

	// `!`: matches one item if pattern doesnt match
	// `~`: matches a pattern without advancing
	assert!(matches!("cba": str, !'a' ~'b' "ba"));

	// `_`: matches any, `..` range match
	assert!(matches!("abc": str, 'a'..'z' _ 'c'));

	// `|`: match any of the pattern
	// `&`: match if all patterns matches
	assert!(matches!("b": str, 'a' | 'b' | 'c'));
	assert!(matches!("b": str, 'a'..'z' & !'c'));

	// capture are done using `(name = pattern)`
	assert!(try_match!("abc": str, 'a' (bc = "bc")).is_ok_and(|v| v.bc == "bc"));

	mod example {
		pub const pat: &str = "b";
	}
	static pat: &str = "a";
	assert!(matches!("abc": str, pat example::pat self::pat2))
}
static pat2: &str = "c";
mod test1 {
	//! tests/doc_examples.rs

	// Note: Replace `gramex` with your actual crate name if it is different.
	use gramex::{gramex, matches, try_match};

	#[test]
	fn test_quick_guide() {
		// Literals, variables, blocks
		assert!(matches!("abc": str, "abc"));
		let pat = "abc";
		assert!(matches!("abc": str, pat));
		assert!(matches!("bc": str, { &pat[1..] }));

		// Sequences
		assert!(matches!("abc": str, 'a' 'b' 'c'));

		// Repetitions
		assert!(matches!("abbccc": str, 'a'? 'b'+ 'c'[3]));

		// Peek and Negation
		assert!(matches!("cba": str, !'a' ~'b' "ba"));

		// Ranges and wildcards
		assert!(matches!("abc": str, 'a'..'z' _ 'c'));

		// Choices and Intersections
		assert!(matches!("b": str, 'a' | 'b' | 'c'));
		assert!(matches!("b": str, 'a'..'z' & !'c'));

		// Captures
		assert!(try_match!("abc": str, 'a' (bc = "bc")).is_ok_and(|v| v.bc == "bc"));
	}

	// -----------------------------------------------------------------------------
	// Atoms
	// -----------------------------------------------------------------------------
	mod example {
		pub const PAT: &str = "b";
	}
	static PAT1: &str = "a";
	static PAT2: &str = "c";

	#[test]
	fn test_atoms() {
		// Literals
		assert!(matches!("abc": str, 'a' "bc"));
		assert!(!matches!("cba": str, "abc"));

		// Paths
		assert!(matches!("abc": str, PAT1 example::PAT PAT2));

		// Blocks
		let pats = ['a', 'b'];
		assert!(matches!("abc": str, { pats[0] } { pats[1] } { 'c' }));

		// Groups
		assert!(matches!("ab123": str, ('a'..'z')+ ('0'..'9' | '-')*));
	}

	// -----------------------------------------------------------------------------
	// Unit & Modifiers & Repetition
	// -----------------------------------------------------------------------------
	#[test]
	fn test_modifiers_and_repetition() {
		// Modifiers: Not (!)
		assert!(matches!("b": str, !'a'));
		assert!(matches!("b": str, !"abc"));
		assert!(!matches!("abc": str, !"abc"));

		// Modifiers: Sequential Not (!)
		assert!(matches!("bbx": str, !'a' !"abc" !"abc"));
		assert!(!matches!("bbabc": str, !'a' !"abc" !"abc"));

		// Modifiers: Peek (~)
		assert!(matches!("abc": str, ~'a' ~"abc" !~"dec" _[3]));

		// Repetition: ?, *, +
		assert!(matches!("ab": str, 'a'? 'b' 'c'?));
		assert!(matches!("aaac": str, 'a'* 'b'* 'c'*));
		assert!(matches!("aaab": str, 'a'+ 'b'+));
		assert!(!matches!("aaac": str, 'a'+ 'b'+ 'c'+));

		// Repetition: Exact and Ranges
		assert!(matches!("aaabb": str, 'a'[3] 'b'[2]));
		assert!(!matches!("a": str, 'a'[2]));
		assert!(!matches!("aaa": str, 'a'[2]));

		assert!(matches!("aaabbcccc": str, 'a'[2..4] 'b'[..3] 'c'[3..]));
		assert!(!matches!("a": str, 'a'[2..]));
		assert!(!matches!("aaaaa": str, 'a'[2..4]));

		// Greedy bounds control
		assert!(matches!("ababababc": str, !"abc"* & "ab"* _[3]));

		// Precedence
		assert!(matches!("bbc": str, !~'b'[3] ~'b'[2] !'a'[3]));
	}

	// -----------------------------------------------------------------------------
	// Expressions (Range, Sequence, Or, And)
	// -----------------------------------------------------------------------------
	#[test]
	fn test_expressions() {
		// Range
		assert!(matches!("abc": str, 'a'..'z' !('0'..'9') ('a'..'z')+));
		assert!(!matches!("1": str, 'a'..'z'));

		// Sequence
		assert!(matches!("abc": str, 'a' 'b' 'c'));

		// Or
		assert!(matches!("a": str, 'a' | 'b' 'c' | "abc"));
		assert!(matches!("bcd": str, ('a' | 'b' 'c' | "abc") 'd'));
		assert!(!matches!("d": str, 'a' | 'b' 'c' | "abc"));
		assert!(!matches!("abc": str, 'a' | 'b' 'c' | "abc"));

		// And
		assert!(matches!("abc": str, ('a'..'z')[3] & ('a' _ _)));
		assert!(matches!("abc": str, ('a'..'z')[3] & 'a' & (_ 'b')));
		assert!(
			!matches!("abc": str, ('a'..'z')[3] & !"abc" /*& { touch(|_| println!("not_reached")) } */)
		);
	}

	// -----------------------------------------------------------------------------
	// Captures (Inline)
	// -----------------------------------------------------------------------------
	#[test]
	fn test_captures_inline() {
		// Repeated captures (?, [min..max])
		assert_eq!(try_match!("abcd": str, 'a' (bc? = "bc") 'd').unwrap().bc, Some("bc"));
		assert_eq!(try_match!("ad": str, 'a' (bc? = "bc") 'd').unwrap().bc, None);
		assert_eq!(
			try_match!("abcbcbcd": str, 'a' (bc[2..5] = "bc") 'd').unwrap().bc,
			vec!["bc", "bc", "bc"]
		);

		// Normal capture
		assert_eq!(try_match!("abcd": str, 'a' (bc = "bc") 'd').unwrap().bc, "bc");

		// Structured (Nested captures)
		let capture = try_match!("abcd": str, 'a' (bc = (b = 'b') (c = 'c')) 'd').unwrap().bc;
		assert_eq!(capture.matched, "bc");
		assert_eq!(capture.b, "b");
		assert_eq!(capture.c, "c");

		// Capture Mapping
		assert_eq!(try_match!("abcd": str, 'a' (bc: String = "bc") 'd').unwrap().bc, "bc");
		assert_eq!(
			try_match!("abcd": str, 'a' (bc = "bc" => { |v| &v[1..] }) 'd').unwrap().bc,
			"c"
		);
		assert_eq!(
			try_match!("abcd": str, 'a' (bc: String = "bc" => { |v: &str| v.to_uppercase() }) 'd')
				.unwrap()
				.bc,
			"BC"
		);
	}

	// -----------------------------------------------------------------------------
	// Captures (gramex! Macros)
	// -----------------------------------------------------------------------------
	// We wrap these in a module so they don't pollute the file-level namespace.
	mod gramex_macro_tests {
		use super::*;

		// Term Captures
		gramex! {
			for str;
			let ident = ('a'..'z' | 'A'..'Z' | '0'..'9' | '_')+;
			let value = (ident = ident);
		}

		#[test]
		fn test_term_captures() {
			assert_eq!(match_value("abc").unwrap().ident, "abc");
		}

		// Enumerated Captures
		gramex! {
			for str;
			let example = 'a' (bc = "bc" | (b2c3 = "bbccc") | (b3c1 = "bbbc")) 'd';
		}

		#[test]
		fn test_enumerated_captures() {
			use example_captures::root_types::bc as BC;
			assert_eq!(match_example("abcd").unwrap().bc, BC::None);
			assert_eq!(match_example("abbcccd").unwrap().bc, BC::b2c3("bbccc"));
			assert_eq!(match_example("abbbcd").unwrap().bc, BC::b3c1("bbbc"));
		}
	}
}
#[test]
fn abc() {
	assert!(matches!("abc": str, 'a' 'b' 'c'));
	assert!(!matches!("abc": str, !'a' "bc"));
	let pat = "abc";
	let value = String::from("abcabcabc");
	assert!(matches!(value.as_str(): str, pat[3]));

	assert_eq!(try_match!("abc": str, 'a' 'b' 'c'), Ok("abc"));
	assert!(try_match!("abd": str, 'a' 'b' 'c').is_err());
	let pat = "bc";
	let value = "abcdef";
	let matched = try_match!(value: str, 'a' (bc = pat) 'd' (e = 'e') 'f').unwrap();
	assert!(matched.matched == "abcdef" && matched.bc == "bc" && matched.e == "e");
}
