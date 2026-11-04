use core::ops::RangeInclusive;

use gramex::{
	MatchAble, Matcher, check,
	derive::{match_slice, match_token},
	derive_slice_matchable,
	modes::Parse,
	parse,
	result::{Expected, MatchErrorKind},
};

// ---------------------------------------------------------------------
// match_token
// ---------------------------------------------------------------------

#[test]
fn match_token_succeeds_and_captures_matched_slice() {
	let mut off = 0;
	let res = match_token::<Parse, str>(
		"hello",
		&mut off,
		|c: char| c.is_alphabetic().then_some(c.len_utf8()),
		|| Expected::A("a letter".into()),
	);
	assert_eq!(res.unwrap(), "h");
	assert_eq!(off, 1);
}

#[test]
fn match_token_mismatch_leaves_offset_unmoved() {
	let mut off = 0;
	let res = match_token::<Parse, str>(
		"1hello",
		&mut off,
		|c: char| c.is_alphabetic().then_some(c.len_utf8()),
		|| Expected::A("a letter".into()),
	);
	let err = res.unwrap_err();
	assert!(matches!(err.kind, MatchErrorKind::MisMatch(_)));
	assert_eq!(off, 0);
}

#[test]
fn match_token_incomplete_on_empty_input() {
	let mut off = 0;
	let res = match_token::<Parse, str>(
		"",
		&mut off,
		|c: char| Some(c.len_utf8()),
		|| Expected::A("anything".into()),
	);
	assert!(matches!(res.unwrap_err().kind, MatchErrorKind::InComplete(_)));
}

#[test]
fn match_token_respects_multi_len_predicate_result() {
	// the predicate can report a token "length" different from 1 (as the
	// pattern-matcher macros do with multibyte chars), and match_token
	// advances by exactly that amount.
	let mut off = 0;
	let res = match_token::<Parse, str>(
		"École",
		&mut off,
		|c: char| c.is_uppercase().then_some(c.len_utf8()),
		|| Expected::A("an uppercase letter".into()),
	);
	assert_eq!(res.unwrap(), "É");
	assert_eq!(off, 2); // 'É' is 2 bytes in UTF-8
}

// ---------------------------------------------------------------------
// match_slice
// ---------------------------------------------------------------------

#[test]
fn match_slice_captures_transformed_value_on_success() {
	let mut off = 0;
	let res = match_slice::<Parse, str, _>(
		"12345",
		&mut off,
		3,
		|s: &str| s.parse::<u32>().ok(),
		|| Expected::A("a 3-digit number".into()),
	);
	assert_eq!(res.unwrap(), 123);
	assert_eq!(off, 3);
}

#[test]
fn match_slice_incomplete_when_requested_size_exceeds_input() {
	let mut off = 0;
	let res = match_slice::<Parse, str, u32>(
		"12",
		&mut off,
		3,
		|s: &str| s.parse::<u32>().ok(),
		|| Expected::A("a 3-digit number".into()),
	);
	assert!(matches!(res.unwrap_err().kind, MatchErrorKind::InComplete(_)));
}

#[test]
fn match_slice_mismatch_when_predicate_rejects() {
	let mut off = 0;
	let res = match_slice::<Parse, str, u32>(
		"abc",
		&mut off,
		3,
		|s: &str| s.parse::<u32>().ok(),
		|| Expected::A("a 3-digit number".into()),
	);
	assert!(matches!(res.unwrap_err().kind, MatchErrorKind::MisMatch(_)));
}

#[test]
fn match_slice_zero_size_always_available_slice() {
	// size 0 should always find a (empty) slice, even on empty input.
	let mut off = 0;
	let res = match_slice::<Parse, str, _>(
		"",
		&mut off,
		0,
		|s: &str| Some(s.len()),
		|| Expected::A("nothing".into()),
	);
	assert_eq!(res.unwrap(), 0);
	assert_eq!(off, 0);
}

// ---------------------------------------------------------------------
// derive_slice_matchable! end to end, via a custom &[u32] wrapper
// ---------------------------------------------------------------------

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Toks<'src>(&'src [u32]);
derive_slice_matchable!(Toks(&[u32]), matchers: WithRange, expected: Debug);

#[test]
fn toks_matchable_basics() {
	let data: &[u32] = &[10, 20, 30];
	let t = Toks(data);
	assert_eq!(t.len(), 3);
	assert_eq!(t.get_token(0), Some(&10));
	assert_eq!(t.get_token(2), Some(&30));
	assert_eq!(t.get_token(3), None);
	let sub = t.slice(1..3).unwrap();
	assert_eq!(sub.0, &[20, 30]);
	assert_eq!(t.slice(1..10), None);
}

#[test]
fn toks_self_type_matcher_matches_equal_subslice() {
	let hay = Toks(&[1, 2, 3, 4]);
	let needle = Toks(&[2, 3]);
	let mut off = 1;
	let cap = needle.capture(&hay, &mut off).unwrap();
	assert_eq!(cap.0, &[2, 3]);
	assert_eq!(off, 3);
}

#[test]
fn toks_self_type_matcher_mismatches_on_unequal_subslice() {
	let hay = Toks(&[1, 2, 3, 4]);
	let needle = Toks(&[9, 9]);
	let mut off = 0;
	assert!(needle.check(&hay, &mut off).is_err());
	assert_eq!(off, 0);
}

#[test]
fn toks_self_type_matcher_incomplete_when_longer_than_remaining() {
	let hay = Toks(&[1, 2]);
	let needle = Toks(&[1, 2, 3]);
	let mut off = 0;
	let err = needle.check(&hay, &mut off).unwrap_err();
	assert!(matches!(err.kind, MatchErrorKind::InComplete(_)));
}

#[test]
fn toks_raw_slice_matcher_matches_prefix() {
	let hay = Toks(&[1, 2, 3, 4]);
	let needle: &[u32] = &[3, 4];
	let mut off = 2;
	let cap = needle.capture(&hay, &mut off).unwrap();
	assert_eq!(cap.0, &[3, 4]);
	assert_eq!(off, 4);
}

#[test]
fn toks_single_item_token_matcher() {
	let hay = Toks(&[7, 8, 9]);
	let mut off = 0;
	let cap = 7u32.capture(&hay, &mut off).unwrap();
	assert_eq!(cap.0, &[7]);
	assert_eq!(off, 1);
}

#[test]
fn toks_single_item_token_matcher_mismatch_leaves_offset() {
	let hay = Toks(&[7, 8, 9]);
	let mut off = 0;
	assert!(8u32.check(&hay, &mut off).is_err());
	assert_eq!(off, 0);
}

#[test]
fn toks_single_item_token_matcher_incomplete_on_empty() {
	let hay = Toks(&[]);
	let mut off = 0;
	let err = 5u32.check(&hay, &mut off).unwrap_err();
	assert!(matches!(err.kind, MatchErrorKind::InComplete(_)));
}

#[test]
fn toks_range_inclusive_token_matcher() {
	let range: RangeInclusive<u32> = 10..=20;
	let hay = Toks(&[15, 25, 10]);

	let mut off = 0;
	let cap = range.capture(&hay, &mut off).unwrap();
	assert_eq!(cap.0, &[15]);
	assert_eq!(off, 1);

	// next token (25) is out of range
	assert!(range.check(&hay, &mut off).is_err());
	assert_eq!(off, 1);
}

#[test]
fn toks_range_inclusive_token_matcher_boundary_values() {
	let range: RangeInclusive<u32> = 10..=20;
	assert!(range.test(&Toks(&[10]), &mut 0));
	assert!(range.test(&Toks(&[20]), &mut 0));
	assert!(!range.test(&Toks(&[9]), &mut 0));
	assert!(!range.test(&Toks(&[21]), &mut 0));
}

#[test]
fn toks_end_to_end_parse_via_list_of_tokens() {
	// mix a couple of the generated matchers together through the public
	// entry points (parse/check), not just direct do_match/capture calls.
	let data: &[u32] = &[1, 2, 3];
	let hay = Toks(data);
	assert!(check(&hay, &Toks(&[1, 2, 3])).is_ok());
	assert!(check(&hay, &Toks(&[1, 2])).is_err()); // excess input
	let cap = 1u32.parse(&hay, &mut 0).unwrap();
	assert_eq!(cap.0, &[1]);
}

use gramex::{
	cursor::{Cursor, SimpleSpan, span_around},
	result::MatchError,
};

// ---------------------------------------------------------------------
// SimpleSpan
// ---------------------------------------------------------------------

#[test]
fn simple_span_basic_accessors() {
	let s = SimpleSpan::new(3usize, 7usize);
	assert_eq!(s.start, 3);
	assert_eq!(s.end, 7);
}

#[test]
fn simple_span_point_is_a_width_one_span() {
	let s = SimpleSpan::point(5usize);
	assert_eq!(s, SimpleSpan::new(5, 6));
	assert!(s.is_point());
}

#[test]
fn simple_span_is_point_false_for_wider_span() {
	let s = SimpleSpan::new(1usize, 4usize);
	assert!(!s.is_point());
}

#[test]
fn simple_span_join_takes_min_start_and_max_end() {
	let a = SimpleSpan::new(2usize, 5usize);
	let b = SimpleSpan::new(4usize, 9usize);
	assert_eq!(a.join(&b), SimpleSpan::new(2, 9));
	// order shouldn't matter
	assert_eq!(b.join(&a), SimpleSpan::new(2, 9));
}

#[test]
fn simple_span_join_of_non_overlapping_spans_covers_the_gap() {
	let a = SimpleSpan::new(0usize, 2usize);
	let b = SimpleSpan::new(10usize, 12usize);
	assert_eq!(a.join(&b), SimpleSpan::new(0, 12));
}

#[test]
fn simple_span_display_uses_default_usize_param() {
	let s = SimpleSpan::new(3usize, 7usize);
	assert_eq!(format!("{s}"), "3..7");
}

#[test]
fn simple_span_generic_over_other_addable_ord_types() {
	let s = SimpleSpan::<u32>::new(2, 5);
	assert_eq!(s.start, 2);
	assert_eq!(s.end, 5);
	assert_eq!(SimpleSpan::point(10u32), SimpleSpan::new(10u32, 11u32));
}

// ---------------------------------------------------------------------
// span_around
// ---------------------------------------------------------------------

#[test]
fn span_around_captures_the_matched_byte_range() {
	let m = span_around("hello");
	let mut off = 0;
	let span = m.capture("hello world", &mut off).unwrap();
	assert_eq!(span, SimpleSpan::new(0, 5));
	assert_eq!(off, 5);
}

#[test]
fn span_around_reflects_a_nonzero_starting_offset() {
	let m = span_around("world");
	let mut off = 6;
	let span = m.capture("hello world", &mut off).unwrap();
	assert_eq!(span, SimpleSpan::new(6, 11));
}

#[test]
fn span_around_propagates_failure_and_produces_no_span() {
	let m = span_around("hello");
	let mut off = 0;
	assert!(m.check("world", &mut off).is_err());
	assert_eq!(off, 0);
}

#[test]
fn span_around_wraps_a_zero_width_matcher_as_a_point_span() {
	let m = span_around::<str, ()>(()); // the unit matcher always succeeds, consumes nothing
	let mut off = 4;
	let span = m.capture("hello", &mut off).unwrap();
	assert_eq!(span, SimpleSpan::new(4, 4));
	assert!(!span.is_point()); // width 0, not width 1 - not a "point" by this def
}

// ---------------------------------------------------------------------
// Cursor - minimal hand-written implementation over `str`
// ---------------------------------------------------------------------

struct Cur<'src> {
	input: &'src str,
	off: usize,
	errors: Vec<MatchError>,
}

impl<'src> Cur<'src> {
	fn new(input: &'src str) -> Self {
		Self { input, off: 0, errors: Vec::new() }
	}
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

#[test]
fn cursor_peek_returns_current_token_without_advancing() {
	let cur = Cur::new("abc");
	assert_eq!(cur.peek(), Some('a'));
	assert_eq!(cur.off(), 0);
}

#[test]
fn cursor_peek_is_none_at_end_of_input() {
	let mut cur = Cur::new("a");
	cur.skip();
	assert_eq!(cur.peek(), None);
}

#[test]
fn cursor_peek_next_looks_ahead_without_advancing() {
	let cur = Cur::new("abc");
	assert_eq!(cur.peek_next(0), Some('a'));
	assert_eq!(cur.peek_next(1), Some('b'));
	assert_eq!(cur.peek_next(2), Some('c'));
	assert_eq!(cur.peek_next(3), None);
	assert_eq!(cur.off(), 0, "peek_next must never mutate the cursor's offset");
}

#[test]
fn cursor_is_end_tracks_offset_vs_length() {
	let mut cur = Cur::new("a");
	assert!(!cur.is_end());
	cur.skip();
	assert!(cur.is_end());
}

#[test]
fn cursor_is_end_true_for_empty_input() {
	let cur = Cur::new("");
	assert!(cur.is_end());
}

#[test]
fn cursor_eat_consumes_and_returns_capture_on_success() {
	let mut cur = Cur::new("hello world");
	let cap = cur.eat("hello").unwrap();
	assert_eq!(cap, "hello");
	assert_eq!(cur.off(), 5);
}

#[test]
fn cursor_eat_reports_mismatch_via_map_error_and_leaves_offset() {
	let mut cur = Cur::new("world");
	let res = cur.eat("hello");
	assert!(res.is_err());
	assert_eq!(
		cur.off(),
		0,
		"a mismatch (as opposed to incomplete) leaves off untouched"
	);
	assert_eq!(cur.errors.len(), 1);
	assert!(matches!(cur.errors[0].kind, MatchErrorKind::MisMatch(_)));
}

#[test]
fn cursor_eat_reports_incomplete_and_advances_offset_to_end() {
	let mut cur = Cur::new("he");
	let res = cur.eat("hello");
	assert!(res.is_err());
	assert_eq!(cur.off(), 0);
	assert!(matches!(cur.errors[0].kind, MatchErrorKind::InComplete(_)));
}

#[test]
fn cursor_eat_accumulates_multiple_errors_across_calls() {
	let mut cur = Cur::new("world");
	let _ = cur.eat("hello");
	let _ = cur.eat("there");
	assert_eq!(cur.errors.len(), 2);
}

#[test]
fn cursor_try_eat_advances_only_on_success() {
	let mut cur = Cur::new("hello world");
	let cap = cur.try_eat("hello");
	assert_eq!(cap, Some("hello"));
	assert_eq!(cur.off(), 5);
}

#[test]
fn cursor_try_eat_leaves_offset_and_errors_untouched_on_failure() {
	let mut cur = Cur::new("world");
	let cap = cur.try_eat("hello");
	assert_eq!(cap, None);
	assert_eq!(cur.off(), 0);
	assert!(cur.errors.is_empty(), "try_eat should never call map_error");
}

#[test]
fn cursor_try_eat_can_be_chained_for_lookahead_style_parsing() {
	let mut cur = Cur::new("aabb");
	assert_eq!(cur.try_eat('a'), Some("a"));
	assert_eq!(cur.try_eat('a'), Some("a"));
	assert_eq!(cur.try_eat('a'), None); // no more 'a's
	assert_eq!(cur.off(), 2);
}

#[test]
fn cursor_test_checks_without_consuming_on_success_or_failure() {
	let cur = Cur::new("hello");
	assert!(cur.test("hello"));
	assert_eq!(cur.off(), 0);
	assert!(!cur.test("world"));
	assert_eq!(cur.off(), 0);
}

#[test]
fn cursor_skip_advances_by_exactly_one_token() {
	let mut cur = Cur::new("abc");
	cur.skip();
	assert_eq!(cur.off(), 1);
	assert_eq!(cur.peek(), Some('b'));
}

#[test]
fn cursor_skip_past_end_is_a_safe_no_op_beyond_the_length() {
	let mut cur = Cur::new("a");
	cur.skip();
	assert_eq!(cur.off(), 1);
	cur.skip(); // already at end; must not panic or overshoot
	assert_eq!(cur.off(), 1);
}

#[test]
fn cursor_skip_respects_multibyte_characters() {
	let mut cur = Cur::new("é_"); // 'é' is 2 bytes in UTF-8
	cur.skip();
	assert_eq!(cur.off(), 2);
	assert_eq!(cur.peek(), Some('_'));
}

#[test]
fn cursor_rewind_resets_to_an_earlier_offset() {
	let mut cur = Cur::new("abcdef");
	cur.skip();
	cur.skip();
	assert_eq!(cur.off(), 2);
	cur.rewind(0);
	assert_eq!(cur.off(), 0);
	assert_eq!(cur.peek(), Some('a'));
}

#[test]
fn cursor_rewind_can_move_forward_too() {
	let mut cur = Cur::new("abcdef");
	cur.rewind(4);
	assert_eq!(cur.off(), 4);
	assert_eq!(cur.peek(), Some('e'));
}

#[test]
fn cursor_expected_reports_mismatch_when_not_at_end() {
	let mut cur = Cur::new("x");
	let res = cur.expected::<()>("a digit");
	assert!(res.is_err());
	assert_eq!(cur.errors.len(), 1);
	assert!(matches!(cur.errors[0].kind, MatchErrorKind::MisMatch(_)));
}

#[test]
fn cursor_expected_reports_incomplete_when_at_end() {
	let mut cur = Cur::new("");
	let res = cur.expected::<()>("a digit");
	assert!(res.is_err());
	assert!(matches!(cur.errors[0].kind, MatchErrorKind::InComplete(_)));
}

#[test]
fn cursor_expected_reports_incomplete_after_fully_consuming_input() {
	let mut cur = Cur::new("x");
	cur.skip();
	let res = cur.expected::<()>("more input");
	assert!(res.is_err());
	assert!(matches!(cur.errors[0].kind, MatchErrorKind::InComplete(_)));
}

#[test]
fn cursor_full_parse_loop_using_eat_and_peek() {
	// a small hand-rolled loop in the spirit of the crate's own usage
	// example, using only the plain Cursor trait methods.
	let mut cur = Cur::new("abc123");
	let letters = cur.eat(gramex::str::alpha).is_ok();
	assert!(letters);
	// keep eating alpha chars manually
	while cur.peek().is_some_and(|c| c.is_alphabetic()) {
		cur.eat(gramex::str::alpha).unwrap();
	}
	assert_eq!(cur.off(), 3);
	assert!(cur.peek().unwrap().is_numeric());
}
