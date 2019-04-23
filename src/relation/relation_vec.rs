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
	use crate::relation;
	use proptest::{prelude::*, collection::vec};

	type Domain = (Set, Set);

	const ALPHABET: [char; 26] = [
		'a','b','c','d','e','f','g','h','i',
		'j','k','l','m','n','o','p','q','r',
		's','t','u','v','w','x','y','z',
	];

	#[test]
	fn eval_at_homogeneous() {
		let x: Set = (0..3).collect();
		let r = RelationVec {
			domain: (x.clone(), x),
			table: vec![
				true,  false, false,
				false, true,  false,
				false, false, true,
			],
		};
		assert!( r.eval_at(0, 0)); assert!(!r.eval_at(0, 1)); assert!(!r.eval_at(0, 2));
		assert!(!r.eval_at(1, 0)); assert!( r.eval_at(1, 1)); assert!(!r.eval_at(1, 2));
		assert!(!r.eval_at(2, 0)); assert!(!r.eval_at(2, 1)); assert!( r.eval_at(2, 2));
	}
	#[test]
	fn eval_at_heterogeneous() {
		let x: Set = (0..3).collect();
		let y: Set = ['a', 'b'].iter().collect();
		let r = RelationVec {
			domain: (x, y),
			table: vec![
				true,  false,
				false, true,
				false, false,
			],
		};
		assert!( r.eval_at(0, 0)); assert!(!r.eval_at(0, 1));
		assert!(!r.eval_at(1, 0)); assert!( r.eval_at(1, 1));
		assert!(!r.eval_at(2, 0)); assert!(!r.eval_at(2, 1));
	}

	mod endorelation {
		use super::*;

		#[test]
		fn is_reflexive() {
			let domain: Domain = ((0..3).collect(), (0..3).collect());
			let table = vec![
				true,  false, false,
				true,  true,  false,
				false, false, true,
			];
			let r1 = RelationVec::new(domain, table);
			assert!(r1.is_reflexive());
		}
		#[test]
		fn is_irreflexive() {
			let domain: Domain = ((0..4).collect(), (0..4).collect());
			let table = vec![
				false, true,  true,  true,
				false, false, true,  true,
				true,  true,  false, true,
				false, true,  false, false,
			];
			let r1 = RelationVec::new(domain, table);
			assert!(r1.is_irreflexive());
		}
		#[test]
		fn is_antisymmetric() {
			let domain: Domain = ((0..4).collect(), (0..4).collect());
			let table = vec![
				true,  false, false, false,
				false, true,  true,  false,
				true,  false, false, false,
				false, true,  false, true,
			];
			let r1 = RelationVec::new(domain, table);
			assert!(r1.is_antisymmetric());
		}
		#[test]
		fn is_transitive() {
			let domain: Domain = ((0..4).collect(), (0..4).collect());
			let table = vec![
				false, false, false, false,
				true,  false, true,  false,
				true,  false, false, false,
				true,  true,  true,  false,
			];
			let r1 = RelationVec::new(domain, table);
			assert!(r1.is_transitive());
		}

		#[test]
		fn is_symmetric() {
			let domain: Domain = ((0..4).collect(), (0..4).collect());
			let table = vec![
				true,  false, true,  false,
				false, true,  true,  true,
				true,  true,  false, false,
				false, true,  false, true,
			];
			let r1 = RelationVec::new(domain, table);
			assert!(r1.is_symmetric());
		}
		#[test]
		fn is_asymmetric() {
			let domain: Domain = ((0..4).collect(), (0..4).collect());
			let table = vec![
				false, false, false, false,
				false, false, true,  false,
				true,  false, false, false,
				false, true,  false, false,
			];
			let r1 = RelationVec::new(domain, table);
			assert!(r1.is_asymmetric());
		}

		#[test]
		fn is_preorder() {
			let domain: Domain = ((0..5).collect(), (0..5).collect());
			let table = vec![
				true,  false, true,  true,  false,
				false, true,  false, false, false,
				false, false, true,  true,  false,
				false, false, true,  true,  false,
				false, false, false, false, true,
			];
			let r1 = RelationVec::new(domain, table);
			assert!(r1.is_reflexive());
			assert!(r1.is_transitive());
			assert!(!r1.is_symmetric());
			assert!(r1.is_preorder());
			assert!(!r1.is_equivalent());
		}
		/*
		fn is_partial_order() {}
		*/
		#[test]
		fn is_equivalent() {
			let domain: Domain = ((0..5).collect(), (0..5).collect());
			let table = vec![
				true,  false, true,  true,  false,
				false, true,  false, false, false,
				true,  false, true,  true,  false,
				true,  false, true,  true,  false,
				false, false, false, false, true,
			];
			let r1 = RelationVec::new(domain, table);
			assert!(r1.is_reflexive());
			assert!(r1.is_transitive());
			assert!(r1.is_symmetric());
			assert!(r1.is_preorder());
			assert!(r1.is_equivalent());
		}

		#[test]
		fn is_difunctional() {
			let domain: Domain = ((0..10).collect(), (0..10).collect());
			let table = vec![
				false, false, true,  false, false, true,  false, true,  true,  true,
				false, false, false, true,  true,  false, false, false, false, false,
				false, false, false, true,  true,  false, false, false, false, false,
				false, false, true,  false, false, true,  false, true,  true,  true,
				false, false, false, false, false, false, false, false, false, false,
				true,  false, false, false, false, false, false, false, false, false,
				true,  false, false, false, false, false, false, false, false, false,
				false, true,  false, false, false, false, false, false, false, false,
				false, false, false, true,  true,  false, false, false, false, false,
				false, false, false, false, false, false, false, false, false, false,
			];
			let r1 = RelationVec::new(domain, table);
			assert!(r1.is_difunctional());
		}

		/*
		fn is_lattice() {}
		fn is_sublattice() {}

		fn is_injective() {}
		fn is_functional() {}
		fn is_lefttotal() {}
		fn is_surjective() {}
		fn is_bijective() {}
		fn is_function() {}
		*/

		#[test]
		fn relation_mod8_equal() {
			let n32: Vec<u8> = (1..=32).collect();
			let r = RelationVec::from_predicate(&n32, |(&x, &y)| x % 8 == y % 8);
			// TODO verify
			assert!(r.is_reflexive());
			assert!(!r.is_irreflexive());
			assert!(!r.is_antisymmetric());
			assert!(r.is_symmetric());
			assert!(!r.is_asymmetric());
			assert!(r.is_transitive());
			assert!(r.is_lefttotal());
			assert!(!r.is_injective());
			assert!(r.is_surjective());
		}
		#[test]
		fn relation_divisible() {
			let n32: Vec<u8> = (1..=32).collect();
			let r = RelationVec::from_predicate(&n32, |(&x, &y)| y % x == 0);
			assert!(r.is_reflexive());
			assert!(!r.is_irreflexive());
			assert!(r.is_antisymmetric());
			assert!(!r.is_symmetric());
			assert!(!r.is_asymmetric());
			assert!(r.is_transitive());
			assert!(r.is_lefttotal());
			assert!(!r.is_injective());
			assert!(r.is_surjective());
		}
	}

	/* property tests */

	/// Generates domains ({1,2, ..., n}, {1,2, ..., n})
	/// with n: [2;32[
	fn domain_homogeneous() -> impl Strategy<Value = Domain> {
		// draw a random number n from [2;32[
		(2..32)
			// generate a sequence [1;n]
			.prop_map(|n| (1..n+1))
			// create a set {1, 2, ..., n}
			.prop_map(|seq| seq.collect::<Set>())
			// create a domain-tuple (x, x)
			.prop_map(|s| (s.clone(), s))
	}

	/// Generates domains ({1,2, ..., n}, {'a, 'b', ..., ch})
	/// with n: [2;32[ and ch: ['b';'z']
	fn domain_heterogeneous() -> impl Strategy<Value = Domain> {
		// draw random numbers n, c from [2;32[, [2;26[
		(2..32usize, 2..26usize)
			// generate sequences [1;n], [0;c]
			.prop_map(|(n, c)| (1..n+1, 0..c+1))
			// map n to i32, c to char
			.prop_map(
				|(n, c)| (
					n.map(|n| n as i32),
					c.map(|c| &ALPHABET[c])
				)
			)
			// create set-tuple
			.prop_map(
				|(nseq, cseq)| (
					nseq.collect::<Set>(), cseq.collect::<Set>()
				)
			)
	}

	fn domain_arbitrary() -> impl Strategy<Value = Domain> {
		prop_oneof![
			domain_homogeneous(),
			domain_heterogeneous(),
		]
	}

	fn relation_for_domain(domain: Domain) -> impl Strategy<Value = RelationVec> {
		let dimension = domain.0.cardinality() * domain.1.cardinality();
		let domains = Just(domain);
		let tables = vec(any::<bool>(), dimension);
		(domains, tables).prop_map(|(d, t)| RelationVec::new(d, t))
	}

	fn relation_arbitrary(domain: impl Strategy<Value = Domain>) -> impl Strategy<Value = RelationVec> {
		domain.prop_flat_map(relation_for_domain)
	}

	prop_compose! {
		fn three_rels() (d in domain_homogeneous()) (
			r in relation_for_domain(d.clone()),
			s in relation_for_domain(d.clone()),
			t in relation_for_domain(d),
		) -> (RelationVec, RelationVec, RelationVec) {
			(r, s, t)
		}
	}

	proptest! {
		#[test]
		fn relation_properties(r in relation_arbitrary(domain_arbitrary())) {
			relation::relation::tests::relation_property_test(&r);
		}
		#[test]
		fn relation_complement(r in relation_arbitrary(domain_arbitrary())) {
			relation::relation::tests::complement(&r);
		}
		#[test]
		fn relation_concatenation((r, s, t) in three_rels()) {
			relation::relation::tests::concatenation(&r, &s, &t);
		}
		#[test]
		fn relation_distributivity_concatenation((r, s, t) in three_rels()) {
			relation::relation::tests::distributivity_concatenation(&r, &s, &t);
		}
		#[test]
		fn relation_converse(r in relation_arbitrary(domain_arbitrary())) {
			relation::relation::tests::converse(&r);
		}
		#[test]
		fn relation_distributivity_converse((r, s, _) in three_rels()) {
			relation::relation::tests::distributivity_converse(&r, &s);
		}
		#[test]
		fn relation_union((r, s, t) in three_rels()) {
			relation::relation::tests::union(&r, &s, &t);
		}
		#[test]
		fn relation_intersection((r, s, t) in three_rels()) {
			relation::relation::tests::intersection(&r, &s, &t);
		}
		#[test]
		fn relation_distributivity_union((r, s, t) in three_rels()) {
			relation::relation::tests::distributivity_union(&r, &s, &t);
		}
		#[test]
		fn relation_distributivity_intersection((r, s, t) in three_rels()) {
			relation::relation::tests::distributivity_intersection(&r, &s, &t);
		}
		#[test]
		fn relation_de_morgan((r, s, _) in three_rels()) {
			relation::relation::tests::de_morgan(&r, &s);
		}
	}

	proptest! {
		#[test]
		fn endorelation_properties(r in relation_arbitrary(domain_homogeneous())) {
			relation::endorelation::tests::endorelation_property_test(&r);
		}
	}
}
