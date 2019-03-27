//! A binary `Relation`, represented as a `std::vec::Vec` of `bool`ean values.

use std::vec::Vec;
use std::fmt;
use std::string::ToString;

use crate::set::Set;
use crate::relation::{Relation, Endorelation};

/// A binary `Relation`, represented as a `std::vec::Vec` of `bool`ean values.
/// Values are storen row-wise, as shown in the following example, to form an
/// incidence matrix:
///
/// ```
/// let table = vec![
/// 	true,  false, // (0,0), (0,1),
/// 	false, true,  // (1,0), (1,1),
/// ];
/// ```
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RelationVec {
//	homogeneous: Option<bool>,
	domain: (Set, Set),
	table: Vec<bool>,
}

impl RelationVec {
	/// Create a new [`RelationVec`] from the given domain and and incidence matrix.
	pub fn new(domain: (Set, Set), table: Vec<bool>) -> Self {
		RelationVec {
			domain: domain,
			table: table,
		}
	}
	/// Create a new [`RelationVec`] from the given `Relation`.
	pub fn from_relation<R: Relation>(r: &R) -> Self {
		let d = r.get_domain();
		RelationVec {
			domain: (d.0.clone(), d.1.clone()),
			table: (0..(d.0.cardinality() * d.1.cardinality()))
				.map(|i| (i / d.0.cardinality(), i % d.1.cardinality()))
				.map(|(ix, iy)| r.eval_at(ix, iy))
				.collect(),
		}
	}
	pub fn from_predicate<'a, T, P>(set: &'a[T], predicate: P) -> Self
	where T: ToString,
	      P: FnMut((&T, &T)) -> bool,
	{
		RelationVec {
			domain: (set.iter().map(T::to_string).collect(), set.iter().map(T::to_string).collect()),
			table: (0..set.len().pow(2))
				.map(|i| (i / set.len(), i % set.len()))
				.map(|(ix, iy)| (&set[ix], &set[iy]))
				.map(predicate)
				.collect(),
		}
	}
	/// Calculate the position of (ix, iy) in the `RelationVec`'s `table`.
	fn get_table_index(&self, ix: usize, iy: usize) -> usize {
		ix * self.domain.1.cardinality() + iy
	}

}

impl Relation for RelationVec {
	fn get_domain(&self) -> (&Set, &Set) {
		(&self.domain.0, &self.domain.1)
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		let i = self.get_table_index(ix, iy);
		return self.table[i];
	}
}

impl Endorelation for RelationVec {}

impl fmt::Display for RelationVec {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "[TODO] fmt::Display for RelationVec :)")
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::set::Set;

	const ALPHABET: [char; 26] = [
		'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
		'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
		'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
		'y', 'z',
	];

	#[test]
	fn eval() {
		let set: Set = ALPHABET[0..3].iter().collect();
		let r1 = RelationVec {
			domain: (set.iter().cloned().collect(), set.iter().cloned().collect()),
			table: vec![
				true, false, false,
				false, true, false,
				false, false, true
			],
		};
		/*
		assert!( r1.eval(&'a', &'a')); assert!(!r1.eval(&'a', &'b')); assert!(!r1.eval(&'a', &'c'));
		assert!(!r1.eval(&'b', &'a')); assert!( r1.eval(&'b', &'b')); assert!(!r1.eval(&'b', &'c'));
		assert!(!r1.eval(&'c', &'a')); assert!(!r1.eval(&'c', &'b')); assert!( r1.eval(&'c', &'c'));
		*/
	}
}
