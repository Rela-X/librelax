use std;

use set::Set;
use relation::Relation;

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

pub trait Endorelation : Relation {
	fn is_reflexive(&self) -> bool { /* xRx */
		if !self.is_homogeneous() { return false; }
		self.ixs().all(|i| self.eval_at(i, i))
	}
	fn is_irreflexive(&self) -> bool { /* aka strict */
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

	fn is_symmetric(&self) -> bool { /* xRy <=> yRx */
		if !self.is_homogeneous() { return false; }
		cross_uniq!(self.ixs(), self.iys()).all(
			|(ix, iy)| self.eval_at(ix, iy) == self.eval_at(iy, ix)
		)
	}
	fn is_asymmetric(&self) -> bool { self.is_irreflexive() && self.is_antisymmetric() }

	fn is_preorder(&self) -> bool { self.is_reflexive() && self.is_transitive() }
	fn is_partial_order(&self) -> bool { self.is_preorder() && self.is_antisymmetric() }
	fn is_equivalent(&self) -> bool { self.is_preorder() && self.is_symmetric() }

	fn is_difunctional(&self) -> bool { /* aka regular: xRy & zRy & zRw => xRw */
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

	fn empty(set: &Set) -> Empty {
		Empty { set: set }
	}
	fn universal(set: &Set) -> Universal {
		Universal { set: set }
	}
	fn identity(set: &Set) -> Identity {
		Identity { set: set }
	}
	
	//fn closure_reflexive<R: Endorelation>(r: &R) -> Union { R::union(r, &R::identity) }
	//fn closure_symmetric<R: Endorelation>(r: &R) -> Union { R::union(r, &R::converse(r)) }
	//fn closure_difunctional<R: Endorelation>(r: &R) -> R {}
	//fn closure_biorder<R: Endorelation>(r: &R) -> R {}
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Empty<'a> {
	set: &'a Set,
}

impl<'a> Relation for Empty<'a> {
	fn get_domain(&self) -> (&Set, &Set) {
		(&self.set, &self.set)
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		false
	}
}
impl<'a> Endorelation for Empty<'a> {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identity<'a> {
	set: &'a Set,
}

impl<'a> Relation for Identity<'a> {
	fn get_domain(&self) -> (&Set, &Set) {
		(&self.set, &self.set)
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		ix == iy
	}
}
impl<'a> Endorelation for Identity<'a> {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Universal<'a> {
	set: &'a Set,
}

impl<'a> Relation for Universal<'a> {
	fn get_domain(&self) -> (&Set, &Set) {
		(&self.set, &self.set)
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		true
	}
}
impl<'a> Endorelation for Universal<'a> {}

mod tests {
	use super::*;

	pub fn relation_property_test<R>(r: &R)
	where R: Endorelation
	{
		assert_eq!(r.is_reflexive() && r.is_irreflexive(), false);

		assert_eq!(r.is_symmetric() && r.is_antisymmetric(), false);
		assert_eq!(r.is_asymmetric(), r.is_irreflexive() && r.is_antisymmetric());

		assert_eq!(r.is_preorder(), r.is_reflexive() && r.is_transitive());
		assert_eq!(r.is_partial_order(), r.is_preorder() && r.is_antisymmetric());
		assert_eq!(r.is_equivalent(), r.is_preorder() && r.is_symmetric());
	}
}
