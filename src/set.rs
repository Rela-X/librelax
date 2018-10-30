use std::collections::BTreeSet;
use std::fmt;
use std::iter;
use std::string::ToString;

/* HashSet does not implement Hash, so it cannot be nested (currently) */
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Set(BTreeSet<SetElement>);

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
}

impl fmt::Display for Set {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{{")?;
		let mut it = self.0.iter();
		if let Some(e) = it.next() {
			write!(f, "{}", e)?;
			for e in it {
				write!(f, " {}", e)?;
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

// Why is this not covered by From<&'a ToString> ??
impl<'a> From<&'a char> for SetElement {
	fn from(c: &'a char) -> SetElement {
		SetElement::Str(c.to_string())
	}
}

impl<'a> From<&'a ToString> for SetElement {
	fn from(x: &'a ToString) -> SetElement {
		SetElement::Str(x.to_string())
	}
}
