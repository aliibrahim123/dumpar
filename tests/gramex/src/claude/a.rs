//! Test suite for the `gramex!` macro itself (not the hand-written trait API).
//!
//! Drop this at `lib/tests/gramex_tests.rs` and run `cargo test`.
//!
//! Every matcher here (`digit`, `word`, `pair`, ...) is produced by a single
//! `gramex!` invocation below, so every test exercises the macro's codegen
//! in `generate.rs` end to end — literals, ranges, or/and/imply, every
//! repetition form, `!`, groups, and all four capture shapes (plain,
//! atomic, struct, enum), including one that specifically forces
//! `gen_capture_normal`'s missing `__start` and `gen_and`'s offset bugs to
//! matter (`and_from_offset`, `word` after a non-trivial prefix).
//!
//! CAVEAT: I could not `cargo build`/`cargo test` this in the sandbox (no
//! Cargo.toml was supplied and the proc-macro depends on an internal
//! `chunked_quote` crate I don't have source for). I traced the grammar in
//! `parse.rs`/`capture.rs` token-by-token to get the syntax right, and in
//! particular caught that:
//!   - the *paren* capture form needs `(name = expr)`, not `(name: atom)`
//!     — `:` after a paren-captured name is parsed as an explicit *type*,
//!     not "just match this atom".
//!   - the generated `mod captures { ... }` is not `pub`, so struct/enum
//!     field and variant access only type-checks from inside `mod grammar`
//!     itself — that's why those tests live next to the macro invocation
//!     instead of in the top-level test functions.
//! Please run this first and send me anything that doesn't compile.

mod grammar {
	use gramex::{Matcher, general::pos, result::Expected};

	gramex::grammar! { for str;
		// -- plain matchers --------------------------------------------
		let digit = '0'..'9';
		let alpha = 'a'..'z' | 'A'..'Z';
		let alnum = digit | alpha;
		let hello = "hello";
		let hello_or_world = "hello" | "world";
		let greet = "hi " alpha+;

		// -- repetition ---------------------------------------------------
		let opt_digit = digit?;
		let star_digits = digit*;
		let plus_digits = digit+;
		let exact3 = digit[3];
		let range2to4 = digit[2..4];
		let atleast2 = digit[2..];

		// -- not / group / and / imply -------------------------------------
		let not_digit = !digit;
		let group_alnum = (digit | alpha)+;
		let and_combo = (digit+) & (alnum+);
		let and_from_offset = "!!" (digit+ & alnum+);
		let imply_combo = digit -> alpha;

		// -- captures -------------------------------------------------------
		let cap_simple = d:digit;             // atomic: single atom, captured once
		let word: struct = (w = alpha+);      // normal: repeated span, needs __start
		let pair: struct = (a = digit) (b = digit);
		let choice: enum = (x = digit) | (y = alpha);
		let digit_count: struct = (len: usize = digit+ => len.len());

		let g: (usize, usize) = start:pos b: 'a' a:'c' d:'d' end:pos => (start, end);
		let h = ((('a' | ('b')? | 'c' -> 'd' | 'g'? 't')) & 'a');
	}

	// ---- tests that need the private `captures` module -----------------

	#[test]
	fn a() {
		let mut off = 0usize;
		let res = g.parse("acd", &mut off).unwrap();
		assert_eq!(res.0, 0);
		assert_eq!(res.1, 3);

		assert_eq!(opt_digit.expected(), Expected::None);
		assert_eq!(h.expected().to_string(), "one of \"a\", \"t\"")
	}

	#[test]
	fn word_struct_captures_full_repeated_span() {
		let mut off = 0usize;
		let w = word.parse("abcXYZ 123", &mut off).unwrap();
		assert_eq!(w.w, "abcXYZ");
		assert_eq!(off, 6);
	}

	#[test]
	fn word_struct_captures_span_after_nonzero_start() {
		// "word" run directly at offset 0 wouldn't distinguish the
		// gen_capture_normal missing-`__start` bug from a correct slice
		// that just happened to start at 0. Verify it also works when
		// wrapped so the capture starts mid-string.
		let combined = "xx".to_string() + "hello world";
		let mut off = 0usize;
		assert!("xx".test(&combined, &mut off));
		let w = word.parse(&combined, &mut off).unwrap();
		assert_eq!(w.w, "hello");
	}

	#[test]
	fn word_struct_fails_when_no_alpha_present() {
		let mut off = 0usize;
		assert!(word.parse("123", &mut off).is_err());
	}

	#[test]
	fn pair_struct_captures_both_fields_in_order() {
		let mut off = 0usize;
		let p = pair.parse("12", &mut off).unwrap();
		assert_eq!(p.a, "1");
		assert_eq!(p.b, "2");
		assert_eq!(off, 2);
	}

	#[test]
	fn pair_struct_fails_when_second_field_missing() {
		let mut off = 0usize;
		assert!(pair.parse("1", &mut off).is_err());
	}

	#[test]
	fn pair_struct_fails_when_first_field_missing() {
		let mut off = 0usize;
		assert!(pair.parse("a1", &mut off).is_err());
	}

	#[test]
	fn choice_enum_selects_x_branch() {
		let mut off = 0usize;
		match choice.parse("9", &mut off).unwrap() {
			Choice::X(v) => assert_eq!(v, "9"),
			Choice::Y(_) => panic!("wrong variant selected"),
		}
		assert_eq!(off, 1);
	}

	#[test]
	fn choice_enum_selects_y_branch() {
		let mut off = 0usize;
		match choice.parse("q", &mut off).unwrap() {
			Choice::Y(v) => assert_eq!(v, "q"),
			Choice::X(_) => panic!("wrong variant selected"),
		}
	}

	#[test]
	fn choice_enum_fails_when_neither_branch_matches() {
		let mut off = 0usize;
		assert!(choice.parse("!", &mut off).is_err());
	}

	#[test]
	fn cap_simple_captures_exactly_one_char_atomically() {
		let mut off = 0usize;
		let (d,) = cap_simple.parse("7up", &mut off).unwrap();
		assert_eq!(d, "7");
		assert_eq!(off, 1);
	}

	#[test]
	fn digit_count_capture_applies_map_to_correct_type() {
		let mut off = 0usize;
		let c = digit_count.parse("98765rest", &mut off).unwrap();
		assert_eq!(c.len, 5usize);
		assert_eq!(off, 5);
	}

	#[test]
	fn digit_count_capture_fails_with_no_digits() {
		let mut off = 0usize;
		assert!(digit_count.parse("rest", &mut off).is_err());
	}
}

use std::assert_matches;

use gramex::{
	Matcher,
	general::list,
	result::{MatchError, MatchErrorKind},
};
use grammar::*;

// =============================================================================
// Plain literal / range / or matchers
// =============================================================================

macro_rules! macro_case {
	($name:ident, $matcher:expr, $input:expr, $should_match:expr, $end_off:expr) => {
		#[test]
		fn $name() {
			let mut off = 0usize;
			let ok = $matcher.test($input, &mut off);
			assert_eq!(ok, $should_match, "test() mismatch in {}", stringify!($name));
			assert_eq!(off, $end_off, "offset mismatch in {}", stringify!($name));
		}
	};
}

macro_case!(hello_matches_prefix, hello, "hello world", true, 5);
macro_case!(hello_mismatch, hello, "goodbye", false, 0);
macro_case!(hello_incomplete_input, hello, "hel", false, 0);
macro_case!(hello_empty_input, hello, "", false, 0);

macro_case!(or_matches_first_branch, hello_or_world, "hello!", true, 5);
macro_case!(or_matches_second_branch, hello_or_world, "world!", true, 5);
macro_case!(or_matches_neither_branch, hello_or_world, "nope", false, 0);
macro_case!(or_empty_input, hello_or_world, "", false, 0);

macro_case!(digit_matches_zero, digit, "0abc", true, 1);
macro_case!(digit_matches_nine, digit, "9abc", true, 1);
macro_case!(digit_rejects_letter, digit, "a", false, 0);
macro_case!(digit_rejects_empty, digit, "", false, 0);

macro_case!(alpha_matches_lower, alpha, "m", true, 1);
macro_case!(alpha_matches_upper, alpha, "M", true, 1);
macro_case!(alpha_rejects_digit, alpha, "5", false, 0);

macro_case!(alnum_matches_digit, alnum, "5x", true, 1);
macro_case!(alnum_matches_letter, alnum, "x5", true, 1);
macro_case!(alnum_rejects_symbol, alnum, "!x", false, 0);

macro_case!(greet_sequence_matches, greet, "hi there", true, 8);
macro_case!(greet_sequence_needs_prefix, greet, "there", false, 0);
macro_case!(greet_sequence_needs_alpha_after_prefix, greet, "hi 123", false, 3);

// =============================================================================
// Repetition operators
// =============================================================================

macro_case!(opt_matches_present_digit, opt_digit, "5x", true, 1);
macro_case!(opt_matches_zero_digits, opt_digit, "xx", true, 0);
macro_case!(opt_matches_zero_on_empty, opt_digit, "", true, 0);

macro_case!(star_matches_multiple, star_digits, "12345abc", true, 5);
macro_case!(star_matches_zero, star_digits, "abc", true, 0);
macro_case!(star_matches_zero_on_empty, star_digits, "", true, 0);
macro_case!(star_matches_all_digits_no_trailer, star_digits, "999", true, 3);

macro_case!(plus_matches_multiple, plus_digits, "123abc", true, 3);
macro_case!(plus_rejects_zero, plus_digits, "abc", false, 0);
macro_case!(plus_rejects_empty, plus_digits, "", false, 0);
macro_case!(plus_matches_single, plus_digits, "5x", true, 1);

macro_case!(exact3_matches_exactly, exact3, "123456", true, 3);
macro_case!(exact3_rejects_too_few, exact3, "12", false, 2);
macro_case!(exact3_rejects_none, exact3, "abc", false, 0);
macro_case!(exact3_matches_when_input_is_exactly_three, exact3, "789", true, 3);

macro_case!(range2to4_takes_max_available_up_to_four, range2to4, "123456", true, 4);
macro_case!(range2to4_matches_at_minimum, range2to4, "12abc", true, 2);
macro_case!(range2to4_rejects_below_min, range2to4, "1abc", false, 1);
macro_case!(range2to4_rejects_zero, range2to4, "abc", false, 0);

macro_case!(atleast2_consumes_all_available, atleast2, "123456", true, 6);
macro_case!(atleast2_matches_at_minimum, atleast2, "12x", true, 2);
macro_case!(atleast2_rejects_below_min, atleast2, "1x", false, 1);
macro_case!(atleast2_rejects_zero, atleast2, "x", false, 0);

// =============================================================================
// not / group / and / imply
// =============================================================================

macro_case!(not_digit_matches_letter, not_digit, "x", true, 1);
macro_case!(not_digit_matches_symbol, not_digit, "!", true, 1);
macro_case!(not_digit_rejects_digit, not_digit, "5", false, 0);
macro_case!(not_digit_rejects_on_empty_input, not_digit, "", false, 0);

macro_case!(group_alnum_matches_run, group_alnum, "a1b2!", true, 4);
macro_case!(group_alnum_stops_at_symbol, group_alnum, "!abc", false, 0);
macro_case!(group_alnum_matches_single_char, group_alnum, "z", true, 1);

macro_case!(and_combo_matches_all_digit_run, and_combo, "123 ", true, 3);
macro_case!(and_combo_rejects_no_digits, and_combo, "abc", false, 0);

macro_case!(and_from_offset_matches_after_prefix, and_from_offset, "!!42rest", true, 4);
macro_case!(and_from_offset_requires_prefix, and_from_offset, "42rest", false, 0);
macro_case!(
	and_from_offset_requires_digits_after_prefix,
	and_from_offset,
	"!!rest",
	false,
	2
);

macro_case!(imply_vacuous_when_no_condition, imply_combo, "xyz", true, 0);
macro_case!(imply_matches_condition_then_consequent, imply_combo, "5az", true, 2);
macro_case!(
	imply_fails_when_condition_true_but_consequent_false,
	imply_combo,
	"5!",
	false,
	1
);
macro_case!(imply_vacuous_on_empty_input, imply_combo, "", true, 0);

// -- extra boundary / edge coverage -----------------------------------------

macro_case!(digit_matches_boundary_zero_char, digit, "0", true, 1);
macro_case!(digit_matches_boundary_nine_char, digit, "9", true, 1);
macro_case!(digit_rejects_char_before_zero, digit, "/", false, 0); // '/' is '0'-1 in ASCII
macro_case!(digit_rejects_char_after_nine, digit, ":", false, 0); // ':' is '9'+1 in ASCII
macro_case!(alpha_rejects_char_before_a, alpha, "`", false, 0);
macro_case!(alpha_rejects_char_after_z, alpha, "{", false, 0);
macro_case!(alpha_rejects_char_before_upper_a, alpha, "@", false, 0);
macro_case!(alpha_rejects_char_after_upper_z, alpha, "[", false, 0);

macro_case!(hello_matches_when_input_is_only_pattern, hello, "hello", true, 5);
macro_case!(
	hello_or_world_matches_longer_input_than_either_word,
	hello_or_world,
	"helloworld",
	true,
	5
);
macro_case!(
	or_first_branch_preferred_when_both_could_apply,
	hello_or_world,
	"helloworld",
	true,
	5
);

macro_case!(star_matches_long_run, star_digits, "0123456789", true, 10);
macro_case!(plus_matches_long_run, plus_digits, "0123456789", true, 10);
macro_case!(exact3_matches_long_run_still_caps_at_three, exact3, "0123456789", true, 3);
macro_case!(range2to4_caps_at_four_on_long_run, range2to4, "0123456789", true, 4);
macro_case!(atleast2_consumes_whole_long_run, atleast2, "0123456789", true, 10);

macro_case!(not_digit_matches_space, not_digit, " x", true, 1);
macro_case!(group_alnum_rejects_empty_input, group_alnum, "", false, 0);
macro_case!(and_combo_rejects_empty_input, and_combo, "", false, 0);
macro_case!(and_from_offset_rejects_empty_input, and_from_offset, "", false, 0);

macro_case!(greet_rejects_partial_prefix, greet, "h", false, 0);
macro_case!(greet_rejects_prefix_no_space, greet, "hithere", false, 0);
macro_case!(greet_matches_single_letter_name, greet, "hi x", true, 4);

macro_case!(opt_digit_offset_stays_zero_on_symbol, opt_digit, "!x", true, 0);
macro_case!(imply_matches_single_digit_then_letter_pair, imply_combo, "1a1a", true, 2);

// =============================================================================
// Miscellaneous invariants
// =============================================================================

#[test]
fn offset_untouched_on_top_level_mismatch_from_nonzero_start() {
	let mut off = 3usize;
	assert!(!hello.test("xxxbye", &mut off));
	assert_eq!(off, 3);
}

#[test]
fn offset_advances_correctly_from_nonzero_start_on_success() {
	let mut off = 2usize;
	assert!(hello.test("xxhello!", &mut off));
	assert_eq!(off, 7);
}

#[test]
fn sequential_terms_compose_left_to_right() {
	let input = "hi there";
	let mut off = 0usize;
	assert!(greet.test(input, &mut off));
	assert_eq!(off, 8);
}

#[test]
fn check_returns_match_error_with_offset_on_failure() {
	let mut off = 0usize;
	let err = hello.check("nope", &mut off).unwrap_err();
	assert_eq!(err.off, 0);
}
use std::matches as omatches;
#[test]
fn incomplete_vs_mismatch_kind_for_literal() {
	let mut off = 0usize;
	let err_incomplete = hello.check("hel", &mut off).unwrap_err();
	assert!(omatches!(
		err_incomplete.kind,
		gramex::result::MatchErrorKind::InComplete(_)
	));

	let mut off2 = 0usize;
	let err_mismatch = hello.check("world", &mut off2).unwrap_err();
	assert!(omatches!(err_mismatch.kind, gramex::result::MatchErrorKind::MisMatch(_)));
}

// =============================================================================
// Fuzzers (deterministic, dependency-free PRNG) — every matcher exercised is
// the macro-generated one, run against hundreds of random inputs.
// =============================================================================

struct Rng(u32);
impl Rng {
	fn new(seed: u32) -> Self {
		Rng(seed | 1)
	}
	fn next_u32(&mut self) -> u32 {
		let mut x = self.0;
		x ^= x << 13;
		x ^= x >> 17;
		x ^= x << 5;
		self.0 = x;
		x
	}
	fn range(&mut self, lo: usize, hi: usize) -> usize {
		lo + (self.next_u32() as usize) % (hi - lo)
	}
	fn ascii_string(&mut self, max_len: usize) -> String {
		let alphabet: &[u8] = b"abcdefghij0123456789 -_!";
		let len = self.range(0, max_len + 1);
		(0..len).map(|_| alphabet[self.range(0, alphabet.len())] as char).collect()
	}
}

#[test]
fn fuzz_digit_matches_iff_leading_ascii_digit() {
	let mut rng = Rng::new(1);
	for _ in 0..500 {
		let s = rng.ascii_string(15);
		let mut off = 0usize;
		let matched = digit.test(&s, &mut off);
		let expected = s.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false);
		assert_eq!(matched, expected, "input={s:?}");
		assert_eq!(off, if matched { 1 } else { 0 });
	}
}

#[test]
fn fuzz_star_matches_maximal_leading_digit_run() {
	let mut rng = Rng::new(2);
	for _ in 0..500 {
		let s = rng.ascii_string(20);
		let mut off = 0usize;
		assert!(star_digits.test(&s, &mut off));
		let expected = s.chars().take_while(|c| c.is_ascii_digit()).count();
		assert_eq!(off, expected, "input={s:?}");
	}
}

#[test]
fn fuzz_plus_matches_iff_leading_digit_and_consumes_full_run() {
	let mut rng = Rng::new(3);
	for _ in 0..500 {
		let s = rng.ascii_string(20);
		let mut off = 0usize;
		let matched = plus_digits.test(&s, &mut off);
		let leading = s.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false);
		assert_eq!(matched, leading, "input={s:?}");
		let expected = if matched {
			s.chars().take_while(|c| c.is_ascii_digit()).count()
		} else {
			0
		};
		assert_eq!(off, expected, "input={s:?}");
	}
}

#[test]
fn fuzz_exact3_never_exceeds_or_undershoots_three() {
	let mut rng = Rng::new(4);
	for _ in 0..500 {
		let s = rng.ascii_string(10);
		let mut off = 0usize;
		let run = s.chars().take_while(|c| c.is_ascii_digit()).count();
		let matched = exact3.test(&s, &mut off);
		assert_eq!(matched, run >= 3, "input={s:?} run={run}");
		assert_eq!(off, usize::min(run, 3));
	}
}

#[test]
fn fuzz_range2to4_respects_both_bounds() {
	let mut rng = Rng::new(5);
	for _ in 0..500 {
		let s = rng.ascii_string(10);
		let mut off = 0usize;
		let run = s.chars().take_while(|c| c.is_ascii_digit()).count();
		let matched = range2to4.test(&s, &mut off);
		assert_eq!(matched, run >= 2, "input={s:?} run={run}");
		if matched {
			assert_eq!(off, run.min(4), "input={s:?} run={run}");
		}
	}
}

#[test]
fn fuzz_not_digit_matches_iff_leading_char_is_not_ascii_digit() {
	let mut rng = Rng::new(6);
	for _ in 0..500 {
		let s = rng.ascii_string(15);
		if s.is_empty() {
			continue;
		}
		let mut off = 0usize;
		let matched = not_digit.test(&s, &mut off);
		let leading_digit = s.chars().next().unwrap().is_ascii_digit();
		assert_eq!(matched, !leading_digit, "input={s:?}");
	}
}

#[test]
fn fuzz_or_matches_iff_one_branch_matches() {
	let mut rng = Rng::new(7);
	for _ in 0..500 {
		let s = rng.ascii_string(10);
		let mut off = 0usize;
		let matched = hello_or_world.test(&s, &mut off);
		let expected = s.starts_with("hello") || s.starts_with("world");
		assert_eq!(matched, expected, "input={s:?}");
	}
}

#[test]
fn fuzz_and_combo_matches_iff_leading_digit_run_nonempty() {
	// digit+ & alnum+: alnum+ is always at least as permissive as digit+,
	// so this should succeed exactly when there's at least one leading digit.
	let mut rng = Rng::new(8);
	for _ in 0..500 {
		let s = rng.ascii_string(15);
		let mut off = 0usize;
		let matched = and_combo.test(&s, &mut off);
		let leading_digit = s.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false);
		assert_eq!(matched, leading_digit, "input={s:?}");
	}
}

#[test]
fn fuzz_imply_never_fails_when_no_leading_digit() {
	let mut rng = Rng::new(9);
	for _ in 0..500 {
		let s = rng.ascii_string(15);
		let leading_digit = s.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false);
		if leading_digit {
			continue;
		}
		let mut off = 0usize;
		assert!(imply_combo.test(&s, &mut off), "input={s:?}");
		assert_eq!(off, 0);
	}
}

use gramex::{check, matcher, matches, parse, try_match};

// =============================================================================
// matches!
// =============================================================================

#[test]
fn matches_true_on_literal_sequence() {
	assert!(matches!("abc", 'a' 'b' 'c'));
}

#[test]
fn matches_false_on_mismatch() {
	assert!(!matches!("abx", 'a' 'b' 'c'));
}

#[test]
fn matches_on_string_literal() {
	assert!(matches!("hello world", "hello" " world"));
}

#[test]
fn matches_on_char_range() {
	assert!(matches!("5", '0'..'9'));
	assert!(!matches!("x", '0'..'9'));
}

#[test]
fn matches_on_or() {
	assert!(matches!("world", "hello" | "world"));
	assert!(matches!("hello", "hello" | "world"));
	assert!(!matches!("nope", "hello" | "world"));
}

#[test]
fn matches_on_repetition() {
	assert!(matches!("111", '1'+));
}

#[test]
fn matches_false_on_empty_required_repetition() {
	assert!(!matches!("", '1'+));
}

// =============================================================================
// check!
// =============================================================================

#[test]
fn check_ok_on_match() {
	assert!(check!("abc", "abc").is_ok());
}

#[test]
fn check_err_on_mismatch() {
	assert!(check!("xyz", "abc").is_err());
}

#[test]
fn check_err_on_short_input() {
	let err = check!("ab", "abc").unwrap_err();
	assert_matches!(err.kind, MatchErrorKind::InComplete(_));
}

// =============================================================================
// try_match!
// =============================================================================

#[test]
fn try_match_returns_some_with_captures() {
	let Some((a, c, d)) =
		try_match!(for str, "abca,a", a:'a' 'b'+ c?:'c' d:list<'a', ','>)
	else {
		panic!("expected a match");
	};
	assert_eq!(a, "a");
	assert_eq!(c, Some("c"));
	assert_eq!(d, vec!["a", "a"]);
}

#[test]
fn try_match_returns_none_on_failure() {
	assert!(try_match!("xyz", a:'a' 'b'+ c?:'c').is_none());
}

#[test]
fn try_match_optional_capture_absent_still_matches() {
	// no trailing 'c' in the input: whole thing still matches, c is None
	let Some((a, c)) = try_match!("ab", a:'a' 'b'+ c?:'c') else {
		panic!("expected a match even without the optional 'c'");
	};
	assert_eq!(a, "a");
	assert_eq!(c, None);
}

#[test]
fn try_match_repeated_capture_consumes_full_run() {
	let Some((a, c)) = try_match!("abbbbc", a:'a' 'b'+ c?:'c') else {
		panic!("expected a match");
	};
	assert_eq!(a, "a");
	assert_eq!(c, Some("c"));
}

#[test]
fn try_match_with_explicit_for_prefix() {
	// mirrors the example exactly
	let Some((a, c)) = try_match!(for str, "abc", a:'a' 'b'+ c?:'c' ) else {
		panic!("expected a match");
	};
	assert_eq!(a, "a");
	assert_eq!(c, Some("c"));
}

// =============================================================================
// parse!
// =============================================================================

#[test]
fn parse_ok_returns_captures() {
	let (a, c) = parse!("abc", a:'a' 'b'+ c?:'c').unwrap();
	assert_eq!(a, "a");
	assert_eq!(c, Some("c"));
}

#[test]
fn parse_err_on_mismatch() {
	assert!(parse!("xyz", a:'a' 'b'+ c?:'c').is_err());
}

#[test]
fn parse_err_carries_offset() {
	let err = parse!("ayz", a:'a' 'b'+ c?:'c').unwrap_err();
	assert_eq!(err.off, 1);
}

#[test]
fn parse_err_excess() {
	let a = &&&&Box::new("abcd".to_string());
	assert_matches!(
		parse!(a, "abc"),
		Err(MatchError { kind: MatchErrorKind::Excess, off: 3 })
	);
}

// =============================================================================
// matcher!
// =============================================================================

#[test]
fn matcher_value_test_method() {
	let m = matcher!(for str, "hello");
	let mut off = 0usize;
	assert!(m.test("hello world", &mut off));
	assert_eq!(off, 5);
}

#[test]
fn matcher_value_is_reusable_across_inputs() {
	let m = matcher!(for str, '0'..'9');
	let mut off1 = 0usize;
	let mut off2 = 0usize;
	assert!(m.test("5x", &mut off1));
	assert!(!m.test("x5", &mut off2));
}

#[test]
fn matcher_value_check_method_reports_mismatch_kind() {
	let m = matcher!(for str, "abc");
	let mut off = 0usize;
	let err = m.check("xyz", &mut off).unwrap_err();
	assert!(omatches!(err.kind, MatchErrorKind::MisMatch(_)));
}

#[test]
fn matcher_value_parse_matches_parse_macro() {
	// straight from the example: matcher!().parse(...) an, parse!(...)
	// should agree exactly, byte for byte.
	assert_eq!(
		parse!("abc", a:'a' 'b'+ c?:'c'),
		matcher!(for str, a:'a' 'b'+ c?:'c').parse("abc", &mut 0)
	);
}

#[test]
fn matcher_value_capture_matches_try_match_macro() {
	let m = matcher!(for str, a:'a' 'b'+ c?:'c');
	let mut off = 0usize;
	assert_eq!(try_match!("abbc", a:'a' 'b'+ c?:'c'), m.capture("abbc", &mut off));
}
