// Paste this in a file like `tests/macro_test.rs` or at the bottom of a relevant module.

#[cfg(test)]
mod tests {
	#![allow(nonstandard_style, unused_variables, dead_code, unused_imports)]

	use gramex::result::MatchResult;
	use gramex::{MatchAble, Matcher, Mode, general::MatchFn, gramex, result::*};
	use std::borrow::Cow;
	use std::fmt::Debug;

	// =========================================================================
	// CUSTOM MATCH ABLE IMPLEMENTATION FOR TESTING
	// =========================================================================

	#[derive(Debug, PartialEq, Clone, Copy)]
	pub enum Token {
		Num(u32),
		Plus,
		Minus,
	}
	#[derive(Debug, PartialEq, Clone)]
	pub struct Tokens(Vec<Token>);

	impl MatchAble for Tokens {
		type Slice<'a> = &'a [Token];
		type Token<'src> = Token;

		fn len(&self) -> usize {
			self.0.len()
		}
		fn get_token<'src>(&'src self, off: usize) -> Option<Self::Token<'src>> {
			self.0.get(off).copied()
		}
		fn slice<'a>(&'a self, range: std::ops::Range<usize>) -> Option<Self::Slice<'a>> {
			self.0.get(range)
		}
	}

	struct MatchExact(Token);
	impl Matcher<Tokens> for MatchExact {
		type Capture<'a> = Token;
		fn do_match<'a, M: Mode>(
			&self, matched: &'a Tokens, off: &mut usize,
		) -> MatchResult<Self::Capture<'a>, M> {
			if *off < matched.len() && matched.0[*off] == self.0 {
				*off += 1;
				M::ok(|| matched.0[*off - 1])
			} else {
				M::err(|| {
					MatchError::mismatch(
						Expected::A(format!("{:?}", self.0).into()),
						*off,
					)
				})
			}
		}
	}

	// =========================================================================
	// MASSIVE GRAMEX MACRO DEFINITIONS
	// =========================================================================
	#[derive(Debug)]
	pub enum A<'a> {
		None,
		UI(UI<'a>),
		Def(&'a str),
	}
	impl<'a> PartialEq for A<'a> {
		fn eq(&self, other: &Self) -> bool {
			match (self, other) {
				(A::None, A::None) => true,
				(A::UI(a), A::UI(b)) => a.0 == b.0,
				(A::Def(a), A::Def(b)) => a == b,
				_ => false,
			}
		}
	}
	gramex!(for str; let g = 'a' g_wrap<'a'> ;);
	gramex! {
		for str;

		// ---------------------------------------------------------
		// 1. Basic Literals and Matchers
		// ---------------------------------------------------------
		let t_lit = "hello";
		let t_char = 'x';
		let t_any = _;
		let t_range = '0'..'9';
		let t_block = { String::from("dyn") } "amic";

		let t_empty = "";
		let t_unicode = "🌟";

		// ---------------------------------------------------------
		// 2. Logic: Sequence, Or, And, Imply
		// ---------------------------------------------------------
		let t_seq = "a" "b" "c";
		let t_or = "a" | "b" | "c";
		let t_or_long = "foo" | "bar" | "baz" | "qux";

		// And (&): Both sides must match the same slice of input.
		let t_and_1 = ("a" | "ab") & _[1];
		let t_and_2 = ("ab" | "a") & _[2];
		let t_and_3 = ("ab" "c" | "a" "b") & ("a" "b");

		// Imply (->): If left matches, right MUST match.
		let t_imply_1 = "a" -> "b";
		let t_imply_2 = "a" -> "b" | "a" "c";
		let t_imply_deep = "a" -> ("b" -> "c");

		// ---------------------------------------------------------
		// 3. Modifiers: Not (!) and Near (~)
		// ---------------------------------------------------------
		let t_not_a = !"a";
		let t_near_a = ~"a" _;
		let t_near_not_a = !~"a" _;
		let t_near_not_seq = !~("a" "b") _;

		// ---------------------------------------------------------
		// 4. Repetitions
		// ---------------------------------------------------------
		let t_opt = "a"?;
		let t_star = "a"*;
		let t_plus = "a"+;
		let t_exact = "a"[3];
		let t_min_max = "a"[2..4];
		let t_min_only = "a"[2..];
		let t_max_only = "a"[..3];
		let t_rep_any = _[2..4];
		let t_star_ab: Vec<&'src str> = a*:"ab" => a;

		// ---------------------------------------------------------
		// 5. Captures (Inline, Tuple, Struct, Enum)
		// ---------------------------------------------------------
		let c_inline = a: "x";
		let c_tuple = a: "x" b: "y";

		// Generates `CStruct`
		let c_struct: struct = a: "x" b: "y";

		// Generates `CEnum`
		let c_enum: enum = first_val: "1" | second_val: "2";
		let c_enum_with_none: enum = some_val: "1" | "2"; // Second branch has no capture

		// Generates explicitly named struct and enum
		let s_named: struct NamedStruct = val_a: "a" val_b: "b";
		let e_named: enum NamedEnum = opt_a: "A" | opt_b: "B";

		let c_opt: Option<&'src str> = a?: "x" => a;
		let c_star: Vec<&'src str> = a*: "x" => a;
		let c_plus: Vec<&'src str> = a+: "x" => a;
		let c_opt_vec: Vec<&'src str> = ((a+ = "x"))? => a;

		// ---------------------------------------------------------
		// 6. Mappings
		// ---------------------------------------------------------
		let m_root: u32 = a: "123" => a.parse().unwrap();
		let m_inline = (a: u32 = "10" => a.parse().unwrap());
		let m_struct: struct =
			(x: u32 = "1" => 1)
			(y: u32 = "2" => 2);
		let m_string: String = "abc";

		let big_struct: struct =
			(id: u32 = ('0'..'9')+ => id.parse().unwrap())
			":"
			(name: String = !";"+)
			";";

		// ---------------------------------------------------------
		// 7. Generics
		// ---------------------------------------------------------
		let g_wrap<T> = "(" T ")";
		let g_use = g_wrap<"a">;
		let g_double = g_wrap<g_wrap<"a">>;
		let g_complex = g_wrap<for str: "a" | "b">;

		// ---------------------------------------------------------
		// 8. JSON-like full parser components (Depth restricted)
		// ---------------------------------------------------------
		let ws = (' ' | '\n' | '\r' | '\t')*;

		let j_null: String = a:"null" => "null".to_string();
		let j_bool: bool = ("" tru: "true" | fal: "false") => tru.map(|_| true).unwrap_or(false);

		let j_escape = '\\' ('"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't' | ('u' ('0'..'9' | 'a'..'f' | 'A'..'F')[4]));
		let j_char = j_escape | !'"';
		let j_str: String = '"' chars*: j_char '"' => chars.join("");

		let digit = '0'..'9';
		let j_num: f64 = (n = '-'? ('0' | '1'..'9' digit*) ('.' digit+)? ('e' ('+' | '-')? digit+)?) => n.parse().unwrap();

		// Nested array support (depth 3 max for test)
		let j_arr3: Vec<String> = '[' ws (elems* = e: j_str ws (',' ws)? => e) ']' => elems;
		let j_arr2: Vec<Vec<String>> = '[' ws (elems* = e: j_arr3 ws (',' ws)? => e) ']' => elems;
		let j_arr1: Vec<Vec<Vec<String>>> = '[' ws (elems* = e: j_arr2 ws (',' ws)? => e) ']' => elems;

		let j_object: Vec<(String, f64)> =
			'{' ws (pairs* = k: j_str ws ':' ws v: j_num ws (',' ws)? => (k, v)) '}' => pairs;
		let test3 = "a";
		let test4<a>: A::<'src> = "abc" | a -> (UI: struct = digit+) | def:"def";
	}

	mod tokens {
		use super::{MatchExact, Token, Tokens};
		use gramex::*;
		gramex! {
			for Tokens;

			let tok_any = _;
			let tok_plus: Token = { MatchExact(Token::Plus) };
			let tok_minus: Token = { MatchExact(Token::Minus) };
			let tok_num: u32 = n: _ => {
				if let Token::Num(v) = n[0] { v } else { panic!() }
			};

			// Parse: Any Token -> Plus -> Any Token
			let tok_expr = tok_any tok_plus tok_any;

			// Parse: Num -> Plus -> Num -> compute addition!
			let tok_add: u32 = a: tok_num tok_plus b: tok_num => a + b;
		}
	}

	// =========================================================================
	// TESTING INFRASTRUCTURE
	// =========================================================================

	macro_rules! pass {
		($matcher:expr, $input:expr, $len:expr, $expected_cap:expr) => {
			let mut off = 0;
			assert!($matcher.test($input, &mut off), "test failed for {:?}", $input);
			assert_eq!(off, $len, "test len mismatch for {:?}", $input);

			let mut off2 = 0;
			assert!($matcher.check($input, &mut off2).is_ok(), "check failed");
			assert_eq!(off2, $len, "check len mismatch");

			let mut off3 = 0;
			let cap = $matcher.capture($input, &mut off3).expect("capture failed");
			assert_eq!(off3, $len, "capture len mismatch");

			let mut off4 = 0;
			let parse_res = $matcher.parse($input, &mut off4).expect("parse failed");
			assert_eq!(parse_res, cap, "parse != capture for {:?}", $input);
			assert_eq!(parse_res, $expected_cap, "capture mismatch for {:?}", $input);
			assert_eq!(off4, $len, "parse len mismatch");
		};
	}

	macro_rules! pass_custom {
		($matcher:expr, $input:expr, $len:expr, |$cap:ident| $verify:block) => {
			let mut off = 0;
			assert!($matcher.test($input, &mut off), "test failed for {:?}", $input);
			assert_eq!(off, $len, "test len mismatch for {:?}", $input);

			let mut off3 = 0;
			let $cap = $matcher.parse($input, &mut off3).expect("parse failed");
			$verify
		};
	}

	macro_rules! fail {
		($matcher:expr, $input:expr, $err_off:expr, $err_check:expr) => {
			let mut off = 0;
			assert!(
				!$matcher.test($input, &mut off),
				"test should have failed: {:?}",
				$input
			);

			let mut off2 = 0;
			let err = $matcher.check($input, &mut off2).expect_err("check should fail");
			assert_eq!(err.off, $err_off, "err off mismatch for {:?}", $input);
			assert!(
				$err_check(&err.kind),
				"err kind mismatch for {:?}: {:?}",
				$input,
				err.kind
			);

			let mut off3 = 0;
			assert!($matcher.parse($input, &mut off3).is_err(), "parse should fail");
		};
	}

	fn is_mismatch(k: &MatchErrorKind) -> bool {
		matches!(k, MatchErrorKind::MisMatch(_))
	}
	fn is_incomplete(k: &MatchErrorKind) -> bool {
		matches!(k, MatchErrorKind::InComplete(_))
	}

	// =========================================================================
	// TEST SUITES
	// =========================================================================

	#[test]
	fn test_01_core_literals_and_ranges() {
		// String Literals
		pass!(t_lit, "hello", 5, "hello");
		pass!(t_lit, "hello world", 5, "hello");
		fail!(t_lit, "hell", 0, is_incomplete);
		fail!(t_lit, "xello", 0, is_mismatch);
		pass!(t_empty, "", 0, "");
		pass!(t_unicode, "🌟", 4, "🌟");

		// Char Literals
		pass!(t_char, "x", 1, "x");
		fail!(t_char, "y", 0, is_mismatch);

		// Any (_)
		pass!(t_any, "a", 1, "a");
		pass!(t_any, "🌟", 4, "🌟");
		fail!(t_any, "", 0, is_incomplete);

		// Ranges
		pass!(t_range, "5", 1, "5");
		pass!(t_range, "0", 1, "0");
		pass!(t_range, "9", 1, "9");
		fail!(t_range, "a", 0, is_mismatch);

		// Matcher Blocks
		pass!(t_block, "dynamic", 7, "dynamic");
		fail!(t_block, "dyno", 3, is_incomplete);
	}

	#[test]
	fn test_02_logic_combinators() {
		// Sequence
		pass!(t_seq, "abc", 3, "abc");
		fail!(t_seq, "ab", 2, is_incomplete);
		fail!(t_seq, "abd", 2, is_mismatch);

		// Or
		pass!(t_or, "a", 1, "a");
		pass!(t_or, "b", 1, "b");
		fail!(t_or, "d", 0, is_mismatch);
		pass!(t_or_long, "baz", 3, "baz");

		// And (Intersection)
		// t_and_1: ("a" | "ab") & _[1]
		pass!(t_and_1, "a", 1, "a");
		pass!(t_and_1, "ab", 1, "a"); // `_[1]` on "ab" produces length 1!

		// t_and_2: ("ab" | "a") & _[2]
		pass!(t_and_2, "ab", 2, "ab");
		fail!(t_and_2, "a", 0, is_incomplete); // _[2] fails on "a"

		// t_and_3: ("ab" "c" | "a" "b") & ("a" "b")
		pass!(t_and_3, "ab", 2, "ab");
		pass!(t_and_3, "abc", 3, "abc"); // Second condition fails

		// Imply (->)
		// t_imply_1: "a" -> "b"
		pass!(t_imply_1, "ab", 2, "ab");
		pass!(t_imply_1, "c", 0, ""); // "a" fails, returns Ok(0)
		fail!(t_imply_1, "ac", 1, is_mismatch); // "a" matches, "b" fails -> Error

		// t_imply_2: "a" -> "b" | "a" "c"
		pass!(t_imply_2, "ab", 2, "ab");
		fail!(t_imply_2, "ac", 1, is_mismatch); // Commits to first branch, then fails.

		// t_imply_deep: "a" -> ("b" -> "c")
		pass!(t_imply_deep, "abc", 3, "abc");
		pass!(t_imply_deep, "a", 1, "a"); // "b" fails, so "b"->"c" returns Ok(0)
		fail!(t_imply_deep, "ab", 2, is_incomplete); // "b" matches, "c" fails -> Error
	}

	#[test]
	fn test_03_modifiers_not_and_near() {
		// Not (!)
		pass!(t_not_a, "b", 1, "b");
		fail!(t_not_a, "a", 0, is_mismatch);
		fail!(t_not_a, "", 0, is_incomplete);

		// Near (~)
		pass!(t_near_a, "a", 1, "a"); // ~"a" succeeds, resets offset. _ matches "a"
		fail!(t_near_a, "b", 0, is_mismatch);

		// Near Not (!~)
		pass!(t_near_not_a, "b", 1, "b"); // !~"a" succeeds, resets offset, _ matches "b"
		fail!(t_near_not_a, "a", 0, is_mismatch);

		// Near Not Sequence (!~("a" "b") _)
		pass!(t_near_not_seq, "ac", 1, "a"); // "ab" fails, so !~ succeeds, _ matches "a"
		pass!(t_near_not_seq, "a", 1, "a");
		fail!(t_near_not_seq, "ab", 0, is_mismatch);
	}

	#[test]
	fn test_04_repetitions() {
		// ? (Optional)
		pass!(t_opt, "a", 1, "a");
		pass!(t_opt, "", 0, "");

		// * (Many Optional)
		pass!(t_star, "", 0, "");
		pass!(t_star, "a", 1, "a");
		pass!(t_star, "aaa", 3, "aaa");

		// + (Plus One)
		fail!(t_plus, "", 0, is_incomplete);
		pass!(t_plus, "a", 1, "a");
		pass!(t_plus, "aaa", 3, "aaa");

		// [3] (Exact)
		fail!(t_exact, "aa", 2, is_incomplete);
		pass!(t_exact, "aaa", 3, "aaa");
		pass!(t_exact, "aaaa", 3, "aaa");

		// [2..4] (Min Max)
		fail!(t_min_max, "a", 1, is_incomplete);
		pass!(t_min_max, "aa", 2, "aa");
		pass!(t_min_max, "aaa", 3, "aaa");
		pass!(t_min_max, "aaaa", 4, "aaaa");
		pass!(t_min_max, "aaaaa", 4, "aaaa");

		// [2..] (Min only)
		fail!(t_min_only, "a", 1, is_incomplete);
		pass!(t_min_only, "aa", 2, "aa");
		pass!(t_min_only, "aaaaaa", 6, "aaaaaa");

		// [..3] (Max only)
		pass!(t_max_only, "", 0, "");
		pass!(t_max_only, "a", 1, "a");
		pass!(t_max_only, "aaa", 3, "aaa");
		pass!(t_max_only, "aaaa", 3, "aaa"); // consumes 3, stops

		// Any Repetition (_[2..4])
		pass!(t_rep_any, "xy", 2, "xy");
		pass!(t_rep_any, "xyzw", 4, "xyzw");
		fail!(t_rep_any, "x", 1, is_incomplete);
	}

	#[test]
	fn test_05_captures_structs_enums() {
		// Inline Capture
		pass!(c_inline, "x", 1, ("x",));

		// Tuple Capture
		pass!(c_tuple, "xy", 2, ("x", "y"));
		// Struct Capture
		pass_custom!(c_struct, "xy", 2, |cap| {
			assert_eq!(cap.a, "x");
			assert_eq!(cap.b, "y");
		});

		// Named Struct Capture
		pass_custom!(s_named, "ab", 2, |cap| {
			let s: NamedStruct = cap;
			assert_eq!(s.val_a, "a");
			assert_eq!(s.val_b, "b");
		});

		// Enum Capture
		pass_custom!(c_enum, "1", 1, |cap| {
			assert!(matches!(cap, CEnum::FirstVal("1")));
		});
		pass_custom!(c_enum, "2", 1, |cap| {
			assert!(matches!(cap, CEnum::SecondVal("2")));
		});

		pass_custom!(c_enum_with_none, "2", 1, |cap| {
			assert!(matches!(cap, CEnumWithNone::None));
		});

		// Named Enum Capture
		pass_custom!(e_named, "B", 1, |cap| {
			let e: NamedEnum = cap;
			assert!(matches!(e, NamedEnum::OptB("B")));
		});

		// Optional Capture
		pass!(c_opt, "x", 1, Some("x"));
		pass!(c_opt, "", 0, None);

		// Vector Captures
		pass!(c_star, "xx", 2, vec!["x", "x"]);
		pass!(c_star, "", 0, Vec::<&str>::new());
		println!("---abc");
		pass!(c_plus, "x", 1, vec!["x"]);

		// Optional Vector Capture (a: "x"+)? => a
		pass!(c_opt_vec, "xx", 2, vec!["x", "x"]);
		pass!(c_opt_vec, "", 0, Vec::<&str>::new());

		pass!(test4(test3), "abc", 3, A::None);
		fail!(test4(test3), "ab", 1, is_mismatch);
		pass!(test4(test3), "a123", 4, A::UI(UI("123")));
		pass!(test4(test3), "def", 3, A::Def("def"));
	}

	#[test]
	fn test_06_mappings() {
		// Root mapping
		pass!(m_root, "123", 3, 123u32);

		// Inline mapping
		pass!(m_inline, "10", 2, (10u32,));

		// Struct mapping fields
		pass_custom!(m_struct, "12", 2, |cap| {
			assert_eq!(cap.x, 1);
			assert_eq!(cap.y, 2);
		});

		// String conversion
		pass!(m_string, "abc", 3, "abc".to_string());

		// Big Struct with mappings
		pass_custom!(big_struct, "404:not found;", 14, |cap| {
			assert_eq!(cap.id, 404);
			assert_eq!(cap.name, "not found");
		});
	}

	#[test]
	fn test_07_generics() {
		pass!(g_use, "(a)", 3, "(a)");
		pass!(g_double, "((a))", 5, "((a))");
		pass!(g_complex, "(b)", 3, "(b)");
	}

	#[test]
	fn test_08_errors_and_formats() {
		// Check exact error format strings for `Mismatch` and `Incomplete`
		let mut off = 0;
		let err = t_lit.check("wrong", &mut off).unwrap_err();
		let err_str = err.to_string();
		assert!(err_str.contains("mismatch"), "Actual error: {}", err_str);

		let mut off2 = 0;
		let err2 = t_range.check("a", &mut off2).unwrap_err();
		assert_eq!(err2.to_string(), "mismatch at 0, expected between \"0\" and \"9\"");

		let mut off3 = 0;
		let err3 = t_or.check("d", &mut off3).unwrap_err();
		assert_eq!(
			err3.to_string(),
			"mismatch at 0, expected one of \"a\", \"b\", \"c\""
		);

		let mut off3 = 0;
		let err3 = t_not_a.check("a", &mut off3).unwrap_err();
		assert_eq!(err3.to_string(), "mismatch at 0, expected not \"a\"");

		let mut off3 = 0;
		let err3 = t_any.check("", &mut off3).unwrap_err();
		assert_eq!(err3.to_string(), "incomplete input at 0, expected something");

		let off3 = 0;
		assert_eq!(g_complex.expected().to_string(), "");
	}

	#[test]
	fn test_09_custom_match_able_tokens() {
		use tokens::*;
		let input = Tokens(vec![Token::Num(10), Token::Plus, Token::Num(32)]);

		// tok_any should match one token
		let mut off = 0;
		assert!(tok_any.test(&input, &mut off));
		assert_eq!(off, 1);

		// tok_expr should match 3 tokens
		let mut off = 0;
		assert!(tok_expr.test(&input, &mut off));
		assert_eq!(off, 3);

		// tok_add should correctly parse and map to 42
		let mut off = 0;
		let res = tok_add.parse(&input, &mut off).unwrap();
		assert_eq!(res, 42);
		assert_eq!(off, 3);

		let fail_input = Tokens(vec![Token::Num(10), Token::Minus, Token::Num(32)]);
		let mut off = 0;
		assert!(!tok_add.test(&fail_input, &mut off));
	}

	#[test]
	fn test_10_json_parser_fuzzing_and_depth() {
		// Primitives
		pass!(j_null, "null", 4, "null".to_string());
		pass!(j_bool, "true", 4, true);
		pass!(j_bool, "false", 5, false);
		pass!(j_num, "-123.45e+6", 10, -123.45e6);
		pass!(j_str, "\"hello\\nworld\"", 14, "hello\\nworld".to_string());

		// Arrays (Up to depth 3)
		let deep_arr = "[[[\"a\", \"b\"], [\"c\"]]]";
		println!("{:?}", j_arr1.parse(deep_arr, &mut 0));
		pass_custom!(j_arr1, deep_arr, deep_arr.len(), |cap| {
			assert_eq!(cap.len(), 1);
			assert_eq!(cap[0].len(), 2);
			assert_eq!(cap[0][0][0], "a");
			assert_eq!(cap[0][0][1], "b");
			assert_eq!(cap[0][1][0], "c");
		});

		// Objects
		let obj = "{\n  \"key1\": 10,\n  \"key2\": -3.14\n}";
		pass_custom!(j_object, obj, obj.len(), |cap| {
			assert_eq!(cap.len(), 2);
			assert_eq!(cap[0].0, "key1");
			assert_eq!(cap[0].1, 10.0);
			assert_eq!(cap[1].0, "key2");
			assert_eq!(cap[1].1, -3.14);
		});

		// Loop array of varied good objects to ensure no panic
		let good_objs =
			["{}", "{ }", "{\"a\": 1}", "{\"a\": 1, \"b\": -2.5}", "{\t\"space\" : 0\n}"];
		for o in good_objs {
			let mut off = 0;
			let _ = j_object.parse(o, &mut off).unwrap();
		}

		// Loop array of bad objects
		let bad_objs = ["{", "{ \"a\" }", "{\"a\": }", "{ 1: 2 }"];
		for o in bad_objs {
			let mut off = 0;
			assert!(j_object.parse(o, &mut off).is_err(), "should fail: {}", o);
		}
	}

	#[test]
	fn test_11_fuzz_massive_repetition_safety() {
		// Create an extremely long input string to ensure no stack overflow on `+` and `*`
		let mut big = String::with_capacity(200_000);
		for _ in 0..100_000 {
			big.push_str("ab");
		}

		// test_star_ab matches "ab"*
		pass!(t_star_ab, &big, 200_000, vec!["ab"; 100_000]);

		// Because gramex expands repetitions into loops (not recursion), this proves
		// the macro handles arbitrarily deep flat repetitions perfectly.
	}

	// You can inject dynamically defined matching behavior effortlessly
	fn custom_fn<'a>(matched: &'a str, off: &mut usize) -> Option<&'a str> {
		if matched[*off..].starts_with("magic") {
			*off += 5;
			Some("magic")
		} else {
			None
		}
	}

	gramex! {
		for str;
		let rule = { MatchFn::new(custom_fn) } " wand";
	}
	#[test]
	fn test_12_dynamic_match_fn() {
		let mut off = 0;
		let res = rule.parse("magic wand", &mut off).unwrap();
		// Since `rule` is an un-typed inherited capture containing no internal explicit captures,
		// the `default_cap` applies and it grabs the matched slice.
		assert_eq!(res, "magic wand");
		assert_eq!(off, 10);
	}
}
