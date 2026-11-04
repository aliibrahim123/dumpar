//! Integration tests for `gramex`.
//!
//! Place this file at `tests/gramex_tests.rs` in the crate root so `cargo test`
//! picks it up. It only exercises the *public* API (`core.rs`, `general.rs`,
//! `str.rs`, `result.rs`) — the `gramex!` proc-macro DSL shown in the prompt
//! isn't covered here since its expansion isn't part of the provided source,
//! so I can't verify what it actually generates.

use std::{borrow::Cow, boxed::Box, rc::Rc, sync::Arc};

use gramex::{
	MatchAble, Matcher, check,
	general::{atomic, delim_list, end, expected, fail_with, list},
	matches, parse,
	result::{Expected, MatchErrorKind},
	try_match,
};

// ---------------------------------------------------------------------
// MatchAble for str
// ---------------------------------------------------------------------

#[test]
fn str_get_token_handles_multibyte_chars() {
	let s = "héllo"; // h=0, é=1..3, l=3, l=4, o=5
	assert_eq!(s.get_token(0), Some('h'));
	assert_eq!(s.get_token(1), Some('é'));
	assert_eq!(s.get_token(3), Some('l'));
	assert_eq!(s.get_token(5), Some('o'));
	assert_eq!(s.get_token(6), None);
}

#[test]
fn str_slice_respects_char_boundaries() {
	let s = "héllo";
	assert_eq!(s.slice(0..1), Some("h"));
	assert_eq!(s.slice(1..3), Some("é"));
	// 2 is inside the 2-byte 'é' -> not a valid boundary
	assert_eq!(s.slice(0..2), None);
}

#[test]
fn str_skip_n_counts_chars_not_bytes() {
	let s = "héllo";
	let mut off = 0;
	<str as MatchAble>::skip_n::<gramex::modes::Check>(s, &mut off, 2).unwrap();
	// 'h' (1 byte) + 'é' (2 bytes) = offset 3
	assert_eq!(off, 3);
}

#[test]
fn str_skip_n_past_end_clamps_and_errors() {
	let s = "abc";
	let mut off = 0;
	let res = <str as MatchAble>::skip_n::<gramex::modes::Check>(s, &mut off, 10);
	assert!(res.is_err());
	assert_eq!(off, 0);
}

// ---------------------------------------------------------------------
// Literal / char / range matchers, and top-level entry points
// ---------------------------------------------------------------------

#[test]
fn literal_str_matches_exactly() {
	assert!(matches("hello", "hello"));
	assert!(!matches("hello", "world"));
}

#[test]
fn matches_requires_full_consumption() {
	// "hello" matches a prefix of "hello world" but leaves excess input,
	// and `matches` (via match_no_excess) requires the whole input consumed.
	assert!(!matches("hello world", "hello"));
}

#[test]
fn char_matcher() {
	assert!(matches("a", 'a'));
	assert!(!matches("a", 'b'));
}

#[test]
fn range_inclusive_char_matcher() {
	let digit = '0'..='9';
	assert!(matches("5", digit.clone()));
	assert!(!matches("x", digit));
}

#[test]
fn owned_string_like_types_are_matchers() {
	assert!(matches("foo", String::from("foo")));
	let b: Box<str> = "bar".into();
	assert!(matches("bar", b));
	let rc: Rc<str> = "baz".into();
	assert!(matches("baz", rc));
	let arc: Arc<str> = "qux".into();
	assert!(matches("qux", arc));
	let cow: Cow<str> = Cow::Borrowed("cow");
	assert!(matches("cow", cow));
}

#[test]
fn try_match_returns_capture_on_success() {
	assert_eq!(try_match("hello", "hello"), Some("hello"));
	assert_eq!(try_match("nope", "hello"), None);
}

#[test]
fn parse_returns_detailed_error_on_mismatch() {
	let err = parse("wrld", "world").unwrap_err();
	assert!(core::matches!(err.kind, MatchErrorKind::InComplete(_)));
	assert_eq!(err.off, 0);
}

#[test]
fn check_ok_on_exact_match() {
	assert!(check("hello", &"hello").is_ok());
}

// ---------------------------------------------------------------------
// End
// ---------------------------------------------------------------------

#[test]
fn end_succeeds_only_at_input_end() {
	let mut at_end = 3;
	assert!(end.test("abc", &mut at_end));

	let mut not_at_end = 2;
	assert!(!end.test("abc", &mut not_at_end));
}

// ---------------------------------------------------------------------
// Atomic
// ---------------------------------------------------------------------

#[test]
fn atomic_advances_offset_on_success() {
	let mut off = 0;
	atomic("hello").check("hello world", &mut off).unwrap();
	assert_eq!(off, 5);
}

#[test]
fn atomic_rewinds_offset_on_failure() {
	let mut off = 0;
	let res = atomic("hello").check("help", &mut off);
	assert!(res.is_err());
	assert_eq!(off, 0, "Atomic must restore the offset on failure");
}

// ---------------------------------------------------------------------
// A / An
// ---------------------------------------------------------------------

#[test]
fn a_predicate_captures_and_advances() {
	let matcher = gramex::general::a::<str, _>(|c: char| c.is_ascii_digit());
	let cap = parse("5", &matcher).unwrap();
	assert_eq!(cap, '5');
}

#[test]
fn a_predicate_rejects_and_resets_offset() {
	let matcher = gramex::general::a::<str, _>(|c: char| c.is_ascii_digit());
	let mut off = 0;
	let res = matcher.check("x", &mut off);
	assert!(res.is_err());
	assert_eq!(off, 0);
}

#[test]
fn an_matches_n_tokens_by_slice_predicate() {
	let matcher = gramex::general::an::<str, _>(3, |s: &str| s == "abc");
	assert_eq!(parse("abc", &matcher).unwrap(), "abc");
}

#[test]
fn an_fails_when_slice_predicate_rejects() {
	let matcher = gramex::general::an::<str, _>(3, |s: &str| s == "abc");
	let mut off = 0;
	assert!(matcher.check("xyz", &mut off).is_err());
	assert_eq!(off, 0);
}

// ---------------------------------------------------------------------
// FailExpected / FailWith
// ---------------------------------------------------------------------

#[test]
fn fail_expected_mismatch_mid_input() {
	let matcher = expected("digit");
	let mut off = 0;
	let err = matcher.check("abc", &mut off).unwrap_err();
	assert_eq!(err.kind, MatchErrorKind::MisMatch(Expected::A("digit".into())));
	assert_eq!(<_ as Matcher<str>>::expected(&matcher), Expected::A("digit".into()));
}

#[test]
fn fail_expected_incomplete_at_end_of_input() {
	let matcher = expected("digit");
	let mut off = 3;
	let err = matcher.check("abc", &mut off).unwrap_err();
	assert_eq!(err.kind, MatchErrorKind::InComplete(Expected::A("digit".into())));
}

#[test]
fn fail_with_produces_custom_message() {
	let matcher = fail_with("custom failure");
	let mut off = 0;
	let err = matcher.check("abc", &mut off).unwrap_err();
	match err.kind {
		MatchErrorKind::Other(msg) => {
			assert_eq!(AsRef::<str>::as_ref(&msg), "custom failure")
		}
		other => panic!("expected Other(..), got {other:?}"),
	}
}

// ---------------------------------------------------------------------
// List
// ---------------------------------------------------------------------

#[test]
fn list_parses_separated_items() {
	let digit = gramex::general::a::<str, _>(|c: char| c.is_ascii_digit());
	let matcher = list(digit, ',');
	let result = parse("1,2,3", &matcher).unwrap();
	assert_eq!(result, vec!['1', '2', '3']);
}

#[test]
fn list_accepts_single_item_with_no_separator() {
	let digit = gramex::general::a::<str, _>(|c: char| c.is_ascii_digit());
	let matcher = list(digit, ',');
	assert_eq!(parse("7", &matcher).unwrap(), vec!['7']);
}

#[test]
fn list_fails_on_empty_input() {
	let digit = gramex::general::a::<str, _>(|c: char| c.is_ascii_digit());
	let matcher = list(digit, ',');
	assert!(check("", &matcher).is_err());
}

#[test]
fn list_stops_before_trailing_non_item_content() {
	// A dangling separator with nothing after it leaves excess input,
	// so the top-level `check` (which forbids excess) should fail.
	let digit = gramex::general::a::<str, _>(|c: char| c.is_ascii_digit());
	let matcher = list(digit, ',');
	assert!(check("1,2,", &matcher).is_err());
}

// ---------------------------------------------------------------------
// DelimList
// ---------------------------------------------------------------------

#[test]
fn delim_list_parses_bracketed_items() {
	let digit = gramex::general::a::<str, _>(|c: char| c.is_ascii_digit());
	let matcher = delim_list('[', digit, ',', ']');
	let result = parse("[1,2,3]", &matcher).unwrap();
	assert_eq!(result, vec!['1', '2', '3']);
}

#[test]
fn delim_list_parses_empty_brackets() {
	let digit = gramex::general::a::<str, _>(|c: char| c.is_ascii_digit());
	let matcher = delim_list('[', digit, ',', ']');
	let result = parse("[]", &matcher).unwrap();
	assert_eq!(result, Vec::<char>::new());
}

#[test]
fn delim_list_requires_closing_end_when_separator_missing() {
	let digit = gramex::general::a::<str, _>(|c: char| c.is_ascii_digit());
	let matcher = delim_list('[', digit, ',', ']');
	println!("{:?}", check("[1", &matcher));
	// no separator and no closing ']' -> the fallback `end.do_match` must fail
	assert!(check("[1", &matcher).is_err());
}

#[test]
fn delim_list_fails_without_opening_delimiter() {
	let digit = gramex::general::a::<str, _>(|c: char| c.is_ascii_digit());
	let matcher = delim_list('[', digit, ',', ']');
	assert!(check("1,2]", &matcher).is_err());
}

// ---------------------------------------------------------------------
// Unit matcher (`impl Matcher<T> for ()`)
// ---------------------------------------------------------------------

#[test]
fn unit_matcher_always_succeeds_without_consuming_input() {
	let mut off = 1;
	assert!(().test("abc", &mut off));
	assert_eq!(off, 1, "the unit matcher must not advance the offset");
}

// ---------------------------------------------------------------------
// Expected / MatchError Display
// ---------------------------------------------------------------------

#[test]
fn expected_display_variants() {
	assert_eq!(format!("{}", Expected::None), "");
	assert_eq!(format!("{}", Expected::SomeThing), "something");
	assert_eq!(format!("{}", Expected::A("foo".into())), "foo");
	assert_eq!(format!("{}", Expected::Not("foo".into())), "not foo");
	assert_eq!(
		format!("{}", Expected::OneOf(vec!["a".into(), "b".into()])),
		"one of a, b"
	);
	assert_eq!(
		format!("{}", Expected::Between("a".into(), "z".into())),
		"between a and z"
	);
}

#[test]
fn match_error_display_includes_offset_and_expected() {
	use gramex::result::MatchError;
	let e = MatchError::mismatch(Expected::A("digit".into()), 3);
	assert_eq!(format!("{e}"), "mismatch at 3, expected digit");

	let e = MatchError::excess(5);
	assert_eq!(format!("{e}"), "excess input at 5");
}
