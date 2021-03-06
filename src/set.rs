use std::collections::BTreeSet;
use std::fmt;
use std::iter;
use std::string::ToString;

/// Implementation of a [`Set`].
/// Slightly rudimentary at the moment.
/* HashSet does not implement Hash, so it cannot be nested (currently) */
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Set(BTreeSet<SetElement>);

/// A [`SetElement`] is either a [`std::string::String`] or a nested [`Set`].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SetElement {
	Str(String),
	Set(Set),
}

impl Set {
	pub fn new() -> Set {
		Set(BTreeSet::new())
	}
	pub fn cardinality(&self) -> usize {
		self.0.len()
	}
	pub fn iter(&self) -> ::std::collections::btree_set::Iter<SetElement> {
		self.0.iter()
	}
	pub fn is_subset(&self, other: &Set) -> bool {
		self.0.is_subset(&other.0)
	}
	pub fn union<'a>(&'a self, other: &'a Set) -> ::std::collections::btree_set::Union<'a, SetElement> {
		self.0.union(&other.0)
	}
	pub fn intersection<'a>(&'a self, other: &'a Set) -> ::std::collections::btree_set::Intersection<'a, SetElement> {
		self.0.intersection(&other.0)
	}
	pub fn intersection_enumerated<'a>(&'a self, other: &'a Set) -> EnumeratedIntersection<'a> {
		EnumeratedIntersection {
			s: self.0.iter().enumerate().peekable(),
			u: other.0.iter().enumerate().peekable(),
		}
	}
}

#[derive(Clone, Debug)]
pub struct EnumeratedIntersection<'a> {
	s: std::iter::Peekable<std::iter::Enumerate<std::collections::btree_set::Iter<'a, SetElement>>>,
	u: std::iter::Peekable<std::iter::Enumerate<std::collections::btree_set::Iter<'a, SetElement>>>,
}
impl<'a> Iterator for EnumeratedIntersection<'a> {
	type Item = ((usize, usize), &'a SetElement);
	fn next(&mut self) -> Option<Self::Item> {
		loop {
			if self.s.peek().is_none() || self.u.peek().is_none() {
				return None
			}
			match Ord::cmp(self.s.peek()?.1, self.u.peek()?.1) {
				std::cmp::Ordering::Less => {
					self.s.next();
				}
				std::cmp::Ordering::Equal => {
					let s = self.s.next()?;
					let u = self.u.next()?;
					return Some(((s.0, u.0), u.1));
				}
				std::cmp::Ordering::Greater => {
					self.u.next();
				}
			}
		}
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		(0, Some(core::cmp::min(self.s.len(), self.u.len())))
	}
}

impl fmt::Display for Set {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{{")?;
		let mut it = self.0.iter();
		if let Some(e) = it.next() {
			write!(f, "{}", e)?;
			for e in it {
				write!(f, "{}{}", " ", e)?;
			}
		}
		write!(f, "}}")
	}
}

impl<T: Into<SetElement>> iter::FromIterator<T> for Set {
	fn from_iter<I: iter::IntoIterator<Item = T>>(iter: I) -> Set {
		let mut s = Set::new();
		s.0.extend(iter.into_iter().map(T::into));
		return s;
	}
}

impl fmt::Display for SetElement {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			SetElement::Str(s) => write!(f, "{}", s),
			SetElement::Set(s) => write!(f, "{}", s),
		}
	}
}

// TODO find more generic SetElement::from(_) implementations

impl From<String> for SetElement {
	fn from(s: String) -> SetElement {
		SetElement::Str(s)
	}
}

impl From<i32> for SetElement {
	fn from(i: i32) -> SetElement {
		SetElement::Str(i.to_string())
	}
}

// Why is this not covered by From<&ToString> ??
impl From<&char> for SetElement {
	fn from(c: &char) -> SetElement {
		SetElement::Str(c.to_string())
	}
}

impl From<&ToString> for SetElement {
	fn from(x: &ToString) -> SetElement {
		SetElement::Str(x.to_string())
	}
}
