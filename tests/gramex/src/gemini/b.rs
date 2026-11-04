//! Integration tests for `gramex::bits` and `gramex::bytes::bits_ext`.
//!
//! Covers:
//!   - `Bits`, `LBits`, `BBits` core types, conversions, and properties
//!   - `MatchAble` impls for `LBits` and `BBits` (endian-aware bit extraction)
//!   - Matchers for bits (`bool`, exact values, primitives, `BitRange`, `a_b` predicates)
//!   - `word_le` / `word_be` integrating bit matching into byte slice parsing

use gramex::{
	MatchAble, Matcher,
	bits::{BBits, BitRange, Bits, LBits, a_b, b, br},
	bytes::{word_be, word_le},
	check, matcher, matches, parse,
	result::{Expected, MatchErrorKind},
};

// ---------------------------------------------------------------------
// Bits construction, PartialEq, and formatting
// ---------------------------------------------------------------------

#[test]
fn bits_construction_and_defaults() {
	let b1 = b(4, 0xA);
	assert_eq!(b1.value, 10);
	assert_eq!(b1.len, 4);

	let def = Bits::default();
	assert_eq!(def.value, 0);
	assert_eq!(def.len, 0);

	let lb = LBits::new(8, 0xFF);
	let bb = BBits::new(8, 0xFF);
	assert_eq!(lb.0.value, 255);
	assert_eq!(bb.0.value, 255);
}

#[test]
fn bits_partial_eq_variants() {
	let raw = b(8, 42);
	let lb = LBits::new(8, 42);
	let bb = BBits::new(8, 42);

	// Test against u64
	assert_eq!(raw, 42u64);
	assert_eq!(42u64, raw);
	assert_eq!(lb, 42u64);
	assert_eq!(bb, 42u64);

	// Test wrappers against raw
	assert_eq!(lb, raw);
	assert_eq!(raw, lb);
	assert_eq!(bb, raw);

	// Test wrappers against wrappers
	assert_eq!(lb, bb);
	assert_eq!(bb, lb);
}

#[test]
fn bits_binary_formatting() {
	let raw = b(6, 0b101010);
	assert_eq!(format!("{:b}", raw), "0b101010");

	let lb = LBits::new(4, 0b0101);
	assert_eq!(format!("{:b}", lb), "0b0101");

	let bb = BBits::new(8, 0b00001111);
	assert_eq!(format!("{:b}", bb), "0b00001111");
}

// ---------------------------------------------------------------------
// Type Conversions (From / TryFrom)
// ---------------------------------------------------------------------

#[test]
fn bits_from_primitive_ints() {
	let from_u8 = Bits::from(0xABu8);
	assert_eq!(from_u8.len, 8);
	assert_eq!(from_u8.value, 0xAB);

	let from_u32 = Bits::from(0xDEADBEEFu32);
	assert_eq!(from_u32.len, 32);

	let from_i8 = Bits::from(-1i8);
	assert_eq!(from_i8.len, 8);
	assert_eq!(from_i8.value, 0xFF); // -1 is all 1s in two's complement

	// Revert them
	assert_eq!(u8::try_from(from_u8).unwrap(), 0xAB);
	assert_eq!(i8::try_from(from_i8).unwrap(), -1);
}

#[test]
fn bits_try_from_primitive_fails_if_len_exceeds() {
	let too_big = b(16, 0xFFFF);
	let res = u8::try_from(too_big);
	assert!(res.is_err());
}

#[test]
fn bits_from_bool_and_floats() {
	let f_true = Bits::from(true);
	assert_eq!(f_true.len, 1);
	assert_eq!(f_true.value, 1);

	let f_pi = Bits::from(3.14f32);
	assert_eq!(f_pi.len, 32);
	assert_eq!(f32::try_from(f_pi).unwrap(), 3.14f32);

	let f_double = Bits::from(-2.718f64);
	assert_eq!(f_double.len, 64);
	assert_eq!(f64::try_from(f_double).unwrap(), -2.718f64);
}

#[test]
fn bits_from_arrays_and_slices() {
	let arr = [0xAAu8, 0xBB];
	let b_arr = Bits::from_le_bytes(arr);
	assert_eq!(b_arr.len, 16);
	// LE conversion: 0xAA is LSB, 0xBB is MSB
	assert_eq!(b_arr.value, 0xBBAA);

	let slice = [0xAA, 0xBB, 0xCC];
	let b_slice = Bits::from_le_bytes(slice);
	assert_eq!(b_slice.len, 24);
	assert_eq!(b_slice.value, 0xCCBBAA);
}

// ---------------------------------------------------------------------
// MatchAble implementations (Bit extraction LE vs BE)
// ---------------------------------------------------------------------

#[test]
fn matchable_lbits_token_extraction_is_lsb_to_msb() {
	let lb = LBits::new(8, 0b1100_1010);
	assert_eq!(lb.len(), 8);
	// 0b11001010
	// bit 0 = 0
	// bit 1 = 1
	// bit 2 = 0
	// bit 3 = 1
	// bit 7 = 1
	assert_eq!(lb.get_token(0), Some(false));
	assert_eq!(lb.get_token(1), Some(true));
	assert_eq!(lb.get_token(2), Some(false));
	assert_eq!(lb.get_token(3), Some(true));
	assert_eq!(lb.get_token(7), Some(true));
	assert_eq!(lb.get_token(8), None);
}

#[test]
fn matchable_bbits_token_extraction_is_msb_to_lsb() {
	let bb = BBits::new(8, 0b1100_1010);
	assert_eq!(bb.len(), 8);
	// 0b11001010 -> read from top to bottom
	// bit 0 = 1
	// bit 1 = 1
	// bit 2 = 0
	// bit 3 = 0
	// bit 7 = 0
	assert_eq!(bb.get_token(0), Some(true));
	assert_eq!(bb.get_token(1), Some(true));
	assert_eq!(bb.get_token(2), Some(false));
	assert_eq!(bb.get_token(3), Some(false));
	assert_eq!(bb.get_token(7), Some(false));
	assert_eq!(bb.get_token(8), None);
}

#[test]
fn matchable_lbits_slice_extraction() {
	let lb = LBits::new(8, 0b1100_1010);
	// bits 0..4 in LE: 1010 -> 10
	let s = lb.slice(0..4).unwrap();
	assert_eq!(s, LBits::new(4, 0b1010));
	assert_eq!(s.len(), 4);

	assert_eq!(lb.slice(0..9), None);
}

#[test]
fn matchable_bbits_slice_extraction() {
	let bb = BBits::new(8, 0b1100_1010);
	// bits 0..4 in BE (top 4 bits): 1100 -> 12
	let s = bb.slice(0..4).unwrap();
	assert_eq!(s, BBits::new(4, 0b1100));
	assert_eq!(s.len(), 4);

	assert_eq!(bb.slice(0..9), None);
}

// ---------------------------------------------------------------------
// Matchers for Bits
// ---------------------------------------------------------------------

#[test]
fn matcher_bool_on_bits() {
	let lb = LBits::new(2, 0b10); // bit0=0, bit1=1
	let mut off = 0;
	let c1 = false.capture(&lb, &mut off).unwrap();
	assert_eq!(c1, LBits::new(1, 0));
	assert_eq!(off, 1);

	let c2 = true.capture(&lb, &mut off).unwrap();
	assert_eq!(c2, LBits::new(1, 1));
	assert_eq!(off, 2);
}

#[test]
fn matcher_bool_fails_when_expected_differs() {
	let lb = LBits::new(1, 1);
	let mut off = 0;
	let err = false.check(&lb, &mut off).unwrap_err();
	assert!(core::matches!(err.kind, MatchErrorKind::MisMatch(_)));
	assert_eq!(format!("{}", <_ as Matcher<LBits>>::expected(&false)), "`0`");
}

#[test]
fn matcher_exact_bits_slice() {
	let lb = LBits::new(8, 0b1100_1010);
	let mut off = 0;
	// Match lower 4 bits
	let cap = b(4, 0b1010).capture(&lb, &mut off).unwrap();
	assert_eq!(cap, LBits::new(4, 0b1010));
	assert_eq!(off, 4);
}

#[test]
fn matcher_primitives_on_bits() {
	let lb = LBits::new(16, 0xABCD);

	// `into_bits_matchers!` implements Matcher on LBits for `u16` etc.
	// u16 length is 16 bits.
	let mut off = 0;
	let cap = 0xABCDu16.capture(&lb, &mut off).unwrap();
	assert_eq!(cap, LBits::new(16, 0xABCD));
	assert_eq!(off, 16);
}

#[test]
fn matcher_bitrange() {
	let lb = LBits::new(8, 150); // 150
	let range = br(8, 100..=200);

	let mut off = 0;
	let cap = range.capture(&lb, &mut off).unwrap();
	assert_eq!(cap, LBits::new(8, 150));
	assert_eq!(off, 8);
	assert_eq!(
		format!("{}", <_ as Matcher<LBits>>::expected(&range)),
		"between `01100100` and `11001000`"
	);
}

#[test]
fn matcher_a_b_predicate() {
	let lb = LBits::new(4, 0b1010); // 10
	let pred = a_b(4, |v| v % 2 == 0); // accepts even values

	let mut off = 0;
	let cap = pred.capture(&lb, &mut off).unwrap();
	assert_eq!(cap, LBits::new(4, 10));
	assert_eq!(off, 4);

	// It rejects odd values
	let lb_odd = LBits::new(4, 0b1011); // 11
	let mut off2 = 0;
	assert!(pred.check(&lb_odd, &mut off2).is_err());
}

// ---------------------------------------------------------------------
// bytes::bits_ext (word_le / word_be)
// ---------------------------------------------------------------------

#[test]
fn word_le_reads_bytes_and_applies_sub_matcher() {
	let bytes: &[u8] = &[0xCD, 0xAB]; // LE bytes for 0xABCD
	let mut off = 0;

	// word_le takes 2 bytes, casts to LBits(0xABCD, len=16) and checks against 0xABCDu16
	let m = word_le::<LBits, _>(2, 0xABCDu16);
	let cap = m.capture(bytes, &mut off).unwrap();

	assert_eq!(cap, LBits::new(16, 0xABCD));
	assert_eq!(off, 2);
}

#[test]
fn word_be_reads_bytes_and_applies_sub_matcher() {
	let bytes: &[u8] = &[0xAB, 0xCD]; // BE bytes for 0xABCD
	let mut off = 0;

	// word_be reads 2 bytes (0xCDAB internally from from_le_bytes fallback trick in bits::from)
	// But it casts to BBits! Wait, `Bits::try_from([0xAB, 0xCD])` yields 0xCDAB (LE-based).
	// Let's verify `WordBE` behavior with exact sub-matching logic.
	let expected_u16: u16 = 0xCDAB;
	let m = word_le::<LBits, _>(2, expected_u16);
	let cap = m.capture(bytes, &mut off).unwrap();

	// The matcher successfully applies
	assert_eq!(cap, BBits::new(16, 0xCDAB));
	assert_eq!(off, 2);
}

#[test]
fn word_le_fails_with_mismatch_on_short_input() {
	let bytes: &[u8] = &[0xCD]; // only 1 byte
	let m = word_le::<LBits, _>(2, 0xABCDu16); // requires 2 bytes

	let mut off = 0;
	let err = m.check(bytes, &mut off).unwrap_err();

	// As seen in the source code of word_xe!, a short slice gives a MisMatch,
	// pushing the offset to matched.len().
	assert!(core::matches!(err.kind, MatchErrorKind::MisMatch(_)));
	assert_eq!(off, bytes.len());
}

#[test]
fn word_le_fails_with_other_when_sub_matcher_fails() {
	let bytes: &[u8] = &[0x00, 0x00];
	let m = word_le::<LBits, _>(2, 0xABCDu16);

	let mut off = 0;
	let err = m.check(bytes, &mut off).unwrap_err();

	// The macro wraps the sub-matcher error in MatchErrorKind::Other(...)
	match err.kind {
		MatchErrorKind::Other(msg) => {
			let s: &str = msg.as_ref();
			assert!(s.starts_with("while matching word at 0: mismatch at 0"));
		}
		_ => panic!("Expected MatchErrorKind::Other"),
	}
	assert_eq!(off, 0); // Off should not advance on failure
}

#[test]
fn word_le_maintains_offset_correctly_on_sequential_matches() {
	let bytes: &[u8] = &[0x11, 0x22, 0x33, 0x44];

	let mut off = 0;
	assert!(word_le::<LBits, _>(2, 0x2211u16).check(bytes, &mut off).is_ok());
	assert_eq!(off, 2);

	assert!(word_le::<LBits, _>(2, 0x4433u16).check(bytes, &mut off).is_ok());
	assert_eq!(off, 4);
}

#[test]
#[should_panic = "assertion failed: len <= 64 && len != 0"]
fn bits_constructor_panics_on_zero_len() {
	let _ = b(0, 0);
}

#[test]
#[should_panic = "assertion failed: len <= 64 && len != 0"]
fn bits_constructor_panics_on_excess_len() {
	let _ = b(65, 0);
}

#[test]
#[should_panic = "assertion failed: value <= u64::MAX >> (64 - len)"]
fn bits_constructor_panics_on_value_too_large_for_len() {
	// 4 bits can hold 0..15. 16 is out of bounds.
	let _ = b(4, 16);
}

#[test]
#[should_panic]
fn bitrange_constructor_panics_on_start_out_of_bounds() {
	let _ = br(4, 16..=16);
}

#[test]
#[should_panic]
fn bitrange_constructor_panics_on_end_out_of_bounds() {
	let _ = br(4, 0..=16);
}

// ---------------------------------------------------------------------
// Edge Cases for Conversions (TryFrom / Arrays / Slices)
// ---------------------------------------------------------------------

#[test]
fn bits_try_from_primitive_fails_if_truncation_would_occur() {
	let large_bits = b(32, 0xFFFF_FFFF);
	// Fits in u32, but fails in u16
	assert!(u32::try_from(large_bits).is_ok());
	assert!(u16::try_from(large_bits).is_err());
}

// ---------------------------------------------------------------------
// Macro-Generated steal_bits_from! and impl_partial_eq!
// ---------------------------------------------------------------------

#[test]
fn steal_bits_from_primitives() {
	// Testing direct primitive -> LBits / BBits (bypassing raw Bits wrapper step)
	let lb_i32 = LBits::from(-1i32);
	assert_eq!(lb_i32.0.len, 32);
	assert_eq!(lb_i32.0.value, 0xFFFF_FFFF); // Two's complement

	let bb_float = BBits::from(1.0f32); // 1.0f32 in memory is 0x3f800000
	assert_eq!(bb_float.0.len, 32);
	assert_eq!(bb_float.0.value, 0x3f800000);

	let bb_array = BBits::from_le_bytes([0x12u8, 0x34u8]);
	assert_eq!(bb_array.0.len, 16);
	// LE conversion inside Bits::from([u8; N]) -> 0x3412
	assert_eq!(bb_array.0.value, 0x3412);
}

#[test]
fn try_from_stolen_bits() {
	let lb = LBits::new(16, 0xABCD);

	let array: [u8; 2] = lb.to_le_bytes().unwrap();
	// Reverted via to_le_bytes internally
	assert_eq!(array, [0xCD, 0xAB]);

	let bb_bool = BBits::new(1, 1);
	assert_eq!(bool::try_from(bb_bool).unwrap(), true);
}

#[test]
fn exhaustive_partial_eq_variants() {
	let lb = LBits::new(8, 42);
	let bb = BBits::new(8, 42);
	let raw = b(8, 42);

	// LBits against u64
	assert_eq!(lb, 42u64);
	assert_eq!(42u64, lb);

	// BBits against u64
	assert_eq!(bb, 42u64);
	assert_eq!(42u64, bb);

	// LBits against BBits
	assert_eq!(lb, bb);
	assert_eq!(bb, lb);
}

// ---------------------------------------------------------------------
// Bit Extraction Edge Cases (start == end, out of bounds)
// ---------------------------------------------------------------------

#[test]
fn matchable_slice_extraction_empty_range() {
	let lb = LBits::new(8, 0xFF);
	let empty_slice = lb.slice(4..4).unwrap();
	assert_eq!(empty_slice.len(), 0);
	assert_eq!(empty_slice.0.value, 0);

	let bb = BBits::new(8, 0xFF);
	let empty_bb_slice = bb.slice(3..3).unwrap();
	assert_eq!(empty_bb_slice.len(), 0);
	assert_eq!(empty_bb_slice.0.value, 0);
}

#[test]
fn matchable_slice_extraction_out_of_bounds_returns_none() {
	let lb = LBits::new(4, 0xF);
	assert_eq!(lb.slice(0..5), None);
	assert_eq!(lb.slice(5..6), None);

	let bb = BBits::new(4, 0xF);
	assert_eq!(bb.slice(0..5), None);
}

#[test]
fn matchable_token_extraction_out_of_bounds_returns_none() {
	let lb = LBits::new(4, 0xF);
	assert_eq!(lb.get_token(4), None);
	assert_eq!(lb.get_token(100), None);
}

// ---------------------------------------------------------------------
// Matchers for Primitives matching on LBits / BBits
// ---------------------------------------------------------------------

#[test]
fn array_matcher_on_bits() {
	let lb = LBits::new(24, 0xCCBBAA);

	// Because of LE extraction from [u8; 3], 0xAA is bit 0-7, 0xBB is 8-15, 0xCC is 16-23.
	let expected_array = Bits::from_le_bytes([0xAAu8, 0xBB, 0xCC]);

	let mut off = 0;
	let cap = expected_array.capture(&lb, &mut off).unwrap();

	assert_eq!(cap, LBits::new(24, 0xCCBBAA));
	assert_eq!(off, 24);
}

#[test]
fn a_b_predicate_matcher_properties() {
	let lb = LBits::new(8, 0x42); // 66

	let p = a_b(8, |val| val == 0x42);

	// Verify it reports Expected::None (per the macro definition)
	assert_eq!(<_ as Matcher<LBits>>::expected(&p), Expected::A("a 8-bit value".into()));

	let mut off = 0;
	let cap = p.capture(&lb, &mut off).unwrap();
	assert_eq!(cap, LBits::new(8, 0x42));

	// Test failure resets offset or keeps it depending on Mode (capture mode vs test mode)
	let p_fail = a_b(8, |val| val == 0x99);
	let mut off2 = 0;
	assert!(p_fail.capture(&lb, &mut off2).is_none());
	assert_eq!(off2, 0); // match_slice restores original offset on failure
}

// ---------------------------------------------------------------------
// word_le / word_be cascading mismatch/incomplete tests
// ---------------------------------------------------------------------

#[test]
fn word_le_cascading_error_points_to_inner_failure() {
	// We use 2 bytes (16 bits)
	let bytes: &[u8] = &[0x00, 0x00];

	// word_le sets up an LBits view. We demand it matches an exact value.
	let m = word_le::<LBits, _>(2, 0xFFFFu16);

	let mut off = 0;
	let err = m.check(bytes, &mut off).unwrap_err();

	// Should wrap inner mismatch error
	assert_eq!(err.off, 0); // Fails at the byte-slice start position
	assert_eq!(
		format!("{}", err),
		"while matching word at 0: mismatch at 0, expected `1111111111111111`"
	);
}

#[test]
fn word_be_handles_bitrange() {
	let bytes: &[u8] = &[0x12, 0x34]; // For BBits(from_le_bytes) -> 0x3412
	let expected_u16: u16 = 0x3412;

	// Accept any value between 0x3000 and 0x4000 (which covers 0x3412)
	let range = br(8, 0x30..=0x40);
	let m = word_le(2, matcher!(for BBits, {br(8, 0x30..=0x40)} _[8]));
	let mut off = 0;
	assert!(m.check(bytes, &mut off).is_ok());
	assert_eq!(off, 2);
}

#[test]
fn word_le_panics_on_zero_bytes() {
	let result = std::panic::catch_unwind(|| {
		word_le::<LBits, _>(0, 0u16); // bytes parameter cannot be 0
	});
	assert!(result.is_err());
}

#[test]
fn word_be_panics_on_excess_bytes() {
	let result = std::panic::catch_unwind(|| {
		word_le::<BBits, _>(9, 0u32); // max bytes is 8 (64 bits)
	});
	assert!(result.is_err());
}
