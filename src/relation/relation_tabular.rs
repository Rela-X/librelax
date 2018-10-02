use std;

use set::{Set, SetElement};
use relation::relation::Relation;
use relation::endorelation::Endorelation;

pub trait RelationTabular {
	fn get_domain(&self) -> (&Set, &Set);
	fn eval_at(&self, ix: usize, iy: usize) -> bool;
	fn ixs(&self) -> std::ops::Range<usize> { 0..self.get_domain().0.cardinality() }
	fn iys(&self) -> std::ops::Range<usize> { 0..self.get_domain().1.cardinality() }
}

macro_rules! cross {
	($p:expr, $q:expr) => ($p.flat_map(
		|e| std::iter::repeat(e).zip($q.clone())
	))
}
macro_rules! cross_uniq {
	($p:expr, $q:expr) => ($p.enumerate().flat_map(
		|(i, e)| std::iter::repeat(e).zip($q.skip(i+1).clone())
	))
}

impl<R> Relation for R
where R: RelationTabular,
{
	fn eval(&self, x: &SetElement, y: &SetElement) -> bool {
		let ix = self.get_domain().0.iter().position(|e| e == x).unwrap();
		let iy = self.get_domain().1.iter().position(|e| e == y).unwrap();
		return self.eval_at(ix, iy);
	}

	fn is_homogeneous(&self) -> bool { self.get_domain().0 == self.get_domain().1 }

	fn is_injective(&self) -> bool {
		/*
		self.iys().all(
			|iy| self.ixs()
				.filter(|ix| self.eval_at(ix, iy))
				.take(2)
				.len() <= 1
		)
		*/
		for iy in self.iys() {
			let mut found_one = false;
			for ix in self.ixs() {
				if self.eval_at(ix, iy) {
					if !found_one {
						found_one = true;
					} else {
						return false;
					}
				}
			}
		}
		return true;
	}
	fn is_functional(&self) -> bool {
		/*
		self.ixs().all(
			|ix| self.iys()
				.filter(|iy| self.eval_at(ix, iy))
				.take(2)
				.len() <= 1
		)
		*/
		for ix in self.ixs() {
			let mut found_one = false;
			for iy in self.iys() {
				if self.eval_at(ix, iy) {
					if !found_one {
						found_one = true;
					} else {
						return false;
					}
				}
			}
		}
		return true;
	}
	fn is_lefttotal(&self) -> bool {
		self.ixs().all(                                 // ∀x:
			|ix| self.iys().any(                    // ∃y:
				|iy| self.eval_at(ix, iy)       // (x,y) ∈ self
			)
		)
	}
	fn is_surjective(&self) -> bool {
		self.iys().all(                                 // ∀y:
			|iy| self.ixs().any(                    // ∃x:
				|ix| self.eval_at(ix, iy)       // (x,y) ∈ self
			)
		)
	}
	//fn is_bijective(&self) -> bool;
	//fn is_function(&self) -> bool;
}

impl<R> Endorelation for R
where R: RelationTabular,
{
	fn is_reflexive(&self) -> bool {
		if !self.is_homogeneous() { return false; }
		self.ixs().all(|i| self.eval_at(i, i))
	}
	fn is_irreflexive(&self) -> bool {
		if !self.is_homogeneous() { return false; }
		self.ixs().all(|i| !self.eval_at(i, i))
	}
	fn is_antisymmetric(&self) -> bool {
		if !self.is_homogeneous() { return false; }
		cross_uniq!(self.ixs(), self.iys()).all(
			|(ix, iy)| !self.eval_at(ix, iy) || !self.eval_at(iy, ix)
		)
		/*
		if !self.is_homogeneous() { return false; }
		for ix in 1..self.get_domain().0.cardinality() {
			for iy in 0..ix {
				if self.eval_at(ix, iy) && self.eval_at(iy, ix) {
					return false;
				}
			}
		}
		return true;
		*/
	}
	fn is_transitive(&self) -> bool {
		if !self.is_homogeneous() { return false; }
		cross!(self.ixs(), self.iys())
			.filter(|(ix, iy)| ix != iy)
			.filter(|(ix, iy)| self.eval_at(*ix, *iy))
			.all(
				|(ix, iy)| self.ixs()
					.filter(|&iz| self.eval_at(iy, iz))
					.all(|iz| self.eval_at(ix, iz))
			)
		/*
		if !self.is_homogeneous() { return false; }
		for ix in self.ixs() {
			for iy in self.iys() {
				if ix == iy { continue; }
				if !self.eval_at(ix, iy) { continue; } // no xRy
				for iz in self.ixs() {
					if !self.eval_at(iy, iz) { continue; } // no yRz
					if !self.eval_at(ix, iz) { return false; } // no xRz
				}
			}
		}
		return true;
		*/
	}

	fn is_symmetric(&self) -> bool {
		if !self.is_homogeneous() { return false; }
		cross_uniq!(self.ixs(), self.iys()).all(
			|(ix, iy)| self.eval_at(ix, iy) == self.eval_at(iy, ix)
		)
	}
	//fn is_asymmetric(&self) -> bool;

	//fn is_preorder(&self) -> bool;
	//fn is_equivalent(&self) -> bool;
	//fn is_partial_order(&self) -> bool;

	fn is_difunctional(&self) -> bool {
		if !self.is_homogeneous() { return false; }
		for (ix, iy) in cross!(self.ixs(), self.iys()) {
			if !self.eval_at(ix, iy) { continue; }
			for iz in (ix+1)..self.get_domain().0.cardinality() {
				if !self.eval_at(iz, iy) { continue; }
				for iw in self.ixs() {
					if self.eval_at(ix, iw) != self.eval_at(iz, iw) {
						return false;
					}
				}
			}
		}
		return true;
	}

	fn is_lattice(&self) -> bool {
		if !self.is_homogeneous() { return false; } // TODO? Error
		if !self.is_partial_order() { return false; } // TODO? Error
		// TODO
		false
	}
	fn is_sublattice<T: Endorelation>(&self, other: &T) -> bool {
		// TODO
		false
	}
}

pub fn eq<P: RelationTabular, Q: RelationTabular>(p: &P, q: &Q) -> bool {
	p.get_domain() == q.get_domain() && cross!(p.ixs(), p.iys()).all(
		|(ix, iy)| p.eval_at(ix, iy) == q.eval_at(ix, iy)
	)
}
pub fn neq<P: RelationTabular, Q: RelationTabular>(p: &P, q: &Q) -> bool {
	!eq(p, q)
}

pub mod tests {
	use super::*;

	pub fn union<P, Q, R, S, T>(neutral: &P, absorbing: &Q, a: &R, b: &S, c: &T)
	where P: RelationTabular + std::fmt::Debug,
	      Q: RelationTabular + std::fmt::Debug,
	      R: RelationTabular + std::fmt::Debug,
	      S: RelationTabular + std::fmt::Debug,
	      T: RelationTabular + std::fmt::Debug,
	{
		let r = a;
		assert!(r.is_homogeneous());

		// union: neutral element
		assert_eq!(R::union(r, neutral), *r);
		// union: absorbing element
		assert_eq!(R::union(r, absorbing), *absorbing);
		// union: idempotence
		assert_eq!(R::union(r, r), *r);
		// union: associativity
		assert_eq!(
			R::union(a, &R::union(b, c)),
			R::union(&R::union(a, b), c),
		);
		// union: commutativity
		assert_eq!(R::union(a, b), R::union(b, a));
	}

	pub fn intersection<P, Q, R, S, T>(neutral: &P, absorbing: &Q, a: &R, b: &S, c: &T)
	where P: RelationTabular + std::fmt::Debug,
	      Q: RelationTabular + std::fmt::Debug,
	      R: RelationTabular + std::fmt::Debug,
	      S: RelationTabular + std::fmt::Debug,
	      T: RelationTabular + std::fmt::Debug,
	{
		let r = a;
		assert!(r.is_homogeneous());

		// intersection: neutral element
		assert_eq!(R::intersection(r, neutral), *r);
		// intersection: absorbing element
		assert_eq!(R::intersection(r, absorbing), *absorbing);
		// intersection: idempotence
		assert_eq!(R::intersection(r, r), *r);
		// intersection: associativity
		assert_eq!(
			R::intersection(a, &R::intersection(b, c)),
			R::intersection(&R::intersection(a, b), c)
		);
		// intersection: commutativity
		assert_eq!(R::intersection(a, b), R::intersection(b, a));
	}

	pub fn distributivity_union_intersection<R, S, T>(a: &R, b: &S, c: &T)
	where R: RelationTabular + std::fmt::Debug,
	      S: RelationTabular + std::fmt::Debug,
	      T: RelationTabular + std::fmt::Debug,
	{
		// left distributivity (union, intersection)
		assert_eq!(
			R::intersection(a, &R::union(b, c)),
			R::union(&R::intersection(a, b), &R::intersection(a, c)),
		);
		// right distributivity (union, intersection)
		assert_eq!(
			R::intersection(&R::union(a, b), c),
			R::union(&R::intersection(a, c), &R::intersection(b, c)),
		);
	}

	pub fn de_morgan<R>(a: &R, b: &R)
	where R: RelationTabular + std::fmt::Debug
	{
		assert_eq!(
			R::complement(&R::union(a, b)),
			R::intersection(&R::complement(a), &R::complement(b)),
		);
		assert_eq!(
			R::complement(&R::intersection(a, b)),
			R::union(&R::complement(a), &R::complement(b)),
		);
	}
}
