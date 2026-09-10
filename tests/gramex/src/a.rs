use std::ops::{Deref, Range};

use gramex::{MatchAble, Matcher, Mode, matcher, matches, result::MatchResult};
use lean_string::LeanString;

fn a() {
	let a = 'a';
	assert!(matches!("abc".to_string(), a {|v, o| true} {Box::new('c')} ));
	assert!(matches!(A(1), _ & 'a'));
}

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
