use gramex::str::{alpha, dec};
use gramex::{
	Matcher, derive::derive_enum_matcher, derive_slice_matchable, result::MatchErrorKind,
};

#[derive_enum_matcher(for S<'slice>, field = .kind)]
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

#[derive(Debug, PartialEq, PartialOrd)]
struct T<'src> {
	kind: A<'src>,
}
impl<'src> Into<T<'src>> for A<'src> {
	fn into(self) -> T<'src> {
		T { kind: self }
	}
}
#[derive(Debug)]
struct S<'src>(&'src [T<'src>]);
derive_slice_matchable!(S(&[T<'slice>]), matchers: WithRange, expected: None);

// ---------------------------------------------------------------------
// U(&'src str) - newtype variant: matcher captures the inner field
// ---------------------------------------------------------------------

#[test]
fn u_matches_and_extracts_the_newtype_inner_str() {
	let data = [A::U("hi", 0).into(), A::D.into()];
	let s = S(&data);
	let mut off = 0;
	let cap = u.capture(&s, &mut off).unwrap();
	assert_eq!(cap, (&"hi", &0));
	assert_eq!(off, 1);
}

#[test]
fn u_mismatches_on_a_different_variant_and_leaves_offset() {
	let data = [A::D.into()];
	let s = S(&data);
	let mut off = 0;
	assert!(u.check(&s, &mut off).is_err());
	assert_eq!(off, 0);
}

// ---------------------------------------------------------------------
// D - plain unit variant: matcher captures the whole token
// ---------------------------------------------------------------------

#[test]
fn d_matches_and_returns_the_whole_token() {
	let data = [A::D.into()];
	let s = S(&data);
	let mut off = 0;
	let cap = d.capture(&s, &mut off).unwrap();
	assert_eq!(*cap, A::D.into());
	assert_eq!(off, 1);
}

#[test]
fn d_mismatches_on_a_different_variant() {
	let data = [A::U("x", 0).into()];
	let s = S(&data);
	let mut off = 0;
	assert!(d.check(&s, &mut off).is_err());
}

// ---------------------------------------------------------------------
// F = 100 - explicit-discriminant unit variant
// ---------------------------------------------------------------------

#[test]
fn f_matches_the_explicit_discriminant_variant() {
	let data = [A::F.into()];
	let s = S(&data);
	let mut off = 0;
	let cap = f.capture(&s, &mut off).unwrap();
	assert_eq!(*cap, A::F.into());
	assert_eq!(off, 1);
}

// ---------------------------------------------------------------------
// T { a: u16, b: u8 } - struct variant: matcher returns the whole token
// ---------------------------------------------------------------------

#[test]
fn t_matches_struct_variant_regardless_of_field_values() {
	let data = [A::T { a: 7, b: 9 }.into()];
	let s = S(&data);
	let mut off = 0;
	let cap = t.capture(&s, &mut off).unwrap();
	assert_eq!(*cap, A::T { a: 7, b: 9 }.into());
	assert_eq!(off, 1);
}

#[test]
fn t_mismatches_on_a_different_variant() {
	let data = [A::I.into()];
	let s = S(&data);
	let mut off = 0;
	assert!(t.check(&s, &mut off).is_err());
}

// ---------------------------------------------------------------------
// EE3 - unit variant with a naming-convention-stressing identifier
// ---------------------------------------------------------------------

#[test]
fn e_e3_matches_the_all_caps_digit_suffixed_variant() {
	let data = [A::EE3.into()];
	let s = S(&data);
	let mut off = 0;
	let cap = e_e3.capture(&s, &mut off).unwrap();
	assert_eq!(*cap, A::EE3.into());
}

// ---------------------------------------------------------------------
// I - single-letter unit variant
// ---------------------------------------------------------------------

#[test]
fn i_matches_the_single_letter_variant() {
	let data = [A::I.into()];
	let s = S(&data);
	let mut off = 0;
	assert!(i.check(&s, &mut off).is_ok());
}

// ---------------------------------------------------------------------
// O(Box<A<'src>>) - boxed newtype: matcher extracts a ref to the Box
// ---------------------------------------------------------------------

#[test]
fn o_matches_and_extracts_the_boxed_inner_value() {
	let data = [A::O(Box::new(A::D.into())).into()];
	let s = S(&data);
	let mut off = 0;
	let cap = o.capture(&s, &mut off).unwrap();
	assert_eq!(**cap, A::D.into());
	assert_eq!(off, 1);
}

#[test]
fn o_works_with_a_nested_boxed_newtype() {
	let data = [A::O(Box::new(A::O(Box::new(A::I)))).into()];
	let s = S(&data);
	let mut off = 0;
	let cap = o.capture(&s, &mut off).unwrap();
	assert_eq!(**cap, A::O(Box::new(A::I)).into());
}

// ---------------------------------------------------------------------
// Cross-cutting: incomplete input, offsets, sequencing
// ---------------------------------------------------------------------

#[test]
fn variant_matcher_incomplete_on_empty_slice() {
	let data: [T<'_>; 0] = [];
	let s = S(&data);
	let mut off = 0;
	let err = d.check(&s, &mut off).unwrap_err();
	assert!(matches!(err.kind, MatchErrorKind::InComplete(_)));
}

#[test]
fn variant_matcher_incomplete_at_end_of_nonempty_slice() {
	let data = [A::D.into()];
	let s = S(&data);
	let mut off = 1; // already past the single element
	let err = d.check(&s, &mut off).unwrap_err();
	assert!(matches!(err.kind, MatchErrorKind::InComplete(_)));
}

#[test]
fn variant_matchers_can_be_sequenced_across_a_run_of_tokens() {
	let data = [A::D.into(), A::I.into(), A::F.into(), A::EE3.into()];
	let s = S(&data);
	let mut off = 0;
	d.check(&s, &mut off).unwrap();
	i.check(&s, &mut off).unwrap();
	f.check(&s, &mut off).unwrap();
	e_e3.check(&s, &mut off).unwrap();
	assert_eq!(off, 4);
	assert!(d.check(&s, &mut off).is_err()); // nothing left
}

#[test]
fn wrong_matcher_at_wrong_position_does_not_advance_and_can_be_retried() {
	let data = [A::D.into(), A::I.into()];
	let s = S(&data);
	let mut off = 0;

	// try the wrong matcher first; it must fail without moving off
	assert!(i.check(&s, &mut off).is_err());
	assert_eq!(off, 0);

	// the correct matcher still works at the same position
	d.check(&s, &mut off).unwrap();
	assert_eq!(off, 1);
	i.check(&s, &mut off).unwrap();
	assert_eq!(off, 2);
}

use gramex::{
	cursor::{Cursor, eat, match_map, test, try_eat},
	general::{an, list},
	result::MatchError,
};

struct Cur<'src> {
	input: &'src str,
	off: usize,
	errors: Vec<MatchError>,
}
impl<'src> Cursor<'src> for Cur<'src> {
	type MatchAble = str;
	type Error = ();
	fn input(&self) -> &'src str {
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
	}
}
fn cur(s: &str) -> Cur<'_> {
	Cur { input: s, off: 0, errors: Vec::new() }
}

// ---------------------------------------------------------------------
// The two documented usage examples, reproduced directly (deterministic
// inputs substituted for the original's `rand`-based one)
// ---------------------------------------------------------------------

#[test]
fn documented_eat_try_eat_test_example() {
	let mut c = cur("abc123def");
	eat!(&mut c, "abc").unwrap();
	let _ = eat!(&mut c, "t"); // deliberately fails, ignored, mirrors the docs

	let (one, two, three): (_, u8, _) =
		try_eat!(&mut c, one:'1' (two = _ => two.parse().unwrap()) three*:"3").unwrap();
	assert_eq!(one, "1");
	assert_eq!(two, 2u8);
	assert_eq!(three, vec!["3"]);

	assert!(test!(&mut c, "d" 'e'? ~'f'));
	// `test!` never consumes, so "def" must still be fully there afterwards
	c.eat(list(an(1, |s: &str| s != "c"), 'e')).unwrap();
	assert_eq!(c.off(), c.input.len());
}

#[test]
fn documented_match_map_example_lowercase_arm() {
	// arm: 'a'..'z' (a = _[2]) => Ok((&a[0..1], &a[1..2]))
	// use an interior char ('m') to dodge inclusive/exclusive boundary doubt
	let mut c = cur("mno!");
	let result = match_map!( c, {
		'a'..'z' (a = _[2]) => Ok((&a[0..1], &a[1..2])),
		'A'..'D' => Err(2),
		'D'..'Z' a:alpha b:dec => Ok((a, b)),
		else => Err(3),
	});
	assert_eq!(result, Ok(("n", "o")));
	assert_eq!(c.off(), 3);
}

#[test]
fn documented_match_map_example_first_uppercase_arm() {
	// arm: 'A'..'D' => Err(2) ; use interior 'B'
	let mut c = cur("B!");
	let result = match_map!( c, {
		'a'..'z' (a = _[2]) => Ok((&a[0..1], &a[1..2])),
		'A'..'D' => Err(2),
		'D'..'Z' a:alpha b:dec => Ok((a, b)),
		else => Err(3),
	});
	assert_eq!(result, Err(2));
	assert_eq!(c.off(), 1);
}

#[test]
fn documented_match_map_example_second_uppercase_arm() {
	// arm: 'D'..'Z' a:alpha b:dec => Ok((a, b)) ; interior 'K'
	let mut c = cur("Ka5!");
	let result = match_map!( c, {
		'a'..'z' (a = _[2]) => Ok((&a[0..1], &a[1..2])),
		'A'..'D' => Err(2),
		'D'..'Z' a:alpha b:dec => Ok((a, b)),
		else => Err(3),
	});
	assert_eq!(result, Ok(("a", "5")));
	assert_eq!(c.off(), 3);
}

#[test]
fn documented_match_map_example_else_arm_consumes_nothing() {
	let mut c = cur("9!"); // matches none of the char-range guards
	let result = match_map!( c, {
		'a'..'z' (a = _[2]) => Ok((&a[0..1], &a[1..2])),
		'A'..'D' => Err(2),
		'D'..'Z' a:alpha b:dec => Ok((a, b)),
		else => Err(3),
	});
	assert_eq!(result, Err(3));
	assert_eq!(c.off(), 0, "the else arm shouldn't consume any input");
}

// ---------------------------------------------------------------------
// 1. Basic atoms
// ---------------------------------------------------------------------

#[test]
fn eat_literal_string() {
	let mut c = cur("hello world");
	assert_eq!(eat!(&mut c, "hello").unwrap(), "hello");
	assert_eq!(c.off(), 5);
}

#[test]
fn eat_literal_char() {
	let mut c = cur("xyz");
	assert_eq!(eat!(&mut c, 'x').unwrap(), "x");
	assert_eq!(c.off(), 1);
}

#[test]
fn eat_wildcard_matches_any_single_token() {
	let mut c = cur("!@#");
	assert_eq!(eat!(&mut c, _).unwrap(), "!");
	assert_eq!(c.off(), 1);
}

#[test]
fn eat_char_range_interior_hit() {
	let mut c = cur("5x");
	assert_eq!(eat!(&mut c, '0'..'9').unwrap(), "5");
}

#[test]
fn eat_char_range_miss() {
	let mut c = cur("x5");
	assert!(eat!(&mut c, '0'..'9').is_err());
}

// ---------------------------------------------------------------------
// 2. Sequence, Or
// ---------------------------------------------------------------------

#[test]
fn eat_sequence_of_literals() {
	let mut c = cur("abc!");
	eat!(&mut c, "a" "b" "c").unwrap();
	assert_eq!(c.off(), 3);
}

#[test]
fn eat_or_picks_the_matching_alternative() {
	let mut c = cur("bar!");
	assert_eq!(eat!(&mut c, "foo" | "bar" | "baz").unwrap(), "bar");
}

#[test]
fn eat_or_fails_when_nothing_matches() {
	let mut c = cur("qux!");
	assert!(eat!(&mut c, "foo" | "bar" | "baz").is_err());
}

// ---------------------------------------------------------------------
// 3. Not (!) and Near (~)
// ---------------------------------------------------------------------

#[test]
fn eat_not_consumes_a_token_that_does_not_match() {
	let mut c = cur("xyz");
	assert_eq!(eat!(&mut c, !"a").unwrap(), "x");
	assert_eq!(c.off(), 1);
}

#[test]
fn eat_not_fails_when_the_token_does_match() {
	let mut c = cur("abc");
	assert!(eat!(&mut c, !"a").is_err());
}

#[test]
fn test_near_is_satisfied_by_a_pure_lookahead() {
	let mut c = cur("ab");
	assert!(test!(c, "a" ~"b"));
}

#[test]
fn eat_near_asserts_but_does_not_consume() {
	let mut c = cur("ab");
	eat!(&mut c, "a" ~"b").unwrap();
	assert_eq!(c.off(), 1, "~ (near) must not consume what it looks ahead at");
}

// ---------------------------------------------------------------------
// 4. And (&) and Imply (->)
// ---------------------------------------------------------------------

#[test]
fn eat_and_forces_the_alternative_matching_the_shared_width() {
	let mut c = cur("ab!");
	// ("a" | "ab") & _[2] can only be satisfied by the "ab" branch, since
	// `&` requires both sides to cover the exact same span as `_[2]`.
	assert!(eat!(&mut c, ("a" | "ab") & _[1]).is_ok());
	assert_eq!(c.off(), 1);
}

#[test]
fn eat_and_fails_when_no_alternative_matches_the_shared_width() {
	let mut c = cur("c!");
	assert!(eat!(&mut c, ("a" | "ab") & _[2]).is_err());
}

#[test]
fn eat_imply_succeeds_when_both_sides_present() {
	let mut c = cur("ab!");
	eat!(&mut c, "a" -> "b").unwrap();
	assert_eq!(c.off(), 2);
}

#[test]
fn eat_imply_fails_outright_when_the_left_side_is_absent() {
	let mut c = cur("xy");
	assert!(eat!(&mut c, "a" -> "b").is_ok());
}

#[test]
fn eat_imply_fails_outright_when_the_left_side_is_absen() {
	let mut c = cur("ay");
	eat!(&mut c, "a" -> "b").unwrap_err();
	assert!(c.errors[0].off == 1);
}

// ---------------------------------------------------------------------
// 5. Repetition
// ---------------------------------------------------------------------

#[test]
fn eat_optional_repetition_present() {
	let mut c = cur("ab");
	eat!(&mut c, "a" "b"?).unwrap();
	assert_eq!(c.off(), 2);
}

#[test]
fn eat_optional_repetition_absent() {
	let mut c = cur("ax");
	eat!(&mut c, "a" "b"?).unwrap();
	assert_eq!(c.off(), 1);
}

#[test]
fn eat_star_repetition_several_occurrences() {
	let mut c = cur("aaab");
	eat!(&mut c, "a" * "b").unwrap();
	assert_eq!(c.off(), 4);
}

#[test]
fn eat_star_repetition_zero_occurrences() {
	let mut c = cur("b");
	eat!(&mut c, "a" * "b").unwrap();
	assert_eq!(c.off(), 1);
}

#[test]
fn eat_plus_repetition_requires_at_least_one() {
	let mut c = cur("b");
	assert!(eat!(&mut c, "a" + "b").is_err());
}

#[test]
fn eat_plus_repetition_several_occurrences() {
	let mut c = cur("aaab");
	eat!(&mut c, "a" + "b").unwrap();
	assert_eq!(c.off(), 4);
}

#[test]
fn eat_exact_repetition_count() {
	let mut c = cur("aaa!");
	eat!(&mut c, "a"[3]).unwrap();
	assert_eq!(c.off(), 3);
}

#[test]
fn eat_exact_repetition_rejects_wrong_count() {
	let mut c = cur("aa!");
	assert!(eat!(&mut c, "a"[3]).is_err());
}

#[test]
fn eat_min_max_repetition_bounded_by_available_input() {
	// only 3 a's available; the range's upper bound (4 or 5, whichever
	// convention) is never actually the limiting factor here
	let mut c = cur("aaa!");
	eat!(&mut c, "a"[2..5]).unwrap();
	assert_eq!(c.off(), 3);
}

#[test]
fn eat_min_max_repetition_rejects_below_minimum() {
	let mut c = cur("a!");
	assert!(eat!(&mut c, "a"[2..5]).is_err());
}

#[test]
fn eat_min_only_repetition() {
	let mut c = cur("aaaaa!");
	eat!(&mut c, "a"[2..]).unwrap();
	assert_eq!(c.off(), 5);
}

#[test]
fn eat_min_only_repetition_rejects_below_minimum() {
	let mut c = cur("a!");
	assert!(eat!(&mut c, "a"[2..]).is_err());
}

#[test]
fn eat_max_only_repetition_consumes_up_to_available() {
	// only 2 a's, safely under any plausible reading of the upper bound (3)
	let mut c = cur("aa!");
	eat!(&mut c, "a"[..3]).unwrap();
	assert_eq!(c.off(), 2);
}

// ---------------------------------------------------------------------
// 6. Captures
// ---------------------------------------------------------------------

#[test]
fn eat_named_capture_alone_yields_the_raw_match() {
	let mut c = cur("x");
	assert_eq!(eat!(&mut c, a: "x").unwrap().0, "x");
}

#[test]
fn eat_two_named_captures_yield_a_tuple() {
	let mut c = cur("xy");
	let (a, b) = eat!(&mut c, a: "x" b: "y").unwrap();
	assert_eq!(a, "x");
	assert_eq!(b, "y");
}

#[test]
fn eat_optional_capture_some_when_present() {
	let mut c = cur("x");
	assert_eq!(eat!(&mut c, a?: "x").unwrap().0, Some("x"));
}

#[test]
fn eat_optional_capture_none_when_absent() {
	let mut c = cur("y");
	assert_eq!(eat!(&mut c, a?: "x").unwrap().0, None);
	assert_eq!(c.off(), 0, "an absent optional capture must consume nothing");
}

#[test]
fn eat_star_capture_collects_into_vec() {
	let mut c = cur("ababab!");
	assert_eq!(eat!(&mut c, a*: "ab").unwrap().0, vec!["ab", "ab", "ab"]);
}

#[test]
fn eat_star_capture_can_be_empty() {
	let mut c = cur("!");
	assert_eq!(eat!(&mut c, a*: "ab").unwrap().0, Vec::<&str>::new());
}

#[test]
fn eat_plus_capture_requires_at_least_one() {
	let mut c = cur("y");
	assert!(eat!(&mut c, a+: "x").is_err());
}

#[test]
fn eat_plus_capture_collects_all_occurrences() {
	let mut c = cur("xxxy");
	assert_eq!(eat!(&mut c, a+: "x").unwrap().0, vec!["x", "x", "x"]);
}

// ---------------------------------------------------------------------
// 7. Mapping
// ---------------------------------------------------------------------

#[test]
fn eat_single_capture_with_mapping_like_the_m_root_example() {
	let mut c = cur("123");
	let n: u32 = try_eat!(&mut c, (a = "123" => a.parse().unwrap())).unwrap().0;
	assert_eq!(n, 123);
}

// ---------------------------------------------------------------------
// try_eat! - Option-returning variant
// ---------------------------------------------------------------------

#[test]
fn try_eat_returns_some_on_success_and_advances() {
	let mut c = cur("hello world");
	assert_eq!(try_eat!(&mut c, "hello"), Some("hello"));
	assert_eq!(c.off(), 5);
}

#[test]
fn try_eat_returns_none_on_failure_without_advancing() {
	let mut c = cur("world");
	assert_eq!(try_eat!(&mut c, a: "hello"), None);
	assert_eq!(c.off(), 0);
}

// ---------------------------------------------------------------------
// test! - boolean, never-consuming variant
// ---------------------------------------------------------------------

#[test]
fn test_macro_true_on_match_without_consuming() {
	let mut c = cur("hello");
	assert!(test!(&mut c, "hello"));
	assert_eq!(c.off(), 0);
}

#[test]
fn test_macro_false_on_mismatch_without_consuming() {
	let mut c = cur("world");
	assert!(!test!(&mut c, "hello"));
	assert_eq!(c.off(), 0);
}
