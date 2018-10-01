use relation::relation::Relation;

use set;
use relation::empty::Empty;
use relation::universal::Universal;
use relation::identity::Identity;

pub trait Endorelation : Relation {
	fn is_reflexive(&self) -> bool; /* xRx */
	fn is_irreflexive(&self) -> bool; /* aka strict */
	fn is_antisymmetric(&self) -> bool;
	fn is_transitive(&self) -> bool;

	fn is_symmetric(&self) -> bool; /* xRy <=> yRx */
	fn is_asymmetric(&self) -> bool { self.is_irreflexive() && self.is_antisymmetric() }

	fn is_preorder(&self) -> bool { self.is_reflexive() && self.is_transitive() }
	fn is_partial_order(&self) -> bool { self.is_preorder() && self.is_antisymmetric() }
	fn is_equivalent(&self) -> bool { self.is_preorder() && self.is_symmetric() }

	fn is_difunctional(&self) -> bool; /* aka regular: xRy & zRy & zRw => xRw */

	fn is_lattice(&self) -> bool; // TODO? Result
	fn is_sublattice<R: Endorelation>(&self, other: &R) -> bool; // TODO? Result

	fn empty(domain: &Vec<set::SetElement>) -> Empty {
		Empty::new(domain)
	}
	fn universal(domain: &Vec<set::SetElement>) -> Universal {
		Universal::new(domain)
	}
	fn identity(domain: &Vec<set::SetElement>) -> Identity {
		Identity::new(domain)
	}
	
	//fn closure_reflexive<R: Endorelation>(r: &R) -> Union { R::union(r, &R::identity) }
	//fn closure_symmetric<R: Endorelation>(r: &R) -> Union { R::union(r, &R::converse(r)) }
	//fn closure_difunctional<R: Endorelation>(r: &R) -> R {}
	//fn closure_biorder<R: Endorelation>(r: &R) -> R {}
}

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
