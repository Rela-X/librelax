use std::vec::Vec;
use std::fmt;
use std::string::ToString;

use set::{Set, SetElement};
use relation::{Relation, Endorelation};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RelationVec {
//	homogeneous: Option<bool>,
	domain: (Set, Set),
	table: Vec<bool>,
}

impl RelationVec {
	pub fn new(p: Set, q: Set, table: Vec<bool>) -> Self {
		RelationVec {
			domain: (p, q),
			table: table,
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
	fn get_table_index(&self, ix: usize, iy: usize) -> usize {
		ix * self.domain.0.cardinality() + iy
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
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "[TODO] fmt::Display for RelationVec :)")
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use Set;

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
