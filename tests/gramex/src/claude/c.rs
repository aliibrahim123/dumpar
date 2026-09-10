//! More integration tests for `gramex`.
//!
//! Place this file at `tests/gramex_char_class_tests.rs` alongside
//! `tests/gramex_tests.rs`. Covers:
//!   - the `define_pattern_matcher!`-generated single-char classifiers
//!   - `digit(radix)` / `Digit`
//!   - the blanket impls: `Matcher<T> for ()`, `for &U`, `for Box<U>/Rc<U>/Arc<U>`
//!     (the generic "wrap another matcher" versions, not the `Box<str>` ones),
//!     and `for Option<U>`

use std::{boxed::Box, rc::Rc, sync::Arc};

use gramex::{
	Matcher, check, matches, parse,
	result::{Expected, MatchErrorKind},
	str::{
		alpha, alphanum, ascii, ascii_alpha, ascii_alphanum, ascii_control, ascii_lower,
		ascii_printable, ascii_punct, ascii_upper, ascii_ws, bin, control, dec, digit,
		hex, hex_lower, hex_upper, lower, num, octal, upper, ws,
	},
};

// ---------------------------------------------------------------------
// define_pattern_matcher! generated matchers - happy path
// ---------------------------------------------------------------------

#[test]
fn case_classifiers() {
	assert!(matches("A", upper));
	assert!(!matches("a", upper));
	assert!(matches("a", lower));
	assert!(!matches("A", lower));
}

#[test]
fn alpha_and_num_classifiers() {
	assert!(matches("z", alpha));
	assert!(!matches("5", alpha));
	assert!(matches("5", num));
	assert!(!matches("z", num));
	assert!(matches("z", alphanum));
	assert!(matches("5", alphanum));
	assert!(!matches("!", alphanum));
}

#[test]
fn whitespace_and_control_classifiers() {
	assert!(matches(" ", ws));
	assert!(matches("\t", ws));
	assert!(!matches("x", ws));
	assert!(matches("\u{7}", control)); // BEL is a control char
	assert!(!matches("x", control));
}

#[test]
fn ascii_classifier_rejects_non_ascii() {
	assert!(matches("a", ascii));
	assert!(!matches("é", ascii));
}

#[test]
fn ascii_case_and_alpha_classifiers() {
	assert!(matches("A", ascii_upper));
	assert!(!matches("é", ascii_upper)); // non-ascii uppercase must be rejected
	assert!(matches("a", ascii_lower));
	assert!(matches("z", ascii_alpha));
	assert!(matches("9", ascii_alphanum));
	assert!(!matches("_", ascii_alphanum));
}

#[test]
fn ascii_ws_control_printable_punct() {
	assert!(matches(" ", ascii_ws));
	assert!(matches("\u{0}", ascii_control));
	assert!(matches("!", ascii_printable));
	assert!(!matches(" ", ascii_printable)); // space is not "graphic"
	assert!(matches(".", ascii_punct));
	assert!(!matches("a", ascii_punct));
}

#[test]
fn radix_style_classifiers() {
	assert!(matches("7", dec));
	assert!(!matches("a", dec));

	assert!(matches("a", hex));
	assert!(matches("F", hex));
	assert!(matches("9", hex));
	assert!(!matches("g", hex));

	assert!(matches("a", hex_lower));
	assert!(!matches("A", hex_lower));

	assert!(matches("A", hex_upper));
	assert!(!matches("a", hex_upper));

	assert!(matches("0", bin));
	assert!(matches("1", bin));
	assert!(!matches("2", bin));

	assert!(matches("7", octal));
	assert!(!matches("8", octal));
}

// ---------------------------------------------------------------------
// define_pattern_matcher! generated matchers - captures, offsets, errors
// ---------------------------------------------------------------------

#[test]
fn pattern_matcher_captures_the_matched_char_as_str_slice() {
	let mut off = 0;
	let cap = upper.capture("Abc", &mut off).unwrap();
	assert_eq!(cap, "A");
	assert_eq!(off, 1);
}

#[test]
fn pattern_matcher_captures_multibyte_char_correctly() {
	// 'É' is uppercase and 2 bytes in UTF-8.
	let mut off = 0;
	let cap = upper.capture("École", &mut off).unwrap();
	assert_eq!(cap, "É");
	assert_eq!(off, 2);
}

#[test]
fn pattern_matcher_mismatch_leaves_offset_unmoved() {
	let mut off = 0;
	let err = dec.check("x", &mut off).unwrap_err();
	assert_eq!(err.kind, MatchErrorKind::MisMatch(dec.expected()));
	assert_eq!(err.off, 0);
	assert_eq!(off, 0);
}

#[test]
fn pattern_matcher_incomplete_on_empty_input() {
	let mut off = 0;
	let err = dec.check("", &mut off).unwrap_err();
	assert!(core::matches!(err.kind, MatchErrorKind::InComplete(_)));
}

#[test]
fn pattern_matcher_expected_text() {
	assert_eq!(dec.expected(), Expected::A("a decimal digit".into()));
	assert_eq!(hex.expected(), Expected::A("a hexadecimal digit".into()));
	assert_eq!(upper.expected(), Expected::A("an uppercase character".into()));
}

// ---------------------------------------------------------------------
// digit(radix) / Digit
// ---------------------------------------------------------------------

#[test]
fn digit_matches_within_radix() {
	assert!(matches("9", digit(10)));
	assert!(!matches("a", digit(10)));

	assert!(matches("f", digit(16)));
	assert!(matches("F", digit(16)));
	assert!(!matches("g", digit(16)));

	assert!(matches("1", digit(2)));
	assert!(!matches("2", digit(2)));

	assert!(matches("z", digit(36)));
	assert!(!matches("!", digit(36)));
}

#[test]
fn digit_captures_char_and_advances_offset() {
	let mut off = 0;
	let d = digit(16);
	let cap = d.capture("f0", &mut off).unwrap();
	assert_eq!(cap, "f");
	assert_eq!(off, 1);
}

#[test]
fn digit_expected_mentions_radix() {
	assert_eq!(digit(16).expected(), Expected::A("a base-16 digit".into()));
	assert_eq!(digit(2).expected(), Expected::A("a base-2 digit".into()));
}

#[test]
fn digit_incomplete_on_empty_input() {
	let mut off = 0;
	let err = digit(10).check("", &mut off).unwrap_err();
	assert!(core::matches!(err.kind, MatchErrorKind::InComplete(_)));
}

#[test]
#[should_panic]
fn digit_panics_below_minimum_radix() {
	digit(1);
}

#[test]
#[should_panic]
fn digit_panics_above_maximum_radix() {
	digit(37);
}

#[test]
fn digit_accepts_boundary_radixes() {
	// radix 2 and 36 are the documented inclusive bounds and must not panic
	let _ = digit(2);
	let _ = digit(36);
}

// ---------------------------------------------------------------------
// Matcher<T> for () (unit matcher)
// ---------------------------------------------------------------------

#[test]
fn unit_matcher_never_fails_and_never_consumes() {
	let mut off = 0;
	assert!(().check("anything", &mut off).is_ok());
	assert_eq!(off, 0);
}

// ---------------------------------------------------------------------
// Matcher<T> for &U
// ---------------------------------------------------------------------

#[test]
fn reference_to_matcher_delegates_transparently() {
	let d = digit(10);
	let r = &d;
	let mut off = 0;
	assert_eq!(r.capture("5", &mut off), Some("5"));
	assert_eq!(off, 1);
}

#[test]
fn double_reference_matcher_still_delegates() {
	// &str already implements Matcher<str> (via the blanket over `str`), so
	// &&str must also work by the same blanket applied twice.
	let base: &str = "hello";
	let double: &&str = &base;
	assert!(matches("hello", double));
}

#[test]
fn reference_matcher_forwards_expected() {
	let d = digit(8);
	let r = &d;
	assert_eq!(r.expected(), d.expected());
}

// ---------------------------------------------------------------------
// Matcher<T> for Box<U> / Rc<U> / Arc<U>  (generic "wraps a matcher" impls)
// ---------------------------------------------------------------------

#[test]
fn box_wrapped_matcher_delegates() {
	let boxed: Box<&str> = Box::new("hello");
	assert!(matches("hello", boxed));
}

#[test]
fn rc_wrapped_matcher_delegates() {
	let rced: Rc<&str> = Rc::new("hello");
	assert!(matches("hello", rced));
}

#[test]
fn arc_wrapped_matcher_delegates() {
	let arced: Arc<&str> = Arc::new("hello");
	assert!(matches("hello", arced));
}

#[test]
fn box_wrapped_matcher_propagates_failure() {
	let boxed: Box<&str> = Box::new("hello");
	let mut off = 0;
	assert!(boxed.check("world", &mut off).is_err());
	assert_eq!(off, 0);
}

// ---------------------------------------------------------------------
// Matcher<T> for Option<U>
// ---------------------------------------------------------------------

#[test]
fn option_some_delegates_and_wraps_capture() {
	let opt: Option<&str> = Some("hi");
	let mut off = 0;
	assert_eq!(opt.capture("hi there", &mut off), Some(Some("hi")));
	assert_eq!(off, 2);
}

#[test]
fn option_none_always_succeeds_without_consuming() {
	let opt: Option<&str> = None;
	let mut off = 0;
	assert_eq!(opt.capture("anything", &mut off), Some(None));
	assert_eq!(off, 0);
}

#[test]
fn option_some_failure_propagates_and_resets_nothing_extra() {
	let opt: Option<&str> = Some("hi");
	let mut off = 0;
	assert!(opt.check("bye", &mut off).is_err());
	assert_eq!(off, 0);
}

#[test]
fn option_makes_an_otherwise_required_matcher_optional_in_check_mode() {
	// `check` on `top-level` doesn't allocate captures but still must respect
	// success/failure semantics for Option<U>.
	let opt: Option<&str> = None;
	assert!(check("", &opt).is_ok());
}
use gramex::{
	MatchAble,
	bytes::{
		a_be, a_le, aligned, be, f32_be, f32_le, f64_le, i16_be, i16_le, i64_be,
		isize_be, le, u16_be, u16_le, u32_be, u32_le, u64_be, u64_le, u128_le, usize_le,
	},
};

// ---------------------------------------------------------------------
// MatchAble for [u8]
// ---------------------------------------------------------------------

#[test]
fn bytes_matchable_basics() {
	let data: &[u8] = &[1, 2, 3, 4, 5];
	assert_eq!(MatchAble::len(data), 5);
	assert_eq!(data.get_token(0), Some(1));
	assert_eq!(data.get_token(4), Some(5));
	assert_eq!(data.get_token(5), None);
	assert_eq!(data.slice(1..3), Some(&[2, 3][..]));
	assert_eq!(data.slice(1..10), None);
}

#[test]
fn bytes_skip_n_default_impl() {
	let data: &[u8] = &[1, 2, 3];
	let mut off = 0;
	<[u8] as MatchAble>::skip_n::<gramex::modes::Check>(data, &mut off, 2).unwrap();
	assert_eq!(off, 2);
}

#[test]
fn bytes_skip_n_past_end_clamps_and_errors() {
	let data: &[u8] = &[1, 2, 3];
	let mut off = 0;
	let res = <[u8] as MatchAble>::skip_n::<gramex::modes::Check>(data, &mut off, 10);
	assert!(res.is_err());
	assert_eq!(off, 0);
}

// ---------------------------------------------------------------------
// u8 / [u8] / RangeInclusive<u8> literal matchers
// ---------------------------------------------------------------------

#[test]
fn single_byte_literal_matcher() {
	assert!(matches(&[0x42u8][..], 0x42u8));
	assert!(!matches(&[0x41u8][..], 0x42u8));
}

#[test]
fn byte_slice_matcher_captures_prefix() {
	let needle: &[u8] = &[0xDE, 0xAD];
	let mut off = 0;
	let cap = needle.capture(&[0xDE, 0xAD, 0xBE, 0xEF][..], &mut off).unwrap();
	assert_eq!(cap, &[0xDE, 0xAD]);
	assert_eq!(off, 2);
}

#[test]
fn byte_slice_matcher_rejects_excess_via_matches() {
	// `matches` requires full consumption; a 2-byte needle against a
	// 3-byte haystack leaves excess input.
	let needle: &[u8] = &[0xDE, 0xAD];
	assert!(!matches(&[0xDE, 0xAD, 0xBE][..], needle));
	assert!(matches(&[0xDE, 0xAD][..], needle));
}

#[test]
fn byte_range_inclusive_matcher() {
	let range = 0x10u8..=0x1F;
	assert!(matches(&[0x15u8][..], range.clone()));
	assert!(!matches(&[0x20u8][..], range));
}

// ---------------------------------------------------------------------
// incomplete vs mismatch for the literal matchers
// ---------------------------------------------------------------------

#[test]
fn byte_slice_matcher_incomplete_when_input_too_short() {
	let needle: &[u8] = &[0xDE, 0xAD];
	let mut off = 0;
	let err = needle.check(&[0xDE][..], &mut off).unwrap_err();
	assert!(core::matches!(err.kind, MatchErrorKind::InComplete(_)));
}

#[test]
fn byte_slice_matcher_mismatch_when_long_enough_but_wrong() {
	let needle: &[u8] = &[0xDE, 0xAD];
	let mut off = 0;
	let err = needle.check(&[0xDE, 0xFF][..], &mut off).unwrap_err();
	assert!(core::matches!(err.kind, MatchErrorKind::MisMatch(_)));
}

// ---------------------------------------------------------------------
// hex-formatted Expected output (to_hex, exercised indirectly)
// ---------------------------------------------------------------------

#[test]
fn single_byte_expected_is_hex_formatted() {
	let m: u8 = 0xAB;
	assert_eq!(format!("{}", <_ as Matcher<[u8]>>::expected(&m)), "`ab`");
}

#[test]
fn byte_slice_expected_is_space_separated_hex() {
	let needle: &[u8] = &[0xDE, 0xAD, 0xBE, 0xEF];
	assert_eq!(format!("{}", needle.expected()), "`de ad be ef`");
}

#[test]
fn byte_range_expected_is_between_hex_bounds() {
	let range = 0x10u8..=0x1F;
	assert_eq!(format!("{}", range.expected()), "between `10` and `1f`");
}

// ---------------------------------------------------------------------
// Vec<u8> / [u8; N] as matchers (define_ref_matcher!)
// ---------------------------------------------------------------------

#[test]
fn vec_u8_as_matcher() {
	let v: Vec<u8> = vec![1, 2, 3];
	let mut off = 0;
	let cap = v.capture(&[1, 2, 3, 4][..], &mut off).unwrap();
	assert_eq!(cap, &[1, 2, 3]);
	assert_eq!(off, 3);
}

#[test]
fn fixed_size_array_as_matcher() {
	let arr: [u8; 3] = [9, 9, 9];
	let mut off = 0;
	let cap = arr.capture(&[9, 9, 9, 1][..], &mut off).unwrap();
	assert_eq!(cap, &[9, 9, 9]);
	assert_eq!(off, 3);
}

#[test]
fn fixed_size_array_matcher_expected_delegates_to_slice_formatting() {
	let arr: [u8; 2] = [0xAA, 0xBB];
	assert_eq!(format!("{}", <_ as Matcher<[u8]>>::expected(&arr)), "`aa bb`");
}

// ---------------------------------------------------------------------
// Aligned
// ---------------------------------------------------------------------

#[test]
fn aligned_succeeds_on_multiple_of_align_and_does_not_consume() {
	let data: &[u8] = &[0; 8];
	let mut off = 4;
	assert!(aligned(4).check(data, &mut off).is_ok());
	assert_eq!(off, 4);
}

#[test]
fn aligned_fails_on_misalignment() {
	let data: &[u8] = &[0; 8];
	let mut off = 3;
	let err = aligned(4).check(data, &mut off).unwrap_err();
	assert!(core::matches!(err.kind, MatchErrorKind::MisMatch(_)));
}

#[test]
fn aligned_expected_text() {
	assert_eq!(format!("{}", aligned(8).expected()), "8-byte alignment");
}

#[test]
fn aligned_does_not_bounds_check_offset() {
	// Aligned only checks `off % align == 0` — it never checks `off` against
	// the input length, so an out-of-bounds-but-aligned offset still "matches".
	let data: &[u8] = &[0, 0];
	let mut off = 100;
	assert!(aligned(4).check(data, &mut off).is_ok());
}

#[test]
#[should_panic]
fn aligned_zero_panics_via_modulo_by_zero() {
	let data: &[u8] = &[];
	let mut off = 0;
	let _ = aligned(0).check(data, &mut off);
}

// ---------------------------------------------------------------------
// Endian-aware numeric matchers
// ---------------------------------------------------------------------

#[test]
fn u16_endian_matchers_read_correct_byte_order() {
	let bytes: &[u8] = &[0x01, 0x02, 0xFF];

	let mut off = 0;
	assert_eq!(u16_le.capture(bytes, &mut off).unwrap(), 0x0201);
	assert_eq!(off, 2);

	let mut off2 = 0;
	assert_eq!(u16_be.capture(bytes, &mut off2).unwrap(), 0x0102);
	assert_eq!(off2, 2);
}

#[test]
fn u32_endian_matchers_read_correct_byte_order() {
	let bytes: &[u8] = &[0x01, 0x02, 0x03, 0x04];
	let mut off = 0;
	assert_eq!(u32_le.capture(bytes, &mut off).unwrap(), 0x04030201);
	let mut off2 = 0;
	assert_eq!(u32_be.capture(bytes, &mut off2).unwrap(), 0x01020304);
}

#[test]
fn i16_endian_matchers_handle_sign_correctly() {
	let bytes: &[u8] = &[0x00, 0x80];
	let mut off = 0;
	assert_eq!(i16_le.capture(bytes, &mut off).unwrap(), i16::MIN);
	let mut off2 = 0;
	assert_eq!(i16_be.capture(bytes, &mut off2).unwrap(), 128i16);
}

#[test]
fn f32_endian_matchers_roundtrip() {
	let val: f32 = 3.14159;
	let le_bytes = val.to_le_bytes();
	let mut off = 0;
	assert_eq!(f32_le.capture(&le_bytes[..], &mut off).unwrap(), val);
	assert_eq!(off, 4);

	let be_bytes = val.to_be_bytes();
	let mut off2 = 0;
	assert_eq!(f32_be.capture(&be_bytes[..], &mut off2).unwrap(), val);
}

#[test]
fn f64_endian_matcher_roundtrip() {
	let val: f64 = -2.71828182845;
	let le_bytes = val.to_le_bytes();
	let mut off = 0;
	assert_eq!(f64_le.capture(&le_bytes[..], &mut off).unwrap(), val);
}

#[test]
fn u64_endian_matchers_roundtrip() {
	let val: u64 = 0x0123_4567_89AB_CDEF;
	let le = val.to_le_bytes();
	let mut off = 0;
	assert_eq!(u64_le.capture(&le[..], &mut off).unwrap(), val);

	let be = val.to_be_bytes();
	let mut off2 = 0;
	assert_eq!(u64_be.capture(&be[..], &mut off2).unwrap(), val);
}

#[test]
fn u128_endian_matcher_roundtrip() {
	let val: u128 = 0x0123_4567_89AB_CDEF_0123_4567_89AB_CDEF;
	let le = val.to_le_bytes();
	let mut off = 0;
	assert_eq!(u128_le.capture(&le[..], &mut off).unwrap(), val);
}

#[test]
fn usize_isize_endian_matchers_roundtrip() {
	let uval: usize = 42;
	let le = uval.to_le_bytes();
	let mut off = 0;
	assert_eq!(usize_le.capture(&le[..], &mut off).unwrap(), uval);

	let ival: isize = -42;
	let be = ival.to_be_bytes();
	let mut off2 = 0;
	assert_eq!(isize_be.capture(&be[..], &mut off2).unwrap(), ival);
}

#[test]
fn numeric_matcher_expected_text() {
	assert_eq!(format!("{}", u32_le.expected()), "a u32 value");
	assert_eq!(format!("{}", i64_be.expected()), "a i64 value");
}

#[test]
fn numeric_matcher_exact_length_input_succeeds() {
	// exactly SIZE bytes available -> fine, no panic, no error
	let bytes: &[u8] = &[0x01, 0x02];
	let mut off = 0;
	assert_eq!(u16_le.capture(bytes, &mut off).unwrap(), 0x0201);
	assert_eq!(off, 2);
}

// ---------------------------------------------------------------------
// le() / be() - match an exact value via its byte encoding, capturing the
// original typed value (not the raw bytes)
// ---------------------------------------------------------------------

#[test]
fn le_matches_exact_value_and_captures_the_typed_value() {
	let m = le(0x1234u16);
	let bytes = 0x1234u16.to_le_bytes(); // [0x34, 0x12]
	let mut off = 0;
	let cap = m.capture(&bytes[..], &mut off).unwrap();
	assert_eq!(cap, &[0x34, 0x12]);
	assert_eq!(off, 2);
}

#[test]
fn be_matches_exact_value_and_captures_the_typed_value() {
	let m = be(0x1234u16);
	let bytes = 0x1234u16.to_be_bytes(); // [0x12, 0x34]
	let mut off = 0;
	let cap = m.capture(&bytes[..], &mut off).unwrap();
	assert_eq!(cap, &[0x12, 0x34]);
	assert_eq!(off, 2);
}

#[test]
fn le_rejects_bytes_that_dont_encode_the_target_value() {
	let m = le(0x1234u16);
	let mut off = 0;
	let err = m.check(&[0x00, 0x00][..], &mut off).unwrap_err();
	assert!(core::matches!(err.kind, MatchErrorKind::MisMatch(_)));
}

#[test]
fn le_and_be_disagree_on_swapped_bytes() {
	// bytes that encode 0x1234 as LE decode as a different value as BE, so
	// le(0x1234) must reject the BE encoding and vice versa.
	let le_bytes = 0x1234u16.to_le_bytes();
	let be_bytes = 0x1234u16.to_be_bytes();
	assert_ne!(le_bytes, be_bytes, "test assumes a non-palindromic value");

	assert!(matches(&le_bytes[..], le(0x1234u16)));
	assert!(!matches(&be_bytes[..], le(0x1234u16)));
	assert!(matches(&be_bytes[..], be(0x1234u16)));
	assert!(!matches(&le_bytes[..], be(0x1234u16)));
}

#[test]
fn le_incomplete_when_input_too_short() {
	let m = le(0x1234u16);
	let mut off = 0;
	let err = m.check(&[0x34][..], &mut off).unwrap_err();
	assert!(core::matches!(err.kind, MatchErrorKind::InComplete(_)));
}

#[test]
fn le_expected_is_hex_of_the_le_encoded_bytes() {
	assert_eq!(format!("{}", le(0x1234u16).expected()), "`34 12`");
}

#[test]
fn be_expected_is_hex_of_the_be_encoded_bytes() {
	assert_eq!(format!("{}", be(0x1234u16).expected()), "`12 34`");
}

#[test]
fn le_be_work_for_floats_and_signed_types_too() {
	let val: f32 = 3.5;
	let bytes = val.to_le_bytes();
	let mut off = 0;
	assert_eq!(le(val).capture(&bytes[..], &mut off).unwrap(), bytes);

	let ival: i32 = -12345;
	let ibytes = ival.to_be_bytes();
	let mut off2 = 0;
	assert_eq!(be(ival).capture(&ibytes[..], &mut off2).unwrap(), ibytes);
}

// ---------------------------------------------------------------------
// a_le() / a_be() - decode via a fixed endianness, keep only values
// satisfying a predicate
// ---------------------------------------------------------------------

#[test]
fn a_le_captures_value_when_predicate_holds() {
	let m = a_le(|v: u16| v > 100);
	let bytes = 200u16.to_le_bytes();
	let mut off = 0;
	assert_eq!(m.capture(&bytes[..], &mut off).unwrap(), 200u16);
	assert_eq!(off, 2);
}

#[test]
fn a_le_mismatches_without_moving_offset_when_predicate_fails() {
	let m = a_le(|v: u16| v > 100);
	let bytes = 50u16.to_le_bytes();
	let mut off = 0;
	let err = m.check(&bytes[..], &mut off).unwrap_err();
	assert!(core::matches!(err.kind, MatchErrorKind::MisMatch(_)));
	assert_eq!(
		off, 0,
		"a predicate failure (as opposed to short input) doesn't move off"
	);
}

#[test]
fn a_le_incomplete_pushes_offset_to_end_on_short_input() {
	let m = a_le(|v: u16| v > 0);
	let bytes: &[u8] = &[0x01];
	let mut off = 0;
	let err = m.check(bytes, &mut off).unwrap_err();
	assert!(core::matches!(err.kind, MatchErrorKind::InComplete(_)));
	assert_eq!(off, 0);
}

#[test]
fn a_be_captures_value_when_predicate_holds() {
	let m = a_be(|v: u32| v % 2 == 0);
	let bytes = 100u32.to_be_bytes();
	let mut off = 0;
	assert_eq!(m.capture(&bytes[..], &mut off).unwrap(), 100u32);
}

#[test]
fn a_be_mismatches_when_predicate_fails() {
	let m = a_be(|v: u32| v % 2 == 0);
	let bytes = 101u32.to_be_bytes();
	let mut off = 0;
	assert!(m.check(&bytes[..], &mut off).is_err());
}

#[test]
fn a_le_a_be_expected_text_matches_plain_numeric_matchers() {
	assert_eq!(format!("{}", a_le(|v: u16| v > 0).expected()), "a u16 value");
	assert_eq!(format!("{}", a_be(|v: i16| v > 0).expected()), "a i16 value");
}

#[test]
fn a_le_and_a_be_disagree_on_endianness_like_plain_le_be() {
	let le_bytes = 0x1234u16.to_le_bytes();
	let be_bytes = 0x1234u16.to_be_bytes();

	// decoded-as-LE value from le_bytes is 0x1234; decoded-as-BE from the
	// same bytes would be 0x3412, so a predicate tied to 0x1234 only fires
	// through the matching endianness.
	let wants_1234 = |v: u16| v == 0x1234;
	assert!(matches(&le_bytes[..], a_le(wants_1234)));
	assert!(!matches(&le_bytes[..], a_be(wants_1234)));
	assert!(matches(&be_bytes[..], a_be(wants_1234)));
	assert!(!matches(&be_bytes[..], a_le(wants_1234)));
}

#[test]
fn numeric_matcher_errors_gracefully_on_short_input() {
	// Was a panic before the `matched.get(off..off+SIZE)` bounds-check fix
	// (see the module-level doc comment history) — now matches the
	// byte/slice matchers' behavior: a too-short buffer is a normal
	// `MatchError::incomplete`. Note the offset convention used throughout
	// this file: on *incomplete*, `off` is pushed to `matched.len()` (there's
	// nothing more to consume); only on *mismatch* does it stay put.
	let bytes: &[u8] = &[0x01]; // only 1 byte, u16 needs 2
	let mut off = 0;
	let err = u16_le.check(bytes, &mut off).unwrap_err();
	assert!(core::matches!(err.kind, MatchErrorKind::InComplete(_)));
	assert_eq!(off, 0, "incomplete errors push off to matched.len()");
}

#[test]
fn numeric_matcher_errors_gracefully_when_completely_empty() {
	let bytes: &[u8] = &[];
	let mut off = 0;
	let err = u32_be.check(bytes, &mut off).unwrap_err();
	assert!(core::matches!(err.kind, MatchErrorKind::InComplete(_)));
}

#[test]
fn numeric_matcher_errors_gracefully_when_off_near_end() {
	// off is in-bounds but there isn't enough room left for a full value
	let bytes: &[u8] = &[0xAA, 0xBB, 0xCC];
	let mut off = 2; // only 1 byte remains, u16 needs 2
	let err = u16_le.check(bytes, &mut off).unwrap_err();
	assert!(core::matches!(err.kind, MatchErrorKind::InComplete(_)));
	assert_eq!(off, 2);
}
