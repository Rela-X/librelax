use crate::relation::Relation;
use crate::relation::Endorelation;
use crate::relation::relation::{Complement, Concatenation, Converse, Intersection, Union};

pub trait PartialOrder : Endorelation {
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
