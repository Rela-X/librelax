//! A homogeneous, binary [`Relation`].
//!
//! This module contains the `Endorelation` type aswell as types to represent
//! empty-, the universal- and the identity-relations.

use std;

use crate::set::Set;
use crate::relation::Relation;
use crate::relation::relation::{
	Empty, Universal,
	Complement, Concatenation, Converse, Intersection, Union
};

/// cross!(1..4, a..d) = (
/// 	(1,a), (1,b), (1,c),
/// 	(2,a), (2,b), (2,c),
/// 	(3,a), (3,b), (3,c),
/// )
macro_rules! cross {
	($p:expr, $q:expr) => ($p.flat_map(
		|e| std::iter::repeat(e).zip($q)
	))
}
/// cross_uniq!(1..4, a..d) = (
/// 	     , (1,b), (1,c),
/// 	     ,      , (2,c),
/// 	     ,      ,      ,
/// )
macro_rules! cross_uniq {
	($p:expr, $q:expr) => ($p.enumerate().flat_map(
		|(i, e)| std::iter::repeat(e).zip($q.skip(i+1))
	))
}


pub trait Endorelation : Relation {
	/// Return `true` if the relation is reflexive.
	/// A relation is reflexive iff `∀x ∈ X: xRx`
	fn is_reflexive(&self) -> bool {
		debug_assert!(self.is_homogeneous());
		self.ixs().all(|i| self.eval_at(i, i))
	}
	/// Return `true` if the relation is irreflexive.
	/// A relation is irreflexive iff `∀x ∈ X: not xRx`
	///
	/// aka strict
	fn is_irreflexive(&self) -> bool {
		debug_assert!(self.is_homogeneous());
		self.ixs().all(|i| !self.eval_at(i, i))
	}
	/// Return `true` if the relation is antisymmetric.
	/// A relation is antisymmetric iff `∀x,y ∈ X: xRy ∧ yRx ⇒ x = y`
	fn is_antisymmetric(&self) -> bool {
		debug_assert!(self.is_homogeneous());
		cross_uniq!(self.ixs(), self.iys()).all(
			|(ix, iy)| !self.eval_at(ix, iy) || !self.eval_at(iy, ix)
		)
		/*
		debug_assert!(self.is_homogeneous());
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
	/// Return `true` if the relation is transitive.
	/// A relation is transitive iff `∀x,y,z ∈ X: xRy ∧ yRz ⇒ xRz`
	fn is_transitive(&self) -> bool {
		debug_assert!(self.is_homogeneous());
		cross!(self.ixs(), self.iys())
			.filter(|(ix, iy)| ix != iy)
			.filter(|(ix, iy)| self.eval_at(*ix, *iy))
			.all(
				|(ix, iy)| self.ixs()
					.filter(|&iz| self.eval_at(iy, iz))
					.all(|iz| self.eval_at(ix, iz))
			)
		/*
		debug_assert!(self.is_homogeneous());
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

	/// Return `true` if the relation is symmetric.
	/// A relation is symmetric iff `∀x,y ∈ X: xRy ⇒ yRx`
	fn is_symmetric(&self) -> bool {
		debug_assert!(self.is_homogeneous());
		cross_uniq!(self.ixs(), self.iys()).all(
			|(ix, iy)| self.eval_at(ix, iy) == self.eval_at(iy, ix)
		)
	}
	/// Return `true` if the relation is asymmetric.
	/// A relation is asymmetric if the relation is irreflexive and antisymmetric.
	fn is_asymmetric(&self) -> bool { self.is_irreflexive() && self.is_antisymmetric() }

	/// Return `true` if the relation is a pre-order.
	/// A relation is a pre-order if the relation is reflexive and transitive.
	fn is_preorder(&self) -> bool { self.is_reflexive() && self.is_transitive() }
	/// Return `true` if the relation is a partial order.
	/// A relation is a partial order if the relation is a pre-order and antisymmetric.
	fn is_partial_order(&self) -> bool { self.is_preorder() && self.is_antisymmetric() }
	/// Return `true` if the relation is equivalent.
	/// A relation is equivalent if the relation is a pre-order and symmetric.
	fn is_equivalent(&self) -> bool { self.is_preorder() && self.is_symmetric() }

	/// Return `true` if the relation is difunctional.
	/// A relation is difunctional iff `∀w,x,y,z ∈ X: xRy ∧ zRy ∧ zRw ⇒ xRw`
	///
	/// aka regular
	fn is_difunctional(&self) -> bool {
		debug_assert!(self.is_homogeneous());
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

	/// Return `true` if the relation is a lattice.
	fn is_lattice(&self) -> bool {
		debug_assert!(self.is_homogeneous());
		if !self.is_partial_order() { return false; } // TODO? Error
		// TODO
		false
	}
	/// Return `true` if the relation is a sublattice.
	fn is_sublattice<T: Endorelation>(&self, other: &T) -> bool {
		// TODO
		false
	}

	/// The identity `Relation I` where `xIy ⇔ x = y`
	fn identity<'a>(domain: (&'a Set, &'a Set)) -> Identity<'a> {
		debug_assert_eq!(domain.0, domain.1);
		Identity(domain.0)
	}

	/// Reflexive closure: `union(r, id)`
	fn closure_reflexive<R: Endorelation>(r: &R) -> Union<'_, R, Identity<'_>> {
		let id = R::identity(r.get_domain());
		return Union::new(r, id);
	}
	/// Symmetric closure: `union(r, converse(r))`
	fn closure_symmetric<R: Endorelation>(r: &R) -> Union<'_, R, Converse<'_, R>> {
		let conv = R::converse(r);
		return Union::new(r, conv);
	}
	/*
	 * Other closures are more efficiently implemented on the
	 * incidence matrices directly.
	 */
	//fn closure_difunctional<R: Endorelation>(r: &R) -> R {}
	//fn closure_biorder<R: Endorelation>(r: &R) -> R {}
}

impl Endorelation for Empty<'_> {}
impl Endorelation for Universal<'_> {}

/// The [`Identity`] `Relation`
#[derive(Clone, Debug)]
pub struct Identity<'a>(&'a Set);

impl Relation for Identity<'_> {
	fn get_domain(&self) -> (&Set, &Set) {
		(&self.0, &self.0)
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		ix == iy
	}
}
impl Endorelation for Identity<'_> {}

impl<R: Relation> Endorelation for Complement<'_, R> {}
impl<P: Relation, Q: Relation> Endorelation for Concatenation<'_, P, Q> {}
impl<R: Relation> Endorelation for Converse<'_, R> {}
impl<P: Relation, Q: Relation> Endorelation for Intersection<'_, P, Q> {}
impl<P: Relation, Q: Relation> Endorelation for Union<'_, P, Q> {}

#[cfg(test)]
pub mod tests {
	use super::*;
	use crate::relation;

	pub fn endorelation_property_test<R>(r: &R)
	where R: Endorelation + std::fmt::Debug
	{
		assert_eq!(r.is_reflexive() && r.is_irreflexive(), false);

		assert_eq!(r.is_symmetric() && r.is_asymmetric(), relation::eq(r, &R::empty(r.get_domain())));
		assert_eq!(r.is_asymmetric(), r.is_irreflexive() && r.is_antisymmetric());

		assert_eq!(r.is_preorder(), r.is_reflexive() && r.is_transitive());
		assert_eq!(r.is_partial_order(), r.is_preorder() && r.is_antisymmetric());
		assert_eq!(r.is_equivalent(), r.is_preorder() && r.is_symmetric());
	}
}
