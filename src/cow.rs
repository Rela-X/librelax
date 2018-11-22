use std::borrow::Cow;
use std::ops::Deref;

/// A local wrapper around `std::borrow::Cow`
/// It is required to add blanket implementations for all types used in Relax.
/// see E0210 and Rust-RFC 1023
#[derive(Clone, Debug)]
pub struct LCow<'a, T: 'a + Clone>(Cow<'a, T>);

impl<'a, T: Clone> Deref for LCow<'a, T> {
	type Target = Cow<'a, T>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a, T: Clone> From<T> for LCow<'a, T> {
	fn from(t: T) -> Self {
		LCow(Cow::Owned(t.to_owned()))
	}
}
impl<'a, T: Clone> From<&'a T> for LCow<'a, T> {
	fn from(t: &'a T) -> Self {
		LCow(Cow::Borrowed(t))
	}
}
