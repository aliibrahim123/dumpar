use gramex::{MatchAble, MatchBy, MatchSignal, MatchStatus, gramex, matcher, try_match};

#[derive(Debug, PartialEq)]
enum Token {
	Ident(String),
	Nb(i64),
}
#[derive(Debug, PartialEq)]
struct Tokens<'a>(&'a [Token]);
impl<'a> AsRef<Tokens<'a>> for Tokens<'a> {
	fn as_ref(&self) -> &Tokens<'a> {
		self
	}
}
impl MatchAble for Tokens<'_> {
	type Slice<'a>
		= Tokens<'a>
	where
		Self: 'a;
	fn len(&self) -> usize {
		self.0.len()
	}
	fn slice(&self, range: std::ops::Range<usize>) -> Tokens<'_> {
		Tokens(&self.0[range])
	}
	fn get_n(
		&self, ind: &mut usize, n: usize, _status: &gramex::MatchStatus,
	) -> Result<Tokens<'_>, MatchSignal> {
		if *ind + n > self.0.len() {
			Err(MatchSignal::InComplete)
		} else {
			*ind += n;
			Ok(Tokens(&self.0[*ind - n..*ind]))
		}
	}
}
impl MatchBy<&str> for Tokens<'_> {
	fn match_by(
		&self, matcher: &str, ind: &mut usize, _status: &gramex::MatchStatus,
	) -> MatchSignal {
		if *ind + 1 > self.0.len() {
			MatchSignal::InComplete
		} else if let Token::Ident(ident) = &self.0[*ind]
			&& ident == matcher
		{
			*ind += 1;
			MatchSignal::Matched
		} else {
			MatchSignal::MisMatched
		}
	}
}
impl MatchBy<i64> for Tokens<'_> {
	fn match_by(
		&self, matcher: i64, ind: &mut usize, _status: &gramex::MatchStatus,
	) -> MatchSignal {
		if *ind + 1 > self.0.len() {
			MatchSignal::InComplete
		} else if let Token::Nb(nb) = &self.0[*ind]
			&& *nb == matcher
		{
			*ind += 1;
			MatchSignal::Matched
		} else {
			MatchSignal::MisMatched
		}
	}
}
/*gramex! {
	for Tokens<'a> ;
	let v1 = v2 (a = 123);
	let v2 = "abc";
	let v3<a> = (v1 = v1) a _+;
	let v4 = "def";
}*/

#[test]
pub fn a() {
	gramex! {
		mod a for Tokens<'a> ;
		let v1 = v2 (a = 123);
		let v2 = "abc";
		let v3<a> = (v1 = v1) a _+;
		let v4 = "def";
	}
	let tokens = Tokens(&[Token::Nb(123), Token::Ident("abc".to_string()), Token::Nb(456)]);
	assert_eq!(tokens.len(), 3);
	assert_eq!(tokens.get_n(&mut 0, 1, &MatchStatus::default()), Ok(Tokens(&[Token::Nb(123)])));
	let tokens = Tokens(&[
		Token::Ident("abc".to_string()),
		Token::Nb(123),
		Token::Ident("def".to_string()),
		Token::Nb(456),
	]);
	println!("{:?}", try_match!(tokens: Tokens<'a>, "abc" (a = 123) _*));
	println!("{:?}", a::match_v3(&tokens, a::v4))
}
mod a {
	//! This test module is designed to be placed outside the `gramex` crate,
	//! such as in `tests/doc_examples.rs`.

	use gramex::{
		MatchAble, MatchBy, MatchResult, MatchSignal, MatchStatus, by, gramex, matcher,
		matcher_for, matches, try_match,
	};

	// ==========================================
	// Custom Type Setup (from doc comments)
	// ==========================================

	#[derive(Debug, PartialEq, Eq, Clone)]
	pub enum Token {
		Ident(String),
		Nb(i64),
	}

	pub struct Tokens<'a>(pub &'a [Token]);

	impl<'a> MatchAble for Tokens<'a> {
		type Slice<'b>
			= Tokens<'b>
		where
			Self: 'b;

		fn len(&self) -> usize {
			self.0.len()
		}

		fn slice(&self, range: std::ops::Range<usize>) -> Tokens<'_> {
			Tokens(&self.0[range])
		}
	}

	impl MatchBy<&str> for Tokens<'_> {
		fn match_by(&self, matcher: &str, ind: &mut usize, _status: &MatchStatus) -> MatchSignal {
			if *ind + 1 > self.0.len() {
				MatchSignal::InComplete
			} else if let Token::Ident(ident) = &self.0[*ind] {
				if ident == matcher {
					*ind += 1;
					MatchSignal::Matched
				} else {
					MatchSignal::MisMatched
				}
			} else {
				MatchSignal::MisMatched
			}
		}
	}

	// ==========================================
	// Tests for Doc Examples
	// ==========================================

	#[cfg(test)]
	mod gramex_doc_tests {
		use super::*;
		use gramex::matches;

		#[test]
		fn test_quick_guide() {
			// quick matching can be done using `matches` macro
			assert!(matches!("abc": str, "abc"));

			let pat = "abc";
			assert!(matches!("abc": str, pat));
			assert!(matches!("bc": str, { &pat[1..] }));

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
		}

		#[test]
		fn test_matches_macro() {
			assert!(matches!("abc": str, 'a' 'b' 'c'));
			assert!(!matches!("abd": str, 'a' 'b' 'c'));
			let pat = "abc";
			let value = String::from("abcabcabc");
			assert!(matches!(value: str, pat[3]));
		}

		#[test]
		fn test_try_match_macro() {
			assert_eq!(try_match!("abc": str, 'a' 'b' 'c'), Ok("abc"));
			assert!(try_match!("abd": str, 'a' 'b' 'c').is_err());
			let pat = "bc";
			let value = "abcdef";
			let matched = try_match!(value: str, 'a' (bc = pat) 'd' (e = 'e') 'f').unwrap();
			assert!(matched.matched == "abcdef" && matched.bc == "bc" && matched.e == "e");
		}

		const A: &str = "a";
		#[test]
		fn test_gramex_macro_local_and_header() {
			gramex! {
				mod matchers for str;
				let abc = A 'b' 'c';
			}
			assert!(matchers::match_abc("abc").is_ok());
		}

		#[test]
		fn test_gramex_macro_terms() {
			gramex! {
				mod matchers for str;
				let abc = 'a' 'b' 'c';
				let abcd = abc (d = 'd');
			}
			assert_eq!(matchers::match_abc("abc"), Ok("abc"));
			let matched = matchers::match_abcd("abcd").unwrap();
			assert!(matched.d == "d");
		}

		#[test]
		fn test_gramex_macro_capture_mapping() {
			gramex! {
				mod matchers for str;
				let ABC: String = 'a' 'b' 'c' => {|v| v.to_uppercase()};
			}
			// Note: Modified slightly from Ok("ABC") to Ok(String::from("ABC")) to align with mapped return types
			assert_eq!(matchers::match_ABC("abc"), Ok(String::from("ABC")));
		}

		#[test]
		fn test_gramex_macro_args() {
			gramex! {
				mod matchers for str;
				let mbm<a, c> = a 'b' c;
				let abc = mbm<'a', 'c'>;
			}
			assert_eq!(matchers::match_abc("abc"), Ok("abc"));
			assert_eq!(matchers::match_mbm("abc", matcher_for(&'a'), matcher_for(&'c')), Ok("abc"));
		}

		#[test]
		fn test_gramex_macro_generated_functions() {
			gramex! {
				mod matchers for str;
				let abc = 'a' 'b' 'c';
			}
			assert_eq!(matchers::match_abc("abc"), Ok("abc"));
			assert_eq!(matchers::abc("abc", &mut 0, &MatchStatus::default()), MatchSignal::Matched);
			assert_eq!(matchers::capture_abc("abc", &mut 0, &MatchStatus::default()), Ok("abc"));
		}

		#[test]
		fn test_matcher_macro() {
			assert!(matches("abc", matcher!(for str, 'a' 'b' 'c')));
			assert!(!matches("abd", matcher!(for str, 'a' 'b' 'c')));

			let pat = "abc";
			let value = String::from("abcabcabc");
			assert!(matches(value.as_str(), matcher!(for str, pat[3])));
		}

		#[test]
		fn test_custom_matchable_behavior() {
			let stream = vec![Token::Ident("hello".to_string()), Token::Ident("world".to_string())];
			let tokens = Tokens(&stream);

			let mut ind = 0;
			let status = MatchStatus::default();

			assert_eq!(tokens.match_by("hello", &mut ind, &status), MatchSignal::Matched);
			assert_eq!(tokens.match_by("world", &mut ind, &status), MatchSignal::Matched);
			assert_eq!(tokens.match_by("missing", &mut ind, &status), MatchSignal::InComplete);
		}

		#[test]
		fn test_functions_matches_matcher_for_by() {
			// `matcher_for` creates a matcher from matching value
			assert!(matches("abc", matcher_for("abc")));

			// `by` passes a closure
			assert!(matches("abc", by(|v: &str, i, s| v.match_by("abc", i, s))));
		}
	}
}
#[cfg(test)]
mod tests3 {
	use std::cell::Cell;

	// Assuming your crate is named 'gramex'
	use gramex::str::{
		a_char, alpha, alphanum, ascii, ascii_alpha, ascii_alphanum, ascii_control, ascii_lower,
		ascii_printable, ascii_punct, ascii_upper, ascii_ws, bin, control, dec, digit, hex,
		hex_lower, hex_upper, lower, num, octal, upper, ws,
	};
	use gramex::{
		MatchSignal, a, an, by, consume, list, matcher, matcher_for, matches, test, touch,
		try_match,
	};

	#[test]
	fn test_unicode_matchers() {
		assert!(matches!("abc": str, lower[3]));
		assert!(matches!("ABC": str, upper[3]));
		assert!(matches!("aB1": str, alpha alpha num));
		assert!(matches!("é": str, lower)); // Unicode check
		assert!(matches!("abc123": str, alphanum[6]));
		assert!(matches!(" \n\t": str, ws[3]));
		assert!(matches!("\u{0007}": str, control));
	}

	#[test]
	fn test_ascii_matchers() {
		assert!(matches!("abc": str, ascii[3]));
		assert!(matches!("abc": str, ascii_lower[3]));
		assert!(matches!("ABC": str, ascii_upper[3]));
		assert!(matches!("abc": str, ascii_alpha[3]));
		assert!(matches!("a1 ": str, ascii_alpha ascii_alphanum ascii_ws));
		assert!(matches!("\x01": str, ascii_control));
		assert!(matches!("a1!": str, ascii_printable[3]));
		assert!(matches!("!@#": str, ascii_punct[3]));
	}

	#[test]
	fn test_numeric_matchers() {
		assert!(matches!("123": str, dec[3]));
		assert!(matches!("1aF": str, hex[3]));
		assert!(matches!("1af": str, hex_lower[3]));
		assert!(matches!("1AF": str, hex_upper[3]));
		assert!(matches!("127": str, octal[3]));
		assert!(matches!("0101": str, bin[4]));
		// Custom radix: Base 4
		assert!(matches!("0123": str, {digit(4)}[4]));
	}

	#[test]
	fn test_predicate_and_ranges() {
		// Test a_char with custom predicate
		assert!(matches!("abc": str, {a_char(|c| c.is_ascii_lowercase())}[3]));

		// Test RangeInclusive support directly
		assert!(matches!("abc": str, ('a'..'z')[3]));
		assert!(matches!("123": str, ('0'..'9')[3]));
	}

	#[test]
	fn test_edge_cases() {
		// Incomplete input
		assert!(!matches!("ab": str, lower[3]));

		// Mismatched input
		assert!(!matches!("a1c": str, lower[3]));

		// Empty string
		assert!(!matches!("": str, lower));
	}

	#[test]
	fn test_utility_matchers() {
		// matcher_for
		assert!(matches("abc", matcher_for("abc")));

		// test (non-advancing predicate)
		assert!(matches!("abc": str, {test(|v: &str, _, _| v.starts_with('a'))} _+));

		// touch (side effects)
		let hit = Cell::new(false);
		assert!(matches!("a": str, {touch(|_, _, _| hit.set(true))} 'a'));
		assert!(hit.get());

		// a and an (token predicates)
		assert!(matches!("abc": str, {a(|v: &str| v.chars().all(char::is_lowercase))}[3]));
		assert!(matches!(&&&"abc": str, {an(3, |v| v == "abc")}));

		// by (type inference helper)
		assert!(matches!("": str, {by(|_, _, _| MatchSignal::Matched)}));
	}

	#[test]
	fn test_consume_interface() {
		// Basic split
		assert_eq!(consume("abcDEF", matcher!(for str, lower*)), Ok(("abc", "DEF")));

		// Failure case
		let res = consume("123", matcher!(for str, alpha+));
		assert!(res.is_err());
	}

	#[test]
	fn test_list() {
		assert!(matches!("a,b,c": str, list<'a'..'z', ','>));
		assert!(matches!("a,b,c,": str, list<'a'..'z', ','> ','));
		println!("{:?}", try_match!("a,b,c": str, list<'a'..'z', ','>));
	}
}
mod bit {
	use gramex::bits::{b1, b3};

	#[cfg(test)]
	mod tests {
		use gramex::bits::bytes_of;

		use super::*;
		use crate::{MatchSignal, MatchStatus, gramex, matches, try_match};

		#[test]
		fn test_match_single_and_ref_u8() {
			let input: &[u8] = b"hello";

			// MatchBy<u8>
			assert!(matches!(input:[u8], b'h' {b'e'} {b'l'} {b'l'} {b'o'}));

			// MatchBy<&u8>
			let byte_h = &b'h';
			assert!(matches!(input: [u8], {byte_h} {b'e'} _ _ {b'o'}));

			// Failing matches
			assert!(!matches!(input: [u8], {b'x'} _*));
			// Incomplete
			assert!(!matches!(b"h".as_ref(): [u8], {b'h'} {b'e'}));
		}

		#[test]
		fn test_match_slices_and_arrays() {
			let input: &[u8] = b"HTTP/1.1 200 OK\r\n";

			// MatchBy<&[u8]>
			assert!(matches!(input: [u8], b"HTTP/1.1" b" " b"200" b" OK\r\n"));

			// MatchBy<[u8; N]> and MatchBy<&[u8; N]>
			let arr1: [u8; 4] = *b"HTTP";
			let arr2: &[u8; 4] = b"/1.1";
			assert!(matches!(input: [u8], {arr1} {arr2} _+));

			// Excess checking (should fail if not matching exactly up to EOF)
			assert!(!matches!(input: [u8], b"HTTP/1.1 200 OK"));
		}

		#[test]
		fn test_match_vec() {
			let input: &[u8] = b"data\x00\x01\x02";
			let vec_matcher = vec![0x00, 0x01, 0x02];

			// MatchBy<&Vec<u8>>
			assert!(matches!(input: [u8], b"data" {&vec_matcher}));
		}

		#[test]
		fn test_ranges() {
			matches!(&[1,2,3][..]: [u8], 1 2 3);
			assert!(matches!(&b"abc"[..]: [u8], {bytes_of("abc")}));
			let input: &[u8] = b"aB9\x7F";

			// In gramex, `A..B` desugars to `RangeInclusive`
			assert!(matches!(input: [u8],
				b'a'..b'z'
				b'A'..b'Z'
				b'0'..b'9'
				{0x00u8}..{0x7Fu8}
			));

			// Out of bounds
			assert!(!matches!(b"0".as_ref(): [u8], b'a'..b'z'));
		}

		#[test]
		fn test_repetitions_and_logic() {
			let input: &[u8] = b"11122333";

			assert!(matches!(input: [u8],
				(b'0'..b'9')[3]   // Exactly 3 times
				b"22"?            // Optional
				b"3"+             // 1 or more times
			));

			// Alternation
			assert!(matches!(b"GET".as_ref(): [u8], b"GET" | b"POST" | b"PUT"));
			assert!(matches!(b"POST".as_ref(): [u8], b"GET" | b"POST" | b"PUT"));

			// Intersection / Boundaries
			// Match 'a' 3 times, AND make sure it's part of an "aaa" sequence
			assert!(matches!(b"aaa".as_ref(): [u8], (b'a'[3]) & b"aaa"));
			assert!(!matches!(b"aab".as_ref(): [u8], (b'a'[3]) & b"aaa"));
		}

		#[test]
		fn test_captures_try_match() {
			let input = b"User-Agent: curl/7.68.0\r\n";

			println!(
				"{:?}",
				try_match!(input: [u8], (header = (b'a'..b'z' | b'A'..b'Z' | b'-')+)
				b": " (value = (!b"\r\n")*) b"\r\n")
			);
			let result = try_match!(input: [u8],
				(header = (b'a'..b'z' | b'A'..b'Z' | b'-')+)
				b": "
				(value = (!b"\r\n")*)
				b"\r\n"
			)
			.unwrap();

			assert_eq!(result.header, b"User-Agent");
			assert_eq!(result.value, b"curl/7.68.0");
		}

		#[test]
		fn test_captures_with_mapping() {
			let input: &[u8] = b"Length: 42";

			let result = try_match!(input: [u8],
				b"Length: "
				// Capture bytes, map to UTF-8 str, then parse to u32
				(len: u32 = (b'0'..b'9')+ => { |v: &[u8]|
					std::str::from_utf8(v).unwrap().parse::<u32>().unwrap()
				})
			)
			.unwrap();

			assert_eq!(result.len, 42);
		}

		gramex! {
			mod byte_parsers for [u8];

			let hex_digit = b'0'..b'9' | b'a'..b'f' | b'A'..b'F';

			let color_code = b"#" (r = hex_digit[2]) (g = hex_digit[2]) (b = hex_digit[2]);

			let magic_header = b"\x7FELF" {1u8}..{2u8}; // ELF header + 32/64 class byte
		}

		#[test]
		fn test_gramex_generated_parsers() {
			assert!(byte_parsers::match_hex_digit(b"a").is_ok());
			assert!(byte_parsers::match_hex_digit(b"G").is_err());

			println!("{:?}", byte_parsers::match_color_code(b"#FF00aa"));
			let color = byte_parsers::match_color_code(b"#FF00aa").unwrap();
			assert_eq!(color.r, b"FF");
			assert_eq!(color.g, b"00");
			assert_eq!(color.b, b"aa");

			assert!(byte_parsers::match_magic_header(b"\x7FELF\x02").is_ok());
		}
	}
	#[cfg(test)]
	mod tests2 {
		use std::{
			borrow::Borrow,
			cell::{Cell, RefCell, UnsafeCell},
		};

		use gramex::{
			MatchAble, MatchSignal, MatchStatus, bits::*, gramex, matches, touch, try_match,
		};

		#[test]
		fn test_bits_basics_and_formatting() {
			assert!(matches!(LBits::from(b8(0x12)): LBits, {b4(0x2)} {b2(0b01)} {b2(0)}));
			let b = LBits::from(0b1011_u8);
			assert_eq!(b.len, 8);
			assert_eq!(b.value, 11);
			matches!([1,2,3]: [u8], 1 2 3);

			// Test the custom Binary format implementation
			let formatted = format!("{:b}", b4(0b1010));
			assert_eq!(formatted, "0b1010");

			// Test Into conversions
			let raw: u8 = b.try_into().unwrap();
			assert_eq!(raw, 11);
		}

		#[test]
		fn test_bit_extraction() {
			// Value: 0b11_010_110 (length: 8)
			let val = LBits::from(b8(0b11_010_110));

			// Extract bits 0 to 3 (exclusive): 0b110
			let sliced = val.slice(0..3);
			assert_eq!(sliced.value, 0b110);
			assert_eq!(sliced.len, 3);

			// Empty extraction
			let empty = val.slice(3..3);
			assert_eq!(empty.value, 0);
			assert_eq!(empty.len, 0);
		}

		#[test]
		fn test_bits_grammar_matching() {
			// Value: 0b_111_000_10 (Length: 8)
			// Remember: Reading LSB (right) to MSB (left)!
			// Index 0-1: 10   (Value 2)
			// Index 2-4: 000  (Value 0)
			// Index 5-7: 111  (Value 7)
			let val = LBits::from(b8(0b111_000_10));

			// Exact matching
			assert!(matches!(val: LBits, {b2(2)} {b3(0)} {b3(7)}));

			// Range matching using the new bNr macros
			assert!(matches!(val: LBits,
				{b2r(1..=3)} // 2 falls in 1..=3
				{b3(0)}      // Exact 0
				{b3r(5..=7)} // 7 falls in 5..=7
			));

			// Excess/Incomplete validation
			assert!(!matches!(val: LBits, {b2(2)} {b3(0)})); // Missing last 3 bits
			assert!(!matches!(LBits::from(b3(0b101)): LBits, {b4(0b101)})); // Over-reading 
		}

		#[test]
		fn test_w_macros_word_parsing() {
			// Let's create a 2-byte array (16 bits)
			// Byte 0: 0xAB -> 0b1010_1011
			// Byte 1: 0xCD -> 0b1100_1101
			let input = &[0xAB, 0xCD];
			let a = w1l!( {b4(0xB)} {b4(0xA)} );

			// --- Test 1: w1l! (Reading a single byte) ---
			// Byte 0 is 0xAB (1010_1011). Read LSB to MSB:
			// Bits 0-3: 1011 (11 / 0xB)
			// Bits 4-7: 1010 (10 / 0xA)
			assert!(matches!(input: [u8],
				{w1l!( {b4(0xB)} {b4(0xA)} )}
				{w1l!( {b4(0xD)} {b4(0xC)} )}
			));

			// --- Test 2: w2l! (Reading 16 bits) ---
			// Your code uses `from_le_bytes`, so it reads as Little Endian.
			// The u16 value is 0xCDAB.
			// In binary: 0b_1100_1101_1010_1011
			// Reading in chunks of 4 bits from right to left (LSB first):
			assert!(matches!(input: [u8],
				{w2l!(
					{b4(0xB)} // Bits 0-3
					{b4(0xA)} // Bits 4-7
					{b4(0xD)} // Bits 8-11
					{b4(0xC)} // Bits 12-15
				)}
			));
		}

		#[test]
		fn test_bit_alignment() {
			// Value: 0b_1010_01_11 (Length 8)
			let val = LBits::from(b8(0b1010_01_11));

			assert!(matches!(val: LBits,
				{b2(0b11)}
				// We have read 2 bits. We are at index 2.
				{alignedl(2)} // 2 is divisible by 2. Success!

				{b2(0b01)}
				// We have read 4 bits total. Index is 4.
				{alignedl(4)} // 4 is divisible by 4. Success!

				{b4(0b1010)}
			));

			// Failing alignment test
			assert!(!matches!(val: LBits,
				{b3(0b111)}
				{alignedl(2)} // Fails! We are at index 3, which is not divisible by 2.
				_*
			));
		}

		#[test]
		fn test_bits_captures() {
			let input = LBits::from(b8(0b1110_0101));

			// Extract sections of the byte using captures inside the word!
			let cap = try_match!(input: LBits,
				(lower_nibble = {b4r(0..=15)})
				(upper_nibble = {b4r(0..=15)})
			)
			.unwrap();

			assert_eq!(cap.lower_nibble.value, 0b0101);
			assert_eq!(cap.upper_nibble.value, 0b1110);
		}

		#[test]
		fn test_advanced_grammar_operators() {
			// Value: 0b_11_10_10_10_01_00 (Length 12)
			// LSB-first chunks of 2: 00 (0), 01 (1), 10 (2), 10 (2), 10 (2), 11 (3)
			let val = LBits::from(b12(0b_11_10_10_10_01_00));

			println!(
				"{:?}",
				try_match!(val: LBits, {b2(0)} {b2(1)} {b2(2)}[2..5] ~{b2(3)} !{b2(0)} {touch(|v: &LBits, i, _| println!("{:?}", v.slice(*i..v.len())))} )
			);
			assert!(matches!(val: LBits,
				{b2(0)}       // Match exactly 0
				{b2(1)}       // Match exactly 1
				{b2(2)}[2..5] // Repetition: Match '2' between 2 and 4 times (matches 3 times)
				~{b2(3)}      // Lookahead: Peek at the next 2 bits, ensure they are '3', without advancing
				!{b2(0)} _
			));
		}

		#[test]
		fn test_intersections_and_alternations() {
			// Value: 0b_1010_0110 (Length 8)
			// LSB chunks of 4: 0110 (6), 1010 (10)
			let val = LBits::from(b8(0b_1010_0110));

			assert!(matches!(val: LBits,
				({b4(5)} | {b4(6)} | {b4(7)}) // Alternation: First chunk must be 5, 6, or 7

				// Intersection: The next chunk must be any 4 bits, BUT it cannot be 0,
				// AND it must fall in the range 8..=15.
				({b4r(0..=15)} & !{b4(0)} & {b4r(8..=15)})
			));
		}

		#[derive(Debug, PartialEq)]
		enum Opcode {
			Read,
			Write,
			Delete,
			Unknown,
		}

		#[test]
		fn test_bits_capture_mapping_to_structs() {
			// Value: 0b_0010_0001 (Length 8)
			// LSB to MSB: 0001 (1), 0010 (2)
			let val = LBits::from(b8(0b_0010_0001));

			// We can capture raw bits and map them immediately into complex Rust enums/structs!
			let res = try_match!(val: LBits,
				(op1: Opcode = {b4r(0..=15)} => { |b: LBits| match b.value {
					1 => Opcode::Read,
					2 => Opcode::Write,
					3 => Opcode::Delete,
					_ => Opcode::Unknown
				}})
				(op2: Opcode = {b4r(0..=15)} => { |b: LBits| match b.value {
					1 => Opcode::Read,
					2 => Opcode::Write,
					3 => Opcode::Delete,
					_ => Opcode::Unknown
				}})
			)
			.unwrap();

			assert_eq!(res.op1, Opcode::Read);
			assert_eq!(res.op2, Opcode::Write);
		}

		#[test]
		fn test_massive_64bit_word_extraction() {
			// Simulate an 8-byte (64-bit) LE packet header
			// Byte order:[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]
			// Value in LE: 0x0807060504030201
			let input: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];

			assert!(matches!(input: [u8],
				{w8l!(
					// Unpack the 64-bit struct from LSB to MSB seamlessly
					{b8(0x01)}         // 1st byte
					{b8(0x02)}         // 2nd byte
					{b16(0x0403)}      // 3rd & 4th byte
					{b32(0x08070605)}  // Upper 32 bits (bytes 5-8)
				)}
			));
		}

		#[test]
		fn test_real_world_protocol_packet() {
			// Let's parse a packed 16-bit hardware/network header.
			// Layout (MSB to LSB):
			// [ Version: 3 bits ] [ Flags: 5 bits ][ Length: 8 bits ]
			// Let's set: Length=20, Flags=3 (00011), Version=4 (100)

			// Packing it into u16: 20 | (3 << 8) | (4 << 13) = 0x8314
			// LE Bytes array: [0x14, 0x83]
			let input = &LBits::from([0x14, 0x83]);

			let packet = try_match!(input: LBits,
					// Since bits are extracted LSB first, we extract the lowest bits (Length) first!
					(length = {b8r(0..=255)})
					(flags = {b5r(0..=31)})
					(version = {b3r(0..=7)})
			)
			.unwrap();

			assert_eq!(packet.length.value, 20);
			assert_eq!(packet.flags.value, 3);
			assert_eq!(packet.version.value, 4);
		}

		#[test]
		fn test_zero_length_edge_case() {
			// Ensure that 0-length repetition loops don't panic bitextract!
			let val = LBits::from(b8(0b_1111_1111));

			// Match `0` length bits by making something optional that fails
			assert!(matches!(val: LBits,
				{b4(0)}?  // This fails, so it consumes 0 bits.
				{b8(0xFF)} // Succeeds on the full byte
			));
		}

		// Use the gramex! macro to generate reusable Bit-level rules
		gramex! {
			mod bit_rules for LBits;

			// Define basic terms
			let bit_one = {b1(1)};
			let bit_zero = {b1(0)};

			// Define composite terms using sequences and repetitions
			let nibble_zero = bit_zero[4];

			// Match pattern: 1, 1, 0, 0, 1, 1, 1, 1
			// Reads as (LSB -> MSB): 1111_00_11 (0xF3)
			let custom_byte_pattern = bit_one[2] bit_zero[2] bit_one[4];
		}

		#[test]
		fn test_gramex_macro_term_reuse() {
			assert!(bit_rules::match_nibble_zero(&b4(0b0000).into()).is_ok());
			assert!(bit_rules::match_nibble_zero(&b4(0b0001).into()).is_err()); // Has a 1

			// Verify complex compiled pattern
			let val = &LBits::from(b8(0xF3)); // 0b1111_0011
			assert!(bit_rules::match_custom_byte_pattern(val).is_ok());
		}
		gramex! {
			pub mod bm for LBits;
			let header =
				(length  = {b8r(0..=255)})
				(flags   = {b5r(0..=31)})
				(version = {b3(4)});
			let routing =
				(src  = {b12r(0..=4095)})
				(dest = {b12r(0..=4095)});
		}
		#[test]
		fn test_insanely_complex_hybrid_protocol_frame() {
			// --- CONSTRUCTING THE PACKET ---
			// 1. Magic: "GMX!" ->[0x47, 0x4D, 0x58, 0x21]
			// 2. Header (16-bit LE): Length=5, Flags=21 (0b10101), Version=4 (0b100)
			//    Binary (MSB->LSB): 100_10101_00000101 = 0x9505 -> LE Bytes:[0x05, 0x95]
			// 3. Routing (24-bit LE): Src=0xABC, Dest=0xDEF
			//    Binary: 0xDEFABC -> LE Bytes:[0xBC, 0xFA, 0xDE]
			// 4. Tag: 0xAA
			// 5. Payload: "HELLO" ->[0x48, 0x45, 0x4C, 0x4C, 0x4F]
			// 6. Checksum: 0xFF -> [0xFF]
			let raw_data = b"\x47\x4D\x58\x21\x05\x95\xBC\xFA\xDE\xAAHELLO\xFF";

			// Use the custom `bytes_of` utility to coerce the fixed array reference into &[u8]
			let input = bytes_of(raw_data);

			let frame = try_match!(input: [u8],
				// 1. Exact Byte Slice Match
				(magic = b"GMX!")

				// 2. Bit-level Lookahead Validation + Bit Extraction Captures!
				//    First, lookahead `~` to ensure Version == 4 without consuming bytes.
				~{w2l!( {b8r(0..=255)} {b5r(0..=31)} {b3(4)} )}
				//    Then, strictly consume 2 bytes (`_[2]`) and extract the bits via `cw2l!`
				(header: [LBits; 3] = _[2] => { |v| {
					let v = LBits::try_from(v).unwrap();
					let v = bm::match_header(&v).unwrap();
					[v.length, v.flags, v.version]
				}})

				// 3. Odd-sized Bit Extraction (24 bits = 3 bytes) mapped to a custom struct type
				(routing: [LBits; 2] = _[3] => { |v| {
					let v = LBits::try_from(v).unwrap();
					let v = bm::match_routing(&v).unwrap();
					[v.src, v.dest]
				}})

				// 4. Alternation & Optional Option (`?`) on byte values
				(tag? = (b"\xAA" | b"\xBB"))

				// 5. Advanced Intersection & Negation on bytes
				//    Read bytes as long as they are NOT 0xFF: `(!b"\xFF")*`
				//    AND (`&`) ensure that exactly matched slice equals "HELLO"
				(payload = (!b"\xFF")* & b"HELLO")

				// 6. Map Block on Bit Captures
				//    Capture 1 byte, map the Bits object directly into a standard u8
				(checksum: u8 = _ => { |v| v[0] })
			)
			.expect("Massive hybrid parser failed!");

			// --- ASSERTIONS TO PROVE IT WORKED FLAWLESSLY ---

			// 1. Magic bytes
			assert_eq!(frame.magic, b"GMX!");

			// 2. Header bits (Safely unpacked from the 16-bit word!)
			assert_eq!(frame.header[0].value, 5);
			assert_eq!(frame.header[1].value, 21);
			assert_eq!(frame.header[2].value, 4);

			// 3. Routing bits (Safely unpacked from the 24-bit word!)
			assert_eq!(frame.routing[0].value, 0xABC);
			assert_eq!(frame.routing[1].value, 0xDEF);

			// 4. Tag correctly captured the 0xAA optional byte
			assert_eq!(frame.tag, Some(&b"\xAA"[..])); // Note: Option<&[u8]>

			// 5. Payload intersection extracted "HELLO" successfully
			assert_eq!(frame.payload, b"HELLO");

			// 6. Checksum mapped directly to a primitive `u8`
			assert_eq!(frame.checksum, 0xFF);
		}
	}
	#[cfg(test)]
	mod tests_bits {
		use std::{
			borrow::Borrow,
			cell::{Cell, RefCell, UnsafeCell},
		};

		use gramex::{
			MatchAble, MatchSignal, MatchStatus, bits::bytes_of, bits::*, gramex, matches, touch,
			try_match,
		};

		#[test]
		fn test_bits_basics_and_formatting() {
			// Value: 0x12 -> 0b0001_0010
			// Reading MSB (Left) to LSB (Right):
			// Bits 0-3: 0001 (1)
			// Bits 4-5: 00 (0)
			// Bits 6-7: 10 (2)
			assert!(matches!(Bits::from(b8(0x12)): Bits, {b4(1)} {b2(0)} {b2(2)}));

			let b = Bits::from(0b1011_u8);
			assert_eq!(b.len, 8);
			assert_eq!(b.value, 11);
			matches!([1,2,3]: [u8], 1 2 3);

			// Test the custom Binary format implementation
			let formatted = format!("{:b}", b4(0b1010));
			assert_eq!(formatted, "0b1010");

			// Test Into conversions
			let raw: u8 = b.try_into().unwrap();
			assert_eq!(raw, 11);
		}

		#[test]
		fn test_bit_extraction() {
			// Value: 0b11_010_110 (length: 8)
			let val = Bits::from(b8(0b11_010_110));

			// Extract bits 0 to 3 (exclusive). MSB first means the TOP 3 bits: 0b110
			let sliced = val.slice(0..3);
			assert_eq!(sliced.value, 0b110);
			assert_eq!(sliced.len, 3);

			// Empty extraction
			let empty = val.slice(3..3);
			assert_eq!(empty.value, 0);
			assert_eq!(empty.len, 0);
		}

		#[test]
		fn test_bits_grammar_matching() {
			// Value: 0b_111_000_10 (Length: 8)
			// Remember: Reading MSB (left) to LSB (right)!
			// Index 0-2: 111  (Value 7)
			// Index 3-5: 000  (Value 0)
			// Index 6-7: 10   (Value 2)
			let val = b8(0b111_000_10);

			// Exact matching
			assert!(matches!(val: Bits, {b3(7)} {b3(0)} {b2(2)}));

			// Range matching using the bNr macros
			assert!(matches!(val: Bits,
				{b3r(5..=7)} // 7 falls in 5..=7
				{b3(0)}      // Exact 0
				{b2r(1..=3)} // 2 falls in 1..=3
			));

			// Excess/Incomplete validation
			assert!(!matches!(val: Bits, {b3(7)} {b3(0)})); // Missing last 2 bits
			assert!(!matches!(Bits::from(b3(0b101)): Bits, {b4(0b101)})); // Over-reading 
		}

		#[test]
		fn test_w_macros_word_parsing() {
			// Let's create a 2-byte array (16 bits)
			// Byte 0: 0xAB -> 0b1010_1011
			// Byte 1: 0xCD -> 0b1100_1101
			let input = &[0xAB, 0xCD];

			// --- Test 1: w1! (Reading a single byte) ---
			// Byte 0 is 0xAB (1010_1011). Read MSB to LSB:
			// Bits 0-3: 1010 (10 / 0xA)
			// Bits 4-7: 1011 (11 / 0xB)
			assert!(matches!(input: [u8],
				{w1!( {b4(0xA)} {b4(0xB)} )}
				{w1!( {b4(0xC)} {b4(0xD)} )}
			));

			// --- Test 2: w2! (Reading 16 bits) ---
			// Your code builds `u64` using `from_le_bytes`, so it reads as Little Endian.
			// The u16 value is 0xCDAB.
			// In binary: 0b_1100_1101_1010_1011
			// Reading in chunks of 4 bits from left to right (MSB first):
			assert!(matches!(input: [u8],
				{w2!(
					{b4(0xC)} // Bits 0-3
					{b4(0xD)} // Bits 4-7
					{b4(0xA)} // Bits 8-11
					{b4(0xB)} // Bits 12-15
				)}
			));
		}

		#[test]
		fn test_bit_alignment() {
			// Value: 0b_1010_01_11 (Length 8)
			let val = Bits::from(b8(0b1010_01_11));

			assert!(matches!(val: Bits,
				{b4(0b1010)}
				// We have read 4 bits. We are at index 4.
				{aligned(4)} // 4 is divisible by 4. Success!

				{b2(0b01)}
				// We have read 6 bits total. Index is 6.
				{aligned(2)} // 6 is divisible by 2. Success!

				{b2(0b11)}
			));

			// Failing alignment test
			assert!(!matches!(val: Bits,
				{b3(0b101)}
				{aligned(2)} // Fails! We are at index 3, which is not divisible by 2.
				_*
			));
		}

		#[test]
		fn test_bits_captures() {
			let input = Bits::from(b8(0b1110_0101));

			// Extract sections of the byte using captures! (MSB first means upper nibble is read first)
			let cap = try_match!(input: Bits,
				(upper_nibble = {b4r(0..=15)})
				(lower_nibble = {b4r(0..=15)})
			)
			.unwrap();

			assert_eq!(cap.upper_nibble.value, 0b1110);
			assert_eq!(cap.lower_nibble.value, 0b0101);
		}

		#[test]
		fn test_advanced_grammar_operators() {
			// Value: 0b_11_10_10_10_01_00 (Length 12)
			// MSB-first chunks of 2: 11 (3), 10 (2), 10 (2), 10 (2), 01 (1), 00 (0)
			let val = Bits::from(b12(0b_11_10_10_10_01_00));
			assert!(matches!(val: Bits,
				{b2(3)}       // Match exactly 3
				{b2(2)}[2..5] // Repetition: Match '2' between 2 and 4 times (matches 3 times)
				~{b2(1)}      // Lookahead: Peek at the next 2 bits, ensure they are '1', without advancing
				!{b2(0)}      // Negation: Ensure the next is NOT '0'
				_[3]             // Wildcard: Match the remaining bits (which is 1 then 0)
			));
		}

		#[test]
		fn test_intersections_and_alternations() {
			// Value: 0b_1010_0110 (Length 8)
			// MSB chunks of 4: 1010 (10), 0110 (6)
			let val = Bits::from(b8(0b_1010_0110));

			assert!(matches!(val: Bits,
				({b4(9)} | {b4(10)} | {b4(11)}) // Alternation: First chunk must be 9, 10, or 11

				// Intersection: The next chunk must be any 4 bits, BUT it cannot be 0,
				// AND it must fall in the range 1..=8.
				({b4r(0..=15)} & !{b4(0)} & {b4r(1..=8)})
			));
		}

		#[derive(Debug, PartialEq)]
		enum Opcode {
			Read,
			Write,
			Delete,
			Unknown,
		}

		#[test]
		fn test_bits_capture_mapping_to_structs() {
			// Value: 0b_0010_0001 (Length 8)
			// MSB to LSB: 0010 (2), 0001 (1)
			let val = Bits::from(b8(0b_0010_0001));

			// We can capture raw bits and map them immediately into complex Rust enums/structs!
			let res = try_match!(val: Bits,
				(op1: Opcode = {b4r(0..=15)} => { |b: Bits| match b.value {
					1 => Opcode::Read,
					2 => Opcode::Write,
					3 => Opcode::Delete,
					_ => Opcode::Unknown
				}})
				(op2: Opcode = {b4r(0..=15)} => { |b: Bits| match b.value {
					1 => Opcode::Read,
					2 => Opcode::Write,
					3 => Opcode::Delete,
					_ => Opcode::Unknown
				}})
			)
			.unwrap();

			// op1 reads the upper MSB nibble (2)
			assert_eq!(res.op1, Opcode::Write);
			// op2 reads the lower LSB nibble (1)
			assert_eq!(res.op2, Opcode::Read);
		}

		#[test]
		fn test_massive_64bit_word_extraction() {
			// Simulate an 8-byte (64-bit) LE packet header
			// Byte order:[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]
			// Value in LE: 0x0807060504030201
			let input: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];

			assert!(matches!(input: [u8],
				{w8!(
					// Unpack the 64-bit struct from MSB to LSB seamlessly!
					// Because LE parses as 0x0807060504030201, the MSB is 0x08.
					{b8(0x08)}         // 1st byte (MSB of the u64)
					{b8(0x07)}         // 2nd byte
					{b16(0x0605)}      // 3rd & 4th byte
					{b32(0x04030201)}  // Lower 32 bits
				)}
			));
		}

		#[test]
		fn test_real_world_protocol_packet() {
			// Layout (MSB to LSB):
			// [ Version: 3 bits ] [ Flags: 5 bits ][ Length: 8 bits ]
			// Let's set: Version=4 (100), Flags=3 (00011), Length=20

			// Packing it into u16 MSB first: (4 << 13) | (3 << 8) | 20 = 0x8314
			// Since your w2! macro builds the u64 via from_le_bytes, we provide the LE bytes for 0x8314
			// LE Bytes array:[0x14, 0x83]
			let input = &Bits::from([0x14, 0x83]);

			let packet = try_match!(input: Bits,
					// Extracted MSB first!
					(version = {b3r(0..=7)})
					(flags = {b5r(0..=31)})
					(length = {b8r(0..=255)})
			)
			.unwrap();

			assert_eq!(packet.version.value, 4);
			assert_eq!(packet.flags.value, 3);
			assert_eq!(packet.length.value, 20);
		}

		#[test]
		fn test_zero_length_edge_case() {
			let val = Bits::from(b8(0b_1111_1111));

			// Match `0` length bits by making something optional that fails
			assert!(matches!(val: Bits,
				{b4(0)}?   // This fails, so it consumes 0 bits.
				{b8(0xFF)} // Succeeds on the full byte
			));
		}

		// Use the gramex! macro to generate reusable Bit-level rules
		gramex! {
			mod bit_rules for Bits;

			// Define basic terms
			let bit_one = {b1(1)};
			let bit_zero = {b1(0)};

			// Define composite terms using sequences and repetitions
			let nibble_zero = bit_zero[4];

			// Match pattern: 1, 1, 0, 0, 1, 1, 1, 1
			// Reads as (MSB -> LSB): 1100_1111 (0xCF)
			let custom_byte_pattern = bit_one[2] bit_zero[2] bit_one[4];
		}

		#[test]
		fn test_gramex_macro_term_reuse() {
			println!("{:?}", try_match!(Bits::new(0x12u64, 5): Bits, {b5(0x12)} _*));
			assert!(
				matches!([1,2,3]: [u8], {word(3, gramex::matcher!(for Bits, {b8(3)} {b8(2)} {b8(1)}))})
			);
			assert!(matches!(b8(0x12): Bits, {b4(1)} {aligned(4)} {b4(2)}));
			assert!(bit_rules::match_nibble_zero(&b4(0b0000).into()).is_ok());
			assert!(bit_rules::match_nibble_zero(&b4(0b1000).into()).is_err()); // Has a 1 at MSB

			// Verify complex compiled pattern
			let val = &Bits::from(b8(0xCF)); // 0b1100_1111
			assert!(bit_rules::match_custom_byte_pattern(val).is_ok());
		}

		gramex! {
			pub mod bm for Bits;
			let header =
				(version = {b3(4)})
				(flags   = {b5r(0..=31)})
				(length  = {b8r(0..=255)});
			let routing =
				(src  = {b12r(0..=4095)})
				(dest = {b12r(0..=4095)});
		}

		#[test]
		fn test_insanely_complex_hybrid_protocol_frame() {
			// --- CONSTRUCTING THE PACKET ---
			// 1. Magic: "GMX!" ->[0x47, 0x4D, 0x58, 0x21]
			// 2. Header (16-bit LE): Version=4 (0b100), Flags=21 (0b10101), Length=5
			//    Binary (MSB->LSB): 100_10101_00000101 = 0x9505 -> LE Bytes:[0x05, 0x95]
			// 3. Routing (24-bit LE): Src=0xABC, Dest=0xDEF
			//    Binary MSB->LSB: 0xABCDEF -> LE Bytes:[0xEF, 0xCD, 0xAB]
			// 4. Tag: 0xAA
			// 5. Payload: "HELLO" ->[0x48, 0x45, 0x4C, 0x4C, 0x4F]
			// 6. Checksum: 0xFF -> [0xFF]
			let raw_data = b"\x47\x4D\x58\x21\x05\x95\xEF\xCD\xAB\xAAHELLO\xFF";

			// Use the custom `bytes_of` utility to coerce the fixed array reference into &[u8]
			let input = bytes_of(raw_data);

			let frame = try_match!(input: [u8],
				// 1. Exact Byte Slice Match
				(magic = b"GMX!")

				// 2. Bit-level Lookahead Validation + Bit Extraction Captures!
				//    Lookahead `~` to ensure Version == 4 without consuming bytes.
				~{w2!( {b3(4)} {b5r(0..=31)} {b8r(0..=255)} )}
				//    Then, strictly consume 2 bytes (`_[2]`) and extract the bits
				(header: [Bits; 3] = _[2] => { |v| {
					let v = Bits::try_from(v).unwrap();
					let v = bm::match_header(&v).unwrap();[v.version, v.flags, v.length]
				}})

				// 3. Odd-sized Bit Extraction (24 bits = 3 bytes) mapped to a custom struct type
				(routing: [Bits; 2] = _[3] => { |v| {
					let v = Bits::try_from(v).unwrap();
					let v = bm::match_routing(&v).unwrap();[v.src, v.dest]
				}})

				// 4. Alternation & Optional Option (`?`) on byte values
				(tag? = (b"\xAA" | b"\xBB"))

				// 5. Advanced Intersection & Negation on bytes
				(payload = (!b"\xFF")* & b"HELLO")

				// 6. Map Block on Bit Captures
				(checksum: u8 = _ => { |v| v[0] })
			)
			.expect("Massive hybrid parser failed!");

			// --- ASSERTIONS TO PROVE IT WORKED FLAWLESSLY ---

			// 1. Magic bytes
			assert_eq!(frame.magic, b"GMX!");

			// 2. Header bits
			assert_eq!(frame.header[0].value, 4);
			assert_eq!(frame.header[1].value, 21);
			assert_eq!(frame.header[2].value, 5);

			// 3. Routing bits
			assert_eq!(frame.routing[0].value, 0xABC);
			assert_eq!(frame.routing[1].value, 0xDEF);

			// 4. Tag correctly captured the 0xAA optional byte
			assert_eq!(frame.tag, Some(&b"\xAA"[..]));

			// 5. Payload intersection extracted "HELLO" successfully
			assert_eq!(frame.payload, b"HELLO");

			// 6. Checksum mapped directly to a primitive `u8`
			assert_eq!(frame.checksum, 0xFF);
		}
		#[test]
		fn a() {
			let bits = Bits::from(0x123u32);
			assert_eq!(bits, Bits { value: 0x123, len: 32 });
			assert_eq!(bits.try_into(), Ok(0x123i32));
			let bits = Bits::from([0x12, 0x34]);
			assert_eq!(bits, Bits { value: 0x3412, len: 16 });
			assert_eq!(bits.try_into(), Ok([0x12, 0x34]));
			assert!(matches!(bits: Bits, {b8(0x34)} {b8(0x12)}));
		}
	}
}
