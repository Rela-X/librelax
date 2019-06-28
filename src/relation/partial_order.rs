//! A partial order.
//!
//! This modue contains the `PartialOrder` type.

use crate::set::{Set, SetElement};
use crate::relation::Relation;
use crate::relation::Endorelation;
use crate::relation::relation::{Complement, Concatenation, Converse, Intersection, Union};

pub trait PartialOrder : Endorelation {
	/// Return the (principal) upset of the relation.
	/// Given a partial order `R` over the set `U` and an element `x ∈ U`,
	/// the upset is the set `{ y ∈ U | xRy }`
	fn upset(&self, x: &SetElement) -> Set {
		debug_assert!(self.is_partial_order());
		debug_assert!(self.get_domain().0.contains(x));
		let ix = self.get_domain().0.iter().position(|e| e == x).unwrap();
		self.get_domain().0.iter().enumerate()
			.filter(|&(iy, _)| self.eval_at(ix, iy))
			.map(|(_, y)| y)
			.cloned()
			.collect()
	}
	/// Return the (principal) downset of the relation.
	/// Given a partial order `R` over the set `U` and an element `x ∈ U`,
	/// the upset is the set `{ y ∈ U | yRx }`
	fn downset(&self, x: &SetElement) -> Set {
		debug_assert!(self.is_partial_order());
		debug_assert!(self.get_domain().0.contains(x));
		PartialOrder::upset(&Self::converse(self), x)
	}
	/// upr_R(u) := { y ∈ U | ∀x ∈ u: xRy }
	fn bound_upper(&self, u: &Set) -> Set {
		debug_assert!(self.is_partial_order());
		debug_assert!(u.is_subset(self.get_domain().0));
		let ixs: Vec<usize> = Set::intersection_enumerated(self.get_domain().0, u)
			.map(|((ix, _), _)| ix)
			.collect();
		self.get_domain().0.iter().enumerate()
			.filter(|&(iy, _)| ixs.iter().all(|&ix| self.eval_at(ix, iy)))
			.map(|(_, y)| y)
			.cloned()
			.collect()
	}
	/// lwr_R(u) := { y ∈ U | ∀x ∈ u: yRx }
	fn bound_lower(&self, u: &Set) -> Set {
		debug_assert!(self.is_partial_order());
		debug_assert!(u.is_subset(self.get_domain().0));
		PartialOrder::bound_upper(&Self::converse(self), u)
	}
	/// grt_R(u) := upr_R(u) ∩ u
	fn elements_greatest(&self, u: &Set) -> Set {
		debug_assert!(self.is_partial_order());
		debug_assert!(u.is_subset(self.get_domain().0));
		Set::intersection(&self.bound_upper(u), u).cloned().collect()
	}
	/// sml_R(u) := lwr_R(u) ∩ u
	fn elements_smallest(&self, u: &Set) -> Set {
		debug_assert!(self.is_partial_order());
		debug_assert!(u.is_subset(self.get_domain().0));
		Set::intersection(&self.bound_lower(u), u).cloned().collect()
	}
	/// Return the set of smallest upper boundaries.
	/// sup_R(u) := sml_R(upr_R(u))
	fn supremum(&self, u: &Set) -> Set {
		debug_assert!(self.is_partial_order());
		debug_assert!(u.is_subset(self.get_domain().0));
		self.elements_smallest(&self.bound_upper(u))
	}
	/// Return the set of greatest lower boundaries.
	/// inf_R(u) := grt_R(lwr_R(u))
	fn infimum(&self, u: &Set) -> Set {
		debug_assert!(self.is_partial_order());
		debug_assert!(u.is_subset(self.get_domain().0));
		self.elements_greatest(&self.bound_lower(u))
	}

	/// Return `true` if the relation is a lattice.
	fn is_lattice(&self) -> bool {
		debug_assert!(self.is_partial_order());
		// TODO
		false
	}
	/// Return `true` if the relation is a sublattice.
	fn is_sublattice<T: PartialOrder>(&self, other: &T) -> bool {
		debug_assert!(self.is_partial_order());
		debug_assert!(other.is_partial_order());
		// TODO
		false
	}
}

impl<R: Relation> PartialOrder for Complement<'_, R> {}
impl<P: Relation, Q: Relation> PartialOrder for Concatenation<'_, P, Q> {}
impl<R: Relation> PartialOrder for Converse<'_, R> {}
impl<P: Relation, Q: Relation> PartialOrder for Intersection<'_, P, Q> {}
impl<P: Relation, Q: Relation> PartialOrder for Union<'_, P, Q> {}

#[cfg(test)]
pub mod tests {
	use super::*;

	pub fn partial_order_property_test<PO>(po: &PO)
	where PO: PartialOrder + std::fmt::Debug
	{
//		assert_eq!(po.upset(???), po.image(???));
//		assert_eq!(po.down(???), po.preimage(???));

		// TODO [MÜLLER]
		let r = PO::closure_reflexive(po);
		for x in r.get_domain().0.iter() {
			assert!(r.upset(x).contains(x) || r.downset(x).contains(x));
		}

		/*
		for x in po.get_domain().0 {
			po.bound_upper(Set::new(x)) == po.upset(x)
			po.bound_lower(Set::new(x)) == po.downset(x)
		}
		*/
		let emptyset = Set::new();
		assert_eq!(&po.bound_upper(&emptyset), po.get_domain().0);
		assert_eq!(&po.bound_lower(&emptyset), po.get_domain().0);
		/*
		for x in po.get_domain().0 {
			po.upset(x) == ?::converse(po).downset(x)
			po.downset(x) == ?::converse(po).upset(x)
		}
		*/

		/*
		upr(u) <= upr(lwr(u))
		upr(u ∪ v) == upr(u) ∩ upr(v)
		upr(upr(u)) <= upr(u)
		*/
	}
}
